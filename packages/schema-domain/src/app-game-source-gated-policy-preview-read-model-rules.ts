import { AppGamePolicyPreviewStatus, AppGamePolicyPreviewTargetDomain } from './app-game-policy-preview-handoff-rules';
import {
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
} from './app-game-source-freshness-preview-gate-rules';

export const AppGameSourceGatedPolicyPreviewReadModelProjectionState = {
  PreviewReadyVisible: 'preview-ready-visible',
  SourceManualRequiredVisible: 'source-manual-required-visible',
  CompilerManualRequiredVisible: 'compiler-manual-required-visible',
} as const;

export const AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary = {
  RedactedEvidenceRefsOnly: 'redacted-evidence-refs-only',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewReadModelNoClaimFlags = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type ProjectionStateValue =
  (typeof AppGameSourceGatedPolicyPreviewReadModelProjectionState)[keyof typeof AppGameSourceGatedPolicyPreviewReadModelProjectionState];
type GateStateValue =
  (typeof AppGameSourceFreshnessPreviewGateState)[keyof typeof AppGameSourceFreshnessPreviewGateState];
type PreviewStatusValue =
  (typeof AppGameSourceFreshnessPreviewGateStatus)[keyof typeof AppGameSourceFreshnessPreviewGateStatus];
type TargetDomainValue = (typeof AppGamePolicyPreviewTargetDomain)[keyof typeof AppGamePolicyPreviewTargetDomain];

type SourceGatedPolicyPreviewReadModelRowLike = {
  readonly targetDomain: TargetDomainValue;
  readonly gateState: GateStateValue;
  readonly projectionState: ProjectionStateValue;
  readonly previewStatus: PreviewStatusValue;
  readonly sourcePolicyCompileAllowed: boolean;
  readonly sourceEvidenceRefs: ReadonlyArray<unknown>;
  readonly previewDecisionRef: unknown | null;
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type SourceGatedPolicyPreviewReadModelLike = {
  readonly rows: ReadonlyArray<SourceGatedPolicyPreviewReadModelRowLike>;
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly previewReadyVisibleCount: number;
  readonly sourceManualRequiredVisibleCount: number;
  readonly compilerManualRequiredVisibleCount: number;
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaims = (
  row: SourceGatedPolicyPreviewReadModelRowLike
) =>
  !row.serviceRuntimeEventClaimed &&
  !row.portalUiRendered &&
  !row.policyEvaluatorRuntimeClaimed &&
  !row.timerRuntimeClaimed &&
  !row.adapterDispatchClaimed &&
  !row.childDeliveryClaimed &&
  !row.platformEnforcementClaimed &&
  !row.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewReadModelRowMatchesGateState = (
  row: SourceGatedPolicyPreviewReadModelRowLike
) => {
  if (row.gateState === AppGameSourceFreshnessPreviewGateState.SourceFresh) {
    return sourceFreshRowMatchesProjection(row);
  }

  if (row.gateState === AppGameSourceFreshnessPreviewGateState.SourceManualRequired) {
    return sourceManualRowMatchesProjection(row);
  }

  if (row.gateState === AppGameSourceFreshnessPreviewGateState.CompilerManualRequired) {
    return compilerManualRowMatchesProjection(row);
  }

  return false;
};

const sourceFreshRowMatchesProjection = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible &&
  row.previewStatus === AppGameSourceFreshnessPreviewGateStatus.PreviewReady &&
  row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef !== null &&
  row.sourceEvidenceRefs.length > 0;

const sourceManualRowMatchesProjection = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible &&
  row.previewStatus === AppGameSourceFreshnessPreviewGateStatus.ManualRequired &&
  !row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef === null;

const compilerManualRowMatchesProjection = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible &&
  row.previewStatus === AppGamePolicyPreviewStatus.ManualRequired &&
  row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef !== null &&
  row.sourceEvidenceRefs.length > 0;

export const countAppGameSourceGatedPolicyPreviewRows = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[],
  targetDomain: TargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGameSourceGatedPolicyPreviewProjectionStates = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[],
  projectionState: ProjectionStateValue
) => rows.filter((row) => row.projectionState === projectionState).length;

export const countAppGameSourceGatedPolicyPreviewReadModelRows = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[]
) => ({
  nativeAppRowCount: countAppGameSourceGatedPolicyPreviewRows(rows, AppGamePolicyPreviewTargetDomain.NativeApp),
  nativeGameRowCount: countAppGameSourceGatedPolicyPreviewRows(rows, AppGamePolicyPreviewTargetDomain.NativeGame),
  previewReadyVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStates(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible
  ),
  sourceManualRequiredVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStates(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible
  ),
  compilerManualRequiredVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStates(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible
  ),
});

export const appGameSourceGatedPolicyPreviewReadModelCountsMatchRows = (
  readModel: SourceGatedPolicyPreviewReadModelLike
) =>
  readModel.nativeAppRowCount ===
    countAppGameSourceGatedPolicyPreviewRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeApp) &&
  readModel.nativeGameRowCount ===
    countAppGameSourceGatedPolicyPreviewRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeGame) &&
  readModel.previewReadyVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStates(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible
    ) &&
  readModel.sourceManualRequiredVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStates(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible
    ) &&
  readModel.compilerManualRequiredVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStates(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible
    );

export const appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaims = (
  readModel: SourceGatedPolicyPreviewReadModelLike
) =>
  !readModel.serviceRuntimeEventClaimed &&
  !readModel.portalUiRendered &&
  !readModel.policyEvaluatorRuntimeClaimed &&
  !readModel.timerRuntimeClaimed &&
  !readModel.adapterDispatchClaimed &&
  !readModel.childDeliveryClaimed &&
  !readModel.platformEnforcementClaimed &&
  !readModel.rawPrivateSourceRowsIncluded;
