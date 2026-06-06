import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState = {
  ProtocolReadModelProofRequired: 'protocol-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaims = [
  'no-agent-protocol-contract-implemented',
  'no-rust-protocol-mirrored',
  'no-service-command-registered',
  'no-service-event-emitted',
  'no-service-read-api-implemented',
  'no-service-read-model-event-emitted',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlags = {
  agentProtocolContractImplemented: false,
  rustProtocolMirrored: false,
  serviceCommandRegistered: false,
  serviceEventEmitted: false,
  serviceReadApiImplemented: false,
  serviceReadModelEventEmitted: false,
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

type ServiceReadinessProtocolReadModelLike = {
  readonly protocolReadModelProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly protocolReadModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue;
    readonly requiredProtocolProofRefs: readonly unknown[];
    readonly agentProtocolContractImplemented: boolean;
    readonly rustProtocolMirrored: boolean;
    readonly serviceCommandRegistered: boolean;
    readonly serviceEventEmitted: boolean;
    readonly serviceReadApiImplemented: boolean;
    readonly serviceReadModelEventEmitted: boolean;
  }[];
  readonly agentProtocolContractImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
  readonly serviceCommandRegistered: boolean;
  readonly serviceEventEmitted: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly serviceReadModelEventEmitted: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelCountsMatch = (
  readModel: ServiceReadinessProtocolReadModelLike
) =>
  readModel.protocolReadModelProofRequiredCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired &&
        row.requiredProtocolProofRefs.length > 0 &&
        !row.agentProtocolContractImplemented &&
        !row.rustProtocolMirrored &&
        !row.serviceCommandRegistered &&
        !row.serviceEventEmitted &&
        !row.serviceReadApiImplemented &&
        !row.serviceReadModelEventEmitted
    ).length &&
  readModel.blockedBySourceFreshnessCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness &&
        row.requiredProtocolProofRefs.length === 0
    ).length &&
  readModel.blockedByCompilerDecisionCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision &&
        row.requiredProtocolProofRefs.length === 0
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelHasNoRuntimeClaims = (
  readModel: ServiceReadinessProtocolReadModelLike
) => noRuntimeClaimValues(readModel).every((value) => value === false);

function noRuntimeClaimValues(readModel: ServiceReadinessProtocolReadModelLike) {
  return [
    readModel.agentProtocolContractImplemented,
    readModel.rustProtocolMirrored,
    readModel.serviceCommandRegistered,
    readModel.serviceEventEmitted,
    readModel.serviceReadApiImplemented,
    readModel.serviceReadModelEventEmitted,
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelMatchesHandoff = (
  handoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue,
  readModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue
) =>
  (handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedBySourceFreshness &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness) ||
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision);
