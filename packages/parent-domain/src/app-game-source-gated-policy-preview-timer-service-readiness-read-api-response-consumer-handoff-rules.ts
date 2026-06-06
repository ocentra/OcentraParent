import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState = {
  ReadApiResponseConsumerProofRequired: 'read-api-response-consumer-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNonClaims = [
  'no-service-command-registration',
  'no-service-handler-implementation',
  'no-service-read-api-implementation',
  'no-service-read-api-response-implementation',
  'no-service-read-api-response-consumer-implementation',
  'no-service-event-emission',
  'no-agent-protocol-implementation',
  'no-rust-protocol-mirror',
  'no-portal-ui-rendering',
  'no-portal-response-consumer-rendering',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduling',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags = {
  serviceCommandRegistered: false,
  serviceHandlerImplemented: false,
  serviceReadApiImplemented: false,
  serviceReadApiResponseImplemented: false,
  serviceReadApiResponseConsumerImplemented: false,
  serviceEventEmitted: false,
  agentProtocolImplemented: false,
  rustProtocolMirrored: false,
  portalUiRendered: false,
  portalResponseConsumerRendered: false,
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

type ResponseConsumerHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    readApiResponseConsumerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  readApiResponseConsumerProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffCountsMatch(
  handoff: ResponseConsumerHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.readApiResponseConsumerProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.readApiResponseConsumerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.readApiResponseConsumerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.readApiResponseConsumerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffMatchesResponseHandoff(
  responseHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue,
  consumerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffStateValue
): boolean {
  if (
    responseHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired
  ) {
    return (
      consumerHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired
    );
  }
  if (
    responseHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedBySourceFreshness
  ) {
    return (
      consumerHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    consumerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedByCompilerDecision
  );
}
