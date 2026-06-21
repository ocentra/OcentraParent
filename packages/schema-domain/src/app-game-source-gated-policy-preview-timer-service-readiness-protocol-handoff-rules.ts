import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState } from './app-game-source-gated-policy-preview-timer-service-readiness-read-model-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState = {
  ProtocolProofRequired: 'protocol-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaims = [
  'no-agent-protocol-contract-implemented',
  'no-rust-protocol-mirrored',
  'no-service-command-registered',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNoClaimFlags = {
  agentProtocolContractImplemented: false,
  rustProtocolMirrored: false,
  serviceCommandRegistered: false,
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

type ServiceReadinessProtocolHandoffLike = {
  readonly protocolProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly protocolHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue;
    readonly requiredProtocolProofRefs: readonly unknown[];
    readonly agentProtocolContractImplemented: boolean;
    readonly rustProtocolMirrored: boolean;
    readonly serviceCommandRegistered: boolean;
    readonly serviceEventEmitted: boolean;
    readonly serviceReadApiImplemented: boolean;
  }[];
  readonly agentProtocolContractImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
  readonly serviceCommandRegistered: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffCountsMatch = (
  handoff: ServiceReadinessProtocolHandoffLike
) =>
  handoff.protocolProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired &&
        row.requiredProtocolProofRefs.length > 0 &&
        !row.agentProtocolContractImplemented &&
        !row.rustProtocolMirrored &&
        !row.serviceCommandRegistered &&
        !row.serviceEventEmitted &&
        !row.serviceReadApiImplemented
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedBySourceFreshness &&
        row.requiredProtocolProofRefs.length === 0
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.protocolHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision &&
        row.requiredProtocolProofRefs.length === 0
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessProtocolHandoffLike
) => noRuntimeClaimValues(handoff).every((value) => value === false);

function noRuntimeClaimValues(handoff: ServiceReadinessProtocolHandoffLike) {
  return [
    handoff.agentProtocolContractImplemented,
    handoff.rustProtocolMirrored,
    handoff.serviceCommandRegistered,
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffMatchesReadModel = (
  readModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue,
  protocolHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue
) =>
  (readModelState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired &&
    protocolHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired) ||
  (readModelState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness &&
    protocolHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedBySourceFreshness) ||
  (readModelState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision &&
    protocolHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision);
