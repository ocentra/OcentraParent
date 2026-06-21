import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffStateValue,
} from './app-game-timer-service-read-api-response-consumer-handoff-rules';

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState = {
  ParentSurfaceProofRequired: 'parent-surface-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue =
  (typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState)[keyof typeof AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState];

export const RequiredAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNonClaims = [
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

export const AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags = {
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
    parentSurfaceHandoffState: AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffCountsMatch(
  handoff: ParentSurfaceHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceHandoffState ===
          AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceHandoffState ===
          AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceHandoffState ===
          AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffNoClaimFlags).every(
    (key) => handoff[key] === false
  );
}

export function appGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffMatchesResponseConsumerHandoff(
  responseConsumerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffStateValue,
  parentSurfaceHandoffState: AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue
): boolean {
  if (
    responseConsumerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.ServiceReadApiResponseConsumerProofRequired
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    );
  }
  if (
    responseConsumerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceHandoffState ===
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
  );
}
