import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState } from './app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState = {
  ServiceReadApiProofRequired: 'service-read-api-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState];
type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims = [
  'no-service-runtime-event',
  'no-service-read-api-implemented',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-scheduler-persistence-runtime',
  'no-durable-scheduler-storage',
  'no-audit-runtime',
  'no-durable-audit-log',
  'no-rollback-runtime',
  'no-rollback-execution',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  serviceReadApiImplemented: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  schedulerPersistenceRuntimeClaimed: false,
  durableSchedulerStorageClaimed: false,
  auditRuntimeClaimed: false,
  durableAuditLogClaimed: false,
  rollbackRuntimeClaimed: false,
  rollbackExecutionClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type ServiceReadinessHandoffLike = {
  readonly serviceReadApiProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly serviceReadinessHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue;
    readonly parentSurfaceProofRequired: boolean;
    readonly serviceReadinessProofRequired: boolean;
    readonly serviceReadApiProofRequired: boolean;
    readonly serviceReadApiImplemented: boolean;
    readonly timerScheduled: boolean;
    readonly adapterDispatchClaimed: boolean;
  }[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceRuntimeClaimed: boolean;
  readonly durableSchedulerStorageClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly durableAuditLogClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly rollbackExecutionClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffCountsMatch = (
  handoff: ServiceReadinessHandoffLike
) =>
  handoff.serviceReadApiProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired &&
        row.parentSurfaceProofRequired &&
        row.serviceReadinessProofRequired &&
        row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented &&
        !row.timerScheduled &&
        !row.adapterDispatchClaimed
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedBySourceFreshness &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessHandoffLike
) => noRuntimeClaimValues(handoff).every((value) => value === false);

function noRuntimeClaimValues(handoff: ServiceReadinessHandoffLike) {
  return [
    handoff.serviceRuntimeEventClaimed,
    handoff.serviceReadApiImplemented,
    handoff.portalUiRendered,
    handoff.policyEvaluatorRuntimeClaimed,
    handoff.timerRuntimeClaimed,
    handoff.timerScheduled,
    handoff.schedulerPersistenceRuntimeClaimed,
    handoff.durableSchedulerStorageClaimed,
    handoff.auditRuntimeClaimed,
    handoff.durableAuditLogClaimed,
    handoff.rollbackRuntimeClaimed,
    handoff.rollbackExecutionClaimed,
    handoff.adapterDispatchClaimed,
    handoff.childDeliveryClaimed,
    handoff.platformEnforcementClaimed,
    handoff.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffMatchesParentSurfaceIntent = (
  parentSurfaceIntentState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue,
  handoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue
) =>
  (parentSurfaceIntentState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired &&
    handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired) ||
  (parentSurfaceIntentState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness &&
    handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedBySourceFreshness) ||
  (parentSurfaceIntentState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision &&
    handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision);
