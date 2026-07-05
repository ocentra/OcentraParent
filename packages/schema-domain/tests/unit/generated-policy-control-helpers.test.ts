import { describe, expect, it } from 'vitest';
import {
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../../src/family-reference-primitives';
import {
  compareGeneratedPolicyActionStrictness,
  resolveGeneratedPolicyAuthority,
  resolveGeneratedPolicyPreviewBudgetBoundaryState,
  selectGeneratedStricterPolicyAction,
} from '../../src/generated-policy-control-helpers';
import {
  PolicyAuthoritySource,
  PolicyAuthorityState,
  resolvePolicyApprovalLifecycle,
  resolvePolicyAuthority,
} from '../../src/authority';
import { AppGameCategoryRiskPolicyRouteSchema } from '../../src/app-game-category-risk-policy-routing';
import {
  PolicyAction,
  PolicyDecisionHandoffState,
  PolicyPreviewBudgetBoundaryState,
  comparePolicyActionStrictness,
  parsePolicyPreview,
  parsePolicyScheduleBoundary,
  resolvePolicyPreviewBudgetBoundaryState,
  selectStricterPolicyAction,
} from '../../src/policy';
import { buildScreenAiStricterParentRuleProof } from '../../src/screen-ai-stricter-parent-rule-proof';
import {
  appGameCategoryRiskPolicyRoute,
  bonusTimeBudgetBoundary,
  expiredPolicyScheduleBoundaryInput,
  invalidPolicyPreviewInput,
  manualBudgetBoundary,
  policyApprovalLifecycleResolution,
  samplePolicyScheduleBoundary,
  screenAiStricterParentRuleProofInput,
} from './generated-policy-control-helpers.fixtures';

describe('policy-control generated helpers', () => {
  preservesStricterPolicyActionOrderingThroughThinAdapters();
  resolvesManualAndBonusTimeBudgetBoundaryStatesThroughThinAdapters();
  resolvesGeneratedDryRunAuthorityThroughThinAdapter();
  resolvesLocalAiAuthorityAsEvidenceOnly();
  rejectsChildSelfApprovalThroughGeneratedLifecycleSurface();
  keepsPolicyScheduleBoundaryAndPreviewParsingOnGeneratedValidationRules();
  keepsAppGameCategoryRoutingOnGeneratedRouteSemantics();
  rewiresScreenAiStricterParentRuleProofToGeneratedHelperSemantics();
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
  });
}

function rejectsChildSelfApprovalThroughGeneratedLifecycleSurface(): void {
  it('rejects child self-approval through the generated lifecycle surface', () => {
    expect(resolvePolicyApprovalLifecycle(policyApprovalLifecycleResolution)).toEqual(policyApprovalLifecycleResolution);
    expect(() =>
      resolvePolicyApprovalLifecycle({
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
    expect(AppGameCategoryRiskPolicyRouteSchema.parse(appGameCategoryRiskPolicyRoute)).toEqual(
      appGameCategoryRiskPolicyRoute
    );
    expect(() =>
      AppGameCategoryRiskPolicyRouteSchema.parse({
        ...appGameCategoryRiskPolicyRoute,
        aiDigestRef: null,
      })
    ).toThrow('Expected local-AI category policy routes to cite an AI digest ref');
  });
}

function rewiresScreenAiStricterParentRuleProofToGeneratedHelperSemantics(): void {
  it('rewires the screen-ai stricter-parent-rule proof to the generated helper semantics', () => {
    const proof = buildScreenAiStricterParentRuleProof(screenAiStricterParentRuleProofInput);

    expect(proof.finalAction).toBe(PolicyAction.Block);
    expect(proof.finalDecision.action).toBe(PolicyAction.Block);
    expect(proof.finalDecision.reasonCodes[0]).toBe('parent-rule-block');
    expect(proof.sourceDecision.evidenceReferences[0]?.kind).toBe(ParentEvidenceReferenceKind.LocalAiResult);
    expect(ParentPlatform.Windows).toBe('windows');
  });
}
