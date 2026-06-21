import {
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema,
  type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-runtime-readiness';
import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema,
  type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistence,
  type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions,
  type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState,
  RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims,
  appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadiness,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-scheduler-persistence-rules';

export function buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
  optionsInput: unknown,
  runtimeReadinessInput: unknown
): AppGameSourceGatedPolicyPreviewTimerSchedulerPersistence {
  const options = AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptionsSchema.parse(optionsInput);
  const runtimeReadiness = AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.parse(runtimeReadinessInput);
  const rows = runtimeReadiness.rows.map((row) => buildSchedulerPersistenceRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.parse({
    schemaVersion: options.schemaVersion,
    persistenceId: options.persistenceId,
    sourceRuntimeReadinessId: runtimeReadiness.readinessId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: runtimeReadiness.nativeAppRowCount,
    nativeGameRowCount: runtimeReadiness.nativeGameRowCount,
    schedulerPersistenceProofRequiredCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision
    ).length,
    schedulerPersistenceNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
  });
}

function buildSchedulerPersistenceRow(
  options: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions,
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
): AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow {
  const schedulerPersistenceState = schedulerPersistenceStateForRuntimeReadiness(runtimeReadinessRow);
  const schedulerProofRequired =
    schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${runtimeReadinessRow.rowId}:scheduler-persistence`,
    sourceRuntimeReadinessRowId: runtimeReadinessRow.rowId,
    targetDomain: runtimeReadinessRow.targetDomain,
    schedulerPersistenceState,
    serviceTimerRuntimeProofRequired: schedulerProofRequired,
    schedulerPersistenceProofRequired: schedulerProofRequired,
    schedulerStateStoreProofRequired: schedulerProofRequired,
    auditProofRequired: schedulerProofRequired,
    rollbackProofRequired: schedulerProofRequired,
    requiredProofRefs: requiredProofRefsForSchedulerPersistence(
      options,
      schedulerPersistenceState,
      runtimeReadinessRow
    ),
    sourceEvidenceRefs: runtimeReadinessRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function schedulerPersistenceStateForRuntimeReadiness(
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadiness(
        runtimeReadinessRow.runtimeReadinessState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision;
}

function requiredProofRefsForSchedulerPersistence(
  options: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions,
  schedulerPersistenceState: string,
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
) {
  if (
    schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
  ) {
    return [
      options.serviceTimerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.schedulerStateStoreProofRef,
      options.auditProofRef,
      options.rollbackProofRef,
    ];
  }
  return runtimeReadinessRow.requiredProofRefs;
}

