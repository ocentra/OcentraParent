import {
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState,
  type AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-read-model-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState = {
  ParentSurfaceStatusProofRequired: 'parent-surface-status-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNonClaims = [
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags = {
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

type ParentSurfaceStatusHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    serviceResponseConsumerParentSurfaceStatusHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceStatusProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffCountsMatch(
  handoff: ParentSurfaceStatusHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceStatusProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffNoClaimFlags).every(
    (key) => handoff[key] === false
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff(
  parentSurfaceReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue,
  parentSurfaceStatusHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffStateValue
): boolean {
  if (
    parentSurfaceReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
  ) {
    return (
      parentSurfaceStatusHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
    );
  }
  if (
    parentSurfaceReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceStatusHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceStatusHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision
  );
}
