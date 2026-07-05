/* generated from crates/schema/src/app_game_timer_service_readiness.rs */

export const AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated = {
  ReadyForTimerSequencing: 'ready-for-timer-sequencing',
  SourceManualRequiredBeforeTimer: 'source-manual-required-before-timer',
  CompilerManualRequiredBeforeTimer: 'compiler-manual-required-before-timer',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

export const AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated = {
  TimerRuntimeProofRequired: 'timer-runtime-proof-required',
  SourceFreshnessProofRequired: 'source-freshness-proof-required',
  CompilerDecisionProofRequired: 'compiler-decision-proof-required',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated = {
  RuntimeProofRequired: 'runtime-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-scheduler-persistence',
  'no-audit-runtime',
  'no-rollback-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  schedulerPersistenceClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated = {
  SchedulerPersistenceProofRequired: 'scheduler-persistence-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-timer-scheduled',
  'no-scheduler-persistence-runtime',
  'no-durable-scheduler-storage',
  'no-audit-runtime',
  'no-rollback-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  timerScheduled: false,
  schedulerPersistenceRuntimeClaimed: false,
  durableSchedulerStorageClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated = {
  ServiceReadApiProofRequired: 'service-read-api-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated = {
  ServiceReadModelProofRequired: 'service-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-service-read-api-implemented',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  serviceReadApiImplemented: false,
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated = {
  ProtocolProofRequired: 'protocol-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated = {
  ProtocolReadModelProofRequired: 'protocol-read-model-proof-required',
  BlockedBySourceFreshness: 'blocked-by-source-freshness',
  BlockedByCompilerDecision: 'blocked-by-compiler-decision',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaimsGenerated = [
  'no-agent-protocol-contract-implemented',
  'no-rust-protocol-mirrored',
  'no-service-command-registered',
  'no-service-event-emitted',
  'no-service-read-api-implemented',
  'no-service-read-model-event-emitted',
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlagsGenerated = {
  agentProtocolContractImplemented: false,
  rustProtocolMirrored: false,
  serviceCommandRegistered: false,
  serviceEventEmitted: false,
  serviceReadApiImplemented: false,
  serviceReadModelEventEmitted: false,
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
