/* generated from crates/schema/src/parent_owned_sync_export.rs */

import type {
  GeneratedParentOwnedSyncExportContractProof,
  GeneratedParentOwnedSyncExportManifestItem,
  GeneratedParentOwnedSyncExportNonClaim,
  GeneratedParentOwnedSyncProviderStatus,
  GeneratedParentOwnedSyncProviderStatusRow,
  GeneratedParentOwnedSyncState,
  GeneratedParentOwnedSyncStateRow,
  GeneratedParentOwnedSyncTombstonePropagationState,
  GeneratedParentOwnedSyncTombstoneRow,
} from './generated-parent-owned-sync-export-contracts';

const requiredDataClasses = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const;

const requiredNonClaims = [
  'no-transfer-runtime',
  'no-connector-oauth',
  'no-upload-runtime',
  'no-delete-runtime',
  'no-default-ocentra-custody',
  'no-raw-child-evidence-upload-by-default',
] as const;

export function syncExportManifestItemIsHonestGenerated(item: GeneratedParentOwnedSyncExportManifestItem): boolean {
  const encryptionMatchesFormat = {
    'encrypted-machine-readable':
      item.encryption.encryptionState === 'encrypted-at-rest' && item.encryption.encryptedBeforeUpload,
    'encrypted-support-bundle':
      item.encryption.encryptionState === 'encrypted-at-rest' && item.encryption.encryptedBeforeUpload,
    'human-readable-parent-report':
      item.encryption.encryptionState === 'human-readable-parent-authorized' && !item.encryption.encryptedBeforeUpload,
  } as const;

  return (
    item.parentActionRequired &&
    !item.rawChildEvidenceUploadedByDefault &&
    !item.ocentraHostedFamilyDataStored &&
    item.claimSafe &&
    item.destinationOwnership !== 'ocentra-hosted-non-activity-metadata' &&
    encryptionMatchesFormat[item.exportFormat]
  );
}

export function syncExportProviderStatusRowIsHonestGenerated(row: GeneratedParentOwnedSyncProviderStatusRow): boolean {
  const statusChecks = {
    ready: row.accountRef !== null && row.folderRef !== null,
    'manual-required':
      row.disconnectVisibilityState === 'manual-required' || row.deleteVisibilityState === 'manual-required',
    revoked: row.revocationRef !== null,
    'wrong-account': true,
    'folder-unavailable': true,
    'partial-upload': true,
    disconnected: row.disconnectVisibilityState === 'disconnect-visible',
    disabled: row.accountRef === null && row.folderRef === null && row.revocationRef === null,
    'not-configured': row.accountRef === null && row.folderRef === null && row.revocationRef === null,
  } as const satisfies Record<GeneratedParentOwnedSyncProviderStatus, boolean>;

  return (
    row.destinationOwnership !== 'ocentra-hosted-non-activity-metadata' &&
    !row.oauthRuntimeClaimed &&
    !row.uploadRuntimeClaimed &&
    !row.deleteRuntimeClaimed &&
    !row.ocentraHostedFamilyDataStored &&
    row.claimSafe &&
    statusChecks[row.providerStatus]
  );
}

export function syncExportSyncStateRowIsHonestGenerated(row: GeneratedParentOwnedSyncStateRow): boolean {
  const stateChecks = {
    synced:
      row.cursorRef !== null &&
      row.batchRef !== null &&
      row.manifestChecksumRef !== null &&
      row.manifestSignatureRef !== null &&
      row.lastSuccessfulSyncAt !== null &&
      row.manifestIntegrityState !== 'corrupt' &&
      row.manifestIntegrityState !== 'not-applicable',
    stale:
      row.cursorRef !== null &&
      row.batchRef !== null &&
      row.manifestChecksumRef !== null &&
      row.manifestSignatureRef !== null &&
      row.lastSuccessfulSyncAt !== null &&
      row.manifestIntegrityState !== 'corrupt' &&
      row.manifestIntegrityState !== 'not-applicable',
    missing: row.retryQueueRef !== null,
    conflict: row.conflictRef !== null && row.retryQueueRef !== null,
    'offline-retry-pending': row.retryQueueRef !== null,
    'partial-outage': row.retryQueueRef !== null,
    'manual-required': row.retryQueueRef !== null,
    'not-started':
      row.cursorRef === null &&
      row.batchRef === null &&
      row.manifestChecksumRef === null &&
      row.manifestSignatureRef === null &&
      row.lastSuccessfulSyncAt === null &&
      row.conflictRef === null &&
      row.retryQueueRef === null &&
      row.manifestIntegrityState === 'not-applicable',
  } as const satisfies Record<GeneratedParentOwnedSyncState, boolean>;

  return row.claimSafe && stateChecks[row.syncState];
}

export function syncExportTombstoneRowIsHonestGenerated(row: GeneratedParentOwnedSyncTombstoneRow): boolean {
  const stateChecks = {
    'not-requested': row.deleteRequestRef === null && row.lastPropagatedAt === null && row.blockedReasonRef === null,
    pending: row.deleteRequestRef !== null,
    propagated: row.deleteRequestRef !== null && row.lastPropagatedAt !== null,
    blocked: row.deleteRequestRef !== null && row.blockedReasonRef !== null,
    'manual-required': row.deleteRequestRef !== null && row.blockedReasonRef !== null,
  } as const satisfies Record<GeneratedParentOwnedSyncTombstonePropagationState, boolean>;

  return row.claimSafe && stateChecks[row.propagationState];
}

export function syncExportContractProofIsHonestGenerated(proof: GeneratedParentOwnedSyncExportContractProof): boolean {
  return (
    hasRequiredNonClaimsGenerated(proof.nonClaims) &&
    proof.manifest.items.every(syncExportManifestItemIsHonestGenerated) &&
    proof.providerStatuses.every(syncExportProviderStatusRowIsHonestGenerated) &&
    proof.syncStates.every(syncExportSyncStateRowIsHonestGenerated) &&
    proof.tombstones.every(syncExportTombstoneRowIsHonestGenerated) &&
    !proof.transferRuntimeClaimed &&
    !proof.connectorOAuthClaimed &&
    !proof.uploadRuntimeClaimed &&
    !proof.deleteRuntimeClaimed &&
    !proof.ocentraHostedChildEvidenceStored
  );
}

export function syncExportCoversRequiredDataClassesGenerated(
  items: ReadonlyArray<GeneratedParentOwnedSyncExportManifestItem>
): boolean {
  const covered = new Set(items.map((item) => item.dataClass));
  return requiredDataClasses.every((dataClass) => covered.has(dataClass));
}

function hasRequiredNonClaimsGenerated(nonClaims: readonly GeneratedParentOwnedSyncExportNonClaim[]): boolean {
  const claims = new Set(nonClaims);
  return claims.size === nonClaims.length && requiredNonClaims.every((claim) => claims.has(claim));
}
