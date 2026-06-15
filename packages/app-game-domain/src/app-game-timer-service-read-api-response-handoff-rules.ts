import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffStateValue,
} from './app-game-timer-service-read-api-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState =
  {
    ServiceReadApiResponseProofRequired: 'parent-surface-read-model-service-read-api-response-proof-required',
    BlockedBySourceFreshness: 'blocked-by-source-freshness',
    BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  } as const;

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffNonClaims =
  [
    'no-service-command-registration',
    'no-service-handler-implementation',
    'no-service-read-model-runtime-emission',
    'no-service-event-runtime-emission',
    'no-service-event-emission',
    'no-service-read-api-implementation',
    'no-service-read-api-response-implementation',
    'no-service-read-api-response-consumer-implementation',
    'no-agent-protocol-implementation',
    'no-rust-protocol-mirror',
    'no-portal-ui-rendering',
    'no-portal-response-consumer-rendering',
    'no-parent-surface-rendering',
    'no-parent-surface-read-model-runtime',
    'no-parent-surface-read-model-persistence',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffNoClaimFlags =
  {
    serviceCommandRegistered: false,
    serviceHandlerImplemented: false,
    serviceReadModelRuntimeEmitted: false,
    serviceEventRuntimeEmitted: false,
    serviceEventEmitted: false,
    serviceReadApiImplemented: false,
    serviceReadApiResponseImplemented: false,
    serviceReadApiResponseConsumerImplemented: false,
    agentProtocolImplemented: false,
    rustProtocolMirrored: false,
    portalUiRendered: false,
    portalResponseConsumerRendered: false,
    parentSurfaceRendered: false,
    parentSurfaceReadModelRuntimeImplemented: false,
    parentSurfaceReadModelPersisted: false,
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

type ServiceReadApiResponseHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    parentSurfaceReadModelServiceReadApiResponseHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  serviceReadApiResponseProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffCountsMatch(
  handoff: ServiceReadApiResponseHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.serviceReadApiResponseProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiResponseHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.ServiceReadApiResponseProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiResponseHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiResponseHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffMatchesServiceReadApiHandoff(
  serviceReadApiHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffStateValue,
  serviceReadApiResponseHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffStateValue
): boolean {
  if (
    serviceReadApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired
  ) {
    return (
      serviceReadApiResponseHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.ServiceReadApiResponseProofRequired
    );
  }
  if (
    serviceReadApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedBySourceFreshness
  ) {
    return (
      serviceReadApiResponseHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    serviceReadApiResponseHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedByCompilerDecision
  );
}
