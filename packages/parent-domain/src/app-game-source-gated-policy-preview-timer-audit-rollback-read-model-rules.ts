import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState } from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState = {
  AuditRollbackReadModelProofRequired: 'audit-rollback-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState];
type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims = [
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags = {
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

type SourceGatedPolicyPreviewTimerAuditRollbackReadModelLike = {
  readonly auditRollbackReadModelProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly readModelState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue;
    readonly serviceTimerRuntimeProofRequired: boolean;
    readonly schedulerPersistenceProofRequired: boolean;
    readonly schedulerStateStoreProofRequired: boolean;
    readonly auditTrailProofRequired: boolean;
    readonly rollbackPlanProofRequired: boolean;
    readonly auditRollbackReadModelProofRequired: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelCountsMatch = (
  readModel: SourceGatedPolicyPreviewTimerAuditRollbackReadModelLike
) =>
  readModel.auditRollbackReadModelProofRequiredCount ===
    readModel.rows.filter(
      (row) =>
        row.readModelState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired &&
        row.serviceTimerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.schedulerStateStoreProofRequired &&
        row.auditTrailProofRequired &&
        row.rollbackPlanProofRequired &&
        row.auditRollbackReadModelProofRequired &&
        !row.timerScheduled &&
        !row.durableAuditLogClaimed &&
        !row.rollbackExecutionClaimed
    ).length &&
  readModel.blockedBySourceFreshnessCount ===
    readModel.rows.filter(
      (row) =>
        row.readModelState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.timerScheduled
    ).length &&
  readModel.blockedByCompilerDecisionCount ===
    readModel.rows.filter(
      (row) =>
        row.readModelState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelHasNoRuntimeClaims = (
  readModel: SourceGatedPolicyPreviewTimerAuditRollbackReadModelLike
) => noRuntimeClaimValues(readModel).every((value) => value === false);

function noRuntimeClaimValues(readModel: SourceGatedPolicyPreviewTimerAuditRollbackReadModelLike) {
  return [
    readModel.serviceRuntimeEventClaimed,
    readModel.portalUiRendered,
    readModel.policyEvaluatorRuntimeClaimed,
    readModel.timerRuntimeClaimed,
    readModel.timerScheduled,
    readModel.schedulerPersistenceRuntimeClaimed,
    readModel.durableSchedulerStorageClaimed,
    readModel.auditRuntimeClaimed,
    readModel.durableAuditLogClaimed,
    readModel.rollbackRuntimeClaimed,
    readModel.rollbackExecutionClaimed,
    readModel.adapterDispatchClaimed,
    readModel.childDeliveryClaimed,
    readModel.platformEnforcementClaimed,
    readModel.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelMatchesHandoff = (
  auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue,
  readModelState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue
) =>
  (auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired) ||
  (auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness &&
    readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness) ||
  (auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision &&
    readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision);
