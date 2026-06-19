import { describe, expect, it } from 'vitest';
import { NativeGameBudgetDryRunDecisionSchema, NativeGameBudgetSignalSchema } from '../../src/native-game-budget-policy';
import {
  NativeGameBudgetCandidatePolicy,
  NativeGameBudgetDurationSource,
  NativeGameBudgetEvidenceKind,
  NativeGameBudgetRecommendedAction,
  NativeGameBudgetSignalKind,
  NativeGameBudgetSignalPolicyRole,
} from '../../src/native-game-budget-policy-rules';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-03T07:30:00Z';
const PolicyVersion = 'native-game-budget-policy-version-1';

const ChildDevice = {
  deviceId: 'device-windows-1',
  childProfileId: 'child-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'native-game-session-evidence-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const GameBudgetPolicy = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  budgetPolicyId: 'native-game-budget-policy-1',
  policyVersion: PolicyVersion,
  device: ChildDevice,
  target: {
    targetKind: 'all-native-games',
    targetRef: null,
  },
  dailyBudgetMinutes: 60,
  durationSource: NativeGameBudgetDurationSource.RunningDuration,
  candidatePolicy: NativeGameBudgetCandidatePolicy.ExcludeCandidates,
  whenExceededAction: NativeGameBudgetRecommendedAction.TimeLimitDryRun,
  previewEvidenceReferences: [EvidenceReference],
} as const;

const KnownGameSession = {
  sessionRef: {
    sessionRefId: 'game-session-1',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  evidenceKind: NativeGameBudgetEvidenceKind.KnownGameSession,
  parentAllowedCandidate: false,
  runningDurationMs: 3_900_000,
  foregroundDurationMs: 3_600_000,
  evidenceReferences: [EvidenceReference],
} as const;

const LauncherOnlySession = {
  sessionRef: {
    sessionRefId: 'launcher-session-1',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  evidenceKind: NativeGameBudgetEvidenceKind.LauncherOnly,
  parentAllowedCandidate: false,
  runningDurationMs: 2_400_000,
  foregroundDurationMs: 2_100_000,
  evidenceReferences: [EvidenceReference],
} as const;

const LauncherGameCandidateSession = {
  sessionRef: {
    sessionRefId: 'launcher-candidate-session-1',
    device: ChildDevice,
    observedAt: Timestamp,
  },
  evidenceKind: NativeGameBudgetEvidenceKind.LauncherGameCandidate,
  parentAllowedCandidate: true,
  runningDurationMs: 1_800_000,
  foregroundDurationMs: 1_500_000,
  evidenceReferences: [EvidenceReference],
} as const;

const BudgetExceededDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'native-game-budget-decision-1',
  policy: GameBudgetPolicy,
  sessions: [KnownGameSession, LauncherOnlySession],
  countedSessionRefs: ['game-session-1'],
  excludedSessionRefs: ['launcher-session-1'],
  countedDurationMs: 3_900_000,
  budgetLimitMs: 3_600_000,
  budgetExceeded: true,
  recommendedAction: NativeGameBudgetRecommendedAction.TimeLimitDryRun,
  dryRun: true,
  enforcementHandoffState: 'not-requested',
  evidenceReferences: [EvidenceReference],
  evaluatedAt: Timestamp,
} as const;

const assertCountsKnownGameAndExcludesLauncherOnly = () => {
  expect(NativeGameBudgetDryRunDecisionSchema.safeParse(BudgetExceededDecision).success).toBe(true);
  expect(
    NativeGameBudgetDryRunDecisionSchema.safeParse({
      ...BudgetExceededDecision,
      countedSessionRefs: ['game-session-1', 'launcher-session-1'],
      excludedSessionRefs: [],
      countedDurationMs: 6_300_000,
    }).success
  ).toBe(false);
};

const assertCountsParentApprovedLauncherGameCandidates = () => {
  expect(
    NativeGameBudgetDryRunDecisionSchema.safeParse({
      ...BudgetExceededDecision,
      decisionId: 'native-game-budget-decision-2',
      policy: {
        ...GameBudgetPolicy,
        candidatePolicy: NativeGameBudgetCandidatePolicy.IncludeParentApprovedCandidates,
      },
      sessions: [LauncherGameCandidateSession],
      countedSessionRefs: ['launcher-candidate-session-1'],
      excludedSessionRefs: [],
      countedDurationMs: 1_800_000,
      budgetExceeded: false,
      recommendedAction: NativeGameBudgetRecommendedAction.Observe,
    }).success
  ).toBe(true);
  expect(
    NativeGameBudgetDryRunDecisionSchema.safeParse({
      ...BudgetExceededDecision,
      decisionId: 'native-game-budget-decision-3',
      policy: {
        ...GameBudgetPolicy,
        candidatePolicy: NativeGameBudgetCandidatePolicy.ExcludeCandidates,
      },
      sessions: [LauncherGameCandidateSession],
      countedSessionRefs: ['launcher-candidate-session-1'],
      excludedSessionRefs: [],
      countedDurationMs: 1_800_000,
      budgetExceeded: false,
      recommendedAction: NativeGameBudgetRecommendedAction.Observe,
    }).success
  ).toBe(false);
};

const assertKeepsGameSignalsAdvisoryOnly = () => {
  expect(
    NativeGameBudgetSignalSchema.safeParse({
      signalRef: 'rating-signal-1',
      signalKind: NativeGameBudgetSignalKind.Rating,
      policyRole: NativeGameBudgetSignalPolicyRole.ParentPreviewOnly,
      evidenceReferences: [EvidenceReference],
    }).success
  ).toBe(true);
  expect(
    NativeGameBudgetSignalSchema.safeParse({
      signalRef: 'rating-signal-2',
      signalKind: NativeGameBudgetSignalKind.Rating,
      policyRole: NativeGameBudgetSignalPolicyRole.DirectEnforcement,
      evidenceReferences: [EvidenceReference],
    }).success
  ).toBe(false);
};

const assertRejectsEnforcementClaims = () => {
  expect(
    NativeGameBudgetDryRunDecisionSchema.safeParse({
      ...BudgetExceededDecision,
      dryRun: false,
    }).success
  ).toBe(false);
  expect(
    NativeGameBudgetDryRunDecisionSchema.safeParse({
      ...BudgetExceededDecision,
      sessions: [LauncherOnlySession],
      countedSessionRefs: [],
      excludedSessionRefs: ['launcher-session-1'],
      countedDurationMs: 0,
      budgetExceeded: false,
      recommendedAction: NativeGameBudgetRecommendedAction.TimeLimitDryRun,
    }).success
  ).toBe(false);
};

describe('native game budget policy contracts', () => {
  it('counts known game sessions and keeps launcher-only evidence excluded', () => {
    assertCountsKnownGameAndExcludesLauncherOnly();
  });

  it('counts launcher-game candidates only when parent policy explicitly allows candidates', () => {
    assertCountsParentApprovedLauncherGameCandidates();
  });

  it('keeps rating, UGC, multiplayer, and purchase signals out of direct enforcement', () => {
    assertKeepsGameSignalsAdvisoryOnly();
  });

  it('rejects non-dry-run enforcement claims and signal-only time-limit decisions', () => {
    assertRejectsEnforcementClaims();
  });
});
