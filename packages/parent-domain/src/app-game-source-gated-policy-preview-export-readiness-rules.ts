import { AppGameSourceGatedPolicyPreviewReadModelProjectionState } from './app-game-source-gated-policy-preview-read-model-rules';

export const AppGameSourceGatedPolicyPreviewExportReadinessState = {
  ReadyForManifestSequencing: 'ready-for-manifest-sequencing',
} as const;

export const AppGameSourceGatedPolicyPreviewExportManifestState = {
  DeferredByPackageManifestLock: 'deferred-by-package-manifest-lock',
} as const;

export const AppGameSourceGatedPolicyPreviewExportSubpath =
  './app-game-source-gated-policy-preview-read-model' as const;

export const RequiredAppGameSourceGatedPolicyPreviewExportSymbols = [
  'AppGameSourceGatedPolicyPreviewReadModelSchema',
  'AppGameSourceGatedPolicyPreviewReadModelRowSchema',
  'buildAppGameSourceGatedPolicyPreviewReadModel',
  'decodeAppGameSourceGatedPolicyPreviewReadModel',
  'AppGameSourceGatedPolicyPreviewReadModelProjectionState',
  'AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary',
] as const;

export const RequiredAppGameSourceGatedPolicyPreviewExportNonClaims = [
  'no-package-manifest-edit',
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewExportNoClaimFlags = {
  packageManifestUpdated: false,
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;

type SourceGatedPolicyPreviewExportReadinessLike = {
  readonly requiredExportSubpath: string;
  readonly requiredExportSymbols: readonly string[];
  readonly readinessState: string;
  readonly manifestState: string;
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly previewReadyVisibleCount: number;
  readonly sourceManualRequiredVisibleCount: number;
  readonly compilerManualRequiredVisibleCount: number;
  readonly packageManifestUpdated: boolean;
  readonly serviceRuntimeEventClaimed: boolean;
  readonly portalUiRendered: boolean;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

export const appGameSourceGatedPolicyPreviewExportReadinessCountsMatch = (
  readiness: SourceGatedPolicyPreviewExportReadinessLike
) =>
  readiness.nativeAppRowCount > 0 &&
  readiness.nativeGameRowCount > 0 &&
  readiness.previewReadyVisibleCount > 0 &&
  readiness.sourceManualRequiredVisibleCount > 0 &&
  readiness.compilerManualRequiredVisibleCount > 0;

export const appGameSourceGatedPolicyPreviewExportReadinessHasRequiredSurface = (
  readiness: SourceGatedPolicyPreviewExportReadinessLike
) =>
  readiness.requiredExportSubpath === AppGameSourceGatedPolicyPreviewExportSubpath &&
  RequiredAppGameSourceGatedPolicyPreviewExportSymbols.every((symbol) =>
    readiness.requiredExportSymbols.includes(symbol)
  ) &&
  readiness.readinessState === AppGameSourceGatedPolicyPreviewExportReadinessState.ReadyForManifestSequencing &&
  readiness.manifestState === AppGameSourceGatedPolicyPreviewExportManifestState.DeferredByPackageManifestLock;

export const appGameSourceGatedPolicyPreviewExportReadinessHasNoRuntimeClaims = (
  readiness: SourceGatedPolicyPreviewExportReadinessLike
) =>
  !readiness.packageManifestUpdated &&
  !readiness.serviceRuntimeEventClaimed &&
  !readiness.portalUiRendered &&
  !readiness.policyEvaluatorRuntimeClaimed &&
  !readiness.timerRuntimeClaimed &&
  !readiness.adapterDispatchClaimed &&
  !readiness.childDeliveryClaimed &&
  !readiness.platformEnforcementClaimed &&
  !readiness.rawPrivateSourceRowsIncluded;

export const appGameSourceGatedPolicyPreviewProjectionStatesReadyForExport = (states: readonly string[]) =>
  states.includes(AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible) &&
  states.includes(AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible) &&
  states.includes(AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible);
