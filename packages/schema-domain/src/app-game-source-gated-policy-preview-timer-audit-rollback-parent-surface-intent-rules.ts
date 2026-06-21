import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState } from './app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState = {
  AuditRollbackParentSurfaceProofRequired: 'audit-rollback-parent-surface-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState];
type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
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

type SourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentLike = {
  readonly auditRollbackParentSurfaceProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly parentSurfaceIntentState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue;
    readonly serviceTimerRuntimeProofRequired: boolean;
    readonly schedulerPersistenceProofRequired: boolean;
    readonly schedulerStateStoreProofRequired: boolean;
    readonly auditTrailProofRequired: boolean;
    readonly rollbackPlanProofRequired: boolean;
    readonly auditRollbackReadModelProofRequired: boolean;
    readonly parentSurfaceProofRequired: boolean;
    readonly timerScheduled: boolean;
    readonly durableAuditLogClaimed: boolean;
    readonly rollbackExecutionClaimed: boolean;
  }[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceRuntimeClaimed: boolean;
  readonly durableSchedulerStorageClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly durableAuditLogClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly rollbackExecutionClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentCountsMatch = (
  intent: SourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentLike
) =>
  intent.auditRollbackParentSurfaceProofRequiredCount ===
    intent.rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired &&
        row.serviceTimerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.schedulerStateStoreProofRequired &&
        row.auditTrailProofRequired &&
        row.rollbackPlanProofRequired &&
        row.auditRollbackReadModelProofRequired &&
        row.parentSurfaceProofRequired &&
        !row.timerScheduled &&
        !row.durableAuditLogClaimed &&
        !row.rollbackExecutionClaimed
    ).length &&
  intent.blockedBySourceFreshnessCount ===
    intent.rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.parentSurfaceProofRequired &&
        !row.timerScheduled
    ).length &&
  intent.blockedByCompilerDecisionCount ===
    intent.rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.parentSurfaceProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentHasNoRuntimeClaims = (
  intent: SourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentLike
) => noRuntimeClaimValues(intent).every((value) => value === false);

function noRuntimeClaimValues(intent: SourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentLike) {
  return [
    intent.serviceRuntimeEventClaimed,
    intent.portalUiRendered,
    intent.policyEvaluatorRuntimeClaimed,
    intent.timerRuntimeClaimed,
    intent.timerScheduled,
    intent.schedulerPersistenceRuntimeClaimed,
    intent.durableSchedulerStorageClaimed,
    intent.auditRuntimeClaimed,
    intent.durableAuditLogClaimed,
    intent.rollbackRuntimeClaimed,
    intent.rollbackExecutionClaimed,
    intent.adapterDispatchClaimed,
    intent.childDeliveryClaimed,
    intent.platformEnforcementClaimed,
    intent.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentMatchesReadModel = (
  readModelState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue,
  intentState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue
) =>
  (readModelState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired &&
    intentState ===
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired) ||
  (readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness &&
    intentState ===
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness) ||
  (readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision &&
    intentState ===
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision);

