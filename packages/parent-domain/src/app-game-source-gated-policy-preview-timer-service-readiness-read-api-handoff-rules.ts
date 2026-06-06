import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState = {
  ServiceReadApiProofRequired: 'service-read-api-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaims = [
  'no-service-command-registered',
  'no-service-handler-implemented',
  'no-service-event-emitted',
  'no-service-read-api-implemented',
  'no-agent-protocol-implemented',
  'no-rust-protocol-mirrored',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNoClaimFlags = {
  serviceCommandRegistered: false,
  serviceHandlerImplemented: false,
  serviceReadApiImplemented: false,
  serviceEventEmitted: false,
  agentProtocolImplemented: false,
  rustProtocolMirrored: false,
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

type ServiceReadinessServiceReadApiHandoffLike = {
  readonly serviceReadApiProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly serviceReadApiHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue;
    readonly inheritedAgentProtocolCommandRefs: readonly unknown[];
    readonly inheritedAgentProtocolEventRefs: readonly unknown[];
    readonly inheritedServiceHandlerRefs: readonly unknown[];
    readonly requiredServiceReadApiProofRefs: readonly unknown[];
    readonly serviceCommandRegistered: boolean;
    readonly serviceHandlerImplemented: boolean;
    readonly serviceReadApiImplemented: boolean;
    readonly serviceEventEmitted: boolean;
  }[];
  readonly serviceCommandRegistered: boolean;
  readonly serviceHandlerImplemented: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly serviceEventEmitted: boolean;
  readonly agentProtocolImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffCountsMatch = (
  handoff: ServiceReadinessServiceReadApiHandoffLike
) =>
  handoff.serviceReadApiProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired &&
        row.inheritedAgentProtocolCommandRefs.length > 0 &&
        row.inheritedAgentProtocolEventRefs.length > 0 &&
        row.inheritedServiceHandlerRefs.length > 0 &&
        row.requiredServiceReadApiProofRefs.length > 0 &&
        !row.serviceCommandRegistered &&
        !row.serviceHandlerImplemented &&
        !row.serviceReadApiImplemented &&
        !row.serviceEventEmitted
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedBySourceFreshness
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessServiceReadApiHandoffLike
) =>
  [
    handoff.serviceCommandRegistered,
    handoff.serviceHandlerImplemented,
    handoff.serviceReadApiImplemented,
    handoff.serviceEventEmitted,
    handoff.agentProtocolImplemented,
    handoff.rustProtocolMirrored,
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
  ].every((value) => value === false);

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffMatchesServiceHandlerHandoff = (
  serviceHandlerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue,
  serviceReadApiState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue
) =>
  (serviceHandlerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired &&
    serviceReadApiState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired) ||
  (serviceHandlerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedBySourceFreshness &&
    serviceReadApiState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedBySourceFreshness) ||
  (serviceHandlerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision &&
    serviceReadApiState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision);
