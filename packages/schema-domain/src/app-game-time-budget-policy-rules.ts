export const AppGameTimeBudgetTargetKind = {
  AllNativeApps: 'all-native-apps',
  NativeApp: 'native-app',
  AppCategory: 'app-category',
  RiskApp: 'risk-app',
  AllNativeGames: 'all-native-games',
  NativeGame: 'native-game',
  GameCategory: 'game-category',
} as const;
export const AppGameTimeBudgetSessionKind = {
  NativeAppSession: 'native-app-session',
  NativeGameSession: 'native-game-session',
  LauncherGameCandidate: 'launcher-game-candidate',
} as const;
export const AppGameTimeBudgetPeriod = { Daily: 'daily', Weekly: 'weekly' } as const;
export const AppGameTimeBudgetDurationSource = {
  RunningDuration: 'running-duration',
  ForegroundDuration: 'foreground-duration',
} as const;
export const AppGameTimeBudgetScheduleState = {
  Active: 'active',
  Inactive: 'inactive',
  MissingProof: 'missing-proof',
  Expired: 'expired',
} as const;
export const AppGameTimeBudgetBonusState = {
  None: 'none',
  Requested: 'requested',
  ApprovedActive: 'approved-active',
  Denied: 'denied',
  Expired: 'expired',
} as const;
export const AppGameTimeBudgetApprovalState = {
  NotRequired: 'not-required',
  Pending: 'pending',
  Approved: 'approved',
  Denied: 'denied',
  Expired: 'expired',
} as const;
export const AppGameTimeBudgetTimerState = {
  NotRequired: 'not-required',
  Active: 'active',
  RestartRecovered: 'restart-recovered',
  Expired: 'expired',
  RecoveryNeeded: 'recovery-needed',
} as const;
export const AppGameTimeBudgetRecommendedAction = {
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'ask-parent',
  TimeLimitDryRun: 'time-limit-dry-run',
  ManualRequired: 'manual-required',
} as const;
export const AppGameTimeBudgetHandoffState = {
  Disabled: 'disabled',
  DryRunOnly: 'dry-run-only',
  ManualRequired: 'manual-required',
} as const;

type TargetKindValue = (typeof AppGameTimeBudgetTargetKind)[keyof typeof AppGameTimeBudgetTargetKind];
type SessionKindValue = (typeof AppGameTimeBudgetSessionKind)[keyof typeof AppGameTimeBudgetSessionKind];
type DurationSourceValue = (typeof AppGameTimeBudgetDurationSource)[keyof typeof AppGameTimeBudgetDurationSource];
type ScheduleStateValue = (typeof AppGameTimeBudgetScheduleState)[keyof typeof AppGameTimeBudgetScheduleState];
type BonusStateValue = (typeof AppGameTimeBudgetBonusState)[keyof typeof AppGameTimeBudgetBonusState];
type ApprovalStateValue = (typeof AppGameTimeBudgetApprovalState)[keyof typeof AppGameTimeBudgetApprovalState];
type TimerStateValue = (typeof AppGameTimeBudgetTimerState)[keyof typeof AppGameTimeBudgetTimerState];
type RecommendedActionValue =
  (typeof AppGameTimeBudgetRecommendedAction)[keyof typeof AppGameTimeBudgetRecommendedAction];
type HandoffStateValue = (typeof AppGameTimeBudgetHandoffState)[keyof typeof AppGameTimeBudgetHandoffState];

type TargetLike = {
  readonly targetKind: TargetKindValue;
  readonly targetRef: unknown;
};

type SessionLike = {
  readonly sessionRef: { readonly sessionRefId: unknown; readonly device: { readonly deviceId: unknown } };
  readonly sessionKind: SessionKindValue;
  readonly targetRef: unknown;
  readonly categoryRef: unknown;
  readonly riskSignalRef: unknown;
  readonly parentAllowedCandidate: boolean;
  readonly runningDurationMs: number;
  readonly foregroundDurationMs: number;
};

type PolicyLike = {
  readonly device: { readonly deviceId: unknown };
  readonly target: TargetLike;
  readonly baseBudgetLimitMs: number;
  readonly durationSource: DurationSourceValue;
};

type BonusGrantLike = {
  readonly bonusState: BonusStateValue;
  readonly bonusDurationMs: number;
  readonly approvalRef: unknown;
  readonly auditRefs: ReadonlyArray<unknown>;
};

type DecisionLike = {
  readonly policy: PolicyLike;
  readonly sessions: ReadonlyArray<SessionLike>;
  readonly countedSessionRefs: ReadonlyArray<unknown>;
  readonly excludedSessionRefs: ReadonlyArray<unknown>;
  readonly countedDurationMs: number;
  readonly effectiveBudgetLimitMs: number;
  readonly budgetExceeded: boolean;
  readonly scheduleState: ScheduleStateValue;
  readonly approvalState: ApprovalStateValue;
  readonly bonusGrant: BonusGrantLike;
  readonly recommendedAction: RecommendedActionValue;
  readonly enforcementHandoffState: HandoffStateValue;
  readonly timerState: TimerStateValue;
  readonly timerRefs: ReadonlyArray<unknown>;
  readonly auditRefs: ReadonlyArray<unknown>;
};

const allAppTargets = [AppGameTimeBudgetTargetKind.AllNativeApps] as const;
const allGameTargets = [AppGameTimeBudgetTargetKind.AllNativeGames] as const;

export const appGameTimeBudgetTargetAllowsNullRef = (target: TargetLike) =>
  [...allAppTargets, ...allGameTargets].some((targetKind) => targetKind === target.targetKind);

export const appGameTimeBudgetSessionMatchesTarget = (session: SessionLike, target: TargetLike) => {
  switch (target.targetKind) {
    case AppGameTimeBudgetTargetKind.AllNativeApps:
      return session.sessionKind === AppGameTimeBudgetSessionKind.NativeAppSession;
    case AppGameTimeBudgetTargetKind.AllNativeGames:
      return session.sessionKind === AppGameTimeBudgetSessionKind.NativeGameSession;
    case AppGameTimeBudgetTargetKind.NativeApp:
    case AppGameTimeBudgetTargetKind.NativeGame:
      return session.targetRef === target.targetRef;
    case AppGameTimeBudgetTargetKind.AppCategory:
    case AppGameTimeBudgetTargetKind.GameCategory:
      return session.categoryRef === target.targetRef;
    case AppGameTimeBudgetTargetKind.RiskApp:
      return session.riskSignalRef === target.targetRef;
  }
};

export const appGameTimeBudgetSessionDurationMs = (session: SessionLike, durationSource: DurationSourceValue) =>
  durationSource === AppGameTimeBudgetDurationSource.ForegroundDuration
    ? session.foregroundDurationMs
    : session.runningDurationMs;

export const appGameTimeBudgetExpectedCountedSessionRefs = (sessions: ReadonlyArray<SessionLike>, target: TargetLike) =>
  sessions
    .filter((session) => appGameTimeBudgetSessionMatchesTarget(session, target))
    .map((session) => session.sessionRef.sessionRefId);

export const appGameTimeBudgetExpectedExcludedSessionRefs = (
  sessions: ReadonlyArray<SessionLike>,
  target: TargetLike
) =>
  sessions
    .filter((session) => !appGameTimeBudgetSessionMatchesTarget(session, target))
    .map((session) => session.sessionRef.sessionRefId);

export const appGameTimeBudgetRefsMatch = (actualRefs: ReadonlyArray<unknown>, expectedRefs: ReadonlyArray<unknown>) =>
  actualRefs.length === expectedRefs.length && actualRefs.every((ref, index) => ref === expectedRefs[index]);

export const appGameTimeBudgetAllSessionsMatchPolicyDevice = (decision: DecisionLike) =>
  decision.sessions.every((session) => session.sessionRef.device.deviceId === decision.policy.device.deviceId);

export const appGameTimeBudgetDecisionCountsAreConsistent = (decision: DecisionLike) => {
  const expectedCountedRefs = appGameTimeBudgetExpectedCountedSessionRefs(decision.sessions, decision.policy.target);
  const expectedExcludedRefs = appGameTimeBudgetExpectedExcludedSessionRefs(decision.sessions, decision.policy.target);
  const expectedDurationMs = decision.sessions
    .filter((session) => appGameTimeBudgetSessionMatchesTarget(session, decision.policy.target))
    .reduce(
      (totalMs, session) => totalMs + appGameTimeBudgetSessionDurationMs(session, decision.policy.durationSource),
      0
    );

  return (
    appGameTimeBudgetRefsMatch(decision.countedSessionRefs, expectedCountedRefs) &&
    appGameTimeBudgetRefsMatch(decision.excludedSessionRefs, expectedExcludedRefs) &&
    decision.countedDurationMs === expectedDurationMs
  );
};

const bonusGrantValidators = {
  [AppGameTimeBudgetBonusState.None]: (bonusGrant: BonusGrantLike) =>
    bonusGrant.bonusDurationMs === 0 && bonusGrant.approvalRef === null && bonusGrant.auditRefs.length === 0,
  [AppGameTimeBudgetBonusState.Requested]: (bonusGrant: BonusGrantLike) =>
    bonusGrant.bonusDurationMs === 0 && bonusGrant.approvalRef === null && bonusGrant.auditRefs.length > 0,
  [AppGameTimeBudgetBonusState.ApprovedActive]: (bonusGrant: BonusGrantLike) =>
    bonusGrant.bonusDurationMs > 0 && bonusGrant.approvalRef !== null && bonusGrant.auditRefs.length > 0,
  [AppGameTimeBudgetBonusState.Denied]: (bonusGrant: BonusGrantLike) =>
    bonusGrant.bonusDurationMs === 0 && bonusGrant.approvalRef !== null && bonusGrant.auditRefs.length > 0,
  [AppGameTimeBudgetBonusState.Expired]: (bonusGrant: BonusGrantLike) =>
    bonusGrant.bonusDurationMs === 0 && bonusGrant.approvalRef !== null && bonusGrant.auditRefs.length > 0,
} satisfies Record<BonusStateValue, (bonusGrant: BonusGrantLike) => boolean>;

const bonusApprovalStateByBonusState = {
  [AppGameTimeBudgetBonusState.None]: AppGameTimeBudgetApprovalState.NotRequired,
  [AppGameTimeBudgetBonusState.Requested]: AppGameTimeBudgetApprovalState.Pending,
  [AppGameTimeBudgetBonusState.ApprovedActive]: AppGameTimeBudgetApprovalState.Approved,
  [AppGameTimeBudgetBonusState.Denied]: AppGameTimeBudgetApprovalState.Denied,
  [AppGameTimeBudgetBonusState.Expired]: AppGameTimeBudgetApprovalState.Expired,
} satisfies Record<BonusStateValue, ApprovalStateValue>;

export const appGameTimeBudgetBonusGrantIsConsistent = (bonusGrant: BonusGrantLike) =>
  bonusGrantValidators[bonusGrant.bonusState](bonusGrant);

export const appGameTimeBudgetBonusApprovalStateMatches = (
  bonusGrant: BonusGrantLike,
  approvalState: ApprovalStateValue
) => approvalState === bonusApprovalStateByBonusState[bonusGrant.bonusState];

export const appGameTimeBudgetDecisionBudgetMathIsConsistent = (decision: DecisionLike) => {
  const expectedLimitMs = decision.policy.baseBudgetLimitMs + decision.bonusGrant.bonusDurationMs;
  const expectedExceeded =
    decision.scheduleState === AppGameTimeBudgetScheduleState.Active && decision.countedDurationMs > expectedLimitMs;

  return decision.effectiveBudgetLimitMs === expectedLimitMs && decision.budgetExceeded === expectedExceeded;
};

export const appGameTimeBudgetRecommendedActionMatchesDecision = (decision: DecisionLike) => {
  if (!decision.budgetExceeded) {
    return decision.recommendedAction === AppGameTimeBudgetRecommendedAction.Observe;
  }

  switch (decision.recommendedAction) {
    case AppGameTimeBudgetRecommendedAction.Warn:
      return decision.enforcementHandoffState === AppGameTimeBudgetHandoffState.Disabled;
    case AppGameTimeBudgetRecommendedAction.AskParent:
      return decision.approvalState === AppGameTimeBudgetApprovalState.Pending;
    case AppGameTimeBudgetRecommendedAction.TimeLimitDryRun:
      return decision.enforcementHandoffState === AppGameTimeBudgetHandoffState.DryRunOnly;
    case AppGameTimeBudgetRecommendedAction.ManualRequired:
      return decision.enforcementHandoffState === AppGameTimeBudgetHandoffState.ManualRequired;
    case AppGameTimeBudgetRecommendedAction.Observe:
      return false;
  }
};

export const appGameTimeBudgetTimerStateIsAuditable = (decision: DecisionLike) =>
  decision.timerState !== AppGameTimeBudgetTimerState.RestartRecovered ||
  (decision.timerRefs.length > 0 && decision.auditRefs.length > 0);
