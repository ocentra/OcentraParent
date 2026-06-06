import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffStateValue,
} from './app-game-timer-service-read-api-response-consumer-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffState = {
  ParentSurfaceProofRequired: 'parent-surface-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceHandoffNonClaims = [
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

export const AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags = {
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

type ParentSurfaceHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    serviceResponseConsumerParentSurfaceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceHandoffCountsMatch(
  handoff: ParentSurfaceHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceHandoffNoClaimFlags).every(
    (key) => handoff[key] === false
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
  responseConsumerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffStateValue,
  parentSurfaceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue
): boolean {
  if (
    responseConsumerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.ServiceReadApiResponseConsumerProofRequired
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    );
  }
  if (
    responseConsumerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceHandoffState === AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
  );
}
