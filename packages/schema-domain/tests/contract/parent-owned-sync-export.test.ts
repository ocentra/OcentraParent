import { describe, expect, it } from 'vitest';
import {
  ParentOwnedSyncExportContractProofReadModel,
  ParentOwnedSyncExportContractProofSchema,
  ParentOwnedSyncExportKnownGaps,
  ParentOwnedSyncExportManifestItemSchema,
  ParentOwnedSyncExportProviderStatusRowSchema,
  ParentOwnedSyncExportSyncStateRowSchema,
  ParentOwnedSyncExportTombstoneRowSchema,
  RequiredParentOwnedSyncExportProviderModes,
  RequiredParentOwnedSyncExportSyncStates,
  RequiredParentOwnedSyncExportTombstoneStates,
  summarizeParentOwnedSyncExportDataClasses,
  summarizeParentOwnedSyncExportProviderModes,
  summarizeParentOwnedSyncExportProviderStatuses,
  summarizeParentOwnedSyncExportSyncStates,
  summarizeParentOwnedSyncExportTombstoneStates,
} from '@ocentra-parent/schema-domain/parent-owned-sync-export';

describe('parent-owned sync export manifest contracts', () => {
  providerModeSplitAndStatusMatrixProof();
  encryptionBeforeUploadAndNoDefaultCustodyProof();
  syncStatesAndManifestIntegrityProof();
  tombstonePropagationStaysSeparateFromSyncSuccessProof();
  runtimeNonClaimsStayExplicitProof();
});

function providerModeSplitAndStatusMatrixProof(): void {
  it('covers every provider mode and required provider status without hiding manual-required or unsupported edges', () => {
    const proof = ParentOwnedSyncExportContractProofSchema.parse(ParentOwnedSyncExportContractProofReadModel);

    expect(Object.keys(summarizeParentOwnedSyncExportProviderModes(proof.providerStatuses))).toEqual(
      RequiredParentOwnedSyncExportProviderModes
    );
    expect(summarizeParentOwnedSyncExportProviderStatuses(proof.providerStatuses)).toMatchObject({
      ready: 4,
      'manual-required': 1,
      revoked: 1,
      'wrong-account': 1,
      'folder-unavailable': 1,
      'partial-upload': 1,
      disconnected: 1,
      disabled: 1,
      'not-configured': 0,
    });
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'manual-required')).toBe(true);
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'revoked')).toBe(true);
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'wrong-account')).toBe(true);
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'folder-unavailable')).toBe(true);
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'partial-upload')).toBe(true);
    expect(proof.providerStatuses.some((row) => row.providerStatus === 'disconnected')).toBe(true);
  });
}

function encryptionBeforeUploadAndNoDefaultCustodyProof(): void {
  it('keeps manifest data classes explicit, encrypts machine-readable payloads before upload, and rejects hosted evidence storage', () => {
    const proof = ParentOwnedSyncExportContractProofReadModel;

    expect(summarizeParentOwnedSyncExportDataClasses(proof.manifest.items)).toEqual({
      'encrypted-journal-segment': 1,
      'sqlite-query-row': 1,
      'parent-rule': 1,
      'approval-decision': 1,
      'device-registry-entry': 1,
      'notification-history': 1,
      'audit-event': 1,
      'generated-summary': 1,
    });
    expect(
      proof.manifest.items.every((item) =>
        item.exportFormat === 'human-readable-parent-report' ? !item.encryption.encryptedBeforeUpload : item.encryption.encryptedBeforeUpload
      )
    ).toBe(true);
    expect(
      proof.manifest.items.every(
        (item) => !item.rawChildEvidenceUploadedByDefault && !item.ocentraHostedFamilyDataStored && item.claimSafe
      )
    ).toBe(true);
    expect(ParentOwnedSyncExportKnownGaps).toContain(
      'Ocentra-hosted cloud metadata is not the default evidence store and no raw child evidence upload is claimed by default.'
    );

    const invalidItem = {
      ...proof.manifest.items[0],
      rawChildEvidenceUploadedByDefault: true,
    };
    expect(ParentOwnedSyncExportManifestItemSchema.safeParse(invalidItem).success).toBe(false);
  });
}

function syncStatesAndManifestIntegrityProof(): void {
  it('keeps sync state, checksum, signature, corruption, retry, and manual-required boundaries explicit', () => {
    const proof = ParentOwnedSyncExportContractProofReadModel;

    expect(Object.keys(summarizeParentOwnedSyncExportSyncStates(proof.syncStates))).toEqual(
      RequiredParentOwnedSyncExportSyncStates
    );
    expect(proof.syncStates.map((row) => row.syncState)).toEqual(RequiredParentOwnedSyncExportSyncStates);
    expect(proof.syncStates.find((row) => row.syncState === 'manual-required')?.manifestIntegrityState).toBe('corrupt');
    expect(proof.syncStates.find((row) => row.syncState === 'missing')?.manifestIntegrityState).toBe('mismatch');
    expect(proof.syncStates.find((row) => row.syncState === 'conflict')?.conflictRef).not.toBeNull();
    expect(proof.syncStates.find((row) => row.syncState === 'offline-retry-pending')?.retryQueueRef).not.toBeNull();

    const invalidSynced = {
      ...proof.syncStates.find((row) => row.syncState === 'synced')!,
      manifestSignatureRef: null,
    };
    expect(ParentOwnedSyncExportSyncStateRowSchema.safeParse(invalidSynced).success).toBe(false);
  });
}

function tombstonePropagationStaysSeparateFromSyncSuccessProof(): void {
  it('shows tombstone propagation as its own matrix rather than folding it into sync success', () => {
    const proof = ParentOwnedSyncExportContractProofReadModel;

    expect(Object.keys(summarizeParentOwnedSyncExportTombstoneStates(proof.tombstones))).toEqual(
      RequiredParentOwnedSyncExportTombstoneStates
    );
    expect(proof.tombstones.map((row) => row.propagationState)).toEqual(RequiredParentOwnedSyncExportTombstoneStates);
    expect(proof.tombstones.find((row) => row.propagationState === 'propagated')?.lastPropagatedAt).not.toBeNull();
    expect(proof.tombstones.find((row) => row.propagationState === 'blocked')?.blockedReasonRef).not.toBeNull();
    expect(proof.syncStates.some((row) => row.syncState === 'synced')).toBe(true);
    expect(proof.tombstones.some((row) => row.propagationState === 'blocked')).toBe(true);

    const invalidBlocked = {
      ...proof.tombstones.find((row) => row.propagationState === 'blocked')!,
      blockedReasonRef: null,
    };
    expect(ParentOwnedSyncExportTombstoneRowSchema.safeParse(invalidBlocked).success).toBe(false);
  });
}

function runtimeNonClaimsStayExplicitProof(): void {
  it('rejects oauth upload delete and hosted evidence runtime overclaims while keeping delete and disconnect visibility explicit', () => {
    const proof = ParentOwnedSyncExportContractProofReadModel;
    const disconnected = proof.providerStatuses.find((row) => row.providerStatus === 'disconnected');
    const revoked = proof.providerStatuses.find((row) => row.providerStatus === 'revoked');

    expect(disconnected?.disconnectVisibilityState).toBe('disconnect-visible');
    expect(revoked?.revocationRef).not.toBeNull();
    expect(proof.transferRuntimeClaimed).toBe(false);
    expect(proof.connectorOAuthClaimed).toBe(false);
    expect(proof.uploadRuntimeClaimed).toBe(false);
    expect(proof.deleteRuntimeClaimed).toBe(false);
    expect(proof.ocentraHostedChildEvidenceStored).toBe(false);

    expect(
      ParentOwnedSyncExportProviderStatusRowSchema.safeParse({
        ...disconnected!,
        disconnectVisibilityState: 'not-disconnected',
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportContractProofSchema.safeParse({
        ...proof,
        deleteRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
}
