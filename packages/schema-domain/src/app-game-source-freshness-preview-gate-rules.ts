import { AppGamePolicyPreviewStatus, AppGamePolicyPreviewTargetDomain } from './app-game-policy-preview-handoff-rules';
import { AppGameSourceFreshnessPolicyReadinessState } from './app-game-source-freshness-policy-consumption-values';

export const AppGameSourceFreshnessPreviewGateStatus = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateState = {
  SourceFresh: 'source-fresh',
  SourceManualRequired: 'source-manual-required',
  CompilerManualRequired: 'compiler-manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

type PreviewGateStatusValue =
  (typeof AppGameSourceFreshnessPreviewGateStatus)[keyof typeof AppGameSourceFreshnessPreviewGateStatus];
type PreviewGateStateValue =
  (typeof AppGameSourceFreshnessPreviewGateState)[keyof typeof AppGameSourceFreshnessPreviewGateState];
type PreviewTargetDomainValue =
  (typeof AppGamePolicyPreviewTargetDomain)[keyof typeof AppGamePolicyPreviewTargetDomain];

type SourceFreshnessPreviewGateRowLike = {
  readonly targetDomain: PreviewTargetDomainValue;
  readonly previewStatus: PreviewGateStatusValue;
  readonly gateState: PreviewGateStateValue;
  readonly sourcePolicyCompileAllowed: boolean;
  readonly sourceReadinessState: unknown;
  readonly sourceRequirementStates: ReadonlyArray<unknown>;
  readonly sourceEvidenceRefs: ReadonlyArray<unknown>;
  readonly previewRow: {
    readonly targetDomain: PreviewTargetDomainValue;
    readonly previewStatus: unknown;
  } | null;
  readonly compiledDecisionProvided: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
};

type SourceFreshnessPreviewGateReadModelLike = {
  readonly rows: ReadonlyArray<SourceFreshnessPreviewGateRowLike>;
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly previewReadyCount: number;
  readonly manualRequiredCount: number;
  readonly sourceManualRequiredCount: number;
  readonly compilerManualRequiredCount: number;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
};

export const appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaims = (row: SourceFreshnessPreviewGateRowLike) =>
  !row.policyEvaluatorRuntimeClaimed &&
  !row.timerRuntimeClaimed &&
  !row.adapterDispatchClaimed &&
  !row.childDeliveryClaimed &&
  !row.platformEnforcementClaimed;

export const appGameSourceFreshnessPreviewGateRowMatchesSourceState = (row: SourceFreshnessPreviewGateRowLike) => {
  if (row.sourceReadinessState === AppGameSourceFreshnessPolicyReadinessState.ManualRequired) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatus.ManualRequired &&
      row.gateState === AppGameSourceFreshnessPreviewGateState.SourceManualRequired &&
      !row.sourcePolicyCompileAllowed &&
      row.previewRow === null &&
      !row.compiledDecisionProvided &&
      row.sourceRequirementStates.length > 0
    );
  }

  if (row.sourceReadinessState !== AppGameSourceFreshnessPolicyReadinessState.PolicyReady) {
    return false;
  }

  return (
    row.sourcePolicyCompileAllowed &&
    row.compiledDecisionProvided &&
    row.previewRow !== null &&
    row.previewRow.targetDomain === row.targetDomain &&
    row.sourceEvidenceRefs.length > 0
  );
};

export const appGameSourceFreshnessPreviewGateRowMatchesPreviewState = (row: SourceFreshnessPreviewGateRowLike) => {
  if (row.previewRow === null) {
    return row.gateState === AppGameSourceFreshnessPreviewGateState.SourceManualRequired;
  }

  if (row.previewRow.previewStatus === AppGamePolicyPreviewStatus.PreviewReady) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatus.PreviewReady &&
      row.gateState === AppGameSourceFreshnessPreviewGateState.SourceFresh
    );
  }

  if (row.previewRow.previewStatus === AppGamePolicyPreviewStatus.ManualRequired) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatus.ManualRequired &&
      row.gateState === AppGameSourceFreshnessPreviewGateState.CompilerManualRequired
    );
  }

  return false;
};

export const countAppGameSourceFreshnessPreviewGateRows = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  targetDomain: PreviewTargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGameSourceFreshnessPreviewGateStatuses = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  previewStatus: PreviewGateStatusValue
) => rows.filter((row) => row.previewStatus === previewStatus).length;

export const countAppGameSourceFreshnessPreviewGateStates = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  gateState: PreviewGateStateValue
) => rows.filter((row) => row.gateState === gateState).length;

export const countAppGameSourceFreshnessPreviewGateReadModelRows = (
  rows: readonly SourceFreshnessPreviewGateRowLike[]
) => ({
  nativeAppRowCount: countAppGameSourceFreshnessPreviewGateRows(rows, AppGamePolicyPreviewTargetDomain.NativeApp),
  nativeGameRowCount: countAppGameSourceFreshnessPreviewGateRows(rows, AppGamePolicyPreviewTargetDomain.NativeGame),
  previewReadyCount: countAppGameSourceFreshnessPreviewGateStatuses(
    rows,
    AppGameSourceFreshnessPreviewGateStatus.PreviewReady
  ),
  manualRequiredCount: countAppGameSourceFreshnessPreviewGateStatuses(
    rows,
    AppGameSourceFreshnessPreviewGateStatus.ManualRequired
  ),
  sourceManualRequiredCount: countAppGameSourceFreshnessPreviewGateStates(
    rows,
    AppGameSourceFreshnessPreviewGateState.SourceManualRequired
  ),
  compilerManualRequiredCount: countAppGameSourceFreshnessPreviewGateStates(
    rows,
    AppGameSourceFreshnessPreviewGateState.CompilerManualRequired
  ),
});

export const appGameSourceFreshnessPreviewGateReadModelCountsMatchRows = (
  readModel: SourceFreshnessPreviewGateReadModelLike
) =>
  readModel.nativeAppRowCount ===
    countAppGameSourceFreshnessPreviewGateRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeApp) &&
  readModel.nativeGameRowCount ===
    countAppGameSourceFreshnessPreviewGateRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeGame) &&
  readModel.previewReadyCount ===
    countAppGameSourceFreshnessPreviewGateStatuses(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStatus.PreviewReady
    ) &&
  readModel.manualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStatuses(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStatus.ManualRequired
    ) &&
  readModel.sourceManualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStates(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateState.SourceManualRequired
    ) &&
  readModel.compilerManualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStates(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateState.CompilerManualRequired
    );

export const appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaims = (
  readModel: SourceFreshnessPreviewGateReadModelLike
) =>
  !readModel.policyEvaluatorRuntimeClaimed &&
  !readModel.timerRuntimeClaimed &&
  !readModel.adapterDispatchClaimed &&
  !readModel.childDeliveryClaimed &&
  !readModel.platformEnforcementClaimed;
