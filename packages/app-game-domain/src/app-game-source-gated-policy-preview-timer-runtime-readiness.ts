import {
  AppGameSourceGatedPolicyPreviewTimerStatusSchema,
  type AppGameSourceGatedPolicyPreviewTimerStatusRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-status';
import {
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema,
  type AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness,
  type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
  type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-runtime-readiness';
import {
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState,
  RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims,
  appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatus,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-runtime-readiness-rules';

export function buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(
  optionsInput: unknown,
  timerStatusInput: unknown
): AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness {
  const options = AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptionsSchema.parse(optionsInput);
  const timerStatus = AppGameSourceGatedPolicyPreviewTimerStatusSchema.parse(timerStatusInput);
  const rows = timerStatus.rows.map((row) => buildRuntimeReadinessRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.parse({
    schemaVersion: options.schemaVersion,
    readinessId: options.readinessId,
    sourceTimerStatusId: timerStatus.statusId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: timerStatus.nativeAppRowCount,
    nativeGameRowCount: timerStatus.nativeGameRowCount,
    runtimeProofRequiredCount: rows.filter(
      (row) =>
        row.runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.runtimeReadinessState ===
        AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision
    ).length,
    runtimeReadinessNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
  });
}

function buildRuntimeReadinessRow(
  options: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
  timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow
): AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow {
  const runtimeReadinessState = runtimeReadinessStateForTimerStatus(timerStatusRow);
  const runtimeProofRequired =
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${timerStatusRow.rowId}:runtime-readiness`,
    sourceTimerStatusRowId: timerStatusRow.rowId,
    targetDomain: timerStatusRow.targetDomain,
    runtimeReadinessState,
    timerRuntimeProofRequired: runtimeProofRequired,
    schedulerPersistenceProofRequired: runtimeProofRequired,
    auditProofRequired: runtimeProofRequired,
    rollbackProofRequired: runtimeProofRequired,
    requiredProofRefs: requiredProofRefsForReadiness(options, runtimeReadinessState, timerStatusRow),
    sourceEvidenceRefs: timerStatusRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function runtimeReadinessStateForTimerStatus(timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState)) {
    if (appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatus(timerStatusRow.timerStatusState, state)) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision;
}

function requiredProofRefsForReadiness(
  options: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
  runtimeReadinessState: string,
  timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow
) {
  if (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired) {
    return [
      options.timerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.auditProofRef,
      options.rollbackProofRef,
    ];
  }
  return timerStatusRow.requiredProofRefs;
}
