/* generated from crates/schema/src/app_game_timer_service_readiness.rs */

import { AppGameSourceGatedPolicyPreviewReadModelProjectionState } from './app-game-source-gated-policy-preview-read-model-rules';
import {
  AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated,
  AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated,
} from './generated-app-game-timer-service-readiness-values';

type ProjectionStateValue =
  (typeof AppGameSourceGatedPolicyPreviewReadModelProjectionState)[keyof typeof AppGameSourceGatedPolicyPreviewReadModelProjectionState];
type TimerHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated];
type TimerStatusStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated];
type TimerRuntimeReadinessStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated];
type TimerSchedulerPersistenceStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated];
type TimerServiceReadinessHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated];
type TimerServiceReadinessReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated];
type TimerServiceReadinessProtocolHandoffStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated];
type TimerServiceReadinessProtocolReadModelStateValue =
  (typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated];

type TimerHandoffRowLike = {
  readonly timerHandoffState: TimerHandoffStateValue;
  readonly timerRuntimeRequired: boolean;
  readonly manualProofRequired: boolean;
};

type TimerHandoffLike = {
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly timerSequenceCandidateCount: number;
  readonly sourceManualBlockedCount: number;
  readonly compilerManualBlockedCount: number;
  readonly rows: readonly TimerHandoffRowLike[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type TimerStatusRowLike = {
  readonly timerStatusState: TimerStatusStateValue;
  readonly timerRuntimeProofRequired: boolean;
  readonly sourceFreshnessProofRequired: boolean;
  readonly compilerDecisionProofRequired: boolean;
  readonly timerScheduled: boolean;
};

type TimerStatusLike = {
  readonly timerRuntimeProofRequiredCount: number;
  readonly sourceFreshnessProofRequiredCount: number;
  readonly compilerDecisionProofRequiredCount: number;
  readonly rows: readonly TimerStatusRowLike[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type TimerStatusProofRefsLike = {
  readonly timerRuntimeProofRef: string;
  readonly sourceFreshnessProofRef: string;
  readonly compilerDecisionProofRef: string;
};

type TimerRuntimeReadinessRowLike = {
  readonly runtimeReadinessState: TimerRuntimeReadinessStateValue;
  readonly timerRuntimeProofRequired: boolean;
  readonly schedulerPersistenceProofRequired: boolean;
  readonly auditProofRequired: boolean;
  readonly rollbackProofRequired: boolean;
  readonly timerScheduled: boolean;
};

type TimerRuntimeReadinessLike = {
  readonly runtimeProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly TimerRuntimeReadinessRowLike[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type TimerRuntimeReadinessProofRefsLike = {
  readonly timerRuntimeProofRef: string;
  readonly schedulerPersistenceProofRef: string;
  readonly auditProofRef: string;
  readonly rollbackProofRef: string;
};

type TimerSchedulerPersistenceRowLike = {
  readonly schedulerPersistenceState: TimerSchedulerPersistenceStateValue;
  readonly serviceTimerRuntimeProofRequired: boolean;
  readonly schedulerPersistenceProofRequired: boolean;
  readonly schedulerStateStoreProofRequired: boolean;
  readonly auditProofRequired: boolean;
  readonly rollbackProofRequired: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceRuntimeClaimed: boolean;
  readonly durableSchedulerStorageClaimed: boolean;
};

type TimerSchedulerPersistenceLike = {
  readonly schedulerPersistenceProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly TimerSchedulerPersistenceRowLike[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly timerScheduled: boolean;
  readonly schedulerPersistenceRuntimeClaimed: boolean;
  readonly durableSchedulerStorageClaimed: boolean;
  readonly auditRuntimeClaimed: boolean;
  readonly rollbackRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type TimerSchedulerPersistenceProofRefsLike = {
  readonly serviceTimerRuntimeProofRef: string;
  readonly schedulerPersistenceProofRef: string;
  readonly schedulerStateStoreProofRef: string;
  readonly auditProofRef: string;
  readonly rollbackProofRef: string;
};

type TimerServiceReadinessReadModelRowLike = {
  readonly serviceReadinessReadModelState: TimerServiceReadinessReadModelStateValue;
  readonly serviceReadinessProofRequired: boolean;
  readonly serviceReadApiProofRequired: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly timerScheduled: boolean;
  readonly adapterDispatchClaimed: boolean;
};

type TimerServiceReadinessReadModelLike = {
  readonly serviceReadModelProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly TimerServiceReadinessReadModelRowLike[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly serviceReadApiImplemented: boolean;
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

type TimerServiceReadinessProtocolReadModelRowLike = {
  readonly protocolReadModelState: TimerServiceReadinessProtocolReadModelStateValue;
  readonly requiredProtocolProofRefs: readonly unknown[];
  readonly agentProtocolContractImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
  readonly serviceCommandRegistered: boolean;
  readonly serviceEventEmitted: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly serviceReadModelEventEmitted: boolean;
};

type TimerServiceReadinessProtocolReadModelLike = {
  readonly protocolReadModelProofRequiredCount: number;
  readonly blockedBySourceFreshnessCount: number;
  readonly blockedByCompilerDecisionCount: number;
  readonly rows: readonly TimerServiceReadinessProtocolReadModelRowLike[];
  readonly agentProtocolContractImplemented: boolean;
  readonly rustProtocolMirrored: boolean;
  readonly serviceCommandRegistered: boolean;
  readonly serviceEventEmitted: boolean;
  readonly serviceReadApiImplemented: boolean;
  readonly serviceReadModelEventEmitted: boolean;
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

export const appGameSourceGatedPolicyPreviewTimerStateMatchesProjectionGenerated = (
  projectionState: ProjectionStateValue,
  timerHandoffState: TimerHandoffStateValue
) =>
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing) ||
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer) ||
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.CompilerManualRequiredBeforeTimer);

export const appGameSourceGatedPolicyPreviewTimerHandoffStateForProjectionGenerated = (
  projectionState: ProjectionStateValue
): TimerHandoffStateValue => {
  if (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible) {
    return AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing;
  }

  if (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible) {
    return AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer;
  }

  return AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.CompilerManualRequiredBeforeTimer;
};

export const countAppGameSourceGatedPolicyPreviewTimerHandoffRowsGenerated = (
  rows: readonly TimerHandoffRowLike[]
) => ({
  timerSequenceCandidateCount: rows.filter(
    (row) => row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing
  ).length,
  sourceManualBlockedCount: rows.filter(
    (row) =>
      row.timerHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer
  ).length,
  compilerManualBlockedCount: rows.filter(
    (row) =>
      row.timerHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.CompilerManualRequiredBeforeTimer
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerHandoffCountsMatchGenerated = (handoff: TimerHandoffLike) =>
  handoff.nativeAppRowCount > 0 &&
  handoff.nativeGameRowCount > 0 &&
  handoff.timerSequenceCandidateCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing &&
        row.timerRuntimeRequired &&
        !row.manualProofRequired
    ).length &&
  handoff.sourceManualBlockedCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer &&
        !row.timerRuntimeRequired &&
        row.manualProofRequired
    ).length &&
  handoff.compilerManualBlockedCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.CompilerManualRequiredBeforeTimer &&
        !row.timerRuntimeRequired &&
        row.manualProofRequired
    ).length;

export const appGameSourceGatedPolicyPreviewTimerHandoffHasNoRuntimeClaimsGenerated = (handoff: TimerHandoffLike) =>
  !handoff.serviceRuntimeEventClaimed &&
  !handoff.portalUiRendered &&
  !handoff.policyEvaluatorRuntimeClaimed &&
  !handoff.timerRuntimeClaimed &&
  !handoff.adapterDispatchClaimed &&
  !handoff.childDeliveryClaimed &&
  !handoff.platformEnforcementClaimed &&
  !handoff.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerStatusMatchesHandoffGenerated = (
  handoffState: TimerHandoffStateValue,
  timerStatusState: TimerStatusStateValue
) =>
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.CompilerManualRequiredBeforeTimer &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.CompilerDecisionProofRequired);

export const appGameSourceGatedPolicyPreviewTimerStatusStateForHandoffGenerated = (
  handoffState: TimerHandoffStateValue
): TimerStatusStateValue => {
  if (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.ReadyForTimerSequencing) {
    return AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired;
  }

  if (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated.SourceManualRequiredBeforeTimer) {
    return AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired;
  }

  return AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.CompilerDecisionProofRequired;
};

export const countAppGameSourceGatedPolicyPreviewTimerStatusRowsGenerated = (rows: readonly TimerStatusRowLike[]) => ({
  timerRuntimeProofRequiredCount: rows.filter(
    (row) => row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired
  ).length,
  sourceFreshnessProofRequiredCount: rows.filter(
    (row) =>
      row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired
  ).length,
  compilerDecisionProofRequiredCount: rows.filter(
    (row) =>
      row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.CompilerDecisionProofRequired
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerStatusRequiredProofRefsGenerated = (
  refs: TimerStatusProofRefsLike,
  timerStatusState: TimerStatusStateValue
) => {
  switch (timerStatusState) {
    case AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired:
      return [refs.timerRuntimeProofRef];
    case AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired:
      return [refs.sourceFreshnessProofRef];
    default:
      return [refs.compilerDecisionProofRef];
  }
};

export const appGameSourceGatedPolicyPreviewTimerStatusCountsMatchGenerated = (status: TimerStatusLike) =>
  status.timerRuntimeProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired &&
        row.timerRuntimeProofRequired &&
        !row.sourceFreshnessProofRequired &&
        !row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length &&
  status.sourceFreshnessProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState ===
          AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired &&
        !row.timerRuntimeProofRequired &&
        row.sourceFreshnessProofRequired &&
        !row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length &&
  status.compilerDecisionProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState ===
          AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.CompilerDecisionProofRequired &&
        !row.timerRuntimeProofRequired &&
        !row.sourceFreshnessProofRequired &&
        row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerStatusHasNoRuntimeClaimsGenerated = (status: TimerStatusLike) =>
  !status.serviceRuntimeEventClaimed &&
  !status.portalUiRendered &&
  !status.policyEvaluatorRuntimeClaimed &&
  !status.timerRuntimeClaimed &&
  !status.timerScheduled &&
  !status.adapterDispatchClaimed &&
  !status.childDeliveryClaimed &&
  !status.platformEnforcementClaimed &&
  !status.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatusGenerated = (
  timerStatusState: TimerStatusStateValue,
  runtimeReadinessState: TimerRuntimeReadinessStateValue
) =>
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired &&
    runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired) ||
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired &&
    runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness) ||
  (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.CompilerDecisionProofRequired &&
    runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedByCompilerDecision);
export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateForStatusGenerated = (
  timerStatusState: TimerStatusStateValue
): TimerRuntimeReadinessStateValue => {
  if (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.TimerRuntimeProofRequired) {
    return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired;
  }

  if (timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated.SourceFreshnessProofRequired) {
    return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness;
  }

  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedByCompilerDecision;
};

export const countAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowsGenerated = (
  rows: readonly TimerRuntimeReadinessRowLike[]
) => ({
  runtimeProofRequiredCount: rows.filter(
    (row) =>
      row.runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired
  ).length,
  blockedBySourceFreshnessCount: rows.filter(
    (row) =>
      row.runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness
  ).length,
  blockedByCompilerDecisionCount: rows.filter(
    (row) =>
      row.runtimeReadinessState ===
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedByCompilerDecision
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessRequiredProofRefsGenerated = (
  refs: TimerRuntimeReadinessProofRefsLike,
  runtimeReadinessState: TimerRuntimeReadinessStateValue,
  inheritedProofRefs: readonly string[]
) =>
  runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired
    ? [refs.timerRuntimeProofRef, refs.schedulerPersistenceProofRef, refs.auditProofRef, refs.rollbackProofRef]
    : [...inheritedProofRefs];

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessCountsMatchGenerated = (
  readiness: TimerRuntimeReadinessLike
) =>
  readiness.runtimeProofRequiredCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState ===
          AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired &&
        row.timerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.auditProofRequired &&
        row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  readiness.blockedBySourceFreshnessCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState ===
          AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness &&
        !row.timerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  readiness.blockedByCompilerDecisionCount ===
    readiness.rows.filter(
      (row) =>
        row.runtimeReadinessState ===
          AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedByCompilerDecision &&
        !row.timerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerRuntimeReadinessHasNoRuntimeClaimsGenerated = (
  readiness: TimerRuntimeReadinessLike
) =>
  !readiness.serviceRuntimeEventClaimed &&
  !readiness.portalUiRendered &&
  !readiness.policyEvaluatorRuntimeClaimed &&
  !readiness.timerRuntimeClaimed &&
  !readiness.timerScheduled &&
  !readiness.schedulerPersistenceClaimed &&
  !readiness.auditRuntimeClaimed &&
  !readiness.rollbackRuntimeClaimed &&
  !readiness.adapterDispatchClaimed &&
  !readiness.childDeliveryClaimed &&
  !readiness.platformEnforcementClaimed &&
  !readiness.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadinessGenerated = (
  runtimeReadinessState: TimerRuntimeReadinessStateValue,
  schedulerPersistenceState: TimerSchedulerPersistenceStateValue
) =>
  (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.SchedulerPersistenceProofRequired) ||
  (runtimeReadinessState ===
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedBySourceFreshness) ||
  (runtimeReadinessState ===
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedByCompilerDecision &&
    schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedByCompilerDecision);

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateForRuntimeReadinessGenerated = (
  runtimeReadinessState: TimerRuntimeReadinessStateValue
): TimerSchedulerPersistenceStateValue => {
  if (
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.RuntimeProofRequired
  ) {
    return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.SchedulerPersistenceProofRequired;
  }

  if (
    runtimeReadinessState ===
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated.BlockedBySourceFreshness
  ) {
    return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedBySourceFreshness;
  }

  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedByCompilerDecision;
};

export const countAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowsGenerated = (
  rows: readonly TimerSchedulerPersistenceRowLike[]
) => ({
  schedulerPersistenceProofRequiredCount: rows.filter(
    (row) =>
      row.schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.SchedulerPersistenceProofRequired
  ).length,
  blockedBySourceFreshnessCount: rows.filter(
    (row) =>
      row.schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedBySourceFreshness
  ).length,
  blockedByCompilerDecisionCount: rows.filter(
    (row) =>
      row.schedulerPersistenceState ===
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedByCompilerDecision
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRequiredProofRefsGenerated = (
  refs: TimerSchedulerPersistenceProofRefsLike,
  schedulerPersistenceState: TimerSchedulerPersistenceStateValue,
  inheritedProofRefs: readonly string[]
) =>
  schedulerPersistenceState ===
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.SchedulerPersistenceProofRequired
    ? [
        refs.serviceTimerRuntimeProofRef,
        refs.schedulerPersistenceProofRef,
        refs.schedulerStateStoreProofRef,
        refs.auditProofRef,
        refs.rollbackProofRef,
      ]
    : [...inheritedProofRefs];

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceCountsMatchGenerated = (
  persistence: TimerSchedulerPersistenceLike
) =>
  persistence.schedulerPersistenceProofRequiredCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.SchedulerPersistenceProofRequired &&
        row.serviceTimerRuntimeProofRequired &&
        row.schedulerPersistenceProofRequired &&
        row.schedulerStateStoreProofRequired &&
        row.auditProofRequired &&
        row.rollbackProofRequired &&
        !row.timerScheduled &&
        !row.schedulerPersistenceRuntimeClaimed &&
        !row.durableSchedulerStorageClaimed
    ).length &&
  persistence.blockedBySourceFreshnessCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedBySourceFreshness &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length &&
  persistence.blockedByCompilerDecisionCount ===
    persistence.rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
          AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated.BlockedByCompilerDecision &&
        !row.serviceTimerRuntimeProofRequired &&
        !row.schedulerPersistenceProofRequired &&
        !row.schedulerStateStoreProofRequired &&
        !row.auditProofRequired &&
        !row.rollbackProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceHasNoRuntimeClaimsGenerated = (
  persistence: TimerSchedulerPersistenceLike
) =>
  !persistence.serviceRuntimeEventClaimed &&
  !persistence.portalUiRendered &&
  !persistence.policyEvaluatorRuntimeClaimed &&
  !persistence.timerRuntimeClaimed &&
  !persistence.timerScheduled &&
  !persistence.schedulerPersistenceRuntimeClaimed &&
  !persistence.durableSchedulerStorageClaimed &&
  !persistence.auditRuntimeClaimed &&
  !persistence.rollbackRuntimeClaimed &&
  !persistence.adapterDispatchClaimed &&
  !persistence.childDeliveryClaimed &&
  !persistence.platformEnforcementClaimed &&
  !persistence.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelMatchesHandoffGenerated = (
  handoffState: TimerServiceReadinessHandoffStateValue,
  readModelState: TimerServiceReadinessReadModelStateValue
) =>
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated.ServiceReadApiProofRequired &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.ServiceReadModelProofRequired) ||
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated.BlockedBySourceFreshness &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedBySourceFreshness) ||
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated.BlockedByCompilerDecision &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedByCompilerDecision);

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateForHandoffGenerated = (
  handoffState: TimerServiceReadinessHandoffStateValue
): TimerServiceReadinessReadModelStateValue => {
  if (
    handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated.ServiceReadApiProofRequired
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.ServiceReadModelProofRequired;
  }

  if (
    handoffState === AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated.BlockedBySourceFreshness
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedBySourceFreshness;
  }

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedByCompilerDecision;
};

export const countAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowsGenerated = (
  rows: readonly TimerServiceReadinessReadModelRowLike[]
) => ({
  serviceReadModelProofRequiredCount: rows.filter(
    (row) =>
      row.serviceReadinessReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.ServiceReadModelProofRequired
  ).length,
  blockedBySourceFreshnessCount: rows.filter(
    (row) =>
      row.serviceReadinessReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedBySourceFreshness
  ).length,
  blockedByCompilerDecisionCount: rows.filter(
    (row) =>
      row.serviceReadinessReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedByCompilerDecision
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelCountsMatchGenerated = (
  readModel: TimerServiceReadinessReadModelLike
) =>
  readModel.serviceReadModelProofRequiredCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.ServiceReadModelProofRequired &&
        row.serviceReadinessProofRequired &&
        row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented &&
        !row.timerScheduled &&
        !row.adapterDispatchClaimed
    ).length &&
  readModel.blockedBySourceFreshnessCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedBySourceFreshness &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length &&
  readModel.blockedByCompilerDecisionCount ===
    readModel.rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated.BlockedByCompilerDecision &&
        !row.serviceReadinessProofRequired &&
        !row.serviceReadApiProofRequired &&
        !row.serviceReadApiImplemented
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelHasNoRuntimeClaimsGenerated = (
  readModel: TimerServiceReadinessReadModelLike
) =>
  !readModel.serviceRuntimeEventClaimed &&
  !readModel.serviceReadApiImplemented &&
  !readModel.portalUiRendered &&
  !readModel.policyEvaluatorRuntimeClaimed &&
  !readModel.timerRuntimeClaimed &&
  !readModel.timerScheduled &&
  !readModel.schedulerPersistenceRuntimeClaimed &&
  !readModel.durableSchedulerStorageClaimed &&
  !readModel.auditRuntimeClaimed &&
  !readModel.durableAuditLogClaimed &&
  !readModel.rollbackRuntimeClaimed &&
  !readModel.rollbackExecutionClaimed &&
  !readModel.adapterDispatchClaimed &&
  !readModel.childDeliveryClaimed &&
  !readModel.platformEnforcementClaimed &&
  !readModel.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelMatchesHandoffGenerated = (
  handoffState: TimerServiceReadinessProtocolHandoffStateValue,
  readModelState: TimerServiceReadinessProtocolReadModelStateValue
) =>
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated.ProtocolProofRequired &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.ProtocolReadModelProofRequired) ||
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated.BlockedBySourceFreshness &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedBySourceFreshness) ||
  (handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated.BlockedByCompilerDecision &&
    readModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedByCompilerDecision);

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateForHandoffGenerated = (
  handoffState: TimerServiceReadinessProtocolHandoffStateValue
): TimerServiceReadinessProtocolReadModelStateValue => {
  if (
    handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated.ProtocolProofRequired
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.ProtocolReadModelProofRequired;
  }

  if (
    handoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated.BlockedBySourceFreshness
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedBySourceFreshness;
  }

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedByCompilerDecision;
};

export const countAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowsGenerated = (
  rows: readonly TimerServiceReadinessProtocolReadModelRowLike[]
) => ({
  protocolReadModelProofRequiredCount: rows.filter(
    (row) =>
      row.protocolReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.ProtocolReadModelProofRequired
  ).length,
  blockedBySourceFreshnessCount: rows.filter(
    (row) =>
      row.protocolReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedBySourceFreshness
  ).length,
  blockedByCompilerDecisionCount: rows.filter(
    (row) =>
      row.protocolReadModelState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedByCompilerDecision
  ).length,
});

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelCountsMatchGenerated = (
  readModel: TimerServiceReadinessProtocolReadModelLike
) =>
  readModel.protocolReadModelProofRequiredCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.ProtocolReadModelProofRequired &&
        row.requiredProtocolProofRefs.length > 0 &&
        !row.agentProtocolContractImplemented &&
        !row.rustProtocolMirrored &&
        !row.serviceCommandRegistered &&
        !row.serviceEventEmitted &&
        !row.serviceReadApiImplemented &&
        !row.serviceReadModelEventEmitted
    ).length &&
  readModel.blockedBySourceFreshnessCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedBySourceFreshness &&
        row.requiredProtocolProofRefs.length === 0
    ).length &&
  readModel.blockedByCompilerDecisionCount ===
    readModel.rows.filter(
      (row) =>
        row.protocolReadModelState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated.BlockedByCompilerDecision &&
        row.requiredProtocolProofRefs.length === 0
    ).length;

export const appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelHasNoRuntimeClaimsGenerated = (
  readModel: TimerServiceReadinessProtocolReadModelLike
) =>
  !readModel.agentProtocolContractImplemented &&
  !readModel.rustProtocolMirrored &&
  !readModel.serviceCommandRegistered &&
  !readModel.serviceEventEmitted &&
  !readModel.serviceReadApiImplemented &&
  !readModel.serviceReadModelEventEmitted &&
  !readModel.portalUiRendered &&
  !readModel.policyEvaluatorRuntimeClaimed &&
  !readModel.timerRuntimeClaimed &&
  !readModel.timerScheduled &&
  !readModel.schedulerPersistenceRuntimeClaimed &&
  !readModel.durableSchedulerStorageClaimed &&
  !readModel.auditRuntimeClaimed &&
  !readModel.durableAuditLogClaimed &&
  !readModel.rollbackRuntimeClaimed &&
  !readModel.rollbackExecutionClaimed &&
  !readModel.adapterDispatchClaimed &&
  !readModel.childDeliveryClaimed &&
  !readModel.platformEnforcementClaimed &&
  !readModel.rawPrivateSourceRowsIncluded;
