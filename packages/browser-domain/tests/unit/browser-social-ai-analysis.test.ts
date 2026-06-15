import { describe, expect, it } from 'vitest';
import {
  BrowserSocialAiAnalysisInputSchema,
  BrowserSocialAiAnalysisResultSchema,
} from '../../src/browser-social-ai-analysis-schemas';
import { buildBrowserSocialAiAnalysisResult } from '../../src/browser-social-ai-analysis-result-builder';

describe('browser social AI analysis contract', () => {
  it('accepts social video analysis from typed route and metadata refs only', acceptsVideoAnalysis);
  it('accepts account and feed analysis only when matching evidence refs are present', acceptsAccountAndFeedAnalysis);
  it('rejects raw content, prompt mismatch, and task/evidence mismatch', rejectsInputBoundaryBreaks);
  it('rejects policy, enforcement, raw model/content, and degraded-state authority claims', rejectsResultClaims);
});

function acceptsVideoAnalysis() {
  const input = BrowserSocialAiAnalysisInputSchema.parse(socialAiInput());
  const result = buildBrowserSocialAiAnalysisResult({
    analysisId: 'social-ai-analysis-video-result',
    analyzedAt: '2026-06-03T06:30:00.000Z',
    expiresAt: '2026-06-03T07:30:00.000Z',
    input,
    classifications: ['video-watch', 'educational-video'],
    riskSignalRefs: ['social-riskSignal-ref-video-low'],
    benefitSignalRefs: ['social-benefit-signal-ref-homework'],
    recommendedPolicyInput: 'allow-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummaryRef: 'parent-summary-ref-video-analysis',
    childSafeSummaryRef: 'child-safe-summary-ref-video-analysis',
    modelRuntimeRef: 'local-social-ai-runtime-ref',
    degradedState: 'none',
  });

  expect(result.platform).toBe('youtube');
  expect(result.routeKind).toBe('video');
  expect(result.finalPolicyActionClaimed).toBe(false);
  expect(result.enforcementActionClaimed).toBe(false);
  expect(result.rawModelTextStored).toBe(false);
}

function acceptsAccountAndFeedAnalysis() {
  const signup = BrowserSocialAiAnalysisInputSchema.safeParse(
    socialAiInput({
      requestId: 'social-ai-request-signup',
      requestedTask: 'signup-attempt-classification',
      promptTemplate: socialPromptTemplate('signup-attempt-classification'),
      routeKind: 'account-signup',
      metadataEvidenceIds: [],
      accountFlowEvidenceIds: ['social-account-flow-evidence-signup'],
    })
  );
  const feed = BrowserSocialAiAnalysisInputSchema.safeParse(
    socialAiInput({
      requestId: 'social-ai-request-feed',
      requestedTask: 'feed-risk-classification',
      promptTemplate: socialPromptTemplate('feed-risk-classification'),
      routeKind: 'feed',
      feedRouteClassificationIds: ['social-feed-route-classification-home'],
      metadataEvidenceIds: [],
    })
  );

  expect(signup.success).toBe(true);
  expect(feed.success).toBe(true);
}

function rejectsInputBoundaryBreaks() {
  const invalidRows = [
    { ...socialAiInput(), rawMessageContentIncluded: true },
    {
      ...socialAiInput(),
      requestedTask: 'video-safety',
      promptTemplate: socialPromptTemplate('feed-risk-classification'),
    },
    { ...socialAiInput(), metadataEvidenceIds: [] },
    {
      ...socialAiInput(),
      requestedTask: 'feed-risk-classification',
      promptTemplate: socialPromptTemplate('feed-risk-classification'),
      routeKind: 'video',
    },
    { ...socialAiInput(), platformConnectorIncluded: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialAiAnalysisInputSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsResultClaims() {
  const input = BrowserSocialAiAnalysisInputSchema.parse(socialAiInput());
  const valid = buildBrowserSocialAiAnalysisResult({
    analysisId: 'social-ai-analysis-video-result',
    analyzedAt: '2026-06-03T06:30:00.000Z',
    expiresAt: '2026-06-03T07:30:00.000Z',
    input,
    classifications: ['video-watch'],
    riskSignalRefs: ['social-riskSignal-ref-video-low'],
    benefitSignalRefs: ['social-benefit-signal-ref-video-neutral'],
    recommendedPolicyInput: 'warn-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummaryRef: 'parent-summary-ref-video-analysis',
    childSafeSummaryRef: null,
    modelRuntimeRef: 'local-social-ai-runtime-ref',
    degradedState: 'none',
  });
  const invalidRows = [
    { ...valid, finalPolicyActionClaimed: true },
    { ...valid, enforcementActionClaimed: true },
    { ...valid, rawModelTextStored: true },
    { ...valid, rawFeedContentStored: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, uncertaintyReasons: ['low-confidence'] },
    { ...valid, classifications: ['unknown'] },
    { ...valid, degradedState: 'degraded', confidence: 'high', uncertaintyReasons: ['low-confidence'] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialAiAnalysisResultSchema.safeParse(invalid).success).toBe(false);
  }
}

function socialAiInput(overrides = {}) {
  return {
    schemaVersion: 1,
    requestId: 'social-ai-request-video',
    requestedAt: '2026-06-03T06:29:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    sourceEvidenceIds: ['social-route-evidence-video', 'social-video-metadata-evidence-youtube'],
    socialRouteEvidenceId: 'social-route-evidence-video',
    urlShapeClassificationId: 'url-shape-social-video-youtube',
    platform: 'youtube',
    routeKind: 'video',
    feedRouteClassificationIds: [],
    metadataEvidenceIds: ['social-video-metadata-evidence-youtube'],
    accountFlowEvidenceIds: [],
    accountIdentityRefs: [],
    screenSummaryEvidenceRefs: [],
    parentRuleRefs: ['parent-rule-homework-window'],
    memoryHitIds: ['memory-hit-known-education-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference: 'local-preferred',
    promptTemplate: socialPromptTemplate('video-safety'),
    custodyLabel: 'child-device-local',
    rawBrowserStateIncluded: false,
    rawPageBodyIncluded: false,
    rawMessageContentIncluded: false,
    rawFeedContentIncluded: false,
    transcriptTextIncluded: false,
    screenshotIncluded: false,
    nativeAppStateIncluded: false,
    platformConnectorIncluded: false,
    ...overrides,
  };
}

function socialPromptTemplate(task = 'video-safety') {
  return {
    promptTemplateId: `social-ai-prompt-template-${task}`,
    promptTemplateVersion: `social-ai-prompt-template-${task}-v1`,
    requestedTask: task,
    allowedInputFieldRefs: ['route-evidence', 'metadata-evidence', 'parent-rule', 'memory-hit'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
    capturesMessageContent: false,
    capturesFeedContent: false,
    capturesScreenshot: false,
  };
}
