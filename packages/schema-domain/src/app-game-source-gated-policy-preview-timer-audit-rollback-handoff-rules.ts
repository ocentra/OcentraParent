import { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState } from './app-game-source-gated-policy-preview-timer-scheduler-persistence-rules';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState = {
  AuditRollbackProofRequired: 'audit-rollback-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState];
type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState)[keyof typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState];

export const RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims = [
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags = {
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

type SourceGatedPolicyPreviewTimerAuditRollbackHandoffLike = {
  readonly auditRollbackProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly {
    readonly auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue;
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

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffCountsMatch = (
  handoff: SourceGatedPolicyPreviewTimerAuditRollbackHandoffLike
) =>
  handoff.auditRollbackProofRequiredCount ===
    handoff.rows.filter(
      (row) =>
        row.auditRollbackState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired &&
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
  handoff.blockedBySourceFreshnessCount ===
    handoff.rows.filter(
      (row) =>
        row.auditRollbackState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.timerScheduled
    ).length &&
  handoff.blockedByCompilerDecisionCount ===
    handoff.rows.filter(
      (row) =>
        row.auditRollbackState ===
          AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditTrailProofRequired &&
        !row.rollbackPlanProofRequired &&
        !row.auditRollbackReadModelProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffHasNoRuntimeClaims = (
  handoff: SourceGatedPolicyPreviewTimerAuditRollbackHandoffLike
) => noRuntimeClaimValues(handoff).every((value) => value === false);

function noRuntimeClaimValues(handoff: SourceGatedPolicyPreviewTimerAuditRollbackHandoffLike) {
  return [
    handoff.serviceRuntimeEventClaimed,
    handoff.portalUiRendered,
    handoff.policyEvaluatorRuntimeClaimed,
    handoff.timerRuntimeClaimed,
    handoff.timerScheduled,
    handoff.schedulerPersistenceRuntimeClaimed,
    handoff.durableSchedulerStorageClaimed,
    handoff.auditRuntimeClaimed,
    handoff.durableAuditLogClaimed,
    handoff.rollbackRuntimeClaimed,
    handoff.rollbackExecutionClaimed,
    handoff.adapterDispatchClaimed,
    handoff.childDeliveryClaimed,
    handoff.platformEnforcementClaimed,
    handoff.rawPrivateSourceRowsIncluded,
  ] as const;
}

export const appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffMatchesSchedulerPersistence = (
  schedulerPersistenceState: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateValue,
  auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue
) =>
  (schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired &&
    auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired) ||
  (schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness &&
    auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness) ||
  (schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision &&
    auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision);
