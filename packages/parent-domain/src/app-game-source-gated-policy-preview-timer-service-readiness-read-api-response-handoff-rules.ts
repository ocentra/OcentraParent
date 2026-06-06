import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState = {
  ReadApiResponseProofRequired: 'read-api-response-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState];
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaims = [
  'no-service-command-registered',
  'no-service-handler-implemented',
  'no-service-event-emitted',
  'no-service-read-api-implemented',
  'no-service-read-api-response-implemented',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNoClaimFlags = {
  serviceCommandRegistered: false,
  serviceHandlerImplemented: false,
  serviceReadApiImplemented: false,
  serviceReadApiResponseImplemented: false,
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

type ServiceReadinessReadApiResponseHandoffLike = {
  readonly readApiResponseProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly readApiResponseHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue;
    readonly inheritedServiceReadApiProofRefs: readonly unknown[];
    readonly requiredReadApiResponseProofRefs: readonly unknown[];
    readonly serviceCommandRegistered: boolean;
    readonly serviceHandlerImplemented: boolean;
    readonly serviceReadApiImplemented: boolean;
    readonly serviceReadApiResponseImplemented: boolean;
    readonly serviceEventEmitted: boolean;
  }[];
  readonly serviceCommandRegistered: boolean;
  readonly serviceHandlerImplemented: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly serviceReadApiResponseImplemented: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffCountsMatch = (
  handoff: ServiceReadinessReadApiResponseHandoffLike
) =>
  handoff.readApiResponseProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired &&
        row.inheritedServiceReadApiProofRefs.length > 0 &&
        row.requiredReadApiResponseProofRefs.length > 0 &&
        !row.serviceCommandRegistered &&
        !row.serviceHandlerImplemented &&
        !row.serviceReadApiImplemented &&
        !row.serviceReadApiResponseImplemented &&
        !row.serviceEventEmitted
    ).length &&
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedBySourceFreshness
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedByCompilerDecision
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffHasNoRuntimeClaims = (
  handoff: ServiceReadinessReadApiResponseHandoffLike
) =>
  [
    handoff.serviceCommandRegistered,
    handoff.serviceHandlerImplemented,
    handoff.serviceReadApiImplemented,
    handoff.serviceReadApiResponseImplemented,
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

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffMatchesReadApiHandoff = (
  readApiHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue,
  responseState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue
) =>
  (readApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired &&
    responseState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired) ||
  (readApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedBySourceFreshness &&
    responseState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedBySourceFreshness) ||
  (readApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision &&
    responseState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedByCompilerDecision);
