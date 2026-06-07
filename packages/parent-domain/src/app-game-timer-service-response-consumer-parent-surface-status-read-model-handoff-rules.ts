import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState = {
  ParentSurfaceStatusReadModelProofRequired: 'parent-surface-status-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims = [
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
  'no-parent-surface-status-runtime',
  'no-parent-surface-status-persistence',
  'no-parent-surface-status-read-model-runtime',
  'no-parent-surface-status-read-model-persistence',
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags = {
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
  parentSurfaceStatusRuntimeImplemented: false,
  parentSurfaceStatusPersisted: false,
  parentSurfaceStatusReadModelRuntimeImplemented: false,
  parentSurfaceStatusReadModelPersisted: false,
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

type ParentSurfaceStatusReadModelHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceStatusReadModelProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffCountsMatch(
  handoff: ParentSurfaceStatusReadModelHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceStatusReadModelProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags).every(
    (key) => handoff[key] === false
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff(
  parentSurfaceStatusHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue,
  parentSurfaceStatusReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue
): boolean {
  if (
    parentSurfaceStatusHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
  ) {
    return (
      parentSurfaceStatusReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
    );
  }
  if (
    parentSurfaceStatusHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceStatusReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceStatusReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision
  );
}
