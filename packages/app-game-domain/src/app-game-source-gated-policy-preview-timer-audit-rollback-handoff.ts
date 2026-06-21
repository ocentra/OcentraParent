import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema,
  type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffMatchesSchedulerPersistence,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-handoff-rules';

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
  optionsInput: unknown,
  schedulerPersistenceInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptionsSchema.parse(optionsInput);
  const schedulerPersistence =
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.parse(schedulerPersistenceInput);
  const rows = schedulerPersistence.rows.map((row) => buildAuditRollbackHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceSchedulerPersistenceId: schedulerPersistence.persistenceId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: schedulerPersistence.nativeAppRowCount,
    nativeGameRowCount: schedulerPersistence.nativeGameRowCount,
    auditRollbackProofRequiredCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision
    ).length,
    auditRollbackNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
  });
}

function buildAuditRollbackHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions,
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow {
  const auditRollbackState = auditRollbackStateForSchedulerPersistence(schedulerPersistenceRow);
  const proofRequired =
    auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${schedulerPersistenceRow.rowId}:audit-rollback-handoff`,
    sourceSchedulerPersistenceRowId: schedulerPersistenceRow.rowId,
    targetDomain: schedulerPersistenceRow.targetDomain,
    auditRollbackState,
    serviceTimerRuntimeProofRequired: proofRequired,
    schedulerPersistenceProofRequired: proofRequired,
    schedulerStateStoreProofRequired: proofRequired,
    auditTrailProofRequired: proofRequired,
    rollbackPlanProofRequired: proofRequired,
    auditRollbackReadModelProofRequired: proofRequired,
    requiredProofRefs: requiredProofRefsForAuditRollback(options, auditRollbackState, schedulerPersistenceRow),
    sourceEvidenceRefs: schedulerPersistenceRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function auditRollbackStateForSchedulerPersistence(
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffMatchesSchedulerPersistence(
        schedulerPersistenceRow.schedulerPersistenceState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision;
}

function requiredProofRefsForAuditRollback(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions,
  auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue,
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
) {
  if (auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired) {
    return [
      options.serviceTimerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.schedulerStateStoreProofRef,
      options.auditTrailProofRef,
      options.rollbackPlanProofRef,
      options.auditRollbackReadModelProofRef,
    ];
  }
  return schedulerPersistenceRow.requiredProofRefs;
}

