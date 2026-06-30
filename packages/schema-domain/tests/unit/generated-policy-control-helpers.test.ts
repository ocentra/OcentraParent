import { describe, expect, it } from 'vitest';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../../src/family-reference-primitives';
import {
  compareGeneratedPolicyActionStrictness,
  resolveGeneratedPolicyAuthority,
  resolveGeneratedPolicyPreviewBudgetBoundaryState,
  selectGeneratedStricterPolicyAction,
} from '../../src/generated/policy-control-helpers';
import {
  PolicyApprovalKind,
  PolicyApprovalOrigin,
  PolicyApprovalState,
  PolicyAuthoritySource,
  PolicyAuthorityState,
  PolicyOverrideState,
  PolicyOverrideType,
  resolvePolicyApprovalLifecycle,
  resolvePolicyAuthority,
} from '../../src/authority';
import {
  AppGameCategoryRiskPolicyCandidateAction,
  AppGameCategoryRiskPolicyRouteFamily,
  AppGameCategoryRiskPolicyRouteSchema,
  AppGameCategoryRiskPolicyRouteSourceKind,
  AppGameCategoryRiskPolicyRoutingState,
} from '../../src/app-game-category-risk-policy-routing';
import {
  PolicyAction,
  PolicyDecisionHandoffState,
  PolicyPreviewBudgetBoundaryState,
  PolicyPreviewConfirmationState,
  PolicyScheduleBoundaryState,
  PolicyScheduleClockSource,
  PolicyScheduleDstResolution,
  PolicyScheduleOfflineRecoveryState,
  comparePolicyActionStrictness,
  parsePolicyPreview,
  parsePolicyScheduleBoundary,
  resolvePolicyPreviewBudgetBoundaryState,
  selectStricterPolicyAction,
} from '../../src/policy';
import { buildScreenAiStricterParentRuleProof } from '../../src/screen-ai-stricter-parent-rule-proof';

function sampleBoundary() {
  return {
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
}

describe('policy-control generated helpers', () => {
  it('preserves stricter policy action ordering through thin adapters', () => {
    expect(compareGeneratedPolicyActionStrictness('block', 'warn')).toBe(40);
    expect(selectGeneratedStricterPolicyAction('ask-parent', 'time-limit')).toBe('time-limit');
    expect(comparePolicyActionStrictness(PolicyAction.Block, PolicyAction.Warn)).toBe(40);
    expect(selectStricterPolicyAction(PolicyAction.AskParent, PolicyAction.TimeLimit)).toBe(PolicyAction.TimeLimit);
  });

  it('resolves manual and bonus-time budget boundary states through thin adapters', () => {
    const manualBoundary = {
      ...sampleBoundary(),
      state: PolicyScheduleBoundaryState.DstGap,
      dstBoundary: {
        transition: 'spring-forward',
        localTime: '02:00',
        offsetBeforeMinutes: -300,
        offsetAfterMinutes: -240,
        resolution: PolicyScheduleDstResolution.ManualRequired,
      },
      timeBudget: {
        ...sampleBoundary().timeBudget,
        bonusTimeMinutes: 20,
        bonusTimeRemainingMinutes: 10,
        bonusTimeExpiresAt: '2026-06-29T14:30:00Z',
      },
    } as const;

    const bonusBoundary = {
      ...manualBoundary,
      state: PolicyScheduleBoundaryState.WithinWindow,
      dstBoundary: null,
      timeBudget: {
        ...manualBoundary.timeBudget,
        bonusTimeMinutes: 20,
        bonusTimeRemainingMinutes: 5,
      },
    } as const;

    expect(resolveGeneratedPolicyPreviewBudgetBoundaryState(manualBoundary)).toBe('manual-required');
    expect(resolveGeneratedPolicyPreviewBudgetBoundaryState(bonusBoundary)).toBe('bonus-time-expiring');
    expect(resolvePolicyPreviewBudgetBoundaryState(manualBoundary)).toBe(PolicyPreviewBudgetBoundaryState.ManualRequired);
    expect(resolvePolicyPreviewBudgetBoundaryState(bonusBoundary)).toBe(
      PolicyPreviewBudgetBoundaryState.BonusTimeExpiring
    );
  });

  it('rewires policy authority and approval lifecycle to the generated helper surface', () => {
    const dryRunDecision = resolveGeneratedPolicyAuthority({
      source: 'parent-policy',
      decision: { dryRun: true },
    });
    expect(dryRunDecision.state).toBe('dry-run');

    const authority = resolvePolicyAuthority({
      source: PolicyAuthoritySource.LocalAiResult,
      decision: {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        decisionId: 'decision-local-ai',
        action: PolicyAction.Warn,
        reasonCodes: ['local-ai-warning'],
        evidenceReferences: [],
        ruleIds: ['rule-local-ai'],
        localAiResultId: 'local-ai-result-001',
        dryRun: false,
        enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
        expiresAt: null,
      },
    });
    expect(authority.state).toBe(PolicyAuthorityState.EvidenceOnly);

    const resolution = {
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
        scheduleBoundary: sampleBoundary(),
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

    expect(resolvePolicyApprovalLifecycle(resolution)).toEqual(resolution);
    expect(() =>
      resolvePolicyApprovalLifecycle({
        ...resolution,
        reviewedBy: {
          ...resolution.reviewedBy,
          actorId: 'child-001',
        },
      })
    ).toThrow('child requests cannot self-approve or self-modify');
  });

  it('keeps policy schedule boundary and preview parsing on generated validation rules', () => {
    expect(parsePolicyScheduleBoundary(sampleBoundary())).toMatchObject(sampleBoundary());

    expect(() =>
      parsePolicyScheduleBoundary({
        ...sampleBoundary(),
        expiry: {
          expiresAt: '2026-06-29T13:00:00Z',
          expiredAt: '2026-06-29T13:00:00Z',
          reasonCode: 'expired',
        },
      })
    ).toThrow('non-expired schedule boundaries cannot be evaluated after expiry');

    expect(() =>
      parsePolicyPreview({
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
        scheduleBoundary: sampleBoundary(),
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
      })
    ).toThrow('preview decisions must remain dry-run');
  });

  it('keeps app/game category routing on generated route semantics', () => {
    const validRoute = {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      routeId: 'route-001',
      categoryCandidateRef: 'category-candidate-001',
      routeFamily: AppGameCategoryRiskPolicyRouteFamily.GameContext,
      sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.LocalAi,
      sourceRef: 'source-001',
      targetKind: 'multiplayer-game',
      targetRef: 'target-ref-001',
      confidence: 0.9,
      candidateAction: AppGameCategoryRiskPolicyCandidateAction.ManualReview,
      requestedAction: 'manual-required',
      policyAction: PolicyAction.AskParent,
      routingState: AppGameCategoryRiskPolicyRoutingState.ManualRequired,
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

    expect(AppGameCategoryRiskPolicyRouteSchema.parse(validRoute)).toMatchObject(validRoute);
    expect(() =>
      AppGameCategoryRiskPolicyRouteSchema.parse({
        ...validRoute,
        aiDigestRef: null,
      })
    ).toThrow('Expected local-AI category policy routes to cite an AI digest ref');
  });

  it('rewires the screen-ai stricter-parent-rule proof to the generated helper semantics', () => {
    const proof = buildScreenAiStricterParentRuleProof({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      proofId: 'proof-screen-ai-stricter-parent-rule',
      generatedAt: '2026-06-29T14:00:00Z',
      sourceProof: 'screen-ai-proof',
      sourceDecision: {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        decisionId: 'decision-local-ai',
        action: PolicyAction.Warn,
        reasonCodes: ['local-ai-warning'],
        evidenceReferences: [
          {
            evidenceReferenceId: 'evidence-local-ai',
            kind: ParentEvidenceReferenceKind.LocalAiResult,
            observedAt: '2026-06-29T13:59:00Z',
          },
        ],
        ruleIds: ['rule-local-ai'],
        localAiResultId: 'local-ai-result-001',
        dryRun: true,
        enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
        expiresAt: null,
      },
      stricterParentRule: {
        ruleId: 'rule-parent-block',
        target: {
          targetId: 'target-game-category',
          targetType: 'category',
          targetValue: 'gaming',
        },
        action: PolicyAction.Block,
        scheduleId: null,
        priority: 100,
        reasonCode: 'parent-rule-block',
        createdBy: {
          actorId: 'actor-parent-001',
          role: ParentActorRole.Parent,
        },
        enabled: true,
        effectiveFrom: null,
        effectiveUntil: null,
      },
      expectedFinalAction: PolicyAction.Block,
      claimBoundaries: {
        localAiAuthorityClaimed: false,
        remoteAiUsed: false,
        apiAiUsed: false,
        rawImageRetained: false,
        enforcementClaimed: false,
      },
    });

    expect(proof.finalAction).toBe(PolicyAction.Block);
    expect(proof.finalDecision.action).toBe(PolicyAction.Block);
    expect(proof.finalDecision.reasonCodes[0]).toBe('parent-rule-block');
    expect(proof.sourceDecision.evidenceReferences[0]?.kind).toBe(ParentEvidenceReferenceKind.LocalAiResult);
    expect(ParentPlatform.Windows).toBe('windows');
  });
});
