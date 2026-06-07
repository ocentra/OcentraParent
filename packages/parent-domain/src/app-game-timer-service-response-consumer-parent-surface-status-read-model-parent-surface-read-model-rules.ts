import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState = {
  ReadyForParentSurfaceReadModel: 'ready-for-parent-surface-status-read-model-parent-surface-read-model-contract',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNonClaims = [
  'no-package-export',
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
  'no-runtime-read-model-persistence',
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags = {
  packageExported: false,
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
  runtimeReadModelPersisted: false,
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

type ParentSurfaceReadModelCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    parentSurfaceReadModelState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  readyForParentSurfaceReadModelCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelCountsMatch(
  readModel: ParentSurfaceReadModelCounts
): boolean {
  return (
    readModel.nativeAppRowCount === readModel.rows.filter((row) => row.targetDomain === 'native-app').length &&
    readModel.nativeGameRowCount === readModel.rows.filter((row) => row.targetDomain === 'native-game').length &&
    readModel.readyForParentSurfaceReadModelCount ===
      readModel.rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
      ).length &&
    readModel.blockedBySourceFreshnessCount ===
      readModel.rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
      ).length &&
    readModel.blockedByCompilerDecisionCount ===
      readModel.rows.filter(
        (row) =>
          row.parentSurfaceReadModelState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHasNoRuntimeClaims(
  readModel: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelNoClaimFlags
  ).every((key) => readModel[key] === false);
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelMatchesHandoff(
  handoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue,
  readModelState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue
): boolean {
  if (
    handoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
  ) {
    return (
      readModelState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
    );
  }
  if (
    handoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
  ) {
    return (
      readModelState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
    );
  }
  return (
    readModelState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision
  );
}
