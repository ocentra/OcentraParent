import {
  type AppGameSourceFreshnessCapabilityStatus as AppGameSourceFreshnessCapabilityStatusValue,
  AppGameSourceFreshnessCapabilityStatus,
  type AppGameSourceFreshnessPolicyTargetKind as AppGameSourceFreshnessPolicyTargetKindValue,
  AppGameSourceFreshnessPolicyTargetKind,
  type AppGameSourceFreshnessReadModelState as AppGameSourceFreshnessReadModelStateValue,
  AppGameSourceFreshnessReadModelState,
  type AppGameSourceFreshnessReasonCode as AppGameSourceFreshnessReasonCodeValue,
  AppGameSourceFreshnessReasonCode,
  type AppGameSourceFreshnessRequirementKind as AppGameSourceFreshnessRequirementKindValue,
  AppGameSourceFreshnessRequirementSourceKinds,
  type AppGameSourceFreshnessRequirementState as AppGameSourceFreshnessRequirementStateValue,
  AppGameSourceFreshnessRequirementState,
  type AppGameSourceFreshnessSourceKind as AppGameSourceFreshnessSourceKindValue,
} from './app-game-source-freshness-policy-consumption-values';

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
  readonly sourceEvidenceRefs: ReadonlyArray<unknown>;
};

type TargetLike = {
  readonly targetKind: AppGameSourceFreshnessPolicyTargetKindValue;
  readonly targetRef: unknown;
};

type ReadinessLike = {
  readonly requirementResults: ReadonlyArray<RequirementResultLike>;
  readonly policyEvidenceRefs: ReadonlyArray<unknown>;
  readonly policyCompileAllowed: boolean;
  readonly directAdapterCallRequested: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
};

type RequirementFailure = {
  readonly requirementState: AppGameSourceFreshnessRequirementStateValue;
  readonly reasonCode: AppGameSourceFreshnessReasonCodeValue;
};

type RequirementFailureCheck = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) => RequirementFailure | null;

export const appGameSourceFreshnessTargetAllowsNullRef = (target: TargetLike) =>
  target.targetKind === AppGameSourceFreshnessPolicyTargetKind.AllNativeApps ||
  target.targetKind === AppGameSourceFreshnessPolicyTargetKind.AllNativeGames;

export const appGameSourceFreshnessSourceKindSatisfiesRequirement = (
  sourceKind: AppGameSourceFreshnessSourceKindValue,
  requirementKind: AppGameSourceFreshnessRequirementKindValue
) => AppGameSourceFreshnessRequirementSourceKinds[requirementKind].some((candidate) => candidate === sourceKind);

export const appGameSourceFreshnessRowsForRequirement = (
  rows: ReadonlyArray<SourceStatusRowLike>,
  requirementKind: AppGameSourceFreshnessRequirementKindValue
) => rows.filter((row) => appGameSourceFreshnessSourceKindSatisfiesRequirement(row.sourceKind, requirementKind));

export const appGameSourceFreshnessRowIsFresh = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) => {
  const observedAtMs = parseTimestampMillis(row.lastObservedAt);
  const evaluatedAtMs = parseTimestampMillis(evaluatedAt);

  if (observedAtMs === null || evaluatedAtMs === null) {
    return false;
  }

  return Math.max(0, evaluatedAtMs - observedAtMs) <= maxSourceAgeMs;
};

export const appGameSourceFreshnessRowHasEvidence = (row: SourceStatusRowLike) =>
  row.rowCount === 0 || row.evidence.length > 0;

export const appGameSourceFreshnessRequirementIsSatisfied = (result: RequirementResultLike) =>
  result.requirementState === AppGameSourceFreshnessRequirementState.Satisfied &&
  result.reasonCode === null &&
  result.sourceEvidenceRefs.length > 0;

export const appGameSourceFreshnessReadinessIsPolicyReady = (readiness: ReadinessLike) =>
  readiness.policyCompileAllowed &&
  readiness.directAdapterCallRequested === false &&
  readiness.rawPrivateSourceRowsIncluded === false &&
  readiness.policyEvidenceRefs.length > 0 &&
  readiness.requirementResults.every(appGameSourceFreshnessRequirementIsSatisfied);

export const appGameSourceFreshnessRequirementFailure = (
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
) =>
  RequirementFailureChecks.reduce<RequirementFailure | null>(
    (failure, check) => failure ?? check(row, evaluatedAt, maxSourceAgeMs),
    null
  );

const RequirementFailureChecks = [
  sourceRowEmptyFailure,
  sourceRowMissingObservedAtFailure,
  sourceRowStaleFailure,
  sourceRowPermissionLimitedFailure,
  sourceRowUnavailableFailure,
  sourceRowAdapterErrorFailure,
  sourceRowManualRequiredFailure,
  sourceRowNotClaimedFailure,
  sourceRowMissingEvidenceFailure,
] satisfies readonly RequirementFailureCheck[];

function sourceRowEmptyFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (row.rowCount === 0 || row.state === AppGameSourceFreshnessReadModelState.Empty) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.Empty,
      reasonCode: AppGameSourceFreshnessReasonCode.EmptySourceStatusRow,
    } as const;
  }

  return null;
}

function sourceRowMissingObservedAtFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (row.lastObservedAt === null) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.MissingObservedAt,
      reasonCode: AppGameSourceFreshnessReasonCode.MissingObservedAt,
    } as const;
  }

  return null;
}

function sourceRowStaleFailure(
  row: SourceStatusRowLike,
  evaluatedAt: unknown,
  maxSourceAgeMs: number
): RequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelState.Stale ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.Stale ||
    !appGameSourceFreshnessRowIsFresh(row, evaluatedAt, maxSourceAgeMs)
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.Stale,
      reasonCode: AppGameSourceFreshnessReasonCode.StaleSourceStatusRow,
    } as const;
  }

  return null;
}

function sourceRowPermissionLimitedFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelState.PermissionRequired ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.PermissionLimited
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.PermissionLimited,
      reasonCode: AppGameSourceFreshnessReasonCode.PermissionLimitedSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowUnavailableFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelState.Unavailable ||
    row.state === AppGameSourceFreshnessReadModelState.Offline ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.Unavailable ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.UnsupportedPlatform
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.Unavailable,
      reasonCode: AppGameSourceFreshnessReasonCode.UnavailableSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowAdapterErrorFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.AdapterError) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.AdapterError,
      reasonCode: AppGameSourceFreshnessReasonCode.AdapterErrorSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowManualRequiredFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (
    row.state === AppGameSourceFreshnessReadModelState.ScaffoldOnly ||
    row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.ManualRequired
  ) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.ManualRequired,
      reasonCode: AppGameSourceFreshnessReasonCode.ManualRequiredSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowNotClaimedFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.NotClaimed) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.NotClaimed,
      reasonCode: AppGameSourceFreshnessReasonCode.NotClaimedSourceStatus,
    } as const;
  }

  return null;
}

function sourceRowMissingEvidenceFailure(row: SourceStatusRowLike): RequirementFailure | null {
  if (row.evidence.length === 0) {
    return {
      requirementState: AppGameSourceFreshnessRequirementState.MissingEvidence,
      reasonCode: AppGameSourceFreshnessReasonCode.MissingSourceEvidence,
    } as const;
  }

  return null;
}

function parseTimestampMillis(value: unknown): number | null {
  if (typeof value !== 'string') {
    return null;
  }

  const parsed = Date.parse(value);
  return Number.isFinite(parsed) ? parsed : null;
}
