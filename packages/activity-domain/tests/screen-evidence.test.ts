import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../src/kinds';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenAnalysisQueueJobSchema,
  ScreenAnalysisResultSchema,
  ScreenEvidenceRecentSummarySchema,
  ScreenEvidenceSchemaVersion,
} from '../src/screen-evidence';

const JournalEvidence = {
  evidenceId: 'journal-entry-screen-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:screen-summary-digest',
  uri: null,
} as const;

const ParentSetting = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisEnabled: false,
  analysisMode: 'observeOnly',
  cadenceCaptureEnabled: false,
  cadenceSeconds: 300,
  strictModeEnabled: false,
  triggerCaptureEnabled: false,
  enabledTriggers: ['manualParentTestCapture'],
  allowedCaptureScope: 'activeWindow',
  ocrTextEnabled: false,
  ocrTextSnippetLimit: 0,
  redactionMode: 'disabled',
  temporaryImageTtlSeconds: 300,
  maxRetryCount: 2,
  deleteAfterSuccess: true,
  deleteAfterExpiry: true,
  retainRawImage: false,
  policyUseEnabled: false,
  changedByParentRef: 'parent-setting-screen-1',
  changedAt: '2026-05-21T06:50:00Z',
  settingVersion: 1,
  reason: 'initial disabled setting',
} as const;

const QueueJob = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  queueJobId: 'screen-queue-job-1',
  createdAt: '2026-05-21T06:51:00Z',
  notBefore: '2026-05-21T06:51:00Z',
  expiresAt: '2026-05-21T06:56:00Z',
  lastAttemptAt: null,
  captureReason: 'manualParentTestCapture',
  captureScope: 'activeWindow',
  sourceId: 'screen-capture-scheduler',
  adapterId: 'windows-screen-capture',
  deviceRef: 'child-device-1',
  localUserRef: 'local-user-1',
  parentSettingRef: 'parent-setting-screen-1',
  settingVersion: 1,
  relatedEvidenceRefs: [JournalEvidence],
  encryptedImageRef: 'screen-queue-ref-1',
  imageDigest: 'sha256:screen-image-digest',
  imageByteSize: 2048,
  imageFormat: 'png',
  status: 'queued',
  attemptCount: 0,
  maxRetryCount: 2,
  failureReason: null,
  unavailableReason: null,
  deletionRequired: true,
  deletedAt: null,
  deletionStatus: 'deletionRequired',
  deletionProofRef: null,
  custodyState: 'child-device-temp-queue',
} as const;

const AnalysisResult = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisResultId: 'screen-analysis-result-1',
  queueJobId: 'screen-queue-job-1',
  analyzedAt: '2026-05-21T06:52:00Z',
  modelRuntimeRef: 'local-vision-runtime-1',
  modelId: 'local-vision-model',
  providerKind: 'localVision',
  promptOrTemplateVersion: 'screen-summary-v1',
  captureReason: 'manualParentTestCapture',
  captureScope: 'activeWindow',
  capabilityStatus: 'ready',
  summary: 'A study page is visible in the active window.',
  visibleCategoryCandidates: [
    {
      category: 'school',
      confidence: 0.88,
      evidenceRefs: [JournalEvidence],
    },
  ],
  primaryCategory: 'school',
  riskSignals: [],
  ocrTextSnippets: [],
  redactionNotes: ['ocrDisabled'],
  confidence: 0.88,
  uncertaintyReason: null,
  sourceEvidenceRefs: [JournalEvidence],
  imageDigest: 'sha256:screen-image-digest',
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-journal',
  policyEligible: true,
} as const;

const RecentSummary = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  generatedAt: '2026-05-21T06:53:00Z',
  custodyState: 'child-device-query-store',
  limit: 10,
  returned: 1,
  queueHealth: {
    schemaVersion: ScreenEvidenceSchemaVersion,
    generatedAt: '2026-05-21T06:53:00Z',
    custodyState: 'child-device-query-store',
    pendingCount: 0,
    expiredCount: 0,
    deletePendingCount: 0,
    deleteFailedCount: 0,
    latestQueueJobId: 'screen-queue-job-1',
    latestStatus: 'deleted',
    lastSuccessfulAnalysisAt: '2026-05-21T06:52:00Z',
  },
  latestResultId: 'screen-analysis-result-1',
  latestSummary: 'A study page is visible in the active window.',
  latestPrimaryCategory: 'school',
  latestConfidence: 0.88,
  latestImageDeletionState: 'deleted',
  latestPolicyEligible: true,
  evidence: [JournalEvidence],
  results: [AnalysisResult],
} as const;

describe('screen evidence contracts', () => {
  it('parses disabled parent settings, encrypted queue jobs, results, and read summaries', () => {
    const setting = ScreenAnalysisParentSettingSchema.parse(ParentSetting);
    const job = ScreenAnalysisQueueJobSchema.parse(QueueJob);
    const result = ScreenAnalysisResultSchema.parse(AnalysisResult);
    const summary = ScreenEvidenceRecentSummarySchema.parse(RecentSummary);

    expect(setting.screenAnalysisEnabled).toBe(false);
    expect(job.custodyState).toBe('child-device-temp-queue');
    expect(result.rawImageRetained).toBe(false);
    expect(summary.latestPrimaryCategory).toBe('school');
  });

  it('rejects unsafe retention, unbounded queue settings, confidence drift, and policy use without evidence', () => {
    const retainedImage = ScreenAnalysisParentSettingSchema.safeParse({
      ...ParentSetting,
      retainRawImage: true,
    });
    const lowCadence = ScreenAnalysisParentSettingSchema.safeParse({
      ...ParentSetting,
      cadenceSeconds: 30,
    });
    const missingDeletionRequired = ScreenAnalysisQueueJobSchema.safeParse({
      ...QueueJob,
      deletionRequired: false,
    });
    const highConfidence = ScreenAnalysisResultSchema.safeParse({
      ...AnalysisResult,
      confidence: 1.2,
    });
    const policyWithoutEvidence = ScreenAnalysisResultSchema.safeParse({
      ...AnalysisResult,
      sourceEvidenceRefs: [],
    });

    expect(retainedImage.success).toBe(false);
    expect(lowCadence.success).toBe(false);
    expect(missingDeletionRequired.success).toBe(false);
    expect(highConfidence.success).toBe(false);
    expect(policyWithoutEvidence.success).toBe(false);
  });
});
