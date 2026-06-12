export const NativeGameBudgetEvidenceKind = {
  KnownGameSession: 'known-game-session',
  LauncherGameCandidate: 'launcher-game-candidate',
  LauncherOnly: 'launcher-only',
  PossiblyGame: 'possibly-game',
  UnknownGameLike: 'unknown-game-like',
} as const;

export const NativeGameBudgetCandidatePolicy = {
  ExcludeCandidates: 'exclude-candidates',
  IncludeParentApprovedCandidates: 'include-parent-approved-candidates',
  ReviewCandidates: 'review-candidates',
} as const;

export const NativeGameBudgetDurationSource = {
  RunningDuration: 'running-duration',
  ForegroundDuration: 'foreground-duration',
} as const;

export const NativeGameBudgetRecommendedAction = {
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'ask-parent',
  TimeLimitDryRun: 'time-limit-dry-run',
} as const;

export const NativeGameBudgetSignalKind = {
  Rating: 'rating',
  Ugc: 'ugc',
  Multiplayer: 'multiplayer',
  Purchase: 'purchase',
  NativeGameCategory: 'native-game-category',
} as const;

export const NativeGameBudgetSignalPolicyRole = {
  ParentPreviewOnly: 'parent-preview-only',
  BudgetTargetingInput: 'budget-targeting-input',
  ManualReviewInput: 'manual-review-input',
  DirectEnforcement: 'direct-enforcement',
} as const;

type NativeGameBudgetEvidenceKindValue =
  (typeof NativeGameBudgetEvidenceKind)[keyof typeof NativeGameBudgetEvidenceKind];
type NativeGameBudgetCandidatePolicyValue =
  (typeof NativeGameBudgetCandidatePolicy)[keyof typeof NativeGameBudgetCandidatePolicy];
type NativeGameBudgetDurationSourceValue =
  (typeof NativeGameBudgetDurationSource)[keyof typeof NativeGameBudgetDurationSource];
type NativeGameBudgetRecommendedActionValue =
  (typeof NativeGameBudgetRecommendedAction)[keyof typeof NativeGameBudgetRecommendedAction];
type NativeGameBudgetSignalKindValue = (typeof NativeGameBudgetSignalKind)[keyof typeof NativeGameBudgetSignalKind];
type NativeGameBudgetSignalPolicyRoleValue =
  (typeof NativeGameBudgetSignalPolicyRole)[keyof typeof NativeGameBudgetSignalPolicyRole];

type NativeGameBudgetSessionLike = {
  readonly sessionRef: { readonly sessionRefId: unknown; readonly device: { readonly deviceId: unknown } };
  readonly evidenceKind: NativeGameBudgetEvidenceKindValue;
  readonly parentAllowedCandidate: boolean;
  readonly runningDurationMs: number;
  readonly foregroundDurationMs: number;
};

type NativeGameBudgetSignalLike = {
  readonly signalKind: NativeGameBudgetSignalKindValue;
  readonly policyRole: NativeGameBudgetSignalPolicyRoleValue;
};

type NativeGameBudgetPolicyLike = {
  readonly device: { readonly deviceId: unknown };
  readonly dailyBudgetMinutes: number;
  readonly durationSource: NativeGameBudgetDurationSourceValue;
  readonly candidatePolicy: NativeGameBudgetCandidatePolicyValue;
};

type NativeGameBudgetDecisionLike = {
  readonly policy: NativeGameBudgetPolicyLike;
  readonly sessions: ReadonlyArray<NativeGameBudgetSessionLike>;
  readonly countedSessionRefs: ReadonlyArray<unknown>;
  readonly excludedSessionRefs: ReadonlyArray<unknown>;
  readonly countedDurationMs: number;
  readonly budgetLimitMs: number;
  readonly budgetExceeded: boolean;
};

export const nativeGameBudgetSessionCountsTowardBudget = (
  session: NativeGameBudgetSessionLike,
  candidatePolicy: NativeGameBudgetCandidatePolicyValue
) => {
  if (session.evidenceKind === NativeGameBudgetEvidenceKind.KnownGameSession) {
    return true;
  }

  return (
    session.evidenceKind === NativeGameBudgetEvidenceKind.LauncherGameCandidate &&
    candidatePolicy === NativeGameBudgetCandidatePolicy.IncludeParentApprovedCandidates &&
    session.parentAllowedCandidate
  );
};

export const nativeGameBudgetSessionDurationMs = (
  session: NativeGameBudgetSessionLike,
  durationSource: NativeGameBudgetDurationSourceValue
) =>
  durationSource === NativeGameBudgetDurationSource.ForegroundDuration
    ? session.foregroundDurationMs
    : session.runningDurationMs;

export const nativeGameBudgetExpectedCountedSessionRefs = (
  sessions: ReadonlyArray<NativeGameBudgetSessionLike>,
  candidatePolicy: NativeGameBudgetCandidatePolicyValue
) =>
  sessions
    .filter((session) => nativeGameBudgetSessionCountsTowardBudget(session, candidatePolicy))
    .map((session) => session.sessionRef.sessionRefId);

export const nativeGameBudgetExpectedExcludedSessionRefs = (
  sessions: ReadonlyArray<NativeGameBudgetSessionLike>,
  candidatePolicy: NativeGameBudgetCandidatePolicyValue
) =>
  sessions
    .filter((session) => !nativeGameBudgetSessionCountsTowardBudget(session, candidatePolicy))
    .map((session) => session.sessionRef.sessionRefId);

export const nativeGameBudgetExpectedCountedDurationMs = (
  sessions: ReadonlyArray<NativeGameBudgetSessionLike>,
  candidatePolicy: NativeGameBudgetCandidatePolicyValue,
  durationSource: NativeGameBudgetDurationSourceValue
) =>
  sessions
    .filter((session) => nativeGameBudgetSessionCountsTowardBudget(session, candidatePolicy))
    .reduce((totalMs, session) => totalMs + nativeGameBudgetSessionDurationMs(session, durationSource), 0);

export const nativeGameBudgetRefsMatch = (actualRefs: ReadonlyArray<unknown>, expectedRefs: ReadonlyArray<unknown>) =>
  actualRefs.length === expectedRefs.length && actualRefs.every((ref, index) => ref === expectedRefs[index]);

export const nativeGameBudgetAllSessionsMatchPolicyDevice = (decision: NativeGameBudgetDecisionLike) =>
  decision.sessions.every((session) => session.sessionRef.device.deviceId === decision.policy.device.deviceId);

export const nativeGameBudgetDecisionCountsAreConsistent = (decision: NativeGameBudgetDecisionLike) => {
  const expectedCountedRefs = nativeGameBudgetExpectedCountedSessionRefs(
    decision.sessions,
    decision.policy.candidatePolicy
  );
  const expectedExcludedRefs = nativeGameBudgetExpectedExcludedSessionRefs(
    decision.sessions,
    decision.policy.candidatePolicy
  );
  const expectedDurationMs = nativeGameBudgetExpectedCountedDurationMs(
    decision.sessions,
    decision.policy.candidatePolicy,
    decision.policy.durationSource
  );

  return (
    nativeGameBudgetRefsMatch(decision.countedSessionRefs, expectedCountedRefs) &&
    nativeGameBudgetRefsMatch(decision.excludedSessionRefs, expectedExcludedRefs) &&
    decision.countedDurationMs === expectedDurationMs
  );
};

export const nativeGameBudgetDecisionBudgetMathIsConsistent = (decision: NativeGameBudgetDecisionLike) => {
  const expectedBudgetLimitMs = decision.policy.dailyBudgetMinutes * 60_000;

  return (
    decision.budgetLimitMs === expectedBudgetLimitMs &&
    decision.budgetExceeded === decision.countedDurationMs > expectedBudgetLimitMs
  );
};

export const nativeGameBudgetRecommendedActionMatchesBudget = (
  recommendedAction: NativeGameBudgetRecommendedActionValue,
  budgetExceeded: boolean,
  countedDurationMs: number
) =>
  recommendedAction !== NativeGameBudgetRecommendedAction.TimeLimitDryRun || (budgetExceeded && countedDurationMs > 0);

export const nativeGameBudgetSignalIsAdvisoryOnly = (signal: NativeGameBudgetSignalLike) => {
  const directEnforcementSignalKinds = [
    NativeGameBudgetSignalKind.Rating,
    NativeGameBudgetSignalKind.Ugc,
    NativeGameBudgetSignalKind.Multiplayer,
    NativeGameBudgetSignalKind.Purchase,
  ] as const;
  const isDirectEnforcementSignalKind = directEnforcementSignalKinds.some(
    (signalKind) => signalKind === signal.signalKind
  );

  return !isDirectEnforcementSignalKind || signal.policyRole !== NativeGameBudgetSignalPolicyRole.DirectEnforcement;
};
