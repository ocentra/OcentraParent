/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

import { AppGamePolicyCompilerOutcomeState, AppGamePolicyTargetKind } from './app-game-policy-target-compiler-rules';
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
  AppGameSourceFreshnessRequirementKindGenerated,
  AppGameSourceFreshnessRequirementSourceKindsGenerated,
  AppGameSourceFreshnessRequirementStateGenerated,
  AppGameSourceFreshnessSourceKindGenerated,
  AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated,
} from './generated-app-game-preview-source-freshness-values';

type AppGamePolicyTargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type AppGamePolicyPreviewTargetDomainValue =
  (typeof AppGamePolicyPreviewTargetDomainGenerated)[keyof typeof AppGamePolicyPreviewTargetDomainGenerated];
type AppGamePolicyPreviewStatusValue =
  (typeof AppGamePolicyPreviewStatusGenerated)[keyof typeof AppGamePolicyPreviewStatusGenerated];
type AppGamePolicyCompilerOutcomeStateValue =
  (typeof AppGamePolicyCompilerOutcomeState)[keyof typeof AppGamePolicyCompilerOutcomeState];
type AppGameSourceFreshnessPolicyTargetKindValue =
  (typeof AppGameSourceFreshnessPolicyTargetKindGenerated)[keyof typeof AppGameSourceFreshnessPolicyTargetKindGenerated];
type AppGameSourceFreshnessRequirementKindValue =
  (typeof AppGameSourceFreshnessRequirementKindGenerated)[keyof typeof AppGameSourceFreshnessRequirementKindGenerated];
type AppGameSourceFreshnessSourceKindValue =
  (typeof AppGameSourceFreshnessSourceKindGenerated)[keyof typeof AppGameSourceFreshnessSourceKindGenerated];
type AppGameSourceFreshnessReadModelStateValue =
  (typeof AppGameSourceFreshnessReadModelStateGenerated)[keyof typeof AppGameSourceFreshnessReadModelStateGenerated];
type AppGameSourceFreshnessCapabilityStatusValue =
  (typeof AppGameSourceFreshnessCapabilityStatusGenerated)[keyof typeof AppGameSourceFreshnessCapabilityStatusGenerated];
type AppGameSourceFreshnessRequirementStateValue =
  (typeof AppGameSourceFreshnessRequirementStateGenerated)[keyof typeof AppGameSourceFreshnessRequirementStateGenerated];
type AppGameSourceFreshnessReasonCodeValue =
  (typeof AppGameSourceFreshnessReasonCodeGenerated)[keyof typeof AppGameSourceFreshnessReasonCodeGenerated];
type AppGameSourceFreshnessPreviewGateStatusValue =
  (typeof AppGameSourceFreshnessPreviewGateStatusGenerated)[keyof typeof AppGameSourceFreshnessPreviewGateStatusGenerated];
type AppGameSourceFreshnessPreviewGateStateValue =
  (typeof AppGameSourceFreshnessPreviewGateStateGenerated)[keyof typeof AppGameSourceFreshnessPreviewGateStateGenerated];
type AppGameSourceGatedPolicyPreviewProjectionStateValue =
  (typeof AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated)[keyof typeof AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated];

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
  row.dryRun && row.enforcementHandoffState === 'disabled';

export const appGamePolicyPreviewRowHasNoRuntimeClaimsGenerated = (row: PreviewRowLike) =>
  row.policyEvaluatorRuntimeClaimState ===
    AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.policyEvaluatorRuntimeClaimState &&
  row.timerRuntimeClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.timerRuntimeClaimState &&
  row.adapterDispatchState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.adapterDispatchState &&
  row.childDeliveryClaimState === AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.childDeliveryClaimState &&
  row.platformEnforcementClaimState ===
    AppGamePolicyPreviewNoRuntimeClaimStatesGenerated.platformEnforcementClaimState &&
  row.policyEvaluatorRuntimeClaimed ===
    AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated.policyEvaluatorRuntimeClaimed &&
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
  nativeGameRowCount: countAppGamePolicyPreviewRowsGenerated(
    rows,
    AppGamePolicyPreviewTargetDomainGenerated.NativeGame
  ),
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
) =>
  rows.filter((row) => appGameSourceFreshnessSourceKindSatisfiesRequirementGenerated(row.sourceKind, requirementKind));

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

export const appGameSourceFreshnessReadinessIsPolicyReadyGenerated = (readiness: SourceFreshnessReadinessLike) =>
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

function sourceRowUnavailableFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
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

function sourceRowAdapterErrorFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.AdapterError) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.AdapterError,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.AdapterErrorSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowManualRequiredFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
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

function sourceRowNotClaimedFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatusGenerated.NotClaimed) {
    return {
      requirementState: AppGameSourceFreshnessRequirementStateGenerated.NotClaimed,
      reasonCode: AppGameSourceFreshnessReasonCodeGenerated.NotClaimedSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowMissingEvidenceFailureGenerated(row: SourceStatusRowLike): SourceFreshnessRequirementFailure | null {
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
