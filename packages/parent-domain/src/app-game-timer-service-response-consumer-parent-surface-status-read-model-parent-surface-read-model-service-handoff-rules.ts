import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState =
  {
    ServiceProofRequired: 'parent-surface-read-model-service-proof-required',
    BlockedBySourceFreshness: 'blocked-by-source-freshness',
    BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  } as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims =
  [
    'no-service-command-registration',
    'no-service-handler-implementation',
    'no-service-read-model-emission',
    'no-service-read-api-implementation',
    'no-service-read-api-response-implementation',
    'no-service-read-api-response-consumer-implementation',
    'no-service-event-emission',
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags =
  {
    serviceCommandRegistered: false,
    serviceHandlerImplemented: false,
    serviceReadModelEmitted: false,
    serviceReadApiImplemented: false,
    serviceReadApiResponseImplemented: false,
    serviceReadApiResponseConsumerImplemented: false,
    serviceEventEmitted: false,
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

type ServiceHandoffCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    parentSurfaceReadModelServiceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  serviceProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffCountsMatch(
  handoff: ServiceHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.serviceProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel(
  readModelState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelStateValue,
  serviceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue
): boolean {
  if (
    readModelState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel
  ) {
    return (
      serviceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
    );
  }
  if (
    readModelState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness
  ) {
    return (
      serviceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    serviceHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision
  );
}
