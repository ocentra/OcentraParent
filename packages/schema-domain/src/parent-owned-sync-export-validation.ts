import type {
  ParentOwnedSyncExportConnectorStatus,
  ParentOwnedSyncExportConnectorStatusRow,
  ParentOwnedSyncExportContractProofCandidate,
  ParentOwnedSyncExportDeleteResult,
  ParentOwnedSyncExportDeleteResultState,
  ParentOwnedSyncExportImportResult,
  ParentOwnedSyncExportImportResultState,
  ParentOwnedSyncExportItemDescriptor,
  ParentOwnedSyncExportNonClaim,
  ParentOwnedSyncExportRecoveryBundleCandidate,
  ParentOwnedSyncExportRecoveryBundleState,
  ParentOwnedSyncExportRecoveryHandoffCandidate,
  ParentOwnedSyncExportRecoveryHandoffState,
  ParentOwnedSyncExportSyncCursor,
  ParentOwnedSyncExportSyncCursorState,
} from './parent-owned-sync-export';

const RequiredDataClasses = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const;

const RequiredNonClaims = [
  'no-transfer-runtime',
  'no-connector-oauth',
  'no-portal-ui',
  'no-default-ocentra-custody',
  'no-raw-child-evidence-upload-by-default',
  'no-report-compiler-runtime',
  'no-account-subscription-backend',
] as const;

export function syncExportRecoveryHandoffIsHonest(handoff: ParentOwnedSyncExportRecoveryHandoffCandidate): boolean {
  return (
    syncExportRecoveryHandoffPreservesTruth(handoff) &&
    syncExportRecoveryHandoffConfirmationIsHonest(handoff) &&
    syncExportRecoveryHandoffDeleteFlowIsHonest(handoff) &&
    syncExportRecoveryHandoffTargetIsHonest(handoff)
  );
}

export function syncExportRecoveryBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  const stateValidator = syncExportRecoveryBundleStateValidators[bundle.bundleState];
  return (
    syncExportRecoveryBundleHasSafePreview(bundle) &&
    syncExportRecoveryBundleDeleteFlowIsHonest(bundle) &&
    stateValidator !== undefined &&
    stateValidator(bundle)
  );
}

export function syncExportContractProofIsHonest(proof: ParentOwnedSyncExportContractProofCandidate): boolean {
  return (
    syncExportProofHasRequiredNonClaims(proof.nonClaims) &&
    syncExportProofCoversConnectorStatuses(proof.connectorStatuses) &&
    syncExportProofCoversCursorStates(proof.syncCursors) &&
    syncExportProofCoversImportAndDeleteResults(proof.importResults, proof.deleteResults) &&
    syncExportProofCoversRecoveryBundles(proof.recoveryBundles) &&
    !proof.transferRuntimeClaimed &&
    !proof.connectorOAuthClaimed &&
    !proof.portalUiClaimed &&
    !proof.reportCompilerRuntimeClaimed &&
    !proof.accountSubscriptionBackendClaimed &&
    !proof.ocentraHostedChildEvidenceStored
  );
}

function syncExportRecoveryHandoffPreservesTruth(handoff: ParentOwnedSyncExportRecoveryHandoffCandidate): boolean {
  return handoff.previewIsNonMutating && handoff.sourceOfTruthPreserved && handoff.tombstonesPreserved;
}

function syncExportRecoveryHandoffConfirmationIsHonest(
  handoff: ParentOwnedSyncExportRecoveryHandoffCandidate
): boolean {
  const confirmationStates: readonly ParentOwnedSyncExportRecoveryHandoffState[] = [
    'preview-only',
    'apply-pending',
    'applied',
    'partial-restore',
  ];
  if (confirmationStates.includes(handoff.handoffState)) {
    return handoff.explicitParentConfirmationRequired && !handoff.deleteRequestRequired;
  }

  return true;
}

function syncExportRecoveryHandoffDeleteFlowIsHonest(handoff: ParentOwnedSyncExportRecoveryHandoffCandidate): boolean {
  const deleteStates: readonly ParentOwnedSyncExportRecoveryHandoffState[] = ['delete-pending', 'delete-confirmed'];
  if (deleteStates.includes(handoff.handoffState)) {
    return (
      handoff.handoffTarget === 'parent-local-delete-runtime' &&
      handoff.deleteRequestRequired &&
      !handoff.explicitParentConfirmationRequired
    );
  }

  return !handoff.deleteRequestRequired;
}

function syncExportRecoveryHandoffTargetIsHonest(handoff: ParentOwnedSyncExportRecoveryHandoffCandidate): boolean {
  if (handoff.handoffTarget !== 'parent-local-delete-runtime') {
    return handoff.handoffState !== 'delete-pending' && handoff.handoffState !== 'delete-confirmed';
  }

  return (
    handoff.handoffState === 'delete-pending' ||
    handoff.handoffState === 'delete-confirmed' ||
    handoff.handoffState === 'manual-required'
  );
}

function syncExportRecoveryBundleHasSafePreview(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return bundle.auditRefs.length > 0 && !bundle.previewMutatedLocalTruth;
}

function syncExportRecoveryBundleDeleteFlowIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  const deleteStates: readonly ParentOwnedSyncExportRecoveryHandoffState[] = ['delete-pending', 'delete-confirmed'];
  if (deleteStates.includes(bundle.handoff.handoffState)) {
    return bundle.deleteRequestRef !== null && bundle.handoff.handoffTarget === 'parent-local-delete-runtime';
  }

  return bundle.deleteRequestRef === null;
}

const syncExportRecoveryBundleStateValidators: Record<
  ParentOwnedSyncExportRecoveryBundleState,
  (bundle: ParentOwnedSyncExportRecoveryBundleCandidate) => boolean
> = {
  bundleQueued: syncExportQueuedWrittenOrVerifiedBundleIsHonest,
  bundleWritten: syncExportQueuedWrittenOrVerifiedBundleIsHonest,
  bundleVerified: syncExportQueuedWrittenOrVerifiedBundleIsHonest,
  bundlePreviewOnly: syncExportPreviewOnlyBundleIsHonest,
  bundleApplyPending: syncExportApplyPendingBundleIsHonest,
  bundleApplied: syncExportAppliedBundleIsHonest,
  bundleRejected: syncExportRejectedBundleIsHonest,
  bundleCorrupt: syncExportCorruptBundleIsHonest,
  bundleWrongHousehold: syncExportWrongHouseholdBundleIsHonest,
  bundleWrongKey: syncExportWrongKeyBundleIsHonest,
  bundleManualRequired: syncExportManualRequiredBundleIsHonest,
};

function syncExportQueuedWrittenOrVerifiedBundleIsHonest(
  bundle: ParentOwnedSyncExportRecoveryBundleCandidate
): boolean {
  return bundle.sourceHouseholdBindingState === 'matched';
}

function syncExportWrongHouseholdBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.sourceHouseholdBindingState === 'mismatched' &&
    bundle.handoff.handoffState === 'rejected' &&
    bundle.rejectionReasonRef !== null &&
    bundle.acceptedDataClasses.length === 0 &&
    !bundle.applyConfirmedByParent
  );
}

function syncExportWrongKeyBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.keyAvailabilityState === 'wrong-key' &&
    bundle.handoff.handoffState === 'rejected' &&
    bundle.rejectionReasonRef !== null &&
    bundle.acceptedDataClasses.length === 0 &&
    !bundle.applyConfirmedByParent
  );
}

function syncExportCorruptBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.handoff.handoffState === 'rejected' &&
    bundle.rejectionReasonRef !== null &&
    bundle.acceptedDataClasses.length === 0 &&
    !bundle.applyConfirmedByParent
  );
}

function syncExportManualRequiredBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.handoff.handoffState === 'manual-required' &&
    bundle.rejectionReasonRef !== null &&
    !bundle.applyConfirmedByParent
  );
}

function syncExportPreviewOnlyBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.handoff.handoffState === 'preview-only' &&
    bundle.acceptedDataClasses.length > 0 &&
    !bundle.applyConfirmedByParent &&
    bundle.rejectionReasonRef === null
  );
}

function syncExportApplyPendingBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return (
    bundle.handoff.handoffState === 'apply-pending' &&
    bundle.acceptedDataClasses.length > 0 &&
    !bundle.applyConfirmedByParent &&
    bundle.rejectionReasonRef === null
  );
}

function syncExportAppliedBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  if (!bundle.applyConfirmedByParent || bundle.rejectionReasonRef !== null) {
    return false;
  }

  return bundle.handoff.handoffState === 'applied'
    ? bundle.rejectedDataClasses.length === 0
    : bundle.handoff.handoffState === 'partial-restore' && bundle.rejectedDataClasses.length > 0;
}

function syncExportRejectedBundleIsHonest(bundle: ParentOwnedSyncExportRecoveryBundleCandidate): boolean {
  return bundle.handoff.handoffState === 'rejected' && bundle.rejectionReasonRef !== null;
}

function syncExportProofHasRequiredNonClaims(nonClaims: readonly ParentOwnedSyncExportNonClaim[]): boolean {
  const claims = new Set(nonClaims);
  return claims.size === nonClaims.length && RequiredNonClaims.every((claim) => claims.has(claim));
}

function syncExportProofCoversConnectorStatuses(rows: readonly ParentOwnedSyncExportConnectorStatusRow[]): boolean {
  const statuses = new Set(rows.map((row) => row.status));
  return [
    'ready',
    'revoked',
    'wrong-account',
    'folder-unavailable',
    'partial-upload',
    'disabled',
    'not-configured',
  ].every((status) => statuses.has(status as ParentOwnedSyncExportConnectorStatus));
}

function syncExportProofCoversCursorStates(cursors: readonly ParentOwnedSyncExportSyncCursor[]): boolean {
  const states = new Set(cursors.map((cursor) => cursor.cursorState));
  return ['fresh', 'stale', 'missing', 'conflict', 'not-started'].every((state) =>
    states.has(state as ParentOwnedSyncExportSyncCursorState)
  );
}

function syncExportProofCoversImportAndDeleteResults(
  importResults: readonly ParentOwnedSyncExportImportResult[],
  deleteResults: readonly ParentOwnedSyncExportDeleteResult[]
): boolean {
  const importStates = new Set(importResults.map((result) => result.resultState));
  const deleteStates = new Set(deleteResults.map((result) => result.resultState));
  return (
    ['accepted-preview', 'rejected-schema-version', 'rejected-scope', 'not-applied'].every((state) =>
      importStates.has(state as ParentOwnedSyncExportImportResultState)
    ) &&
    ['pending', 'confirmed', 'failed', 'not-requested'].every((state) =>
      deleteStates.has(state as ParentOwnedSyncExportDeleteResultState)
    )
  );
}

function syncExportProofCoversRecoveryBundles(
  recoveryBundles: readonly ParentOwnedSyncExportRecoveryBundleCandidate[]
): boolean {
  const bundleStates = new Set(recoveryBundles.map((bundle) => bundle.bundleState));
  const handoffStates = new Set(recoveryBundles.map((bundle) => bundle.handoff.handoffState));
  const handoffTargets = new Set(recoveryBundles.map((bundle) => bundle.handoff.handoffTarget));
  return (
    [
      'bundlePreviewOnly',
      'bundleApplyPending',
      'bundleApplied',
      'bundleCorrupt',
      'bundleWrongHousehold',
      'bundleWrongKey',
      'bundleManualRequired',
    ].every((state) => bundleStates.has(state as ParentOwnedSyncExportRecoveryBundleState)) &&
    [
      'preview-only',
      'apply-pending',
      'partial-restore',
      'delete-pending',
      'delete-confirmed',
      'rejected',
      'manual-required',
    ].every((state) => handoffStates.has(state as ParentOwnedSyncExportRecoveryHandoffState)) &&
    ['setup-restore-preview', 'device-trust-recovery-persistence', 'parent-local-delete-runtime'].every((target) =>
      handoffTargets.has(target as ParentOwnedSyncExportRecoveryHandoffCandidate['handoffTarget'])
    )
  );
}

export function syncExportCoversRequiredDataClasses(
  items: ReadonlyArray<ParentOwnedSyncExportItemDescriptor>
): boolean {
  const covered = new Set(items.map((item) => item.dataClass));
  return RequiredDataClasses.every((dataClass) => covered.has(dataClass));
}
