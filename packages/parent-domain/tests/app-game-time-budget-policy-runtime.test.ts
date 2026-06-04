import { describe, expect, it } from 'vitest';
import {
  AppGameTimeBudgetDurationSource,
  AppGameTimeBudgetPeriod,
  AppGameTimeBudgetScheduleState,
  AppGameTimeBudgetSessionKind,
  AppGameTimeBudgetTargetKind,
} from '../src/app-game-time-budget-policy-rules';
import {
  AppGameTimeBudgetRuntimeMode,
  buildAppGameTimeBudgetRuntimeDecision,
} from '../src/app-game-time-budget-policy-runtime';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-04T15:58:00Z';
const PolicyVersion = 'app-game-runtime-evaluator-policy-v1';

const ChildDevice = {
  deviceId: 'device-windows-app-game-runtime-1',
  childProfileId: 'child-profile-app-game-runtime-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const OtherDevice = {
  ...ChildDevice,
  deviceId: 'device-windows-app-game-runtime-2',
} as const;

const SessionEvidence = {
  evidenceReferenceId: 'evidence-app-game-runtime-session-1',
  kind: ParentEvidenceReferenceKind.QueryStoreSummary,
  observedAt: Timestamp,
} as const;

const GameEvidence = {
  evidenceReferenceId: 'evidence-app-game-runtime-session-2',
  kind: ParentEvidenceReferenceKind.QueryStoreSummary,
  observedAt: Timestamp,
} as const;

const ScheduleEvidence = {
  evidenceReferenceId: 'evidence-app-game-runtime-schedule-1',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentBonusApproval = {
  actionReferenceId: 'parent-action-runtime-bonus-approved-1',
  actor: {
    actorId: 'parent-actor-runtime-1',
    role: ParentActorRole.Parent,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const RuntimeBudgetPolicy = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  budgetPolicyId: 'app-game-runtime-budget-policy-1',
  policyVersion: PolicyVersion,
  ruleId: 'rule-app-runtime-budget-daily',
  device: ChildDevice,
  target: {
    targetKind: AppGameTimeBudgetTargetKind.AllNativeApps,
    targetRef: null,
  },
  period: AppGameTimeBudgetPeriod.Daily,
  baseBudgetLimitMs: 3_600_000,
  durationSource: AppGameTimeBudgetDurationSource.RunningDuration,
  scheduleRef: 'schedule-runtime-school-day',
  previewEvidenceReferences: [SessionEvidence],
} as const;

const NativeAppSession = {
  sessionRef: {
    sessionRefId: 'session-runtime-native-app',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  sessionKind: AppGameTimeBudgetSessionKind.NativeAppSession,
  targetRef: 'target-runtime-native-app',
  categoryRef: 'category-runtime-native-app',
  riskSignalRef: 'risk-runtime-native-app',
  parentAllowedCandidate: false,
  runningDurationMs: 4_200_000,
  foregroundDurationMs: 1_500_000,
  evidenceReferences: [SessionEvidence],
} as const;

const NativeGameSession = {
  sessionRef: {
    sessionRefId: 'session-runtime-native-game',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  sessionKind: AppGameTimeBudgetSessionKind.NativeGameSession,
  targetRef: 'target-runtime-native-game',
  categoryRef: 'category-runtime-native-game',
  riskSignalRef: null,
  parentAllowedCandidate: false,
  runningDurationMs: 4_800_000,
  foregroundDurationMs: 4_500_000,
  evidenceReferences: [GameEvidence],
} as const;

const NoBonusGrant = {
  bonusState: 'none',
  bonusDurationMs: 0,
  approvalRef: null,
  auditRefs: [],
} as const;

const baseRuntimeInput = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'app-game-runtime-budget-decision-1',
  policy: RuntimeBudgetPolicy,
  sessions: [NativeAppSession, NativeGameSession],
  scheduleState: AppGameTimeBudgetScheduleState.Active,
  scheduleEvidenceReferences: [ScheduleEvidence],
  bonusGrant: NoBonusGrant,
  runtimeMode: AppGameTimeBudgetRuntimeMode.DryRunPreview,
  timerRefs: ['timer-app-game-runtime-budget-1'],
  auditRefs: ['audit-app-game-runtime-budget-1'],
  evaluatedAt: Timestamp,
} as const;

const assertBuildsDryRunTimeLimitFromMatchingSessions = () => {
  const decision = buildAppGameTimeBudgetRuntimeDecision(baseRuntimeInput);

  expect(decision.countedSessionRefs).toEqual(['session-runtime-native-app']);
  expect(decision.excludedSessionRefs).toEqual(['session-runtime-native-game']);
  expect(decision.countedDurationMs).toBe(4_200_000);
  expect(decision.budgetExceeded).toBe(true);
  expect(decision.recommendedAction).toBe('time-limit-dry-run');
  expect(decision.enforcementHandoffState).toBe('dry-run-only');
  expect(decision.timerState).toBe('active');
  expect(decision.policy.ruleId).toBe('rule-app-runtime-budget-daily');
  expect(decision.evidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
    'evidence-app-game-runtime-session-1',
    'evidence-app-game-runtime-session-1',
    'evidence-app-game-runtime-session-2',
    'evidence-app-game-runtime-schedule-1',
  ]);
};

const assertForegroundModeUsesForegroundDuration = () => {
  const decision = buildAppGameTimeBudgetRuntimeDecision({
    ...baseRuntimeInput,
    decisionId: 'app-game-runtime-budget-decision-foreground',
    policy: {
      ...RuntimeBudgetPolicy,
      durationSource: AppGameTimeBudgetDurationSource.ForegroundDuration,
    },
  });

  expect(decision.countedDurationMs).toBe(1_500_000);
  expect(decision.budgetExceeded).toBe(false);
  expect(decision.recommendedAction).toBe('observe');
  expect(decision.enforcementHandoffState).toBe('disabled');
  expect(decision.timerState).toBe('not-required');
};

const assertPendingBonusRequestBecomesAskParent = () => {
  const decision = buildAppGameTimeBudgetRuntimeDecision({
    ...baseRuntimeInput,
    decisionId: 'app-game-runtime-budget-decision-ask-parent',
    bonusGrant: {
      bonusState: 'requested',
      bonusDurationMs: 0,
      approvalRef: null,
      auditRefs: ['audit-app-game-runtime-bonus-requested'],
    },
  });

  expect(decision.approvalState).toBe('pending');
  expect(decision.recommendedAction).toBe('ask-parent');
  expect(decision.enforcementHandoffState).toBe('disabled');
  expect(decision.timerState).toBe('not-required');
};

const assertManualModeStaysManualRequired = () => {
  const decision = buildAppGameTimeBudgetRuntimeDecision({
    ...baseRuntimeInput,
    decisionId: 'app-game-runtime-budget-decision-manual',
    runtimeMode: AppGameTimeBudgetRuntimeMode.ManualRequired,
    timerRefs: [],
  });

  expect(decision.recommendedAction).toBe('manual-required');
  expect(decision.enforcementHandoffState).toBe('manual-required');
  expect(decision.timerState).toBe('not-required');
};

const assertBonusApprovalExtendsTheEffectiveLimit = () => {
  const decision = buildAppGameTimeBudgetRuntimeDecision({
    ...baseRuntimeInput,
    decisionId: 'app-game-runtime-budget-decision-approved-bonus',
    bonusGrant: {
      bonusState: 'approved-active',
      bonusDurationMs: 900_000,
      approvalRef: ParentBonusApproval,
      auditRefs: ['audit-app-game-runtime-bonus-approved'],
    },
  });

  expect(decision.effectiveBudgetLimitMs).toBe(4_500_000);
  expect(decision.budgetExceeded).toBe(false);
  expect(decision.approvalState).toBe('approved');
  expect(decision.recommendedAction).toBe('observe');
};

const assertRejectsCrossDeviceSessions = () => {
  const crossDeviceSession = {
    ...NativeAppSession,
    sessionRef: {
      ...NativeAppSession.sessionRef,
      device: OtherDevice,
    },
  } as const;
  let message = '';

  try {
    buildAppGameTimeBudgetRuntimeDecision({
      ...baseRuntimeInput,
      sessions: [crossDeviceSession, NativeGameSession],
    });
  } catch (error) {
    message = error instanceof Error ? error.message : '';
  }

  expect(message).toContain('Expected app/game time budget decisions to stay on one policy device');
};

describe('app/game time budget policy runtime evaluator', () => {
  it(
    'builds a dry-run time-limit decision from matching stored sessions',
    assertBuildsDryRunTimeLimitFromMatchingSessions
  );
  it('uses foreground duration when the policy selects foreground budgets', assertForegroundModeUsesForegroundDuration);
  it('turns a pending bonus request into an ask-parent decision', assertPendingBonusRequestBecomesAskParent);
  it('keeps manual-required runtime mode out of adapter handoff', assertManualModeStaysManualRequired);
  it('extends the effective limit only with approved bonus time proof', assertBonusApprovalExtendsTheEffectiveLimit);
  it('rejects cross-device session inputs through the existing schema', assertRejectsCrossDeviceSessions);
});
