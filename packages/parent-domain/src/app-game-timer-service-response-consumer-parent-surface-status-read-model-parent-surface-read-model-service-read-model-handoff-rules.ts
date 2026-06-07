import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-rules';

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState =
  {
    ServiceReadModelProofRequired: 'parent-surface-read-model-service-read-model-proof-required',
    BlockedBySourceFreshness: 'blocked-by-source-freshness',
    BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  } as const;

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue =
  (typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState)[keyof typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState];

export const RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNonClaims =
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

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags =
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
    parentSurfaceReadModelServiceReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  serviceReadModelProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffCountsMatch(
  handoff: ServiceReadModelHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.serviceReadModelProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffMatchesServiceHandoff(
  serviceHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
  serviceReadModelHandoffState: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue
): boolean {
  if (
    serviceHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
  ) {
    return (
      serviceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
    );
  }
  if (
    serviceHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
  ) {
    return (
      serviceReadModelHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    serviceReadModelHandoffState ===
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision
  );
}
