import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '../src/browser-ai-analysis-schemas';
import {
  BrowserAnalysisJobSchema,
  BrowserAnalysisQueueSchemaVersion,
  BrowserAnalysisTimeoutPolicySchema,
  browserAnalysisTimeoutPolicyFor,
  createBrowserAnalysisQueuedJob,
} from '../src/browser-ai-analysis-queue-schemas';

describe('browser URL video analysis queue contract', () => {
  it('creates parent-timeout-owned p0 queued jobs', createsP0QueuedJob);
  it('rejects p0 timeout policies that are slow or background-only', rejectsDishonestP0Timeout);
  it('accepts completed jobs only when the result matches the input request', acceptsCompletedMatchingResult);
  it('rejects completed jobs with mismatched result request ids', rejectsMismatchedCompletedResult);
  it('rejects queued jobs that already carry a result', rejectsQueuedJobWithResult);
  it('accepts background review timeout semantics for p3 jobs', acceptsBackgroundReviewTimeout);
  it('rejects queue jobs that claim worker policy or enforcement authority', rejectsQueueAuthorityClaims);
});

function createsP0QueuedJob() {
  const job = createBrowserAnalysisQueuedJob(jobRequest());

  expect(job.status).toBe('queued');
  expect(job.priority).toBe('p0-strict-hold');
  expect(job.timeoutPolicy.timeoutDisposition).toBe('parent-policy-fallback');
  expect(job.timeoutPolicy.timeoutMs).toBe(3000);
  expect(job.workerRuntimeClaimed).toBe(false);
}

function rejectsDishonestP0Timeout() {
  const parsed = BrowserAnalysisTimeoutPolicySchema.safeParse({
    ...browserAnalysisTimeoutPolicyFor('p0-strict-hold'),
    timeoutMs: 15000,
    timeoutDisposition: 'background-only',
  });

  expect(parsed.success).toBe(false);
}

function acceptsCompletedMatchingResult() {
  const parsed = BrowserAnalysisJobSchema.safeParse({
    ...createBrowserAnalysisQueuedJob(jobRequest()),
    status: 'completed',
    result: aiAnalysisResult(),
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.result?.requestId).toBe('browser-ai-analysis-request-youtube-video');
    expect(parsed.data.finalPolicyActionClaimed).toBe(false);
  }
}

function rejectsMismatchedCompletedResult() {
  const parsed = BrowserAnalysisJobSchema.safeParse({
    ...createBrowserAnalysisQueuedJob(jobRequest()),
    status: 'completed',
    result: {
      ...aiAnalysisResult(),
      requestId: 'browser-ai-analysis-request-other-video',
    },
  });

  expect(parsed.success).toBe(false);
}

function rejectsQueuedJobWithResult() {
  const parsed = BrowserAnalysisJobSchema.safeParse({
    ...createBrowserAnalysisQueuedJob(jobRequest()),
    result: aiAnalysisResult(),
  });

  expect(parsed.success).toBe(false);
}

function acceptsBackgroundReviewTimeout() {
  const policy = browserAnalysisTimeoutPolicyFor('p3-background-review');

  expect(policy.timeoutDisposition).toBe('wait-or-degrade');
  expect(policy.timeoutMs).toBe(60000);
}

function rejectsQueueAuthorityClaims() {
  const parsed = BrowserAnalysisJobSchema.safeParse({
    ...createBrowserAnalysisQueuedJob(jobRequest()),
    timeoutPolicyOwnedByParent: false,
    workerRuntimeClaimed: true,
    finalPolicyActionClaimed: true,
    enforcementActionClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function jobRequest() {
  return {
    jobId: 'browser-analysis-job-youtube-video',
    queuedAt: '2026-06-03T03:45:00.000Z',
    input: aiAnalysisInput(),
    priority: 'p0-strict-hold',
    queuedEvidenceIds: ['browser-evidence-youtube-video'],
  };
}

function aiAnalysisInput() {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    requestId: 'browser-ai-analysis-request-youtube-video',
    requestedAt: '2026-06-03T03:44:00.000Z',
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
    analyzedAt: '2026-06-03T03:45:02.000Z',
    expiresAt: '2026-06-03T04:45:02.000Z',
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
