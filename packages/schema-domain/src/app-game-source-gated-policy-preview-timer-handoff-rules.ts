import { AppGameSourceGatedPolicyPreviewReadModelProjectionState } from './app-game-source-gated-policy-preview-read-model-rules';

export const AppGameSourceGatedPolicyPreviewTimerHandoffState = {
  ReadyForTimerSequencing: 'ready-for-timer-sequencing',
  SourceManualRequiredBeforeTimer: 'source-manual-required-before-timer',
  CompilerManualRequiredBeforeTimer: 'compiler-manual-required-before-timer',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaims = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type SourceGatedPolicyPreviewTimerHandoffLike = {
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly timerSequenceCandidateCount: number;
  readonly sourceManualBlockedCount: number;
  readonly compilerManualBlockedCount: number;
  readonly rows: readonly {
    readonly timerHandoffState: string;
    readonly timerRuntimeRequired: boolean;
    readonly manualProofRequired: boolean;
  }[];
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewTimerHandoffCountsMatch = (
  handoff: SourceGatedPolicyPreviewTimerHandoffLike
) =>
  handoff.nativeAppRowCount > 0 &&
  handoff.nativeGameRowCount > 0 &&
  handoff.timerSequenceCandidateCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing &&
        row.timerRuntimeRequired &&
        !row.manualProofRequired
    ).length &&
  handoff.sourceManualBlockedCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer &&
        !row.timerRuntimeRequired &&
        row.manualProofRequired
    ).length &&
  handoff.compilerManualBlockedCount ===
    handoff.rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer &&
        !row.timerRuntimeRequired &&
        row.manualProofRequired
    ).length;

export const appGameSourceGatedPolicyPreviewTimerHandoffHasNoRuntimeClaims = (
  handoff: SourceGatedPolicyPreviewTimerHandoffLike
) =>
  !handoff.serviceRuntimeEventClaimed &&
  !handoff.portalUiRendered &&
  !handoff.policyEvaluatorRuntimeClaimed &&
  !handoff.timerRuntimeClaimed &&
  !handoff.adapterDispatchClaimed &&
  !handoff.childDeliveryClaimed &&
  !handoff.platformEnforcementClaimed &&
  !handoff.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewTimerStateMatchesProjection = (
  projectionState: string,
  timerHandoffState: string
) =>
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing) ||
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer) ||
  (projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible &&
    timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer);

