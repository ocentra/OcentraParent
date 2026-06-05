import {
  type AppGameTimeBudgetBonusGrant,
  type AppGameTimeBudgetDryRunDecision,
  AppGameTimeBudgetDryRunDecisionSchema,
  type AppGameTimeBudgetPolicy,
  type AppGameTimeBudgetSessionInput,
} from './app-game-time-budget-policy';
import {
  AppGameTimeBudgetApprovalState,
  AppGameTimeBudgetBonusState,
  AppGameTimeBudgetHandoffState,
  AppGameTimeBudgetRecommendedAction,
  AppGameTimeBudgetScheduleState,
  AppGameTimeBudgetTimerState,
  appGameTimeBudgetExpectedCountedSessionRefs,
  appGameTimeBudgetExpectedExcludedSessionRefs,
  appGameTimeBudgetSessionDurationMs,
  appGameTimeBudgetSessionMatchesTarget,
} from './app-game-time-budget-policy-rules';

export const AppGameTimeBudgetRuntimeMode = {
  DryRunPreview: 'dry-run-preview',
  WarnOnly: 'warn-only',
  AskParent: 'ask-parent',
  ManualRequired: 'manual-required',
} as const;

type AppGameTimeBudgetRuntimeModeValue =
  (typeof AppGameTimeBudgetRuntimeMode)[keyof typeof AppGameTimeBudgetRuntimeMode];
type AppGameTimeBudgetScheduleStateValue =
  (typeof AppGameTimeBudgetScheduleState)[keyof typeof AppGameTimeBudgetScheduleState];
type AppGameTimeBudgetRecommendedActionValue =
  (typeof AppGameTimeBudgetRecommendedAction)[keyof typeof AppGameTimeBudgetRecommendedAction];

export type AppGameTimeBudgetRuntimeEvaluationInput = {
  readonly schemaVersion: AppGameTimeBudgetDryRunDecision['schemaVersion'];
  readonly decisionId: AppGameTimeBudgetDryRunDecision['decisionId'];
  readonly policy: AppGameTimeBudgetPolicy;
  readonly sessions: ReadonlyArray<AppGameTimeBudgetSessionInput>;
  readonly scheduleState: AppGameTimeBudgetScheduleStateValue;
  readonly scheduleEvidenceReferences: AppGameTimeBudgetDryRunDecision['scheduleEvidenceReferences'];
  readonly bonusGrant: AppGameTimeBudgetBonusGrant;
  readonly runtimeMode: AppGameTimeBudgetRuntimeModeValue;
  readonly timerRefs: AppGameTimeBudgetDryRunDecision['timerRefs'];
  readonly auditRefs: AppGameTimeBudgetDryRunDecision['auditRefs'];
  readonly evaluatedAt: AppGameTimeBudgetDryRunDecision['evaluatedAt'];
};

const approvalStateByBonusState = {
  [AppGameTimeBudgetBonusState.None]: AppGameTimeBudgetApprovalState.NotRequired,
  [AppGameTimeBudgetBonusState.Requested]: AppGameTimeBudgetApprovalState.Pending,
  [AppGameTimeBudgetBonusState.ApprovedActive]: AppGameTimeBudgetApprovalState.Approved,
  [AppGameTimeBudgetBonusState.Denied]: AppGameTimeBudgetApprovalState.Denied,
  [AppGameTimeBudgetBonusState.Expired]: AppGameTimeBudgetApprovalState.Expired,
} as const;

const recommendedActionForExceededBudget = (
  input: AppGameTimeBudgetRuntimeEvaluationInput
): AppGameTimeBudgetRecommendedActionValue => {
  if (input.bonusGrant.bonusState === AppGameTimeBudgetBonusState.Requested) {
    return AppGameTimeBudgetRecommendedAction.AskParent;
  }

  switch (input.runtimeMode) {
    case AppGameTimeBudgetRuntimeMode.WarnOnly:
      return AppGameTimeBudgetRecommendedAction.Warn;
    case AppGameTimeBudgetRuntimeMode.AskParent:
      return AppGameTimeBudgetRecommendedAction.AskParent;
    case AppGameTimeBudgetRuntimeMode.ManualRequired:
      return AppGameTimeBudgetRecommendedAction.ManualRequired;
    case AppGameTimeBudgetRuntimeMode.DryRunPreview:
      return AppGameTimeBudgetRecommendedAction.TimeLimitDryRun;
  }
};

const handoffStateForAction = (recommendedAction: AppGameTimeBudgetRecommendedActionValue) => {
  switch (recommendedAction) {
    case AppGameTimeBudgetRecommendedAction.TimeLimitDryRun:
      return AppGameTimeBudgetHandoffState.DryRunOnly;
    case AppGameTimeBudgetRecommendedAction.ManualRequired:
      return AppGameTimeBudgetHandoffState.ManualRequired;
    case AppGameTimeBudgetRecommendedAction.Warn:
    case AppGameTimeBudgetRecommendedAction.AskParent:
    case AppGameTimeBudgetRecommendedAction.Observe:
      return AppGameTimeBudgetHandoffState.Disabled;
  }
};

const timerStateForAction = (
  input: AppGameTimeBudgetRuntimeEvaluationInput,
  recommendedAction: AppGameTimeBudgetRecommendedActionValue
) => {
  if (recommendedAction !== AppGameTimeBudgetRecommendedAction.TimeLimitDryRun) {
    return AppGameTimeBudgetTimerState.NotRequired;
  }

  return input.timerRefs.length > 0 ? AppGameTimeBudgetTimerState.Active : AppGameTimeBudgetTimerState.RecoveryNeeded;
};

const countedDurationMsFor = (
  policy: AppGameTimeBudgetPolicy,
  sessions: ReadonlyArray<AppGameTimeBudgetSessionInput>
) =>
  sessions
    .filter((session) => appGameTimeBudgetSessionMatchesTarget(session, policy.target))
    .reduce((totalMs, session) => totalMs + appGameTimeBudgetSessionDurationMs(session, policy.durationSource), 0);

const evidenceReferencesFor = (input: AppGameTimeBudgetRuntimeEvaluationInput) => [
  ...input.policy.previewEvidenceReferences,
  ...input.sessions.flatMap((session) => session.evidenceReferences),
  ...input.scheduleEvidenceReferences,
];

export const buildAppGameTimeBudgetRuntimeDecision = (
  input: AppGameTimeBudgetRuntimeEvaluationInput
): AppGameTimeBudgetDryRunDecision => {
  const countedDurationMs = countedDurationMsFor(input.policy, input.sessions);
  const effectiveBudgetLimitMs = input.policy.baseBudgetLimitMs + input.bonusGrant.bonusDurationMs;
  const budgetExceeded =
    input.scheduleState === AppGameTimeBudgetScheduleState.Active && countedDurationMs > effectiveBudgetLimitMs;
  const recommendedAction = budgetExceeded
    ? recommendedActionForExceededBudget(input)
    : AppGameTimeBudgetRecommendedAction.Observe;

  return AppGameTimeBudgetDryRunDecisionSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionId: input.decisionId,
    policy: input.policy,
    sessions: input.sessions,
    countedSessionRefs: appGameTimeBudgetExpectedCountedSessionRefs(input.sessions, input.policy.target),
    excludedSessionRefs: appGameTimeBudgetExpectedExcludedSessionRefs(input.sessions, input.policy.target),
    countedDurationMs,
    effectiveBudgetLimitMs,
    budgetExceeded,
    scheduleState: input.scheduleState,
    scheduleEvidenceReferences: input.scheduleEvidenceReferences,
    bonusGrant: input.bonusGrant,
    approvalState: approvalStateByBonusState[input.bonusGrant.bonusState],
    recommendedAction,
    dryRun: true,
    enforcementHandoffState: handoffStateForAction(recommendedAction),
    timerState: timerStateForAction(input, recommendedAction),
    timerRefs: recommendedAction === AppGameTimeBudgetRecommendedAction.TimeLimitDryRun ? input.timerRefs : [],
    auditRefs: input.auditRefs,
    evidenceReferences: evidenceReferencesFor(input),
    evaluatedAt: input.evaluatedAt,
  });
};
