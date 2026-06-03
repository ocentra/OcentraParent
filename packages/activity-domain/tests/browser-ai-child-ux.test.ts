import { describe, expect, it } from 'vitest';
import { BrowserAiChildUxSnapshotSchema, BrowserAiChildUxSchemaVersion } from '../src/browser-ai-child-ux-schemas';
import { BrowserAiPolicyEvaluatorSchemaVersion } from '../src/browser-ai-policy-evaluator-schemas';
import { BrowserAiPostAnalysisActionSchemaVersion } from '../src/browser-ai-post-analysis-action-schemas';

describe('browser AI child UX contract', () => {
  it('accepts a proof-backed checking hold snapshot with calm text token', acceptsCheckingHold);
  it('accepts a proof-backed warning snapshot linked to post-analysis action', acceptsWarningAfterReview);
  it('rejects raw or shaming child copy claims', rejectsUnsafeCopyClaims);
  it('rejects rendered child pages without adapter proof', rejectsRenderedPageWithoutProof);
  it('rejects state and text token mismatch', rejectsTokenMismatch);
  it('rejects warning state without matching post-analysis action', rejectsActionMismatch);
  it('accepts unclassified fallback without claiming a rendered child UI', acceptsUnclassifiedFallback);
});

function acceptsCheckingHold() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('checking'),
    deliveryState: 'checking-hold-rendered',
    adapterProofRef: 'managed-browser-checking-page-proof',
    postAnalysisActionPlan: null,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.primaryTextToken).toBe('browser.child.checking.title');
    expect(parsed.data.visualRenderClaimed).toBe(false);
  }
}

function acceptsWarningAfterReview() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('warning'),
    deliveryState: 'warn-page-rendered',
    adapterProofRef: 'managed-browser-warning-page-proof',
    postAnalysisActionPlan: postAnalysisActionPlan('warn'),
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.postAnalysisActionPlan?.actionLabels).toContain('warning_shown_after_review');
  }
}

function rejectsUnsafeCopyClaims() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('checking'),
    rawCopyClaimed: true,
    surveillanceCopyClaimed: true,
    shamingCopyClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function rejectsRenderedPageWithoutProof() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('blocked'),
    deliveryState: 'block-page-rendered',
    adapterProofRef: null,
    postAnalysisActionPlan: postAnalysisActionPlan('block'),
  });

  expect(parsed.success).toBe(false);
}

function rejectsTokenMismatch() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('warning'),
    primaryTextToken: 'browser.child.allowed.title',
    postAnalysisActionPlan: postAnalysisActionPlan('warn'),
  });

  expect(parsed.success).toBe(false);
}

function rejectsActionMismatch() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('warning'),
    postAnalysisActionPlan: postAnalysisActionPlan('allow'),
  });

  expect(parsed.success).toBe(false);
}

function acceptsUnclassifiedFallback() {
  const parsed = BrowserAiChildUxSnapshotSchema.safeParse({
    ...childUxSnapshot('unclassified'),
    deliveryState: 'portal-row-only',
    adapterProofRef: null,
    postAnalysisActionPlan: null,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.primaryTextToken).toBe('browser.child.unclassified.title');
  }
}

function childUxSnapshot(state: unknown) {
  return {
    schemaVersion: BrowserAiChildUxSchemaVersion,
    snapshotId: 'browser-child-ux-youtube-video',
    createdAt: '2026-06-03T04:22:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    state,
    tone: 'calm',
    surface: 'modeled-only',
    primaryTextToken: textTokenForState(state),
    secondaryTextToken: null,
    deliveryState: 'portal-row-only',
    adapterProofRef: null,
    postAnalysisActionPlan: state === 'allowed' ? postAnalysisActionPlan('allow') : null,
    rawCopyClaimed: false,
    visualRenderClaimed: false,
    surveillanceCopyClaimed: false,
    shamingCopyClaimed: false,
  };
}

function textTokenForState(state: unknown) {
  switch (state) {
    case 'opening':
      return 'browser.child.opening.title';
    case 'checking':
      return 'browser.child.checking.title';
    case 'allowed':
      return 'browser.child.allowed.title';
    case 'warning':
      return 'browser.child.warning.title';
    case 'approval_required':
      return 'browser.child.approval.title';
    case 'limited':
      return 'browser.child.limited.title';
    case 'blocked':
      return 'browser.child.blocked.title';
    case 'unclassified':
      return 'browser.child.unclassified.title';
    case 'manual_required':
      return 'browser.child.manual.title';
    default:
      return 'browser.child.unavailable.title';
  }
}

function postAnalysisActionPlan(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPostAnalysisActionSchemaVersion,
    actionPlanId: 'browser-post-analysis-action-plan-youtube-video',
    createdAt: '2026-06-03T04:21:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    aiAnalysisId: 'browser-ai-analysis-result-youtube-video',
    policyDecision: policyDecision(outcome),
    policyDecisionAuditRefs: ['browser-policy-decision-audit-youtube-video'],
    parentRuleRefs: ['parent-rule-homework-window'],
    actionLabels: actionLabelsForOutcome(outcome),
    trigger: 'policy_decision',
    timing: outcome === 'allow' ? 'background_only' : 'after_playback_started',
    childAlreadyEngaged: outcome !== 'allow',
    deliveryState: outcome === 'allow' ? 'modeled_only' : 'delivered',
    adapterProofRef: outcome === 'allow' ? null : 'managed-browser-adapter-proof-page',
    rememberUntil: null,
    actionAuditRefs: ['browser-post-analysis-action-audit-youtube-video'],
    realtimeBlockClaimed: false,
    browserRuntimeMutationClaimed: false,
    directEnforcementClaimed: false,
  };
}

function actionLabelsForOutcome(outcome: unknown) {
  switch (outcome) {
    case 'allow':
      return ['background_reviewed', 'continue_allowed'];
    case 'warn':
      return ['warning_shown_after_review'];
    case 'block':
      return ['playback_stopped_after_review'];
    default:
      return ['manual_required'];
  }
}

function policyDecision(outcome: unknown) {
  return {
    schemaVersion: BrowserAiPolicyEvaluatorSchemaVersion,
    decisionId: 'browser-policy-decision-youtube-video',
    requestId: 'browser-policy-evaluator-request-youtube-video',
    decidedAt: '2026-06-03T04:20:59.000Z',
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
    adapterProofRef: outcome === 'allow' ? null : 'managed-browser-adapter-proof-page',
    fallbackUsed: false,
    aiClaimedAsAuthority: false,
    portalEvaluatedClaimed: false,
    directEnforcementClaimed: false,
  };
}
