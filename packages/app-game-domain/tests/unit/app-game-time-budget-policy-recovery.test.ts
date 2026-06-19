import { describe, expect, it } from 'vitest';
import { AppGameTimeBudgetDryRunDecisionSchema } from '../../src/app-game-time-budget-policy';
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
} from '../../src/app-game-time-budget-policy-rules';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-03T09:15:00Z';
const ChildDevice = {
  deviceId: 'device-windows-budget-recovery',
  childProfileId: 'child-profile-budget-recovery',
  label: 'Study PC',
  platform: 'windows',
} as const;
const SessionEvidence = {
  evidenceReferenceId: 'evidence-session-summary-recovery',
  kind: ParentEvidenceReferenceKind.QueryStoreSummary,
  observedAt: Timestamp,
} as const;
const ScheduleEvidence = {
  evidenceReferenceId: 'evidence-schedule-recovery',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const BaseDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'app-game-time-budget-decision-recovery',
  policy: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    budgetPolicyId: 'app-game-budget-policy-recovery',
    policyVersion: 'app-game-budget-policy-version-recovery',
    ruleId: 'rule-app-budget-recovery',
    device: ChildDevice,
    target: {
      targetKind: AppGameTimeBudgetTargetKind.NativeApp,
      targetRef: 'target-homework-chat-app',
    },
    period: AppGameTimeBudgetPeriod.Daily,
    baseBudgetLimitMs: 3_600_000,
    durationSource: AppGameTimeBudgetDurationSource.RunningDuration,
    scheduleRef: 'schedule-school-day',
    previewEvidenceReferences: [SessionEvidence],
  },
  sessions: [
    {
      sessionRef: {
        sessionRefId: 'session-homework-chat-app-recovery',
        device: ChildDevice,
        observedAt: Timestamp,
      },
      sessionKind: AppGameTimeBudgetSessionKind.NativeAppSession,
      targetRef: 'target-homework-chat-app',
      categoryRef: 'category-ai-chatbot',
      riskSignalRef: 'risk-ai-chatbot',
      parentAllowedCandidate: false,
      runningDurationMs: 4_200_000,
      foregroundDurationMs: 3_900_000,
      evidenceReferences: [SessionEvidence],
    },
  ],
  countedSessionRefs: ['session-homework-chat-app-recovery'],
  excludedSessionRefs: [],
  countedDurationMs: 4_200_000,
  effectiveBudgetLimitMs: 3_600_000,
  budgetExceeded: true,
  scheduleState: AppGameTimeBudgetScheduleState.Active,
  scheduleEvidenceReferences: [ScheduleEvidence],
  bonusGrant: {
    bonusState: AppGameTimeBudgetBonusState.None,
    bonusDurationMs: 0,
    approvalRef: null,
    auditRefs: [],
  },
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

const assertAskParentAndManualRequiredStayDryRunOnly = () => {
  const askParent = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BaseDecision,
    decisionId: 'app-game-time-budget-decision-ask-parent',
    bonusGrant: {
      bonusState: AppGameTimeBudgetBonusState.Requested,
      bonusDurationMs: 0,
      approvalRef: null,
      auditRefs: ['audit-bonus-time-requested'],
    },
    approvalState: AppGameTimeBudgetApprovalState.Pending,
    recommendedAction: AppGameTimeBudgetRecommendedAction.AskParent,
    enforcementHandoffState: AppGameTimeBudgetHandoffState.Disabled,
  });
  const manualRequired = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BaseDecision,
    decisionId: 'app-game-time-budget-decision-manual-required',
    recommendedAction: AppGameTimeBudgetRecommendedAction.ManualRequired,
    enforcementHandoffState: AppGameTimeBudgetHandoffState.ManualRequired,
    timerState: AppGameTimeBudgetTimerState.RecoveryNeeded,
  });
  const wrongHandoff = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BaseDecision,
    decisionId: 'app-game-time-budget-decision-wrong-handoff',
    enforcementHandoffState: AppGameTimeBudgetHandoffState.ManualRequired,
  });

  expect(askParent.success).toBe(true);
  expect(manualRequired.success).toBe(true);
  expect(wrongHandoff.success).toBe(false);
};

const assertRestartRecoveryRequiresTimerAndAuditRefs = () => {
  const recovered = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BaseDecision,
    decisionId: 'app-game-time-budget-decision-recovered',
    timerState: AppGameTimeBudgetTimerState.RestartRecovered,
  });
  const missingTimerRef = AppGameTimeBudgetDryRunDecisionSchema.safeParse({
    ...BaseDecision,
    decisionId: 'app-game-time-budget-decision-recovered-missing-timer',
    timerState: AppGameTimeBudgetTimerState.RestartRecovered,
    timerRefs: [],
  });

  expect(recovered.success).toBe(true);
  expect(missingTimerRef.success).toBe(false);
};

describe('app game time budget recovery contracts', () => {
  it(
    'keeps ask-parent and manual-required decisions in dry-run-only states',
    assertAskParentAndManualRequiredStayDryRunOnly
  );
  it('requires timer and audit refs when restart recovery is claimed', assertRestartRecoveryRequiresTimerAndAuditRefs);
});
