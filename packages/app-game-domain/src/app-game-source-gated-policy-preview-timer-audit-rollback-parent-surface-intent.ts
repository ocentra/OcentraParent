import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptions,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentMatchesReadModel,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-rules';

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
  optionsInput: unknown,
  auditRollbackReadModelInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.parse(auditRollbackReadModelInput);
  const rows = readModel.rows.map((row) => buildParentSurfaceIntentRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.parse({
    schemaVersion: options.schemaVersion,
    intentId: options.intentId,
    sourceAuditRollbackReadModelId: readModel.readModelId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    auditRollbackParentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision
    ).length,
    parentSurfaceIntentNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
  });
}

function buildParentSurfaceIntentRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptions,
  readModelRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow {
  const parentSurfaceIntentState = parentSurfaceIntentStateForReadModel(readModelRow);
  const parentSurfaceProofRequired =
    parentSurfaceIntentState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readModelRow.rowId}:parent-surface-intent`,
    sourceAuditRollbackReadModelRowId: readModelRow.rowId,
    sourceAuditRollbackHandoffRowId: readModelRow.sourceAuditRollbackHandoffRowId,
    sourceSchedulerPersistenceRowId: readModelRow.sourceSchedulerPersistenceRowId,
    targetDomain: readModelRow.targetDomain,
    parentSurfaceIntentState,
    serviceTimerRuntimeProofRequired: readModelRow.serviceTimerRuntimeProofRequired,
    schedulerPersistenceProofRequired: readModelRow.schedulerPersistenceProofRequired,
    schedulerStateStoreProofRequired: readModelRow.schedulerStateStoreProofRequired,
    auditTrailProofRequired: readModelRow.auditTrailProofRequired,
    rollbackPlanProofRequired: readModelRow.rollbackPlanProofRequired,
    auditRollbackReadModelProofRequired: readModelRow.auditRollbackReadModelProofRequired,
    parentSurfaceProofRequired,
    requiredProofRefs: parentSurfaceProofRequired
      ? [...readModelRow.requiredProofRefs, options.parentSurfaceProofRef]
      : readModelRow.requiredProofRefs,
    sourceEvidenceRefs: readModelRow.sourceEvidenceRefs,
    parentSurfaceDrillInRef: options.parentSurfaceDrillInRef,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function parentSurfaceIntentStateForReadModel(
  readModelRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentMatchesReadModel(
        readModelRow.readModelState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision;
}

