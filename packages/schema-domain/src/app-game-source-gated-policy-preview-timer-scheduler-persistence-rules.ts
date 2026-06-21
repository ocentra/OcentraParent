import { AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState } from './app-game-source-gated-policy-preview-timer-runtime-readiness-rules';

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState = {
  SchedulerPersistenceProofRequired: 'scheduler-persistence-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-scheduler-persistence-runtime',
  'no-durable-scheduler-storage',
  'no-audit-runtime',
  'no-rollback-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  schedulerPersistenceRuntimeClaimed: false,
  durableSchedulerStorageClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type SourceGatedPolicyPreviewTimerSchedulerPersistenceLike = {
  readonly schedulerPersistenceProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly schedulerPersistenceState: string;
    readonly serviceTimerRuntimeProofRequired: boolean;
    readonly schedulerPersistenceProofRequired: boolean;
    readonly schedulerStateStoreProofRequired: boolean;
    readonly auditProofRequired: boolean;
    readonly rollbackProofRequired: boolean;
    readonly timerScheduled: boolean;
    readonly schedulerPersistenceRuntimeClaimed: boolean;
    readonly durableSchedulerStorageClaimed: boolean;
  }[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceRuntimeClaimed: boolean;
  readonly durableSchedulerStorageClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceCountsMatch = (
  persistence: SourceGatedPolicyPreviewTimerSchedulerPersistenceLike
) =>
  persistence.schedulerPersistenceProofRequiredCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired &&
        row.serviceTimerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.schedulerStateStoreProofRequired &&
        row.auditProofRequired &&
        row.rollbackProofRequired &&
        !row.timerScheduled &&
        !row.schedulerPersistenceRuntimeClaimed &&
        !row.durableSchedulerStorageClaimed
    ).length &&
  persistence.blockedBySourceFreshnessCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  persistence.blockedByCompilerDecisionCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceHasNoRuntimeClaims = (
  persistence: SourceGatedPolicyPreviewTimerSchedulerPersistenceLike
) => noRuntimeClaimValues(persistence).every((value) => value === false);

function noRuntimeClaimValues(persistence: SourceGatedPolicyPreviewTimerSchedulerPersistenceLike) {
  return [
    persistence.serviceRuntimeEventClaimed,
    persistence.portalUiRendered,
    persistence.policyEvaluatorRuntimeClaimed,
    persistence.timerRuntimeClaimed,
    persistence.timerScheduled,
    persistence.schedulerPersistenceRuntimeClaimed,
    persistence.durableSchedulerStorageClaimed,
    persistence.auditRuntimeClaimed,
    persistence.rollbackRuntimeClaimed,
    persistence.adapterDispatchClaimed,
    persistence.childDeliveryClaimed,
    persistence.platformEnforcementClaimed,
    persistence.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadiness = (
  runtimeReadinessState: string,
  schedulerPersistenceState: string
) =>
  (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired) ||
  (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness) ||
  (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision);

