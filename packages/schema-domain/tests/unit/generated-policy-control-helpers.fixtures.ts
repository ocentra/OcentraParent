import {
  GeneratedAppGameCategoryRiskPolicyCandidateActionValues,
  GeneratedAppGameCategoryRiskPolicyRouteFamilyValues,
  GeneratedAppGameCategoryRiskPolicyRouteSourceKindValues,
  GeneratedAppGameCategoryRiskPolicyRoutingStateValues,
} from '../../src/generated-policy-control-helpers-contracts';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../../src/family-reference-primitives';
import {
  PolicyApprovalKind,
  PolicyApprovalOrigin,
  PolicyApprovalState,
  PolicyOverrideState,
  PolicyOverrideType,
} from '../../src/policy-authority';
import {
  PolicyAction,
  PolicyDecisionHandoffState,
  PolicyPreviewConfirmationState,
  PolicyScheduleBoundaryState,
  PolicyScheduleClockSource,
  PolicyScheduleDstResolution,
  PolicyScheduleOfflineRecoveryState,
} from '../../src/policy';

export const samplePolicyScheduleBoundary = {
  scheduleId: 'schedule-homework',
  timeZone: 'America/Toronto',
  evaluatedAt: '2026-06-29T14:00:00Z',
  localTime: '10:00',
  state: PolicyScheduleBoundaryState.WithinWindow,
  dstBoundary: null,
  clockSkew: null,
  exception: null,
  expiry: null,
  timeBudget: {
    budgetWindowMinutes: 60,
    usedMinutes: 10,
    remainingMinutes: 50,
    carryoverMinutes: 0,
    gracePeriodMinutes: 5,
    resetAt: '2026-06-29T15:00:00Z',
    clockSource: PolicyScheduleClockSource.TrustedService,
    offlineRecovery: {
      state: PolicyScheduleOfflineRecoveryState.NotNeeded,
      recoveredAt: null,
      recoveredOfflineMinutes: 0,
    },
    bonusTimeMinutes: null,
    bonusTimeRemainingMinutes: null,
    bonusTimeExpiresAt: null,
  },
} as const;

export const manualBudgetBoundary = {
  ...samplePolicyScheduleBoundary,
  state: PolicyScheduleBoundaryState.DstGap,
  dstBoundary: {
    transition: 'spring-forward',
    localTime: '02:00',
    offsetBeforeMinutes: -300,
    offsetAfterMinutes: -240,
    resolution: PolicyScheduleDstResolution.ManualRequired,
  },
  timeBudget: {
    ...samplePolicyScheduleBoundary.timeBudget,
    bonusTimeMinutes: 20,
    bonusTimeRemainingMinutes: 10,
    bonusTimeExpiresAt: '2026-06-29T14:30:00Z',
  },
} as const;

export const bonusTimeBudgetBoundary = {
  ...manualBudgetBoundary,
  state: PolicyScheduleBoundaryState.WithinWindow,
  dstBoundary: null,
  timeBudget: {
    ...manualBudgetBoundary.timeBudget,
    bonusTimeMinutes: 20,
    bonusTimeRemainingMinutes: 5,
  },
} as const;

export const policyApprovalLifecycleResolution = {
  approval: {
    approvalId: 'approval-001',
    permissionRequestId: 'permission-request-001',
    origin: PolicyApprovalOrigin.ChildRequest,
    kind: PolicyApprovalKind.BonusTime,
    childProfile: {
      childProfileId: 'child-001',
      displayName: 'Child',
    },
    device: {
      deviceId: 'device-001',
      childProfileId: 'child-001',
      label: 'Laptop',
      platform: ParentPlatform.Windows,
    },
    requestedTarget: {
      targetId: 'target-game-category',
      targetType: 'category',
      targetValue: 'gaming',
    },
    requestedAction: PolicyAction.Allow,
    requestedAt: '2026-06-29T13:00:00Z',
    expiresAt: '2026-06-29T15:00:00Z',
    requestedBonusTimeMinutes: 20,
    scheduleBoundary: samplePolicyScheduleBoundary,
  },
  state: PolicyApprovalState.Approved,
  evaluatedAt: '2026-06-29T14:00:00Z',
  reviewedBy: {
    actorId: 'parent-001',
    role: ParentActorRole.Parent,
  },
  reviewedAt: '2026-06-29T13:30:00Z',
  auditReferenceId: 'audit-001',
  override: {
    overrideId: 'override-001',
    overrideType: PolicyOverrideType.BonusTime,
    state: PolicyOverrideState.Active,
    action: PolicyAction.Allow,
    effectiveFrom: '2026-06-29T13:30:00Z',
    effectiveUntil: '2026-06-29T14:30:00Z',
    bonusTimeMinutes: 20,
  },
  replayOfApprovalId: null,
} as const;

export const expiredPolicyScheduleBoundaryInput = {
  ...samplePolicyScheduleBoundary,
  expiry: {
    expiresAt: '2026-06-29T13:00:00Z',
    expiredAt: '2026-06-29T13:00:00Z',
    reasonCode: 'expired',
  },
} as const;

export const invalidPolicyPreviewInput = {
  previewId: 'preview-001',
  origin: 'parent-preview',
  confirmationState: PolicyPreviewConfirmationState.ConfirmationRequired,
  confirmedBy: null,
  confirmedAt: null,
  target: {
    targetId: 'target-game-category',
    targetType: 'category',
    targetValue: 'gaming',
  },
  requestedAction: PolicyAction.Warn,
  scheduleBoundary: samplePolicyScheduleBoundary,
  decision: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    decisionId: 'decision-preview',
    action: PolicyAction.Warn,
    reasonCodes: ['preview'],
    evidenceReferences: [],
    ruleIds: ['rule-preview'],
    localAiResultId: null,
    dryRun: false,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: null,
  },
} as const;

export const appGameCategoryRiskPolicyRoute = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  routeId: 'route-001',
  categoryCandidateRef: 'category-candidate-001',
  routeFamily: GeneratedAppGameCategoryRiskPolicyRouteFamilyValues[3],
  sourceKind: GeneratedAppGameCategoryRiskPolicyRouteSourceKindValues[4],
  sourceRef: 'source-001',
  targetKind: 'multiplayer-game',
  targetRef: 'target-ref-001',
  confidence: 0.9,
  candidateAction: GeneratedAppGameCategoryRiskPolicyCandidateActionValues[3],
  requestedAction: 'manual-required',
  policyAction: PolicyAction.AskParent,
  routingState: GeneratedAppGameCategoryRiskPolicyRoutingStateValues[1],
  categoryProof: {
    evidenceReference: {
      evidenceReferenceId: 'evidence-001',
      kind: ParentEvidenceReferenceKind.LocalAiResult,
      observedAt: '2026-06-29T13:59:00Z',
    },
    proofKind: 'category-proof',
    evidenceState: 'active',
    device: {
      deviceId: 'device-001',
      childProfileId: 'child-001',
      label: 'Laptop',
      platform: ParentPlatform.Windows,
    },
    localUserRef: 'local-user-001',
    observedAt: '2026-06-29T13:59:00Z',
  },
  supportingEvidence: [
    {
      evidenceReferenceId: 'evidence-001',
      kind: ParentEvidenceReferenceKind.LocalAiResult,
      observedAt: '2026-06-29T13:59:00Z',
    },
  ],
  aiDigestRef: 'digest-001',
  adapterDispatchState: 'not-dispatched',
} as const;
