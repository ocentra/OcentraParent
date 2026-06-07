import {
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState as AppGameTimerServiceResponseConsumerParentSurfaceHandoffState,
  type AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffStateValue as AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue,
} from './app-game-timer-service-read-api-response-consumer-parent-surface-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState = {
  ParentSurfaceReadModelProofRequired: 'parent-surface-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNonClaims = [
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

export const AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags = {
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

type ParentSurfaceReadModelHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    serviceResponseConsumerParentSurfaceReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceReadModelProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffCountsMatch(
  handoff: ParentSurfaceReadModelHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceReadModelProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags).every(
    (key) => handoff[key] === false
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
  parentSurfaceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceHandoffStateValue,
  parentSurfaceReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffStateValue
): boolean {
  if (
    parentSurfaceHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
  ) {
    return (
      parentSurfaceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    );
  }
  if (
    parentSurfaceHandoffState === AppGameTimerServiceResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
  );
}
