import { describe, expect, it } from 'vitest';
import {
  BrowserAiPostAnalysisActionPlanSchema,
  BrowserAiPostAnalysisActionSchemaVersion,
} from '../src/browser-ai-post-analysis-action-schemas';
import { BrowserAiPolicyEvaluatorSchemaVersion } from '../src/browser-ai-policy-evaluator-schemas';

describe('browser AI post-analysis action model contract', () => {
  it('accepts playback stopped after review when adapter proof exists', acceptsPlaybackStoppedAfterReview);
  it('accepts remembered background allow with expiry', acceptsRememberedAllow);
  it('rejects real-time block claims after playback started', rejectsRealtimeBlockClaim);
  it('rejects delivered stop or warning actions without adapter proof', rejectsDeliveredActionWithoutProof);
  it('rejects remembered actions without expiry', rejectsRememberedWithoutExpiry);
  it('accepts modeled future-visit block without claiming delivery', acceptsModeledFutureVisitBlock);
  it('rejects unknown decisions without manual or parent fallback action', rejectsUnknownWithoutFallbackAction);
});

function acceptsPlaybackStoppedAfterReview() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse(postAnalysisActionPlan('block'));

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.childAlreadyEngaged).toBe(true);
    expect(parsed.data.actionLabels).toContain('playback_stopped_after_review');
    expect(parsed.data.realtimeBlockClaimed).toBe(false);
  }
}

function acceptsRememberedAllow() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('allow'),
    actionLabels: ['background_reviewed', 'continue_allowed', 'remembered_with_expiry'],
    timing: 'background_only',
    deliveryState: 'modeled_only',
    adapterProofRef: null,
    rememberUntil: '2026-07-03T04:14:00.000Z',
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.rememberUntil).toBe('2026-07-03T04:14:00.000Z');
  }
}

function rejectsRealtimeBlockClaim() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('block'),
    realtimeBlockClaimed: true,
    timing: 'before_playback',
  });

  expect(parsed.success).toBe(false);
}

function rejectsDeliveredActionWithoutProof() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('warn'),
    actionLabels: ['warning_shown_after_review'],
    adapterProofRef: null,
  });

  expect(parsed.success).toBe(false);
}

function rejectsRememberedWithoutExpiry() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('allow'),
    actionLabels: ['continue_allowed', 'remembered_with_expiry'],
    deliveryState: 'modeled_only',
    adapterProofRef: null,
    rememberUntil: null,
  });

  expect(parsed.success).toBe(false);
}

function acceptsModeledFutureVisitBlock() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('block'),
    actionLabels: ['future_visits_blocked'],
    timing: 'future_visit_only',
    deliveryState: 'modeled_only',
    adapterProofRef: null,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.deliveryState).toBe('modeled_only');
  }
}

function rejectsUnknownWithoutFallbackAction() {
  const parsed = BrowserAiPostAnalysisActionPlanSchema.safeParse({
    ...postAnalysisActionPlan('unknown'),
    actionLabels: ['background_reviewed'],
    deliveryState: 'modeled_only',
    adapterProofRef: null,
  });

  expect(parsed.success).toBe(false);
}

function postAnalysisActionPlan(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPostAnalysisActionSchemaVersion,
    actionPlanId: 'browser-post-analysis-action-plan-youtube-video',
    createdAt: '2026-06-03T04:14:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    policyDecision: policyDecision(outcome),
    policyDecisionAuditRefs: ['browser-policy-decision-audit-youtube-video'],
    parentRuleRefs: ['parent-rule-homework-window'],
    actionLabels: ['playback_stopped_after_review', 'parent_approval_requested_after_review'],
    trigger: 'policy_decision',
    timing: 'after_playback_started',
    childAlreadyEngaged: true,
    deliveryState: 'delivered',
    adapterProofRef: 'managed-browser-adapter-proof-block-page',
    rememberUntil: null,
    actionAuditRefs: ['browser-post-analysis-action-audit-youtube-video'],
    realtimeBlockClaimed: false,
    browserRuntimeMutationClaimed: false,
    directEnforcementClaimed: false,
  };
}

function policyDecision(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPolicyEvaluatorSchemaVersion,
    decisionId: 'browser-policy-decision-youtube-video',
    requestId: 'browser-policy-evaluator-request-youtube-video',
    decidedAt: '2026-06-03T04:13:59.000Z',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    outcome,
    evaluatorMode: 'active',
    confidence: outcome === 'unknown' ? 'low' : 'high',
    reasonCodes: reasonCodesForOutcome(outcome),
    auditRefs: ['browser-policy-decision-audit-youtube-video'],
    adapterProofRef: outcome === 'unknown' ? null : 'managed-browser-adapter-proof-block-page',
    fallbackUsed: outcome === 'unknown',
    aiClaimedAsAuthority: false,
    portalEvaluatedClaimed: false,
    directEnforcementClaimed: false,
  };
}

function reasonCodesForOutcome(outcome: unknown) {
  if (outcome === 'unknown') {
    return ['ai_low_confidence', 'unknown_evidence', 'parent_fallback', 'memory_hit', 'graph_ref'];
  }
  return ['explicit_parent_rule', 'schedule_match', 'ai_high_confidence', 'memory_hit', 'graph_ref'];
}
