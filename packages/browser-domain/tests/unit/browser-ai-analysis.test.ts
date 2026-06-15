import { describe, expect, it } from 'vitest';
import {
  BrowserAiAnalysisSchemaVersion,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisResultSchema,
} from '../../src/browser-ai-analysis-schemas';

describe('browser URL AI analysis input contract', () => {
  it('accepts structured AI input with evidence, URL shape, metadata, memory, graph, and parent rule refs', () => {
    const parsed = BrowserUrlAiAnalysisInputSchema.safeParse(aiAnalysisInput());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.requestedTask).toBe('video-safety');
      expect(parsed.data.rawBrowserStateIncluded).toBe(false);
      expect(parsed.data.promptTemplate.rawPromptTextIncluded).toBe(false);
    }
  });

  it('rejects AI input that includes raw browser state, DevTools payload, SQLite, journal, or OS state', () => {
    const rawBrowserState = BrowserUrlAiAnalysisInputSchema.safeParse({
      ...aiAnalysisInput(),
      rawBrowserStateIncluded: true,
    });
    const rawSystemState = BrowserUrlAiAnalysisInputSchema.safeParse({
      ...aiAnalysisInput(),
      devToolsPayloadIncluded: true,
      sqlitePathIncluded: true,
      journalPathIncluded: true,
      osStateIncluded: true,
    });

    expect(rawBrowserState.success).toBe(false);
    expect(rawSystemState.success).toBe(false);
  });

  it('rejects AI input without source evidence refs', () => {
    const parsed = BrowserUrlAiAnalysisInputSchema.safeParse({
      ...aiAnalysisInput(),
      sourceEvidenceIds: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects prompt templates that do not match the requested task', () => {
    const parsed = BrowserUrlAiAnalysisInputSchema.safeParse({
      ...aiAnalysisInput(),
      requestedTask: 'educational-relevance',
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser URL AI analysis result contract', () => {
  it('accepts candidate-only AI output with content, risk, benefit, model, prompt, and evidence refs', () => {
    const parsed = BrowserUrlAiAnalysisResultSchema.safeParse(aiAnalysisResult());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.recommendedPolicyInput).toBe('warn-candidate');
      expect(parsed.data.finalPolicyActionClaimed).toBe(false);
      expect(parsed.data.enforcementActionClaimed).toBe(false);
    }
  });

  it('rejects AI output that claims final policy action, enforcement, or raw content storage', () => {
    const parsed = BrowserUrlAiAnalysisResultSchema.safeParse({
      ...aiAnalysisResult(),
      finalPolicyActionClaimed: true,
      enforcementActionClaimed: true,
      rawContentStored: true,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects degraded AI output without uncertainty reasons', () => {
    const parsed = BrowserUrlAiAnalysisResultSchema.safeParse({
      ...aiAnalysisResult(),
      degradedState: 'degraded',
      confidence: 'low',
      uncertaintyReasons: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('accepts degraded AI output with explicit uncertainty and manual review recommendation', () => {
    const parsed = BrowserUrlAiAnalysisResultSchema.safeParse({
      ...aiAnalysisResult(),
      recommendedPolicyInput: 'manual-review-candidate',
      confidence: 'low',
      uncertaintyReasons: ['timeout', 'metadata-missing'],
      degradedState: 'manual-required',
    });

    expect(parsed.success).toBe(true);
  });
});

function aiAnalysisInput() {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    requestId: 'browser-ai-analysis-request-youtube-video',
    requestedAt: '2026-06-03T00:09:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-2026-06-03-youtube-video',
    metadataEvidenceIds: ['metadata-evidence-youtube-video'],
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    normalizedUrl: 'https://www.youtube.com/watch?v=abc123',
    normalizedDomain: 'youtube.com',
    platform: 'youtube',
    platformIds: {
      videoId: 'abc123',
      channelId: 'channel-abc123',
      playlistId: null,
      postId: null,
      query: null,
    },
    title: 'Example math lesson',
    description: 'A short fractions lesson for middle school.',
    transcriptRefs: ['transcript-summary-ref-abc123'],
    thumbnailRefs: ['thumbnail-hash-ref-abc123'],
    screenEvidenceRefs: ['screen-evidence-youtube-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference: 'local-preferred',
    promptTemplate: promptTemplate(),
    custodyLabel: 'child-device-local',
    rawBrowserStateIncluded: false,
    devToolsPayloadIncluded: false,
    sqlitePathIncluded: false,
    journalPathIncluded: false,
    osStateIncluded: false,
  };
}

function aiAnalysisResult() {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    analysisId: 'browser-ai-analysis-result-youtube-video',
    requestId: 'browser-ai-analysis-request-youtube-video',
    analyzedAt: '2026-06-03T00:09:02.000Z',
    expiresAt: '2026-06-03T01:09:02.000Z',
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
