import { describe, expect, it } from 'vitest';
import {
  ParentOwnedSyncExportConnectorStatusRowSchema,
  ParentOwnedSyncExportContractProofReadModel,
  ParentOwnedSyncExportContractProofSchema,
  ParentOwnedSyncExportDeleteResultSchema,
  ParentOwnedSyncExportImportResultSchema,
  ParentOwnedSyncExportItemDescriptorSchema,
  ParentOwnedSyncExportKnownGaps,
  ParentOwnedSyncExportRecoveryBundleSchema,
  ParentOwnedSyncExportSyncCursorSchema,
  summarizeParentOwnedSyncExportConnectorStatuses,
  summarizeParentOwnedSyncExportDataClasses,
  summarizeParentOwnedSyncExportRecoveryBundleStates,
  summarizeParentOwnedSyncExportRecoveryHandoffStates,
} from '@ocentra-parent/schema-domain/parent-owned-sync-export';

describe('parent-owned sync export manifest contracts', () => {
  acceptsTheContractOnlySyncExportProof();
  rejectsRuntimeConnectorPortalAndCustodyOverclaims();
  rejectsUnsafeManifestItemsAndDefaultRawEvidenceUpload();
  rejectsIncoherentConnectorCursorImportAndDeleteStates();
  rejectsUnsafeRecoveryBundleAndHandoffStates();
});

function acceptsTheContractOnlySyncExportProof(): void {
  it('covers manifest data classes connector status cursor conflict import and delete boundaries', () => {
    const proof = ParentOwnedSyncExportContractProofSchema.parse(ParentOwnedSyncExportContractProofReadModel);

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
    expect(summarizeParentOwnedSyncExportConnectorStatuses(proof.connectorStatuses)).toEqual({
      ready: 1,
      revoked: 1,
      'wrong-account': 1,
      'folder-unavailable': 1,
      'partial-upload': 1,
      disabled: 1,
      'not-configured': 1,
    });
    expect(proof.syncCursors.map((cursor) => cursor.cursorState)).toEqual([
      'fresh',
      'stale',
      'missing',
      'conflict',
      'not-started',
    ]);
    expect(proof.importResults.map((result) => result.resultState)).toEqual([
      'accepted-preview',
      'rejected-schema-version',
      'rejected-scope',
      'not-applied',
    ]);
    expect(proof.deleteResults.map((result) => result.resultState)).toEqual([
      'pending',
      'confirmed',
      'failed',
      'not-requested',
    ]);
    expect(summarizeParentOwnedSyncExportRecoveryBundleStates(proof.recoveryBundles)).toEqual({
      bundleQueued: 0,
      bundleWritten: 2,
      bundleVerified: 0,
      bundlePreviewOnly: 1,
      bundleApplyPending: 1,
      bundleApplied: 1,
      bundleRejected: 0,
      bundleCorrupt: 1,
      bundleWrongHousehold: 1,
      bundleWrongKey: 1,
      bundleManualRequired: 1,
    });
    expect(summarizeParentOwnedSyncExportRecoveryHandoffStates(proof.recoveryBundles)).toEqual({
      'preview-only': 1,
      'apply-pending': 1,
      applied: 0,
      'partial-restore': 1,
      'delete-pending': 1,
      'delete-confirmed': 1,
      rejected: 3,
      'manual-required': 1,
    });
    expect(ParentOwnedSyncExportKnownGaps).toContain(
      'Parent-owned local export/delete execution remains a separate parent-domain holdout; this proof only defines delete and recovery handoff contracts.'
    );
  });
}

function rejectsRuntimeConnectorPortalAndCustodyOverclaims(): void {
  it('rejects transfer runtime connector OAuth portal report compiler and Ocentra custody claims', () => {
    const proof = ParentOwnedSyncExportContractProofReadModel;

    for (const invalidProof of [
      { ...proof, transferRuntimeClaimed: true },
      { ...proof, connectorOAuthClaimed: true },
      { ...proof, portalUiClaimed: true },
      { ...proof, reportCompilerRuntimeClaimed: true },
      { ...proof, accountSubscriptionBackendClaimed: true },
      { ...proof, ocentraHostedChildEvidenceStored: true },
      { ...proof, nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-default-ocentra-custody') },
    ]) {
      expect(ParentOwnedSyncExportContractProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsUnsafeManifestItemsAndDefaultRawEvidenceUpload(): void {
  it('rejects item descriptors that upload raw evidence by default or use unsafe custody and format states', () => {
    const journalItem = itemFor('encrypted-journal-segment');
    const summaryItem = itemFor('generated-summary');

    for (const invalidItem of [
      { ...journalItem, rawChildEvidenceUploadedByDefault: true },
      { ...journalItem, ocentraHostedFamilyDataStored: true },
      { ...journalItem, transferRuntimeClaimed: true },
      { ...journalItem, parentActionRequired: false },
      { ...journalItem, exportFormat: 'human-readable-parent-report' },
      { ...summaryItem, destinationOwnership: 'ocentra-hosted-non-activity-metadata' },
      {
        ...journalItem,
        retention: { ...journalItem.retention, parentActionRequired: false },
      },
    ]) {
      expect(ParentOwnedSyncExportItemDescriptorSchema.safeParse(invalidItem).success).toBe(false);
    }
  });
}

function rejectsIncoherentConnectorCursorImportAndDeleteStates(): void {
  it('rejects connector cursor import and delete rows that omit required refs or apply untrusted data', () => {
    const readyConnector = connectorFor('ready');
    const conflictCursor = cursorFor('conflict');
    const acceptedImport = importResultFor('accepted-preview');
    const pendingDelete = deleteResultFor('pending');

    expect(
      ParentOwnedSyncExportConnectorStatusRowSchema.safeParse({
        ...readyConnector,
        accountRef: null,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportConnectorStatusRowSchema.safeParse({
        ...readyConnector,
        oauthRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportSyncCursorSchema.safeParse({
        ...conflictCursor,
        conflictRefs: [],
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportImportResultSchema.safeParse({
        ...acceptedImport,
        appliedToLocalEvidence: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportDeleteResultSchema.safeParse({
        ...pendingDelete,
        deleteRequestRef: null,
      }).success
    ).toBe(false);
  });
}

function rejectsUnsafeRecoveryBundleAndHandoffStates(): void {
  it('rejects recovery bundles that mutate preview state or omit required rejection and delete refs', () => {
    const previewBundle = recoveryBundleFor('bundlePreviewOnly');
    const wrongHouseholdBundle = recoveryBundleFor('bundleWrongHousehold');
    const deletePendingBundle = recoveryBundleFor('delete-pending');

    expect(
      ParentOwnedSyncExportRecoveryBundleSchema.safeParse({
        ...previewBundle,
        previewMutatedLocalTruth: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportRecoveryBundleSchema.safeParse({
        ...previewBundle,
        applyConfirmedByParent: true,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportRecoveryBundleSchema.safeParse({
        ...wrongHouseholdBundle,
        rejectionReasonRef: null,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportRecoveryBundleSchema.safeParse({
        ...deletePendingBundle,
        deleteRequestRef: null,
      }).success
    ).toBe(false);
    expect(
      ParentOwnedSyncExportRecoveryBundleSchema.safeParse({
        ...deletePendingBundle,
        handoff: {
          ...deletePendingBundle.handoff,
          handoffTarget: 'setup-restore-preview',
        },
      }).success
    ).toBe(false);
  });
}

function itemFor(dataClass: 'encrypted-journal-segment' | 'generated-summary') {
  const item = ParentOwnedSyncExportContractProofReadModel.manifest.items.find(
    (candidate) => candidate.dataClass === dataClass
  );
  if (item === undefined) {
    throw new Error(`missing sync export item: ${dataClass}`);
  }
  return item;
}

function connectorFor(status: 'ready') {
  const row = ParentOwnedSyncExportContractProofReadModel.connectorStatuses.find(
    (candidate) => candidate.status === status
  );
  if (row === undefined) {
    throw new Error(`missing sync export connector status: ${status}`);
  }
  return row;
}

function cursorFor(cursorState: 'conflict') {
  const cursor = ParentOwnedSyncExportContractProofReadModel.syncCursors.find(
    (candidate) => candidate.cursorState === cursorState
  );
  if (cursor === undefined) {
    throw new Error(`missing sync export cursor: ${cursorState}`);
  }
  return cursor;
}

function importResultFor(resultState: 'accepted-preview') {
  const result = ParentOwnedSyncExportContractProofReadModel.importResults.find(
    (candidate) => candidate.resultState === resultState
  );
  if (result === undefined) {
    throw new Error(`missing sync export import result: ${resultState}`);
  }
  return result;
}

function deleteResultFor(resultState: 'pending') {
  const result = ParentOwnedSyncExportContractProofReadModel.deleteResults.find(
    (candidate) => candidate.resultState === resultState
  );
  if (result === undefined) {
    throw new Error(`missing sync export delete result: ${resultState}`);
  }
  return result;
}

function recoveryBundleFor(state: 'bundlePreviewOnly' | 'bundleWrongHousehold' | 'delete-pending') {
  const bundle = ParentOwnedSyncExportContractProofReadModel.recoveryBundles.find((candidate) =>
    state === 'delete-pending' ? candidate.handoff.handoffState === state : candidate.bundleState === state
  );
  if (bundle === undefined) {
    throw new Error(`missing sync export recovery bundle: ${state}`);
  }
  return bundle;
}
