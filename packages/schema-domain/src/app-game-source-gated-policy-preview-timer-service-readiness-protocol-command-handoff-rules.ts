import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState = {
  ProtocolCommandHandoffProofRequired: 'protocol-command-handoff-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaims = [
  'no-agent-protocol-command-implemented',
  'no-agent-protocol-event-implemented',
  'no-rust-protocol-mirrored',
  'no-service-command-registered',
  'no-service-handler-implemented',
  'no-service-event-emitted',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNoClaimFlags = {
  agentProtocolCommandImplemented: false,
  agentProtocolEventImplemented: false,
  rustProtocolMirrored: false,
  serviceCommandRegistered: false,
  serviceHandlerImplemented: false,
  serviceEventEmitted: false,
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

type ServiceReadinessProtocolCommandHandoffLike = {
  readonly protocolCommandHandoffProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly protocolCommandHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue;
    readonly requiredProtocolProofRefs: readonly unknown[];
    readonly requiredAgentProtocolCommandRefs: readonly unknown[];
    readonly requiredAgentProtocolEventRefs: readonly unknown[];
    readonly requiredServiceHandlerRefs: readonly unknown[];
    readonly agentProtocolCommandImplemented: boolean;
    readonly agentProtocolEventImplemented: boolean;
    readonly rustProtocolMirrored: boolean;
    readonly serviceCommandRegistered: boolean;
    readonly serviceHandlerImplemented: boolean;
    readonly serviceEventEmitted: boolean;
    readonly serviceReadApiImplemented: boolean;
  }[];
  readonly agentProtocolCommandImplemented: boolean;
  readonly agentProtocolEventImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
  readonly serviceCommandRegistered: boolean;
  readonly serviceHandlerImplemented: boolean;
  readonly serviceEventEmitted: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffCountsMatch = (
  handoff: ServiceReadinessProtocolCommandHandoffLike
) =>
  handoff.protocolCommandHandoffProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired &&
        row.requiredProtocolProofRefs.length > 0 &&
        row.requiredAgentProtocolCommandRefs.length > 0 &&
        row.requiredAgentProtocolEventRefs.length > 0 &&
        row.requiredServiceHandlerRefs.length > 0 &&
        !row.agentProtocolCommandImplemented &&
        !row.agentProtocolEventImplemented &&
        !row.rustProtocolMirrored &&
        !row.serviceCommandRegistered &&
        !row.serviceHandlerImplemented &&
        !row.serviceEventEmitted &&
        !row.serviceReadApiImplemented
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedBySourceFreshness &&
        row.requiredProtocolProofRefs.length === 0 &&
        row.requiredAgentProtocolCommandRefs.length === 0 &&
        row.requiredAgentProtocolEventRefs.length === 0 &&
        row.requiredServiceHandlerRefs.length === 0
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision &&
        row.requiredProtocolProofRefs.length === 0 &&
        row.requiredAgentProtocolCommandRefs.length === 0 &&
        row.requiredAgentProtocolEventRefs.length === 0 &&
        row.requiredServiceHandlerRefs.length === 0
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessProtocolCommandHandoffLike
) => noRuntimeClaimValues(handoff).every((value) => value === false);

function noRuntimeClaimValues(handoff: ServiceReadinessProtocolCommandHandoffLike) {
  return [
    handoff.agentProtocolCommandImplemented,
    handoff.agentProtocolEventImplemented,
    handoff.rustProtocolMirrored,
    handoff.serviceCommandRegistered,
    handoff.serviceHandlerImplemented,
    handoff.serviceEventEmitted,
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffMatchesReadModel = (
  readModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue,
  commandHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue
) =>
  (readModelState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired &&
    commandHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired) ||
  (readModelState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness &&
    commandHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedBySourceFreshness) ||
  (readModelState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision &&
    commandHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision);
