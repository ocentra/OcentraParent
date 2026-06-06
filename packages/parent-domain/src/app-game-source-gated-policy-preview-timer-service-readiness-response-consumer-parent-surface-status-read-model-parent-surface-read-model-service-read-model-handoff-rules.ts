import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState =
  {
    ServiceReadModelProofRequired: 'parent-surface-read-model-service-read-model-proof-required',
    BlockedBySourceFreshness: 'blocked-by-source-freshness',
    BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  } as const;

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNonClaims =
  [
    'no-service-command-registration',
    'no-service-handler-implementation',
    'no-service-read-model-runtime-emission',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags =
  {
    serviceCommandRegistered: false,
    serviceHandlerImplemented: false,
    serviceReadModelRuntimeEmitted: false,
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

type ServiceReadModelHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    parentSurfaceReadModelServiceReadModelHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  serviceReadModelProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffCountsMatch(
  handoff: ServiceReadModelHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.serviceReadModelProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffMatchesServiceHandoff(
  serviceHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
  serviceReadModelHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue
): boolean {
  if (
    serviceHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
  ) {
    return (
      serviceReadModelHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
    );
  }
  if (
    serviceHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
  ) {
    return (
      serviceReadModelHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    serviceReadModelHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision
  );
}
