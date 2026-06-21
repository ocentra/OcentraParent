import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState } from './app-game-source-gated-policy-preview-timer-service-readiness-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState = {
  ServiceReadModelProofRequired: 'service-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaims = [
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlags = {
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

type ServiceReadinessReadModelLike = {
  readonly serviceReadModelProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly serviceReadinessReadModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelCountsMatch = (
  readModel: ServiceReadinessReadModelLike
) =>
  readModel.serviceReadModelProofRequiredCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired &&
        row.serviceReadinessProofRequired &&
        row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented &&
        !row.timerScheduled &&
        !row.adapterDispatchClaimed
    ).length &&
  readModel.blockedBySourceFreshnessCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length &&
  readModel.blockedByCompilerDecisionCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelHasNoRuntimeClaims = (
  readModel: ServiceReadinessReadModelLike
) => noRuntimeClaimValues(readModel).every((value) => value === false);

function noRuntimeClaimValues(readModel: ServiceReadinessReadModelLike) {
  return [
    readModel.serviceRuntimeEventClaimed,
    readModel.serviceReadApiImplemented,
    readModel.portalUiRendered,
    readModel.policyEvaluatorRuntimeClaimed,
    readModel.timerRuntimeClaimed,
    readModel.timerScheduled,
    readModel.schedulerPersistenceRuntimeClaimed,
    readModel.durableSchedulerStorageClaimed,
    readModel.auditRuntimeClaimed,
    readModel.durableAuditLogClaimed,
    readModel.rollbackRuntimeClaimed,
    readModel.rollbackExecutionClaimed,
    readModel.adapterDispatchClaimed,
    readModel.childDeliveryClaimed,
    readModel.platformEnforcementClaimed,
    readModel.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelMatchesHandoff = (
  handoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue,
  readModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue
) =>
  (handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedBySourceFreshness &&
    readModelState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision &&
    readModelState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision);
