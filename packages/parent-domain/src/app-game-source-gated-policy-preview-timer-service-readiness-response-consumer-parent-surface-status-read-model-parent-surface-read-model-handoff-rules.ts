import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState =
  {
    ParentSurfaceReadModelProofRequired: 'parent-surface-status-read-model-parent-surface-read-model-proof-required',
    BlockedBySourceFreshness: 'blocked-by-source-freshness',
    BlockedByCompilerDecision: 'blocked-by-compiler-decision',
  } as const;

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNonClaims =
  [
    'no-service-command-registration',
    'no-service-handler-implementation',
    'no-service-read-api-implementation',
    'no-service-read-api-response-implementation',
    'no-service-read-api-response-consumer-implementation',
    'no-parent-surface-read-model-implementation',
    'no-parent-surface-status-implementation',
    'no-parent-surface-status-read-model-implementation',
    'no-parent-surface-status-read-model-parent-surface-implementation',
    'no-parent-surface-status-read-model-parent-surface-read-model-implementation',
    'no-service-event-emission',
    'no-agent-protocol-implementation',
    'no-rust-protocol-mirror',
    'no-portal-ui-rendering',
    'no-portal-response-consumer-rendering',
    'no-parent-surface-rendering',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags =
  {
    serviceCommandRegistered: false,
    serviceHandlerImplemented: false,
    serviceReadApiImplemented: false,
    serviceReadApiResponseImplemented: false,
    serviceReadApiResponseConsumerImplemented: false,
    parentSurfaceReadModelImplemented: false,
    parentSurfaceStatusImplemented: false,
    parentSurfaceStatusReadModelImplemented: false,
    parentSurfaceStatusReadModelParentSurfaceImplemented: false,
    parentSurfaceStatusReadModelParentSurfaceReadModelImplemented: false,
    serviceEventEmitted: false,
    agentProtocolImplemented: false,
    rustProtocolMirrored: false,
    portalUiRendered: false,
    portalResponseConsumerRendered: false,
    parentSurfaceRendered: false,
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
    responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  parentSurfaceReadModelProofRequiredCount: number;
  blockedBySourceFreshnessCount: number;
  blockedByCompilerDecisionCount: number;
};

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffCountsMatch(
  handoff: ParentSurfaceReadModelHandoffCounts
): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.parentSurfaceReadModelProofRequiredCount ===
      handoff.rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
      ).length &&
    handoff.blockedBySourceFreshnessCount ===
      handoff.rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
      ).length &&
    handoff.blockedByCompilerDecisionCount ===
      handoff.rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
      ).length
  );
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffNoClaimFlags
  ).every((key) => handoff[key] === false);
}

export function appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffMatchesStatusReadModelHandoff(
  statusReadModelHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue,
  parentSurfaceReadModelHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffStateValue
): boolean {
  if (
    statusReadModelHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
  ) {
    return (
      parentSurfaceReadModelHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    );
  }
  if (
    statusReadModelHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness
  ) {
    return (
      parentSurfaceReadModelHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    );
  }
  return (
    parentSurfaceReadModelHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
  );
}
