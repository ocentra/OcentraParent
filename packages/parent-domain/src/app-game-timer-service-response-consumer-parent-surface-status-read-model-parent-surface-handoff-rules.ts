import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState = {
  ParentSurfaceProofRequired: 'parent-surface-status-read-model-parent-surface-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims = [
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
  'no-parent-surface-status-read-model-parent-surface-runtime',
  'no-parent-surface-status-read-model-parent-surface-persistence',
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags = {
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
  parentSurfaceStatusReadModelParentSurfaceRuntimeImplemented: false,
  parentSurfaceStatusReadModelParentSurfacePersisted: false,
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
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffCountsMatch(
  handoff: ParentSurfaceHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffMatchesStatusReadModelHandoff(
  statusReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffStateValue,
  parentSurfaceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue
): boolean {
  if (
    statusReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
    );
  }
  if (
    statusReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision
  );
}
