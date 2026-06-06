import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState = {
  ServiceHandlerProofRequired: 'service-handler-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaims = [
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNoClaimFlags = {
  serviceCommandRegistered: false,
  serviceHandlerImplemented: false,
  serviceEventEmitted: false,
  serviceReadApiImplemented: false,
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

type ServiceReadinessServiceHandlerHandoffLike = {
  readonly serviceHandlerProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly serviceHandlerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue;
    readonly inheritedAgentProtocolCommandRefs: readonly unknown[];
    readonly inheritedAgentProtocolEventRefs: readonly unknown[];
    readonly requiredServiceHandlerRefs: readonly unknown[];
    readonly requiredServiceReadApiProofRefs: readonly unknown[];
    readonly serviceCommandRegistered: boolean;
    readonly serviceHandlerImplemented: boolean;
    readonly serviceEventEmitted: boolean;
    readonly serviceReadApiImplemented: boolean;
  }[];
  readonly serviceCommandRegistered: boolean;
  readonly serviceHandlerImplemented: boolean;
  readonly serviceEventEmitted: boolean;
  readonly serviceReadApiImplemented: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffCountsMatch = (
  handoff: ServiceReadinessServiceHandlerHandoffLike
) =>
  handoff.serviceHandlerProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired &&
        row.inheritedAgentProtocolCommandRefs.length > 0 &&
        row.inheritedAgentProtocolEventRefs.length > 0 &&
        row.requiredServiceHandlerRefs.length > 0 &&
        row.requiredServiceReadApiProofRefs.length > 0 &&
        !row.serviceCommandRegistered &&
        !row.serviceHandlerImplemented &&
        !row.serviceEventEmitted &&
        !row.serviceReadApiImplemented
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedBySourceFreshness
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessServiceHandlerHandoffLike
) =>
  [
    handoff.serviceCommandRegistered,
    handoff.serviceHandlerImplemented,
    handoff.serviceEventEmitted,
    handoff.serviceReadApiImplemented,
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffMatchesCommandHandoff = (
  commandHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue,
  serviceHandlerState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue
) =>
  (commandHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired &&
    serviceHandlerState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired) ||
  (commandHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedBySourceFreshness &&
    serviceHandlerState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedBySourceFreshness) ||
  (commandHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision &&
    serviceHandlerState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision);
