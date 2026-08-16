import { describe, expect, it } from 'vitest';
import {
  compareGeneratedPolicyActionStrictness,
  generatedAppGameCategoryRiskPolicyRouteActionMatchesCandidate,
  generatedAppGameCategoryRiskPolicyRouteKeepsSoftBoundary,
  generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest,
  generatedAppGameCategoryRiskPolicyRouteManualReviewRequiresManualState,
  generatedAppGameCategoryRiskPolicyRouteTargetMatchesFamily,
  generatedAppGameCategoryRiskPolicyRouteUsesCategoryProof,
  resolveGeneratedPolicyAuthority,
  resolveGeneratedPolicyApprovalLifecycle,
  resolveGeneratedPolicyPreviewBudgetBoundaryState,
  selectGeneratedStricterPolicyAction,
} from '../../src/generated-policy-control-helpers';
import {
  GeneratedPermissionRequestStateValues,
  GeneratedPolicyActionValues,
  GeneratedPolicyApprovalKindValues,
  GeneratedPolicyApprovalOriginValues,
  GeneratedPolicyApprovalStateValues,
  GeneratedPolicyAuthoritySourceValues,
  GeneratedPolicyAuthorityStateValues,
  GeneratedPolicyCompilerCapabilityStateValues,
  GeneratedPolicyCompilerDomainValues,
  GeneratedPolicyCompilerNoClaimLabelValues,
  GeneratedPolicyCompilerRuleStatusValues,
  GeneratedPolicyCompilerSourceStatusValues,
  GeneratedPolicyCompilerTargetKindValues,
  GeneratedPolicyDecisionHandoffStateValues,
  GeneratedPolicyPreviewBudgetBoundaryStateValues,
  GeneratedPolicyPreviewConfirmationStateValues,
  GeneratedPolicyPreviewOriginValues,
  GeneratedPolicyScheduleBoundaryStateValues,
  GeneratedPolicyScheduleBudgetCarryoverModeValues,
  GeneratedPolicyScheduleBudgetResetKindValues,
  GeneratedPolicyScheduleClockSourceValues,
  GeneratedPolicyScheduleDayValues,
  GeneratedPolicyScheduleDstResolutionValues,
  GeneratedPolicyScheduleDstTransitionValues,
  GeneratedPolicyScheduleOfflineRecoveryStateValues,
  GeneratedPolicyScheduleOfflineRecoveryValues,
  GeneratedPolicyOverrideStateValues,
  GeneratedPolicyOverrideTypeValues,
  GeneratedPolicyTargetTypeValues,
} from '../../src/generated-policy-control-helpers-contracts';
import {
  PolicyAction,
  PolicyPreviewBudgetBoundaryState,
  comparePolicyActionStrictness,
  parsePolicyPreview,
  parsePolicyScheduleBoundary,
  resolvePolicyPreviewBudgetBoundaryState,
  selectStricterPolicyAction,
} from '../../src/policy';
import {
  PermissionRequestState as PolicyPermissionRequestState,
  PolicyDecisionHandoffState as PolicyDecisionHandoffStateContract,
  PolicyPreviewBudgetBoundaryState as PolicyPreviewBudgetBoundaryStateContract,
  PolicyPreviewConfirmationState as PolicyPreviewConfirmationStateContract,
  PolicyScheduleBoundaryState as PolicyScheduleBoundaryStateContract,
  PolicyScheduleBudgetCarryoverMode as PolicyScheduleBudgetCarryoverModeContract,
  PolicyScheduleBudgetResetKind as PolicyScheduleBudgetResetKindContract,
  PolicyScheduleClockSource as PolicyScheduleClockSourceContract,
  PolicyScheduleDay as PolicyScheduleDayContract,
  PolicyScheduleDstResolution as PolicyScheduleDstResolutionContract,
  PolicyScheduleDstTransition as PolicyScheduleDstTransitionContract,
  PolicyScheduleOfflineRecovery as PolicyScheduleOfflineRecoveryContract,
  PolicyScheduleOfflineRecoveryState as PolicyScheduleOfflineRecoveryStateContract,
  PolicyPreviewOrigin as PolicyPreviewOriginContract,
  PolicyTargetType as PolicyTargetTypeContract,
} from '../../src/policy-contracts';
import {
  PolicyApprovalKind,
  PolicyApprovalOrigin,
  PolicyApprovalState,
  PolicyAuthoritySource,
  PolicyAuthorityState,
  PolicyOverrideState,
  PolicyOverrideType,
} from '../../src/policy-authority';
import {
  PolicyCompilerCapabilityState,
  PolicyCompilerDomain,
  PolicyCompilerNoClaimLabel,
  PolicyCompilerRuleStatus,
  PolicyCompilerSourceStatusLiteral,
  PolicyCompilerTargetKindLiteral,
} from '../../src/policy-compiler';
import {
  appGameCategoryRiskPolicyRoute,
  bonusTimeBudgetBoundary,
  expiredPolicyScheduleBoundaryInput,
  invalidPolicyPreviewInput,
  manualBudgetBoundary,
  policyApprovalLifecycleResolution,
  samplePolicyScheduleBoundary,
} from './generated-policy-control-helpers.fixtures';

describe('policy-control generated helpers', () => {
  preservesStricterPolicyActionOrderingThroughThinAdapters();
  resolvesManualAndBonusTimeBudgetBoundaryStatesThroughThinAdapters();
  resolvesGeneratedDryRunAuthorityThroughThinAdapter();
  resolvesLocalAiAuthorityAsEvidenceOnly();
  locksGeneratedLiteralBoundaryAdapters();
  rejectsChildSelfApprovalThroughGeneratedLifecycleSurface();
  keepsPolicyScheduleBoundaryAndPreviewParsingOnGeneratedValidationRules();
  keepsAppGameCategoryRoutingOnGeneratedRouteSemantics();
});

function preservesStricterPolicyActionOrderingThroughThinAdapters(): void {
  it('preserves stricter policy action ordering through thin adapters', () => {
    expect(compareGeneratedPolicyActionStrictness('block', 'warn')).toBe(40);
    expect(selectGeneratedStricterPolicyAction('ask-parent', 'time-limit')).toBe('time-limit');
    expect(comparePolicyActionStrictness(PolicyAction.Block, PolicyAction.Warn)).toBe(40);
    expect(selectStricterPolicyAction(PolicyAction.AskParent, PolicyAction.TimeLimit)).toBe(PolicyAction.TimeLimit);
  });
}

function resolvesManualAndBonusTimeBudgetBoundaryStatesThroughThinAdapters(): void {
  it('resolves manual and bonus-time budget boundary states through thin adapters', () => {
    expect(resolveGeneratedPolicyPreviewBudgetBoundaryState(manualBudgetBoundary)).toBe('manual-required');
    expect(resolveGeneratedPolicyPreviewBudgetBoundaryState(bonusTimeBudgetBoundary)).toBe('bonus-time-expiring');
    expect(resolvePolicyPreviewBudgetBoundaryState(manualBudgetBoundary)).toBe(
      PolicyPreviewBudgetBoundaryState.ManualRequired
    );
    expect(resolvePolicyPreviewBudgetBoundaryState(bonusTimeBudgetBoundary)).toBe(
      PolicyPreviewBudgetBoundaryState.BonusTimeExpiring
    );
  });
}

function resolvesGeneratedDryRunAuthorityThroughThinAdapter(): void {
  it('resolves generated dry-run authority through the thin adapter surface', () => {
    const dryRunDecision = resolveGeneratedPolicyAuthority({
      source: 'parent-policy',
      decision: { dryRun: true },
    });

    expect(dryRunDecision.state).toBe('dry-run');
  });
}

function resolvesLocalAiAuthorityAsEvidenceOnly(): void {
  it('resolves local-AI policy authority as evidence-only', () => {
    const authority = resolveGeneratedPolicyAuthority({
      source: 'local-ai-result',
      decision: {
        dryRun: false,
      },
    });

    expect(authority.state).toBe('evidence-only');
  });
}

function locksGeneratedLiteralBoundaryAdapters(): void {
  it('keeps policy literals aligned with generated Rust-owned value tables', () => {
    expect(Object.values(PolicyAction)).toEqual(GeneratedPolicyActionValues);
    expect(Object.values(PolicyTargetTypeContract)).toEqual(GeneratedPolicyTargetTypeValues);
    expect(Object.values(PolicyScheduleDayContract)).toEqual(GeneratedPolicyScheduleDayValues);
    expect(Object.values(PolicyDecisionHandoffStateContract)).toEqual(GeneratedPolicyDecisionHandoffStateValues);
    expect(Object.values(PolicyPermissionRequestState)).toEqual(GeneratedPermissionRequestStateValues);
    expect(Object.values(PolicyScheduleBoundaryStateContract)).toEqual(GeneratedPolicyScheduleBoundaryStateValues);
    expect(Object.values(PolicyScheduleDstTransitionContract)).toEqual(GeneratedPolicyScheduleDstTransitionValues);
    expect(Object.values(PolicyScheduleDstResolutionContract)).toEqual(GeneratedPolicyScheduleDstResolutionValues);
    expect(Object.values(PolicyScheduleClockSourceContract)).toEqual(GeneratedPolicyScheduleClockSourceValues);
    expect(Object.values(PolicyScheduleBudgetResetKindContract)).toEqual(GeneratedPolicyScheduleBudgetResetKindValues);
    expect(Object.values(PolicyScheduleBudgetCarryoverModeContract)).toEqual(
      GeneratedPolicyScheduleBudgetCarryoverModeValues
    );
    expect(Object.values(PolicyScheduleOfflineRecoveryContract)).toEqual(GeneratedPolicyScheduleOfflineRecoveryValues);
    expect(Object.values(PolicyScheduleOfflineRecoveryStateContract)).toEqual(
      GeneratedPolicyScheduleOfflineRecoveryStateValues
    );
    expect(Object.values(PolicyPreviewOriginContract)).toEqual(GeneratedPolicyPreviewOriginValues);
    expect(Object.values(PolicyPreviewConfirmationStateContract)).toEqual(
      GeneratedPolicyPreviewConfirmationStateValues
    );
    expect(Object.values(PolicyPreviewBudgetBoundaryStateContract)).toEqual(
      GeneratedPolicyPreviewBudgetBoundaryStateValues
    );
    expect(Object.values(PolicyAuthoritySource)).toEqual(GeneratedPolicyAuthoritySourceValues);
    expect(Object.values(PolicyAuthorityState)).toEqual(GeneratedPolicyAuthorityStateValues);
    expect(Object.values(PolicyApprovalOrigin)).toEqual(GeneratedPolicyApprovalOriginValues);
    expect(Object.values(PolicyApprovalKind)).toEqual(GeneratedPolicyApprovalKindValues);
    expect(Object.values(PolicyApprovalState)).toEqual(GeneratedPolicyApprovalStateValues);
    expect(Object.values(PolicyOverrideType)).toEqual(GeneratedPolicyOverrideTypeValues);
    expect(Object.values(PolicyOverrideState)).toEqual(GeneratedPolicyOverrideStateValues);
    expect(Object.values(PolicyCompilerDomain)).toEqual(GeneratedPolicyCompilerDomainValues);
    expect(Object.values(PolicyCompilerRuleStatus)).toEqual(GeneratedPolicyCompilerRuleStatusValues);
    expect(Object.values(PolicyCompilerCapabilityState)).toEqual(GeneratedPolicyCompilerCapabilityStateValues);
    expect(Object.values(PolicyCompilerSourceStatusLiteral)).toEqual(GeneratedPolicyCompilerSourceStatusValues);
    expect(Object.values(PolicyCompilerTargetKindLiteral)).toEqual(GeneratedPolicyCompilerTargetKindValues);
    expect(Object.values(PolicyCompilerNoClaimLabel)).toEqual(GeneratedPolicyCompilerNoClaimLabelValues);
  });
}

function rejectsChildSelfApprovalThroughGeneratedLifecycleSurface(): void {
  it('rejects child self-approval through the generated lifecycle surface', () => {
    expect(resolveGeneratedPolicyApprovalLifecycle(policyApprovalLifecycleResolution)).toEqual(
      policyApprovalLifecycleResolution
    );
    expect(() =>
      resolveGeneratedPolicyApprovalLifecycle({
        ...policyApprovalLifecycleResolution,
        reviewedBy: {
          ...policyApprovalLifecycleResolution.reviewedBy,
          actorId: 'child-001',
        },
      })
    ).toThrow('child requests cannot self-approve or self-modify');
  });
}

function keepsPolicyScheduleBoundaryAndPreviewParsingOnGeneratedValidationRules(): void {
  it('keeps policy schedule boundary and preview parsing on generated validation rules', () => {
    expect(parsePolicyScheduleBoundary(samplePolicyScheduleBoundary)).toEqual(samplePolicyScheduleBoundary);

    expect(() => parsePolicyScheduleBoundary(expiredPolicyScheduleBoundaryInput)).toThrow(
      'non-expired schedule boundaries cannot be evaluated after expiry'
    );

    expect(() => parsePolicyPreview(invalidPolicyPreviewInput)).toThrow('preview decisions must remain dry-run');
  });
}

function keepsAppGameCategoryRoutingOnGeneratedRouteSemantics(): void {
  it('keeps app/game category routing on generated route semantics', () => {
    expect(generatedAppGameCategoryRiskPolicyRouteTargetMatchesFamily(appGameCategoryRiskPolicyRoute)).toBe(true);
    expect(generatedAppGameCategoryRiskPolicyRouteUsesCategoryProof(appGameCategoryRiskPolicyRoute)).toBe(true);
    expect(generatedAppGameCategoryRiskPolicyRouteActionMatchesCandidate(appGameCategoryRiskPolicyRoute)).toBe(true);
    expect(generatedAppGameCategoryRiskPolicyRouteKeepsSoftBoundary(appGameCategoryRiskPolicyRoute)).toBe(true);
    expect(generatedAppGameCategoryRiskPolicyRouteManualReviewRequiresManualState(appGameCategoryRiskPolicyRoute)).toBe(
      true
    );
    expect(generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest(appGameCategoryRiskPolicyRoute)).toBe(true);
    expect(
      generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest({
        ...appGameCategoryRiskPolicyRoute,
        aiDigestRef: null,
      })
    ).toBe(false);
  });
}
