import { describe, expect, it } from 'vitest';
import { AppGameTimeBudgetDryRunDecisionSchema } from '../src/app-game-time-budget-policy';
import {
  AppGameTimeBudgetApprovalState,
  AppGameTimeBudgetBonusState,
  AppGameTimeBudgetDurationSource,
  AppGameTimeBudgetHandoffState,
  AppGameTimeBudgetPeriod,
  AppGameTimeBudgetRecommendedAction,
  AppGameTimeBudgetScheduleState,
  AppGameTimeBudgetSessionKind,
  AppGameTimeBudgetTargetKind,
  AppGameTimeBudgetTimerState,
} from '../src/app-game-time-budget-policy-rules';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-03T09:05:00Z';
const PolicyVersion = 'app-game-time-budget-policy-v1';

const ChildDevice = {
  deviceId: 'device-windows-app-game-1',
  childProfileId: 'child-profile-app-game-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const SessionEvidence = {
  evidenceReferenceId: 'evidence-session-summary-app-game-1',
  kind: ParentEvidenceReferenceKind.QueryStoreSummary,
  observedAt: Timestamp,
} as const;

const ScheduleEvidence = {
  evidenceReferenceId: 'evidence-schedule-window-1',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentBonusApproval = {
  actionReferenceId: 'parent-action-bonus-approved-1',
  actor: {
    actorId: 'parent-actor-1',
    role: ParentActorRole.Parent,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const AppBudgetPolicy = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  budgetPolicyId: 'app-game-budget-policy-apps-1',
  policyVersion: PolicyVersion,
  ruleId: 'rule-app-budget-daily',
  device: ChildDevice,
  target: {
    targetKind: AppGameTimeBudgetTargetKind.AllNativeApps,
    targetRef: null,
  },
  period: AppGameTimeBudgetPeriod.Daily,
  baseBudgetLimitMs: 3_600_000,
  durationSource: AppGameTimeBudgetDurationSource.RunningDuration,
  scheduleRef: 'schedule-school-day',
  previewEvidenceReferences: [SessionEvidence],
} as const;

const NativeAppSession = {
  sessionRef: {
    sessionRefId: 'session-homework-chat-app',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  sessionKind: AppGameTimeBudgetSessionKind.NativeAppSession,
  targetRef: 'target-homework-chat-app',
  categoryRef: 'category-ai-chatbot',
  riskSignalRef: 'risk-ai-chatbot',
  parentAllowedCandidate: false,
  runningDurationMs: 4_200_000,
  foregroundDurationMs: 1_800_000,
  evidenceReferences: [SessionEvidence],
} as const;

const NativeGameSession = {
  sessionRef: {
    sessionRefId: 'session-native-game',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  sessionKind: AppGameTimeBudgetSessionKind.NativeGameSession,
  targetRef: 'target-native-game',
  categoryRef: 'category-game-rpg',
  riskSignalRef: null,
  parentAllowedCandidate: false,
  runningDurationMs: 4_800_000,
  foregroundDurationMs: 4_200_000,
  evidenceReferences: [SessionEvidence],
} as const;

const NoBonusGrant = {
  bonusState: AppGameTimeBudgetBonusState.None,
  bonusDurationMs: 0,
  approvalRef: null,
  auditRefs: [],
} as const;

const BudgetExceededDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'app-game-time-budget-decision-1',
  policy: AppBudgetPolicy,
  sessions: [NativeAppSession, NativeGameSession],
  countedSessionRefs: ['session-homework-chat-app'],
  excludedSessionRefs: ['session-native-game'],
  countedDurationMs: 4_200_000,
  effectiveBudgetLimitMs: 3_600_000,
  budgetExceeded: true,
  scheduleState: AppGameTimeBudgetScheduleState.Active,
  scheduleEvidenceReferences: [ScheduleEvidence],
  bonusGrant: NoBonusGrant,
  approvalState: AppGameTimeBudgetApprovalState.NotRequired,
  recommendedAction: AppGameTimeBudgetRecommendedAction.TimeLimitDryRun,
  dryRun: true,
  enforcementHandoffState: AppGameTimeBudgetHandoffState.DryRunOnly,
  timerState: AppGameTimeBudgetTimerState.Active,
  timerRefs: ['timer-app-budget-active'],
  auditRefs: ['audit-app-budget-dry-run'],
  evidenceReferences: [SessionEvidence],
  evaluatedAt: Timestamp,
} as const;

const assertUsesStoredSessionSummaries = () => {
  const parsed = AppGameTimeBudgetDryRunDecisionSchema.safeParse(BudgetExceededDecision);
  const wrongPartition = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BudgetExceededDecision,
    countedSessionRefs: ['session-homework-chat-app', 'session-native-game'],
    excludedSessionRefs: [],
    countedDurationMs: 9_000_000,
  });

  expect(parsed.success).toBe(true);
  expect(wrongPartition.success).toBe(false);
  if (parsed.success) {
    expect(parsed.data.policy.target.targetKind).toBe('all-native-apps');
    expect(parsed.data.countedSessionRefs).toEqual(['session-homework-chat-app']);
    expect(parsed.data.evidenceReferences[0]?.kind).toBe('query-store-summary');
  }
};

const assertForegroundOnlyBudgetsStayDistinct = () => {
  const parsed = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BudgetExceededDecision,
    decisionId: 'app-game-time-budget-decision-foreground',
    policy: {
      ...AppBudgetPolicy,
      durationSource: AppGameTimeBudgetDurationSource.ForegroundDuration,
    },
    countedDurationMs: 1_800_000,
    budgetExceeded: false,
    recommendedAction: AppGameTimeBudgetRecommendedAction.Observe,
    enforcementHandoffState: AppGameTimeBudgetHandoffState.Disabled,
    timerState: AppGameTimeBudgetTimerState.NotRequired,
    timerRefs: [],
  });
  const runningDurationClaim = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BudgetExceededDecision,
    decisionId: 'app-game-time-budget-decision-foreground-wrong',
    policy: {
      ...AppBudgetPolicy,
      durationSource: AppGameTimeBudgetDurationSource.ForegroundDuration,
    },
  });

  expect(parsed.success).toBe(true);
  expect(runningDurationClaim.success).toBe(false);
};

const assertBonusTimeRequiresApprovalAndAuditRefs = () => {
  const parsed = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BudgetExceededDecision,
    decisionId: 'app-game-time-budget-decision-bonus',
    effectiveBudgetLimitMs: 4_500_000,
    budgetExceeded: false,
    bonusGrant: {
      bonusState: AppGameTimeBudgetBonusState.ApprovedActive,
      bonusDurationMs: 900_000,
      approvalRef: ParentBonusApproval,
      auditRefs: ['audit-bonus-time-approved'],
    },
    approvalState: AppGameTimeBudgetApprovalState.Approved,
    recommendedAction: AppGameTimeBudgetRecommendedAction.Observe,
    enforcementHandoffState: AppGameTimeBudgetHandoffState.Disabled,
  });
  const missingApproval = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BudgetExceededDecision,
    decisionId: 'app-game-time-budget-decision-bonus-missing-approval',
    effectiveBudgetLimitMs: 4_500_000,
    bonusGrant: {
      bonusState: AppGameTimeBudgetBonusState.ApprovedActive,
      bonusDurationMs: 900_000,
      approvalRef: null,
      auditRefs: ['audit-bonus-time-approved'],
    },
    approvalState: AppGameTimeBudgetApprovalState.Approved,
  });

  expect(parsed.success).toBe(true);
  expect(missingApproval.success).toBe(false);
};

describe('app game time budget policy contracts', () => {
  it('uses stored app/game session summaries to count only matching targets', assertUsesStoredSessionSummaries);
  it('keeps foreground-only and running-time budget modes distinct', assertForegroundOnlyBudgetsStayDistinct);
  it(
    'requires approval and audit refs before bonus time extends the limit',
    assertBonusTimeRequiresApprovalAndAuditRefs
  );
});
