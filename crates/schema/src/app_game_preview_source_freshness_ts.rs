use super::app_game_preview_source_freshness::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID;

const APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN: &str =
    "__APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID__";

const VALUES_TEMPLATE: &str = r#"/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

export const AppGamePolicyPreviewTargetDomainGenerated = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AppGamePolicyPreviewStatusGenerated = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
  Rejected: 'rejected',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimStatesGenerated = {
  policyEvaluatorRuntimeClaimState: 'not-claimed',
  timerRuntimeClaimState: 'not-claimed',
  adapterDispatchState: 'not-dispatched',
  childDeliveryClaimState: 'not-claimed',
  platformEnforcementClaimState: 'not-claimed',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

export const AppGameSourceFreshnessPolicyConsumptionMatrixIdGenerated =
  '__APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID__' as const;

export const AppGameSourceFreshnessPolicyTargetKindGenerated = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
  AllNativeApps: 'all-native-apps',
  AllNativeGames: 'all-native-games',
} as const;

export const AppGameSourceFreshnessRequirementKindGenerated = {
  Inventory: 'inventory',
  Runtime: 'runtime',
  Foreground: 'foreground',
  Launcher: 'launcher',
} as const;

export const AppGameSourceFreshnessSourceKindGenerated = {
  OsInstalledRecord: 'osInstalledRecord',
  Shortcut: 'shortcut',
  StorePackage: 'storePackage',
  LauncherManifest: 'launcherManifest',
  ParentCatalog: 'parentCatalog',
  ManagedDevice: 'managedDevice',
  PortableApp: 'portableApp',
  UnknownSource: 'unknownSource',
  ProcessSnapshot: 'processSnapshot',
  ForegroundWindow: 'foregroundWindow',
  ProcessStart: 'processStart',
  ProcessExit: 'processExit',
  InventoryScan: 'inventoryScan',
} as const;

export const AppGameSourceFreshnessReadModelStateGenerated = {
  Ready: 'ready',
  Empty: 'empty',
  Unavailable: 'unavailable',
  Offline: 'offline',
  Stale: 'stale',
  PermissionRequired: 'permission-required',
  ScaffoldOnly: 'scaffold-only',
} as const;

export const AppGameSourceFreshnessCapabilityStatusGenerated = {
  Available: 'available',
  Unavailable: 'unavailable',
  PermissionLimited: 'permissionLimited',
  UnsupportedPlatform: 'unsupportedPlatform',
  AdapterError: 'adapterError',
  Stale: 'stale',
  Degraded: 'degraded',
  ManualRequired: 'manualRequired',
  NotClaimed: 'notClaimed',
} as const;

export const AppGameSourceFreshnessRequirementStateGenerated = {
  Satisfied: 'satisfied',
  Missing: 'missing',
  Empty: 'empty',
  MissingObservedAt: 'missing-observed-at',
  Stale: 'stale',
  PermissionLimited: 'permission-limited',
  Unavailable: 'unavailable',
  AdapterError: 'adapter-error',
  ManualRequired: 'manual-required',
  NotClaimed: 'not-claimed',
  MissingEvidence: 'missing-evidence',
} as const;

export const AppGameSourceFreshnessPolicyReadinessStateGenerated = {
  PolicyReady: 'policy-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessAdapterDispatchStateGenerated = {
  NotDispatched: 'not-dispatched',
} as const;

export const AppGameSourceFreshnessReasonCodeGenerated = {
  MissingSourceStatusRow: 'missing-source-status-row',
  EmptySourceStatusRow: 'empty-source-status-row',
  MissingObservedAt: 'missing-observed-at',
  StaleSourceStatusRow: 'stale-source-status-row',
  PermissionLimitedSourceStatus: 'permission-limited-source-status',
  UnavailableSourceStatus: 'unavailable-source-status',
  AdapterErrorSourceStatus: 'adapter-error-source-status',
  ManualRequiredSourceStatus: 'manual-required-source-status',
  NotClaimedSourceStatus: 'not-claimed-source-status',
  MissingSourceEvidence: 'missing-source-evidence',
} as const;

export const AppGameSourceFreshnessRequirementSourceKindsGenerated = {
  [AppGameSourceFreshnessRequirementKindGenerated.Inventory]: [
    AppGameSourceFreshnessSourceKindGenerated.OsInstalledRecord,
    AppGameSourceFreshnessSourceKindGenerated.Shortcut,
    AppGameSourceFreshnessSourceKindGenerated.StorePackage,
    AppGameSourceFreshnessSourceKindGenerated.ParentCatalog,
    AppGameSourceFreshnessSourceKindGenerated.ManagedDevice,
    AppGameSourceFreshnessSourceKindGenerated.PortableApp,
    AppGameSourceFreshnessSourceKindGenerated.UnknownSource,
    AppGameSourceFreshnessSourceKindGenerated.InventoryScan,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Runtime]: [
    AppGameSourceFreshnessSourceKindGenerated.ProcessSnapshot,
    AppGameSourceFreshnessSourceKindGenerated.ProcessStart,
    AppGameSourceFreshnessSourceKindGenerated.ProcessExit,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Foreground]: [
    AppGameSourceFreshnessSourceKindGenerated.ForegroundWindow,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Launcher]: [
    AppGameSourceFreshnessSourceKindGenerated.LauncherManifest,
  ],
} as const;

export const AppGameSourceFreshnessPreviewGateStatusGenerated = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateStateGenerated = {
  SourceFresh: 'source-fresh',
  SourceManualRequired: 'source-manual-required',
  CompilerManualRequired: 'compiler-manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlagsGenerated = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

export const AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated = {
  PreviewReadyVisible: 'preview-ready-visible',
  SourceManualRequiredVisible: 'source-manual-required-visible',
  CompilerManualRequiredVisible: 'compiler-manual-required-visible',
} as const;

export const AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundaryGenerated = {
  RedactedEvidenceRefsOnly: 'redacted-evidence-refs-only',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewReadModelNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;
"#;

const RULES_TEMPLATE: &str = r#"/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

import { AppGamePolicyCompilerOutcomeState, AppGamePolicyTargetKind } from '../app-game-policy-target-compiler-rules';
import { PolicyDecisionHandoffState } from '../policy-contracts';
import {
  AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated,
  AppGamePolicyPreviewNoRuntimeClaimStatesGenerated,
  AppGamePolicyPreviewStatusGenerated,
  AppGamePolicyPreviewTargetDomainGenerated,
  AppGameSourceFreshnessAdapterDispatchStateGenerated,
  AppGameSourceFreshnessCapabilityStatusGenerated,
  AppGameSourceFreshnessPolicyReadinessStateGenerated,
  AppGameSourceFreshnessPolicyTargetKindGenerated,
  AppGameSourceFreshnessPreviewGateStateGenerated,
  AppGameSourceFreshnessPreviewGateStatusGenerated,
  AppGameSourceFreshnessReadModelStateGenerated,
  AppGameSourceFreshnessReasonCodeGenerated,
  AppGameSourceFreshnessRequirementSourceKindsGenerated,
  AppGameSourceFreshnessRequirementStateGenerated,
  AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated,
} from './app-game-preview-source-freshness-values';

type AppGamePolicyTargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type AppGamePolicyPreviewTargetDomainValue = string;
type AppGamePolicyPreviewStatusValue = string;
type AppGamePolicyCompilerOutcomeStateValue =
  (typeof AppGamePolicyCompilerOutcomeState)[keyof typeof AppGamePolicyCompilerOutcomeState];
type AppGameSourceFreshnessPolicyTargetKindValue = string;
type AppGameSourceFreshnessRequirementKindValue = string;
type AppGameSourceFreshnessSourceKindValue = string;
type AppGameSourceFreshnessReadModelStateValue = string;
type AppGameSourceFreshnessCapabilityStatusValue = string;
type AppGameSourceFreshnessRequirementStateValue = string;
type AppGameSourceFreshnessReasonCodeValue = string;
type AppGameSourceFreshnessPreviewGateStatusValue = string;
type AppGameSourceFreshnessPreviewGateStateValue = string;
type AppGameSourceGatedPolicyPreviewProjectionStateValue = string;

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

type SourceStatusRowLike = {
  readonly sourceKind: AppGameSourceFreshnessSourceKindValue;
  readonly state: AppGameSourceFreshnessReadModelStateValue;
  readonly rowCount: number;
  readonly lastObservedAt: unknown;
  readonly capabilityStatus: AppGameSourceFreshnessCapabilityStatusValue;
  readonly evidence: ReadonlyArray<unknown>;
};

type RequirementResultLike = {
  readonly requirementKind: AppGameSourceFreshnessRequirementKindValue;
  readonly requirementState: AppGameSourceFreshnessRequirementStateValue;
  readonly reasonCode: AppGameSourceFreshnessReasonCodeValue | null;
  readonly matchedSourceKinds: ReadonlyArray<AppGameSourceFreshnessSourceKindValue>;
  readonly sourceEvidenceRefs: ReadonlyArray<unknown>;
  readonly lastObservedAt: unknown;
};

type SourceFreshnessTargetLike = {
  readonly targetKind: AppGameSourceFreshnessPolicyTargetKindValue;
  readonly targetRef: unknown;
};

type SourceFreshnessReadinessLike = {
  readonly requirementResults: ReadonlyArray<RequirementResultLike>;
  readonly policyEvidenceRefs: ReadonlyArray<unknown>;
  readonly policyCompileAllowed: boolean;
  readonly directAdapterCallRequested: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type SourceFreshnessRequirementFailure = {
  readonly requirementState: AppGameSourceFreshnessRequirementStateValue;
  readonly reasonCode: AppGameSourceFreshnessReasonCodeValue;
};

type SourceFreshnessRequirementFailureCheck = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) => SourceFreshnessRequirementFailure | null;

type SourceFreshnessRequestLike = {
  readonly schemaVersion: string;
  readonly policyRequestId: string;
  readonly target: SourceFreshnessTargetLike;
  readonly requiredSources: ReadonlyArray<AppGameSourceFreshnessRequirementKindValue>;
  readonly maxSourceAgeMs: number;
  readonly sourceStatusRows: ReadonlyArray<SourceStatusRowLike>;
  readonly requestedAt: string;
  readonly sourceRowsFromActivityReadModel: true;
  readonly rawPrivateSourceRowsIncluded: false;
};

type SourceFreshnessPreviewGateRowLike = {
  readonly targetDomain: AppGamePolicyPreviewTargetDomainValue;
  readonly previewStatus: AppGameSourceFreshnessPreviewGateStatusValue;
  readonly gateState: AppGameSourceFreshnessPreviewGateStateValue;
  readonly sourcePolicyCompileAllowed: boolean;
  readonly sourceReadinessState: unknown;
  readonly sourceRequirementStates: ReadonlyArray<unknown>;
  readonly sourceEvidenceRefs: ReadonlyArray<unknown>;
  readonly previewRow: {
    readonly targetDomain: AppGamePolicyPreviewTargetDomainValue;
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

type SourceGatedPolicyPreviewReadModelRowLike = {
  readonly targetDomain: AppGamePolicyPreviewTargetDomainValue;
  readonly gateState: AppGameSourceFreshnessPreviewGateStateValue;
  readonly projectionState: AppGameSourceGatedPolicyPreviewProjectionStateValue;
  readonly previewStatus: AppGameSourceFreshnessPreviewGateStatusValue;
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

export const appGamePolicyPreviewTargetDomainForKindGenerated = (targetKind: AppGamePolicyTargetKindValue) =>
  gameTargetKinds.has(targetKind)
    ? AppGamePolicyPreviewTargetDomainGenerated.NativeGame
    : AppGamePolicyPreviewTargetDomainGenerated.NativeApp;

export function appGamePolicyPreviewStatusForOutcomeGenerated(
  outcomeState: AppGamePolicyCompilerOutcomeStateValue
): AppGamePolicyPreviewStatusValue {
  switch (outcomeState) {
    case AppGamePolicyCompilerOutcomeState.DryRunReady:
      return AppGamePolicyPreviewStatusGenerated.PreviewReady;
    case AppGamePolicyCompilerOutcomeState.ManualRequired:
      return AppGamePolicyPreviewStatusGenerated.ManualRequired;
    case AppGamePolicyCompilerOutcomeState.Rejected:
      return AppGamePolicyPreviewStatusGenerated.Rejected;
  }
}

export const appGamePolicyPreviewStatusMatchesOutcomeGenerated = (row: PreviewRowLike) =>
  row.previewStatus === appGamePolicyPreviewStatusForOutcomeGenerated(row.outcomeState);

export const appGamePolicyPreviewRowIsDryRunOnlyGenerated = (row: PreviewRowLike) =>
  row.dryRun && row.enforcementHandoffState === PolicyDecisionHandoffState.Disabled;

export const appGamePolicyPreviewRowHasNoRuntimeClaimsGenerated = (row: PreviewRowLike) =>
  row.policyEvaluatorRuntimeClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.policyEvaluatorRuntimeClaimState &&
  row.timerRuntimeClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.timerRuntimeClaimState &&
  row.adapterDispatchState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.adapterDispatchState &&
  row.childDeliveryClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.childDeliveryClaimState &&
  row.platformEnforcementClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.platformEnforcementClaimState &&
  row.policyEvaluatorRuntimeClaimed === AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.policyEvaluatorRuntimeClaimed &&
  row.timerRuntimeClaimed === AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.timerRuntimeClaimed &&
  row.adapterDispatchClaimed === AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.adapterDispatchClaimed &&
  row.childDeliveryClaimed === AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.childDeliveryClaimed &&
  row.platformEnforcementClaimed === AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.platformEnforcementClaimed;

export const appGamePolicyPreviewRowHasProofRefsGenerated = (row: PreviewRowLike) =>
  row.evidenceReferences.length > 0 &&
  row.ruleRefs.length > 0 &&
  row.capabilityRefs.length > 0 &&
  row.auditRefs.length > 0;

export const countAppGamePolicyPreviewRowsGenerated = (
  rows: readonly PreviewRowLike[],
  targetDomain: AppGamePolicyPreviewTargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGamePolicyPreviewStatusesGenerated = (
  rows: readonly PreviewRowLike[],
  previewStatus: AppGamePolicyPreviewStatusValue
) => rows.filter((row) => row.previewStatus === previewStatus).length;

export const countAppGamePolicyPreviewReadModelRowsGenerated = (rows: readonly PreviewRowLike[]) => ({
  nativeAppRowCount: countAppGamePolicyPreviewRowsGenerated(rows, AppGamePolicyPreviewTargetDomainGenerated.NativeApp),
  nativeGameRowCount: countAppGamePolicyPreviewRowsGenerated(rows, AppGamePolicyPreviewTargetDomainGenerated.NativeGame),
  previewReadyCount: countAppGamePolicyPreviewStatusesGenerated(rows, AppGamePolicyPreviewStatusGenerated.PreviewReady),
  manualRequiredCount: countAppGamePolicyPreviewStatusesGenerated(
    rows,
    AppGamePolicyPreviewStatusGenerated.ManualRequired
  ),
  rejectedCount: countAppGamePolicyPreviewStatusesGenerated(rows, AppGamePolicyPreviewStatusGenerated.Rejected),
});

export const appGamePolicyPreviewReadModelCountsMatchRowsGenerated = (readModel: PreviewReadModelLike) =>
  readModel.nativeAppRowCount ===
    countAppGamePolicyPreviewRowsGenerated(readModel.rows, AppGamePolicyPreviewTargetDomainGenerated.NativeApp) &&
  readModel.nativeGameRowCount ===
    countAppGamePolicyPreviewRowsGenerated(readModel.rows, AppGamePolicyPreviewTargetDomainGenerated.NativeGame) &&
  readModel.previewReadyCount ===
    countAppGamePolicyPreviewStatusesGenerated(readModel.rows, AppGamePolicyPreviewStatusGenerated.PreviewReady) &&
  readModel.manualRequiredCount ===
    countAppGamePolicyPreviewStatusesGenerated(readModel.rows, AppGamePolicyPreviewStatusGenerated.ManualRequired) &&
  readModel.rejectedCount ===
    countAppGamePolicyPreviewStatusesGenerated(readModel.rows, AppGamePolicyPreviewStatusGenerated.Rejected);

export const appGameSourceFreshnessTargetAllowsNullRefGenerated = (target: SourceFreshnessTargetLike) =>
  target.targetKind === AppGameSourceFreshnessPolicyTargetKindGenerated.AllNativeApps ||
  target.targetKind === AppGameSourceFreshnessPolicyTargetKindGenerated.AllNativeGames;

export const appGameSourceFreshnessSourceKindSatisfiesRequirementGenerated = (
  sourceKind: AppGameSourceFreshnessSourceKindValue,
  requirementKind: AppGameSourceFreshnessRequirementKindValue
) => {
  const candidates =
    AppGameSourceFreshnessRequirementSourceKindsGenerated[
      requirementKind as keyof typeof AppGameSourceFreshnessRequirementSourceKindsGenerated
    ] ?? [];
  return candidates.some((candidate: string) => candidate === sourceKind);
};

export const appGameSourceFreshnessRowsForRequirementGenerated = (
  rows: ReadonlyArray<SourceStatusRowLike>,
  requirementKind: AppGameSourceFreshnessRequirementKindValue
) => rows.filter((row) => appGameSourceFreshnessSourceKindSatisfiesRequirementGenerated(row.sourceKind, requirementKind));

export const appGameSourceFreshnessRowIsFreshGenerated = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) => {
  const observedAtMs = parseTimestampMillisGenerated(row.lastObservedAt);
  const evaluatedAtMs = parseTimestampMillisGenerated(evaluatedAt);

  if (observedAtMs === null || evaluatedAtMs === null) {
    return false;
  }

  return Math.max(0, evaluatedAtMs - observedAtMs) <= maxSourceAgeMs;
};

export const appGameSourceFreshnessRowHasEvidenceGenerated = (row: SourceStatusRowLike) =>
  row.rowCount === 0 || row.evidence.length > 0;

export const appGameSourceFreshnessRequirementIsSatisfiedGenerated = (result: RequirementResultLike) =>
  result.requirementState === AppGameSourceFreshnessRequirementStateGenerated.Satisfied &&
  result.reasonCode === null &&
  result.sourceEvidenceRefs.length > 0;

export const appGameSourceFreshnessReadinessIsPolicyReadyGenerated = (
  readiness: SourceFreshnessReadinessLike
) =>
  readiness.policyCompileAllowed &&
  readiness.directAdapterCallRequested === false &&
  readiness.rawPrivateSourceRowsIncluded === false &&
  readiness.policyEvidenceRefs.length > 0 &&
  readiness.requirementResults.every(appGameSourceFreshnessRequirementIsSatisfiedGenerated);

export const appGameSourceFreshnessRequirementFailureGenerated = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) =>
  SourceFreshnessRequirementFailureChecksGenerated.reduce<SourceFreshnessRequirementFailure | null>(
    (failure, check) => failure ?? check(row, evaluatedAt, maxSourceAgeMs),
    null
  );

const SourceFreshnessRequirementFailureChecksGenerated = [
  sourceRowEmptyFailureGenerated,
  sourceRowMissingObservedAtFailureGenerated,
  sourceRowStaleFailureGenerated,
  sourceRowPermissionLimitedFailureGenerated,
  sourceRowUnavailableFailureGenerated,
  sourceRowAdapterErrorFailureGenerated,
  sourceRowManualRequiredFailureGenerated,
  sourceRowNotClaimedFailureGenerated,
  sourceRowMissingEvidenceFailureGenerated,
] satisfies readonly SourceFreshnessRequirementFailureCheck[];

function sourceRowEmptyFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
  if (row.rowCount === 0 || row.state === AppGameSourceFreshnessReadModelStateGenerated.Empty) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.Empty,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.EmptySourceStatusRow,
    } as const;
  }

  return null;
}

function sourceRowMissingObservedAtFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (row.lastObservedAt === null) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.MissingObservedAt,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.MissingObservedAt,
    } as const;
  }

  return null;
}

function sourceRowStaleFailureGenerated(
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
): SourceFreshnessRequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelStateGenerated.Stale ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.Stale ||
    !appGameSourceFreshnessRowIsFreshGenerated(row, evaluatedAt, maxSourceAgeMs)
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.Stale,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.StaleSourceStatusRow,
    } as const;
  }

  return null;
}

function sourceRowPermissionLimitedFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelStateGenerated.PermissionRequired ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.PermissionLimited
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.PermissionLimited,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.PermissionLimitedSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowUnavailableFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelStateGenerated.Unavailable ||
    row.state === AppGameSourceFreshnessReadModelStateGenerated.Offline ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.Unavailable ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.UnsupportedPlatform
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.Unavailable,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.UnavailableSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowAdapterErrorFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.AdapterError) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.AdapterError,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.AdapterErrorSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowManualRequiredFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelStateGenerated.ScaffoldOnly ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.ManualRequired
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.ManualRequired,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.ManualRequiredSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowNotClaimedFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.NotClaimed) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.NotClaimed,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.NotClaimedSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowMissingEvidenceFailureGenerated(
  row: SourceStatusRowLike
): SourceFreshnessRequirementFailure | null {
  if (row.evidence.length === 0) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.MissingEvidence,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.MissingSourceEvidence,
    } as const;
  }

  return null;
}

function evaluateRequirementGenerated(
  request: SourceFreshnessRequestLike,
  requirementKind: AppGameSourceFreshnessRequirementKindValue,
  evaluatedAt: string
): RequirementResultLike {
  const rows = appGameSourceFreshnessRowsForRequirementGenerated(request.sourceStatusRows, requirementKind);

  if (rows.length === 0) {
    return {
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.Missing,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.MissingSourceStatusRow,
      matchedSourceKinds: [],
      sourceEvidenceRefs: [],
      lastObservedAt: null,
    };
  }

  const freshRow = rows.find(
    (row) =>
      row.state === AppGameSourceFreshnessReadModelStateGenerated.Ready &&
      row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.Available &&
      row.rowCount > 0 &&
      row.evidence.length > 0 &&
      appGameSourceFreshnessRequirementFailureGenerated(row, evaluatedAt, request.maxSourceAgeMs) === null
  );

  if (freshRow !== undefined) {
    return {
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.Satisfied,
      reasonCode: null,
      matchedSourceKinds: [freshRow.sourceKind],
      sourceEvidenceRefs: freshRow.evidence,
      lastObservedAt: freshRow.lastObservedAt,
    };
  }

  const firstRow = rows[0];

  if (firstRow === undefined) {
    throw new Error('Expected app/game source freshness rows after empty-row guard');
  }

  const firstFailure = appGameSourceFreshnessRequirementFailureGenerated(
    firstRow,
    evaluatedAt,
    request.maxSourceAgeMs
  ) ?? {
    requirementState: AppGameSourceFreshnessRequirementStateGenerated.MissingEvidence,
    reasonCode: AppGameSourceFreshnessReasonCodeGenerated.MissingSourceEvidence,
  };

  return {
    requirementKind,
    requirementState: firstFailure.requirementState,
    reasonCode: firstFailure.reasonCode,
    matchedSourceKinds: rows.map((row) => row.sourceKind),
    sourceEvidenceRefs: rows.flatMap((row) => row.evidence),
    lastObservedAt: firstRow.lastObservedAt,
  };
}

export function evaluateAppGameSourceFreshnessPolicyReadinessGenerated(
  request: SourceFreshnessRequestLike,
  readinessId: string,
  evaluatedAt: string
) {
  const requirementResults = request.requiredSources.map((requirementKind) =>
    evaluateRequirementGenerated(request, requirementKind, evaluatedAt)
  );
  const policyEvidenceRefs = [...new Set(requirementResults.flatMap((result) => result.sourceEvidenceRefs))];
  const allSatisfied = requirementResults.every(
    (result) => result.requirementState === AppGameSourceFreshnessRequirementStateGenerated.Satisfied
  );

  return {
    schemaVersion: request.schemaVersion,
    readinessId,
    request,
    readinessState: allSatisfied
      ? AppGameSourceFreshnessPolicyReadinessStateGenerated.PolicyReady
      : AppGameSourceFreshnessPolicyReadinessStateGenerated.ManualRequired,
    requirementResults,
    policyEvidenceRefs,
    policyCompileAllowed: allSatisfied,
    adapterDispatchState: AppGameSourceFreshnessAdapterDispatchStateGenerated.NotDispatched,
    directAdapterCallRequested: false,
    rawPrivateSourceRowsIncluded: false,
    evaluatedAt,
  };
}

export const appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaimsGenerated = (
  row: SourceFreshnessPreviewGateRowLike
) =>
  !row.policyEvaluatorRuntimeClaimed &&
  !row.timerRuntimeClaimed &&
  !row.adapterDispatchClaimed &&
  !row.childDeliveryClaimed &&
  !row.platformEnforcementClaimed;

export const appGameSourceFreshnessPreviewGateRowMatchesSourceStateGenerated = (
  row: SourceFreshnessPreviewGateRowLike
) => {
  if (row.sourceReadinessState === AppGameSourceFreshnessPolicyReadinessStateGenerated.ManualRequired) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatusGenerated.ManualRequired &&
      row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.SourceManualRequired &&
      !row.sourcePolicyCompileAllowed &&
      row.previewRow === null &&
      !row.compiledDecisionProvided &&
      row.sourceRequirementStates.length > 0
    );
  }

  if (row.sourceReadinessState !== AppGameSourceFreshnessPolicyReadinessStateGenerated.PolicyReady) {
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

export const appGameSourceFreshnessPreviewGateRowMatchesPreviewStateGenerated = (
  row: SourceFreshnessPreviewGateRowLike
) => {
  if (row.previewRow === null) {
    return row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.SourceManualRequired;
  }

  if (row.previewRow.previewStatus === AppGamePolicyPreviewStatusGenerated.PreviewReady) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatusGenerated.PreviewReady &&
      row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.SourceFresh
    );
  }

  if (row.previewRow.previewStatus === AppGamePolicyPreviewStatusGenerated.ManualRequired) {
    return (
      row.previewStatus === AppGameSourceFreshnessPreviewGateStatusGenerated.ManualRequired &&
      row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.CompilerManualRequired
    );
  }

  return false;
};

export const countAppGameSourceFreshnessPreviewGateRowsGenerated = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  targetDomain: AppGamePolicyPreviewTargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGameSourceFreshnessPreviewGateStatusesGenerated = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  previewStatus: AppGameSourceFreshnessPreviewGateStatusValue
) => rows.filter((row) => row.previewStatus === previewStatus).length;

export const countAppGameSourceFreshnessPreviewGateStatesGenerated = (
  rows: readonly SourceFreshnessPreviewGateRowLike[],
  gateState: AppGameSourceFreshnessPreviewGateStateValue
) => rows.filter((row) => row.gateState === gateState).length;

export const countAppGameSourceFreshnessPreviewGateReadModelRowsGenerated = (
  rows: readonly SourceFreshnessPreviewGateRowLike[]
) => ({
  nativeAppRowCount: countAppGameSourceFreshnessPreviewGateRowsGenerated(
    rows,
    AppGamePolicyPreviewTargetDomainGenerated.NativeApp
  ),
  nativeGameRowCount: countAppGameSourceFreshnessPreviewGateRowsGenerated(
    rows,
    AppGamePolicyPreviewTargetDomainGenerated.NativeGame
  ),
  previewReadyCount: countAppGameSourceFreshnessPreviewGateStatusesGenerated(
    rows,
    AppGameSourceFreshnessPreviewGateStatusGenerated.PreviewReady
  ),
  manualRequiredCount: countAppGameSourceFreshnessPreviewGateStatusesGenerated(
    rows,
    AppGameSourceFreshnessPreviewGateStatusGenerated.ManualRequired
  ),
  sourceManualRequiredCount: countAppGameSourceFreshnessPreviewGateStatesGenerated(
    rows,
    AppGameSourceFreshnessPreviewGateStateGenerated.SourceManualRequired
  ),
  compilerManualRequiredCount: countAppGameSourceFreshnessPreviewGateStatesGenerated(
    rows,
    AppGameSourceFreshnessPreviewGateStateGenerated.CompilerManualRequired
  ),
});

export const appGameSourceFreshnessPreviewGateReadModelCountsMatchRowsGenerated = (
  readModel: SourceFreshnessPreviewGateReadModelLike
) =>
  readModel.nativeAppRowCount ===
    countAppGameSourceFreshnessPreviewGateRowsGenerated(
      readModel.rows,
      AppGamePolicyPreviewTargetDomainGenerated.NativeApp
    ) &&
  readModel.nativeGameRowCount ===
    countAppGameSourceFreshnessPreviewGateRowsGenerated(
      readModel.rows,
      AppGamePolicyPreviewTargetDomainGenerated.NativeGame
    ) &&
  readModel.previewReadyCount ===
    countAppGameSourceFreshnessPreviewGateStatusesGenerated(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStatusGenerated.PreviewReady
    ) &&
  readModel.manualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStatusesGenerated(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStatusGenerated.ManualRequired
    ) &&
  readModel.sourceManualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStatesGenerated(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStateGenerated.SourceManualRequired
    ) &&
  readModel.compilerManualRequiredCount ===
    countAppGameSourceFreshnessPreviewGateStatesGenerated(
      readModel.rows,
      AppGameSourceFreshnessPreviewGateStateGenerated.CompilerManualRequired
    );

export const appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaimsGenerated = (
  readModel: SourceFreshnessPreviewGateReadModelLike
) =>
  !readModel.policyEvaluatorRuntimeClaimed &&
  !readModel.timerRuntimeClaimed &&
  !readModel.adapterDispatchClaimed &&
  !readModel.childDeliveryClaimed &&
  !readModel.platformEnforcementClaimed;

export const appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaimsGenerated = (
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

export const appGameSourceGatedPolicyPreviewReadModelRowMatchesGateStateGenerated = (
  row: SourceGatedPolicyPreviewReadModelRowLike
) => {
  if (row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.SourceFresh) {
    return sourceFreshRowMatchesProjectionGenerated(row);
  }

  if (row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.SourceManualRequired) {
    return sourceManualRowMatchesProjectionGenerated(row);
  }

  if (row.gateState === AppGameSourceFreshnessPreviewGateStateGenerated.CompilerManualRequired) {
    return compilerManualRowMatchesProjectionGenerated(row);
  }

  return false;
};

const sourceFreshRowMatchesProjectionGenerated = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState === AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.PreviewReadyVisible &&
  row.previewStatus === AppGameSourceFreshnessPreviewGateStatusGenerated.PreviewReady &&
  row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef !== null &&
  row.sourceEvidenceRefs.length > 0;

const sourceManualRowMatchesProjectionGenerated = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState ===
    AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.SourceManualRequiredVisible &&
  row.previewStatus === AppGameSourceFreshnessPreviewGateStatusGenerated.ManualRequired &&
  !row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef === null;

const compilerManualRowMatchesProjectionGenerated = (row: SourceGatedPolicyPreviewReadModelRowLike) =>
  row.projectionState ===
    AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.CompilerManualRequiredVisible &&
  row.previewStatus === AppGamePolicyPreviewStatusGenerated.ManualRequired &&
  row.sourcePolicyCompileAllowed &&
  row.previewDecisionRef !== null &&
  row.sourceEvidenceRefs.length > 0;

export const countAppGameSourceGatedPolicyPreviewRowsGenerated = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[],
  targetDomain: AppGamePolicyPreviewTargetDomainValue
) => rows.filter((row) => row.targetDomain === targetDomain).length;

export const countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[],
  projectionState: AppGameSourceGatedPolicyPreviewProjectionStateValue
) => rows.filter((row) => row.projectionState === projectionState).length;

export const countAppGameSourceGatedPolicyPreviewReadModelRowsGenerated = (
  rows: readonly SourceGatedPolicyPreviewReadModelRowLike[]
) => ({
  nativeAppRowCount: countAppGameSourceGatedPolicyPreviewRowsGenerated(
    rows,
    AppGamePolicyPreviewTargetDomainGenerated.NativeApp
  ),
  nativeGameRowCount: countAppGameSourceGatedPolicyPreviewRowsGenerated(
    rows,
    AppGamePolicyPreviewTargetDomainGenerated.NativeGame
  ),
  previewReadyVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.PreviewReadyVisible
  ),
  sourceManualRequiredVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.SourceManualRequiredVisible
  ),
  compilerManualRequiredVisibleCount: countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
    rows,
    AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.CompilerManualRequiredVisible
  ),
});

export const appGameSourceGatedPolicyPreviewReadModelCountsMatchRowsGenerated = (
  readModel: SourceGatedPolicyPreviewReadModelLike
) =>
  readModel.nativeAppRowCount ===
    countAppGameSourceGatedPolicyPreviewRowsGenerated(
      readModel.rows,
      AppGamePolicyPreviewTargetDomainGenerated.NativeApp
    ) &&
  readModel.nativeGameRowCount ===
    countAppGameSourceGatedPolicyPreviewRowsGenerated(
      readModel.rows,
      AppGamePolicyPreviewTargetDomainGenerated.NativeGame
    ) &&
  readModel.previewReadyVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.PreviewReadyVisible
    ) &&
  readModel.sourceManualRequiredVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.SourceManualRequiredVisible
    ) &&
  readModel.compilerManualRequiredVisibleCount ===
    countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated(
      readModel.rows,
      AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated.CompilerManualRequiredVisible
    );

export const appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaimsGenerated = (
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

function parseTimestampMillisGenerated(value: unknown): number | null {
  if (typeof value !== 'string') {
    return null;
  }

  const parsed = Date.parse(value);
  return Number.isFinite(parsed) ? parsed : null;
}
"#;

const DATA_TEMPLATE: &str = r#"/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

import {
  AppGameSourceFreshnessCapabilityStatusGenerated as CapabilityStatus,
  AppGameSourceFreshnessPolicyTargetKindGenerated as TargetKind,
  AppGameSourceFreshnessReadModelStateGenerated as ReadModelState,
  AppGameSourceFreshnessRequirementKindGenerated as RequirementKind,
  AppGameSourceFreshnessSourceKindGenerated as SourceKind,
} from './app-game-preview-source-freshness-values';

export const AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated = '2026-06-04T12:55:00.000Z' as const;
export const AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated =
  '2026-06-04T12:54:00.000Z' as const;
export const AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated =
  '2026-06-04T09:00:00.000Z' as const;

export const AppGameSourceFreshnessPolicyConsumptionRequestsGenerated = [
  {
    schemaVersion: 'v0.6',
    policyRequestId: 'source-freshness-native-app-ready-request',
    target: {
      targetKind: TargetKind.NativeApp,
      targetRef: 'app-target-parental-controls-helper',
    },
    requiredSources: [RequirementKind.Inventory, RequirementKind.Runtime, RequirementKind.Foreground],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.OsInstalledRecord,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-inventory-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Ready,
        rowCount: 2,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-runtime-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-foreground-parental-controls-helper'],
      },
    ],
  },
  {
    schemaVersion: 'v0.6',
    policyRequestId: 'source-freshness-native-game-ready-request',
    target: {
      targetKind: TargetKind.NativeGame,
      targetRef: 'game-target-launcher-child-game',
    },
    requiredSources: [
      RequirementKind.Inventory,
      RequirementKind.Runtime,
      RequirementKind.Foreground,
      RequirementKind.Launcher,
    ],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.StorePackage,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-store-package-child-game'],
      },
      {
        sourceKind: SourceKind.ProcessStart,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-runtime-child-game'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-foreground-child-game'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-launcher-child-game'],
      },
    ],
  },
  {
    schemaVersion: 'v0.6',
    policyRequestId: 'source-freshness-native-game-manual-request',
    target: {
      targetKind: TargetKind.NativeGame,
      targetRef: 'game-target-stale-or-missing-game',
    },
    requiredSources: [RequirementKind.Runtime, RequirementKind.Foreground, RequirementKind.Launcher],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Stale,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Stale,
        evidence: ['evidence-game-runtime-stale'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.NotClaimed,
        evidence: ['evidence-game-launcher-not-claimed'],
      },
    ],
  },
] as const;
"#;

pub fn app_game_preview_source_freshness_values_typescript() -> String {
    VALUES_TEMPLATE.replace(
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID_TOKEN,
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID,
    )
}

pub fn app_game_preview_source_freshness_rules_typescript() -> String {
    RULES_TEMPLATE.to_string()
}

pub fn app_game_preview_source_freshness_data_typescript() -> String {
    DATA_TEMPLATE.to_string()
}
