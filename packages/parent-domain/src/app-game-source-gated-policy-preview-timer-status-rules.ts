import { AppGameSourceGatedPolicyPreviewTimerHandoffState } from './app-game-source-gated-policy-preview-timer-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerStatusState = {
  TimerRuntimeProofRequired: 'timer-runtime-proof-required',
  SourceFreshnessProofRequired: 'source-freshness-proof-required',
  CompilerDecisionProofRequired: 'compiler-decision-proof-required',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaims = [
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

export const AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlags = {
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

type SourceGatedPolicyPreviewTimerStatusLike = {
  readonly timerRuntimeProofRequiredCount: number;
  readonly sourceFreshnessProofRequiredCount: number;
  readonly compilerDecisionProofRequiredCount: number;
  readonly rows: readonly {
    readonly timerStatusState: string;
    readonly timerRuntimeProofRequired: boolean;
    readonly sourceFreshnessProofRequired: boolean;
    readonly compilerDecisionProofRequired: boolean;
    readonly timerScheduled: boolean;
  }[];
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

export const appGameSourceGatedPolicyPreviewTimerStatusCountsMatch = (
  status: SourceGatedPolicyPreviewTimerStatusLike
) =>
  status.timerRuntimeProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState ===
          AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired &&
        row.timerRuntimeProofRequired &&
        !row.sourceFreshnessProofRequired &&
        !row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length &&
  status.sourceFreshnessProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState ===
          AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired &&
        !row.timerRuntimeProofRequired &&
        row.sourceFreshnessProofRequired &&
        !row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length &&
  status.compilerDecisionProofRequiredCount ===
    status.rows.filter(
      (row) =>
        row.timerStatusState ===
          AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired &&
        !row.timerRuntimeProofRequired &&
        !row.sourceFreshnessProofRequired &&
        row.compilerDecisionProofRequired &&
        !row.timerScheduled
    ).length;

export const appGameSourceGatedPolicyPreviewTimerStatusHasNoRuntimeClaims = (
  status: SourceGatedPolicyPreviewTimerStatusLike
) =>
  !status.serviceRuntimeEventClaimed &&
  !status.portalUiRendered &&
  !status.policyEvaluatorRuntimeClaimed &&
  !status.timerRuntimeClaimed &&
  !status.timerScheduled &&
  !status.adapterDispatchClaimed &&
  !status.childDeliveryClaimed &&
  !status.platformEnforcementClaimed &&
  !status.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerStatusMatchesHandoff = (
  handoffState: string,
  timerStatusState: string
) =>
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired) ||
  (handoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer &&
    timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired);
