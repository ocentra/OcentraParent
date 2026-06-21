import { AppGamePolicyCompilerOutcomeState, AppGamePolicyTargetKind } from './app-game-policy-target-compiler-rules';
import { PolicyDecisionHandoffState } from './policy-contracts';

export const AppGamePolicyPreviewTargetDomain = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AppGamePolicyPreviewStatus = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
  Rejected: 'rejected',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimStates = {
  policyEvaluatorRuntimeClaimState: 'not-claimed',
  timerRuntimeClaimState: 'not-claimed',
  adapterDispatchState: 'not-dispatched',
  childDeliveryClaimState: 'not-claimed',
  platformEnforcementClaimState: 'not-claimed',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimFlags = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

type AppGamePolicyTargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type AppGamePolicyPreviewTargetDomainValue =
  (typeof AppGamePolicyPreviewTargetDomain)[keyof typeof AppGamePolicyPreviewTargetDomain];
type AppGamePolicyPreviewStatusValue = (typeof AppGamePolicyPreviewStatus)[keyof typeof AppGamePolicyPreviewStatus];
type AppGamePolicyCompilerOutcomeStateValue =
  (typeof AppGamePolicyCompilerOutcomeState)[keyof typeof AppGamePolicyCompilerOutcomeState];

type PreviewRowLike = {
  readonly targetDomain: AppGamePolicyPreviewTargetDomainValue;
  readonly outcomeState: AppGamePolicyCompilerOutcomeStateValue;
  readonly previewStatus: AppGamePolicyPreviewStatusValue;
  readonly dryRun: boolean;
  readonly enforcementHandoffState: unknown;
  readonly evidenceReferences: ReadonlyArray<unknown>;
  readonly ruleRefs: ReadonlyArray<unknown>;
  readonly capabilityRefs: ReadonlyArray<unknown>;
  readonly auditRefs: ReadonlyArray<unknown>;
  readonly policyEvaluatorRuntimeClaimState: unknown;
  readonly timerRuntimeClaimState: unknown;
  readonly adapterDispatchState: unknown;
  readonly childDeliveryClaimState: unknown;
  readonly platformEnforcementClaimState: unknown;
  readonly policyEvaluatorRuntimeClaimed: boolean;
  readonly timerRuntimeClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
};

type PreviewReadModelLike = {
  readonly rows: ReadonlyArray<PreviewRowLike>;
  readonly nativeAppRowCount: number;
  readonly nativeGameRowCount: number;
  readonly previewReadyCount: number;
  readonly manualRequiredCount: number;
  readonly rejectedCount: number;
};

const gameTargetKinds = new Set<AppGamePolicyTargetKindValue>([
  AppGamePolicyTargetKind.SpecificGame,
  AppGamePolicyTargetKind.LauncherGameId,
  AppGamePolicyTargetKind.StoreGameId,
  AppGamePolicyTargetKind.GameCategory,
  AppGamePolicyTargetKind.UnknownGame,
  AppGamePolicyTargetKind.NewGame,
  AppGamePolicyTargetKind.LauncherGameCandidate,
  AppGamePolicyTargetKind.MultiplayerGame,
  AppGamePolicyTargetKind.UgcGame,
  AppGamePolicyTargetKind.PurchaseCapableGame,
  AppGamePolicyTargetKind.MatureGame,
  AppGamePolicyTargetKind.AllGames,
]);

export const appGamePolicyPreviewTargetDomainForKind = (targetKind: AppGamePolicyTargetKindValue) =>
  gameTargetKinds.has(targetKind)
    ? AppGamePolicyPreviewTargetDomain.NativeGame
    : AppGamePolicyPreviewTargetDomain.NativeApp;

export function appGamePolicyPreviewStatusForOutcome(
  outcomeState: AppGamePolicyCompilerOutcomeStateValue
): AppGamePolicyPreviewStatusValue {
  switch (outcomeState) {
    case AppGamePolicyCompilerOutcomeState.DryRunReady:
      return AppGamePolicyPreviewStatus.PreviewReady;
    case AppGamePolicyCompilerOutcomeState.ManualRequired:
      return AppGamePolicyPreviewStatus.ManualRequired;
    case AppGamePolicyCompilerOutcomeState.Rejected:
      return AppGamePolicyPreviewStatus.Rejected;
  }
}

export const appGamePolicyPreviewStatusMatchesOutcome = (row: PreviewRowLike) =>
  row.previewStatus === appGamePolicyPreviewStatusForOutcome(row.outcomeState);

export const appGamePolicyPreviewRowIsDryRunOnly = (row: PreviewRowLike) =>
  row.dryRun && row.enforcementHandoffState === PolicyDecisionHandoffState.Disabled;

export const appGamePolicyPreviewRowHasNoRuntimeClaims = (row: PreviewRowLike) =>
  row.policyEvaluatorRuntimeClaimState === 'not-claimed' &&
  row.timerRuntimeClaimState === 'not-claimed' &&
  row.adapterDispatchState === 'not-dispatched' &&
  row.childDeliveryClaimState === 'not-claimed' &&
  row.platformEnforcementClaimState === 'not-claimed' &&
  !row.policyEvaluatorRuntimeClaimed &&
  !row.timerRuntimeClaimed &&
  !row.adapterDispatchClaimed &&
  !row.childDeliveryClaimed &&
  !row.platformEnforcementClaimed;

export const appGamePolicyPreviewRowHasProofRefs = (row: PreviewRowLike) =>
  row.evidenceReferences.length > 0 &&
  row.ruleRefs.length > 0 &&
  row.capabilityRefs.length > 0 &&
  row.auditRefs.length > 0;

export const appGamePolicyPreviewReadModelCountsMatchRows = (readModel: PreviewReadModelLike) =>
  readModel.nativeAppRowCount ===
    countAppGamePolicyPreviewRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeApp) &&
  readModel.nativeGameRowCount ===
    countAppGamePolicyPreviewRows(readModel.rows, AppGamePolicyPreviewTargetDomain.NativeGame) &&
  readModel.previewReadyCount ===
    countAppGamePolicyPreviewStatuses(readModel.rows, AppGamePolicyPreviewStatus.PreviewReady) &&
  readModel.manualRequiredCount ===
    countAppGamePolicyPreviewStatuses(readModel.rows, AppGamePolicyPreviewStatus.ManualRequired) &&
  readModel.rejectedCount === countAppGamePolicyPreviewStatuses(readModel.rows, AppGamePolicyPreviewStatus.Rejected);

export const countAppGamePolicyPreviewRows = (
  rows: readonly PreviewRowLike[],
  targetDomain: AppGamePolicyPreviewTargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGamePolicyPreviewStatuses = (
  rows: readonly PreviewRowLike[],
  previewStatus: AppGamePolicyPreviewStatusValue
) => rows.filter((row) => row.previewStatus === previewStatus).length;

export const countAppGamePolicyPreviewReadModelRows = (rows: readonly PreviewRowLike[]) => ({
  nativeAppRowCount: countAppGamePolicyPreviewRows(rows, AppGamePolicyPreviewTargetDomain.NativeApp),
  nativeGameRowCount: countAppGamePolicyPreviewRows(rows, AppGamePolicyPreviewTargetDomain.NativeGame),
  previewReadyCount: countAppGamePolicyPreviewStatuses(rows, AppGamePolicyPreviewStatus.PreviewReady),
  manualRequiredCount: countAppGamePolicyPreviewStatuses(rows, AppGamePolicyPreviewStatus.ManualRequired),
  rejectedCount: countAppGamePolicyPreviewStatuses(rows, AppGamePolicyPreviewStatus.Rejected),
});
