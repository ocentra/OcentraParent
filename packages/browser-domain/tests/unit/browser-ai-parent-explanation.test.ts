import { describe, expect, it } from 'vitest';
import {
  BrowserAiParentExplanationBundleSchema,
  BrowserAiParentExplanationSchemaVersion,
} from '@ocentra-parent/schema-domain/browser-ai-parent-explanation-schemas';
import { BrowserAiAnalysisSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-analysis-schemas';
import { BrowserAiChildUxSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-child-ux-schemas';
import { BrowserAiPolicyEvaluatorSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-policy-evaluator-schemas';
import { BrowserAiPostAnalysisActionSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-post-analysis-action-schemas';

describe('browser AI parent explanation contract', () => {
  it('accepts evidence-linked parent explanation with audit visibility', acceptsReadyExplanation);
  it('rejects raw content, prompt text, portal authority, or enforcement claims', rejectsAuthorityCreep);
  it('rejects explanations missing required audit section', rejectsMissingAuditSection);
  it('rejects degraded or manual fallback explanations that hide fallback state', rejectsHiddenFallback);
  it('rejects engaged child experience when child-saw-page visibility is hidden', rejectsHiddenChildExperience);
  it('rejects linked records that do not share source evidence', rejectsEvidenceMismatch);
  it('accepts degraded manual-required explanation when fallback is visible', acceptsVisibleManualFallback);
});

function acceptsReadyExplanation() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse(parentExplanationBundle());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sections).toContain('audit');
    expect(parsed.data.childSawPageVisible).toBe(true);
  }
}

function rejectsAuthorityCreep() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse({
    ...parentExplanationBundle(),
    rawPageContentIncluded: true,
    rawPromptTextIncluded: true,
    portalEvaluatedClaimed: true,
    directEnforcementClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function rejectsMissingAuditSection() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse({
    ...parentExplanationBundle(),
    sections: ['summary', 'evidence', 'ai-analysis', 'policy-decision', 'action-taken'],
  });

  expect(parsed.success).toBe(false);
}

function rejectsHiddenFallback() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse({
    ...parentExplanationBundle('unknown', 'manual_required'),
    degradedStateVisible: false,
    manualFallbackVisible: false,
  });

  expect(parsed.success).toBe(false);
}

function rejectsHiddenChildExperience() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse({
    ...parentExplanationBundle(),
    childSawPageVisible: false,
  });

  expect(parsed.success).toBe(false);
}

function rejectsEvidenceMismatch() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse({
    ...parentExplanationBundle(),
    sourceEvidenceIds: ['browser-evidence-other-video'],
  });

  expect(parsed.success).toBe(false);
}

function acceptsVisibleManualFallback() {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse(
    parentExplanationBundle('unknown', 'manual_required')
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.state).toBe('manual_required');
    expect(parsed.data.manualFallbackVisible).toBe(true);
  }
}

function parentExplanationBundle(outcome: unknown = 'warn', state: unknown = 'ready') {
  const analysis = aiAnalysisResult(outcome);
  const decision = policyDecision(outcome);
  const actionPlan = postAnalysisActionPlan(outcome, decision);

  return {
    schemaVersion: BrowserAiParentExplanationSchemaVersion,
    explanationId: 'browser-parent-explanation-youtube-video',
    createdAt: '2026-06-03T04:31:00.000Z',
    state,
    titleTextToken: state === 'ready' ? 'browser.parent.explanation.title' : 'browser.parent.explanation.degraded',
    summaryTextToken: 'browser.parent.explanation.summary',
    sections: [
      'summary',
      'evidence',
      'ai-analysis',
      'policy-decision',
      'action-taken',
      'child-experience',
      'memory-cache',
      'knowledge-graph',
      'degraded-fallback',
      'audit',
    ],
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysis: analysis,
    policyDecision: decision,
    postAnalysisActionPlan: actionPlan,
    childUxSnapshot: childUxSnapshot(outcome, actionPlan),
    memoryCacheEntryIds: ['browser-ai-cache-entry-youtube-video'],
    knowledgeGraphRefs: ['knowledge-graph-node-fractions'],
    explanationAuditRefs: ['browser-parent-explanation-audit-youtube-video'],
    evidenceVisible: true,
    modelRuntimeVisible: true,
    promptVersionVisible: true,
    policyRuleVisible: true,
    actionVisible: true,
    memoryCacheVisible: true,
    childExperienceVisible: true,
    childSawPageVisible: outcome !== 'allow',
    degradedStateVisible: outcome === 'unknown',
    manualFallbackVisible: outcome === 'unknown',
    auditTrailVisible: true,
    rawPageContentIncluded: false,
    rawPromptTextIncluded: false,
    portalEvaluatedClaimed: false,
    policyAuthorityClaimed: false,
    directEnforcementClaimed: false,
  };
}

function aiAnalysisResult(outcome: unknown) {
  const degraded = outcome === 'unknown';

  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    analysisId: 'browser-ai-analysis-result-youtube-video',
    requestId: 'browser-ai-analysis-request-youtube-video',
    analyzedAt: '2026-06-03T04:29:00.000Z',
    expiresAt: '2026-06-03T05:29:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    metadataEvidenceIds: ['browser-url-metadata-youtube-video'],
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    contentKind: 'video',
    videoKind: 'video',
    contentCategory: degraded ? 'unknown' : 'educational',
    contentModifiers: ['metadata-only'],
    benefitSignals: degraded ? ['unknown-benefit'] : ['homework-help'],
    riskSignals: degraded ? ['unknown-risk'] : ['addictive-design'],
    recommendedPolicyInput: degraded ? 'manual-review-candidate' : 'warn-candidate',
    confidence: degraded ? 'low' : 'high',
    uncertaintyReasons: degraded ? ['low-confidence'] : [],
    parentSummary: 'Evidence-backed video review summary',
    childSafeSummary: 'This video was reviewed against your family rules.',
    modelRuntimeRef: 'local-model-runtime-browser-video',
    promptTemplate: promptTemplate(),
    degradedState: degraded ? 'manual-required' : 'none',
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
    rawContentStored: false,
  };
}

function promptTemplate() {
  return {
    promptTemplateId: 'browser-video-safety-template',
    promptTemplateVersion: '2026-06-03',
    requestedTask: 'video-safety',
    allowedInputFieldRefs: ['url-shape-ref', 'metadata-ref', 'parent-rule-ref'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
  };
}

function policyDecision(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPolicyEvaluatorSchemaVersion,
    decisionId: 'browser-policy-decision-youtube-video',
    requestId: 'browser-policy-evaluator-request-youtube-video',
    decidedAt: '2026-06-03T04:29:59.000Z',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    outcome,
    evaluatorMode: outcome === 'unknown' ? 'manual_required' : 'active',
    confidence: outcome === 'unknown' ? 'low' : 'high',
    reasonCodes: reasonCodesForOutcome(outcome),
    auditRefs: ['browser-policy-decision-audit-youtube-video'],
    adapterProofRef: outcome === 'unknown' ? null : 'managed-browser-adapter-proof-warning-page',
    fallbackUsed: outcome === 'unknown',
    aiClaimedAsAuthority: false,
    portalEvaluatedClaimed: false,
    directEnforcementClaimed: false,
  };
}

function postAnalysisActionPlan(outcome: unknown, decision: ReturnType<typeof policyDecision>) {
  return {
    schemaVersion: BrowserAiPostAnalysisActionSchemaVersion,
    actionPlanId: 'browser-post-analysis-action-plan-youtube-video',
    createdAt: '2026-06-03T04:30:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    policyDecision: decision,
    policyDecisionAuditRefs: ['browser-policy-decision-audit-youtube-video'],
    parentRuleRefs: ['parent-rule-homework-window'],
    actionLabels: outcome === 'unknown' ? ['manual_required'] : ['warning_shown_after_review'],
    trigger: 'policy_decision',
    timing: outcome === 'unknown' ? 'background_only' : 'after_playback_started',
    childAlreadyEngaged: outcome !== 'unknown',
    deliveryState: outcome === 'unknown' ? 'manual_required' : 'delivered',
    adapterProofRef: outcome === 'unknown' ? null : 'managed-browser-adapter-proof-warning-page',
    rememberUntil: null,
    actionAuditRefs: ['browser-post-analysis-action-audit-youtube-video'],
    realtimeBlockClaimed: false,
    browserRuntimeMutationClaimed: false,
    directEnforcementClaimed: false,
  };
}

function childUxSnapshot(outcome: unknown, actionPlan: ReturnType<typeof postAnalysisActionPlan>) {
  const manual = outcome === 'unknown';

  return {
    schemaVersion: BrowserAiChildUxSchemaVersion,
    snapshotId: 'browser-child-ux-youtube-video',
    createdAt: '2026-06-03T04:30:30.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    state: manual ? 'manual_required' : 'warning',
    tone: 'calm',
    surface: manual ? 'modeled-only' : 'managed-browser-warning-page',
    primaryTextToken: manual ? 'browser.child.manual.title' : 'browser.child.warning.title',
    secondaryTextToken: null,
    deliveryState: manual ? 'portal-row-only' : 'warn-page-rendered',
    adapterProofRef: manual ? null : 'managed-browser-adapter-proof-warning-page',
    postAnalysisActionPlan: actionPlan,
    rawCopyClaimed: false,
    visualRenderClaimed: false,
    surveillanceCopyClaimed: false,
    shamingCopyClaimed: false,
  };
}

function reasonCodesForOutcome(outcome: unknown) {
  if (outcome === 'unknown') {
    return ['ai_low_confidence', 'unknown_evidence', 'parent_fallback', 'memory_hit', 'graph_ref'];
  }
  return ['explicit_parent_rule', 'schedule_match', 'ai_high_confidence', 'memory_hit', 'graph_ref'];
}
