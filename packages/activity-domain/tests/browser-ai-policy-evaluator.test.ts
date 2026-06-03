import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '../src/browser-ai-analysis-schemas';
import {
  BrowserAiPolicyEvaluatorInputSchema,
  BrowserAiPolicyEvaluatorSchemaVersion,
  BrowserPolicyDecisionSchema,
} from '../src/browser-ai-policy-evaluator-schemas';

describe('browser AI policy evaluator integration contract', () => {
  it('accepts ready evaluator input with validated AI, memory, graph, and parent rule refs', acceptsReadyInput);
  it('rejects input with raw model text or final authority claims', rejectsRawOrFinalInput);
  it('accepts an active block decision only with adapter proof', acceptsActiveBlockWithProof);
  it('rejects an active block decision without adapter proof', rejectsActiveBlockWithoutProof);
  it('rejects AI, portal, or direct enforcement authority on a decision', rejectsAuthorityCreep);
  it('accepts unknown fallback decisions for low-confidence AI', acceptsUnknownFallback);
  it('rejects memory and graph refs without matching reason codes', rejectsUnreasonedRefs);
});

function acceptsReadyInput() {
  const parsed = BrowserAiPolicyEvaluatorInputSchema.safeParse(policyEvaluatorInput());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.evaluatorMode).toBe('active');
    expect(parsed.data.handoffState).toBe('ready');
    expect(parsed.data.aiResult.recommendedPolicyInput).toBe('warn-candidate');
  }
}

function rejectsRawOrFinalInput() {
  const parsed = BrowserAiPolicyEvaluatorInputSchema.safeParse({
    ...policyEvaluatorInput(),
    rawModelTextIncluded: true,
    unvalidatedAiOutputIncluded: true,
    finalDecisionClaimedByInput: true,
    aiResult: {
      ...aiAnalysisResult(),
      finalPolicyActionClaimed: true,
    },
  });

  expect(parsed.success).toBe(false);
}

function acceptsActiveBlockWithProof() {
  const parsed = BrowserPolicyDecisionSchema.safeParse(policyDecision('block'));

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.outcome).toBe('block');
    expect(parsed.data.adapterProofRef).toBe('managed-browser-adapter-proof-block-page');
    expect(parsed.data.directEnforcementClaimed).toBe(false);
  }
}

function rejectsActiveBlockWithoutProof() {
  const parsed = BrowserPolicyDecisionSchema.safeParse({
    ...policyDecision('block'),
    adapterProofRef: null,
  });

  expect(parsed.success).toBe(false);
}

function rejectsAuthorityCreep() {
  const parsed = BrowserPolicyDecisionSchema.safeParse({
    ...policyDecision('warn'),
    aiClaimedAsAuthority: true,
    portalEvaluatedClaimed: true,
    directEnforcementClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function acceptsUnknownFallback() {
  const parsed = BrowserPolicyDecisionSchema.safeParse({
    ...policyDecision('unknown'),
    confidence: 'low',
    reasonCodes: ['ai_low_confidence', 'unknown_evidence', 'parent_fallback', 'memory_hit', 'graph_ref'],
    adapterProofRef: null,
    fallbackUsed: true,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.outcome).toBe('unknown');
    expect(parsed.data.fallbackUsed).toBe(true);
  }
}

function rejectsUnreasonedRefs() {
  const parsed = BrowserPolicyDecisionSchema.safeParse({
    ...policyDecision('warn'),
    reasonCodes: ['explicit_parent_rule', 'schedule_match', 'ai_high_confidence'],
  });

  expect(parsed.success).toBe(false);
}

function policyEvaluatorInput() {
  return {
    schemaVersion: BrowserAiPolicyEvaluatorSchemaVersion,
    requestId: 'browser-policy-evaluator-request-youtube-video',
    requestedAt: '2026-06-03T04:06:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-2026-06-03-youtube-video',
    metadataEvidenceIds: ['metadata-evidence-youtube-video'],
    aiResult: aiAnalysisResult(),
    memoryHitIds: ['memory-hit-known-education-video'],
    memoryCacheEntryIds: ['memory-cache-entry-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    evaluatorMode: 'active',
    handoffState: 'ready',
    rawModelTextIncluded: false,
    unvalidatedAiOutputIncluded: false,
    portalUiStateIncluded: false,
    finalDecisionClaimedByInput: false,
    directEnforcementClaimedByInput: false,
  };
}

function policyDecision(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPolicyEvaluatorSchemaVersion,
    decisionId: 'browser-policy-decision-youtube-video',
    requestId: 'browser-policy-evaluator-request-youtube-video',
    decidedAt: '2026-06-03T04:06:01.000Z',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    outcome,
    evaluatorMode: 'active',
    confidence: 'high',
    reasonCodes: ['explicit_parent_rule', 'schedule_match', 'ai_high_confidence', 'memory_hit', 'graph_ref'],
    auditRefs: ['browser-policy-decision-audit-youtube-video'],
    adapterProofRef: 'managed-browser-adapter-proof-block-page',
    fallbackUsed: false,
    aiClaimedAsAuthority: false,
    portalEvaluatedClaimed: false,
    directEnforcementClaimed: false,
  };
}

function aiAnalysisResult() {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    analysisId: 'browser-ai-analysis-result-youtube-video',
    requestId: 'browser-ai-analysis-request-youtube-video',
    analyzedAt: '2026-06-03T04:05:58.000Z',
    expiresAt: '2026-06-03T05:05:58.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    metadataEvidenceIds: ['metadata-evidence-youtube-video'],
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    contentKind: 'video',
    videoKind: 'video',
    contentCategory: 'educational',
    contentModifiers: ['comments-enabled'],
    benefitSignals: ['homework-help', 'skill-building'],
    riskSignals: ['unknown-risk'],
    recommendedPolicyInput: 'warn-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummary: 'Structured metadata indicates an educational fractions video with comments enabled.',
    childSafeSummary: 'This looks like a math lesson, but comments may still need parent rules.',
    modelRuntimeRef: 'local-model-runtime-ref-browser-ai',
    promptTemplate: promptTemplate(),
    degradedState: 'none',
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
    rawContentStored: false,
  };
}

function promptTemplate() {
  return {
    promptTemplateId: 'browser-ai-video-safety-template',
    promptTemplateVersion: 'browser-ai-video-safety-template-v1',
    requestedTask: 'video-safety',
    allowedInputFieldRefs: ['url-shape', 'metadata-evidence', 'memory-hit', 'parent-rule', 'schedule-context'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
  };
}
