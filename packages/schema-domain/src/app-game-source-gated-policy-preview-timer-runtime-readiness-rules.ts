import { AppGameSourceGatedPolicyPreviewTimerStatusState } from './app-game-source-gated-policy-preview-timer-status-rules';

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState = {
  RuntimeProofRequired: 'runtime-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-scheduler-persistence',
  'no-audit-runtime',
  'no-rollback-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  schedulerPersistenceClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type SourceGatedPolicyPreviewTimerRuntimeReadinessLike = {
  readonly runtimeProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly runtimeReadinessState: string;
    readonly timerRuntimeProofRequired: boolean;
    readonly schedulerPersistenceProofRequired: boolean;
    readonly auditProofRequired: boolean;
    readonly rollbackProofRequired: boolean;
    readonly timerScheduled: boolean;
  }[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessCountsMatch = (
  readiness: SourceGatedPolicyPreviewTimerRuntimeReadinessLike
) =>
  readiness.runtimeProofRequiredCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired &&
        row.timerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.auditProofRequired &&
        row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  readiness.blockedBySourceFreshnessCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState ===
          AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness &&
        !row.timerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  readiness.blockedByCompilerDecisionCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState ===
          AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision &&
        !row.timerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessHasNoRuntimeClaims = (
  readiness: SourceGatedPolicyPreviewTimerRuntimeReadinessLike
) =>
  !readiness.serviceRuntimeEventClaimed &&
  !readiness.portalUiRendered &&
  !readiness.policyEvaluatorRuntimeClaimed &&
  !readiness.timerRuntimeClaimed &&
  !readiness.timerScheduled &&
  !readiness.schedulerPersistenceClaimed &&
  !readiness.auditRuntimeClaimed &&
  !readiness.rollbackRuntimeClaimed &&
  !readiness.adapterDispatchClaimed &&
  !readiness.childDeliveryClaimed &&
  !readiness.platformEnforcementClaimed &&
  !readiness.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatus = (
  timerStatusState: string,
  runtimeReadinessState: string
) =>
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired &&
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired) ||
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired &&
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness) ||
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired &&
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision);

