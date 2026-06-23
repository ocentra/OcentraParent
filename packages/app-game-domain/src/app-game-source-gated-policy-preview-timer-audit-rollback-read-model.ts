import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptions,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelMatchesHandoff,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules';

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
  optionsInput: unknown,
  auditRollbackHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptionsSchema.parse(optionsInput);
  const handoff = AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.parse(auditRollbackHandoffInput);
  const rows = handoff.rows.map((row) => buildAuditRollbackReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    readModelId: options.readModelId,
    sourceAuditRollbackHandoffId: handoff.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    parentVisibleSummaryRef: options.parentVisibleSummaryRef,
    rows,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    auditRollbackReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.readModelState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision
    ).length,
    auditRollbackReadModelNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
  });
}

function buildAuditRollbackReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptions,
  handoffRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow {
  const readModelState = readModelStateForAuditRollbackHandoff(handoffRow);

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${handoffRow.rowId}:audit-rollback-read-model`,
    sourceAuditRollbackHandoffRowId: handoffRow.rowId,
    sourceSchedulerPersistenceRowId: handoffRow.sourceSchedulerPersistenceRowId,
    targetDomain: handoffRow.targetDomain,
    readModelState,
    serviceTimerRuntimeProofRequired: handoffRow.serviceTimerRuntimeProofRequired,
    schedulerPersistenceProofRequired: handoffRow.schedulerPersistenceProofRequired,
    schedulerStateStoreProofRequired: handoffRow.schedulerStateStoreProofRequired,
    auditTrailProofRequired: handoffRow.auditTrailProofRequired,
    rollbackPlanProofRequired: handoffRow.rollbackPlanProofRequired,
    auditRollbackReadModelProofRequired: handoffRow.auditRollbackReadModelProofRequired,
    requiredProofRefs: handoffRow.requiredProofRefs,
    sourceEvidenceRefs: handoffRow.sourceEvidenceRefs,
    parentVisibleSummaryRef: options.parentVisibleSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function readModelStateForAuditRollbackHandoff(
  handoffRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelMatchesHandoff(handoffRow.auditRollbackState, state)
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision;
}
