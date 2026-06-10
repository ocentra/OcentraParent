import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../src/kinds';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenAnalysisQueueJobSchema,
  ScreenAnalysisResultSchema,
  ScreenCapabilitySnapshotSchema,
  ScreenEvidenceRecentSummarySchema,
  ScreenEvidenceSchemaVersion,
  ScreenLocalModelOutputSchema,
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
  ocrTextRetentionMode: 'disabled',
  credentialSuppressionEnabled: true,
  piiRedactionEnabled: false,
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
  specifyScreenEvidenceParserContracts();
  specifyScreenEvidenceRejectionContracts();
  specifyScreenEvidenceModelOutputContracts();
  specifyScreenEvidenceDeterministicContracts();
  specifyScreenEvidencePolicyGateContracts();
  specifyScreenEvidenceProtectedSurfaceContracts();
});

function specifyScreenEvidenceParserContracts() {
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
}

function specifyScreenEvidenceRejectionContracts() {
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
}

function specifyScreenEvidenceModelOutputContracts() {
  it('rejects malformed local model output before it can become policy input', () => {
    const validOutput = ScreenLocalModelOutputSchema.parse({
      primary_category: 'school',
      confidence: 0.82,
      visible_text: 'A study page is visible in the active window.',
      risk_signals: [],
    });
    const invalidCategory = ScreenLocalModelOutputSchema.safeParse({
      ...validOutput,
      primary_category: 'vpn',
    });
    const invalidConfidence = ScreenLocalModelOutputSchema.safeParse({
      ...validOutput,
      confidence: 1.2,
    });
    const invalidRiskSignal = ScreenLocalModelOutputSchema.safeParse({
      ...validOutput,
      risk_signals: ['random-model-string'],
    });

    expect(validOutput.primary_category).toBe('school');
    expect(invalidCategory.success).toBe(false);
    expect(invalidConfidence.success).toBe(false);
    expect(invalidRiskSignal.success).toBe(false);
  });
}

function specifyScreenEvidenceDeterministicContracts() {
  it('accepts deterministic structured evidence without pretending an image was captured', () => {
    const deterministicResult = ScreenAnalysisResultSchema.parse({
      ...AnalysisResult,
      screenAnalysisResultId: 'screen-analysis-deterministic-result-1',
      queueJobId: 'screen-queue-deterministic-1',
      modelRuntimeRef: 'screen-deterministic-rules-runtime',
      modelId: 'structured-evidence-rule-engine',
      providerKind: 'deterministicRules',
      promptOrTemplateVersion: 'screen-structured-evidence-v1',
      captureReason: 'policyAmbiguity',
      captureScope: 'unsupported',
      summary: 'Managed structured evidence already identifies a school activity surface.',
      visibleCategoryCandidates: [
        {
          category: 'school',
          confidence: 0.95,
          evidenceRefs: [JournalEvidence],
        },
      ],
      primaryCategory: 'school',
      riskSignals: [],
      ocrTextSnippets: [],
      redactionNotes: [],
      confidence: 0.95,
      sourceEvidenceRefs: [JournalEvidence],
      imageDigest: 'sha256:deterministic-structured-evidence',
      imageDeletionState: 'unavailableNoImage',
      custodyState: 'child-device-query-store',
      policyEligible: true,
    });
    const imageBackedDeterministicResult = ScreenAnalysisResultSchema.safeParse({
      ...deterministicResult,
      imageDeletionState: 'deleted',
    });

    expect(deterministicResult.providerKind).toBe('deterministicRules');
    expect(deterministicResult.imageDeletionState).toBe('unavailableNoImage');
    expect(imageBackedDeterministicResult.success).toBe(false);
  });
}

function specifyScreenEvidencePolicyGateContracts() {
  it('allows observe-only analysis while blocking policy handoff', () => {
    const observeOnlySetting = ScreenAnalysisParentSettingSchema.parse({
      ...ParentSetting,
      screenAnalysisEnabled: true,
      triggerCaptureEnabled: true,
      enabledTriggers: ['manualParentTestCapture'],
      policyUseEnabled: false,
    });
    const observeOnlyWithPolicy = ScreenAnalysisParentSettingSchema.safeParse({
      ...observeOnlySetting,
      policyUseEnabled: true,
    });

    expect(observeOnlySetting.analysisMode).toBe('observeOnly');
    expect(observeOnlySetting.policyUseEnabled).toBe(false);
    expect(observeOnlyWithPolicy.success).toBe(false);
  });
}

function specifyScreenEvidenceProtectedSurfaceContracts() {
  it('records protected surfaces as degraded skips instead of policy input', () => {
    const protectedSnapshot = ScreenCapabilitySnapshotSchema.parse({
      schemaVersion: ScreenEvidenceSchemaVersion,
      observedAt: '2026-05-21T06:54:00Z',
      capabilityStatus: 'protectedSurface',
      captureScope: 'selectedWindow',
      parentSettingRef: 'parent-setting-screen-1',
      settingVersion: 1,
      unavailableReason: 'protected surface blocked capture',
      custodyState: 'unavailable',
    });
    const protectedSkip = ScreenAnalysisResultSchema.parse({
      ...AnalysisResult,
      screenAnalysisResultId: 'screen-analysis-protected-skip',
      queueJobId: 'screen-queue-protected-skip',
      modelRuntimeRef: 'local-model-unavailable-protected',
      modelId: 'unavailable-protected-surface',
      providerKind: 'unavailable',
      promptOrTemplateVersion: 'screen-protected-surface-v1',
      captureScope: 'selectedWindow',
      capabilityStatus: 'protectedSurface',
      summary: 'Protected surface blocked local capture and analysis.',
      visibleCategoryCandidates: [],
      primaryCategory: null,
      riskSignals: [],
      ocrTextSnippets: [],
      redactionNotes: ['protectedRegionSkipped'],
      confidence: 0,
      uncertaintyReason: 'protectedSurface',
      imageDigest: 'sha256:protected-surface-no-image',
      imageDeletionState: 'unavailableNoImage',
      custodyState: 'unavailable',
      policyEligible: false,
    });
    const policyEligibleProtectedSurface = ScreenAnalysisResultSchema.safeParse({
      ...protectedSkip,
      primaryCategory: 'unknown',
      policyEligible: true,
    });

    expect(protectedSnapshot.capabilityStatus).toBe('protectedSurface');
    expect(protectedSkip.policyEligible).toBe(false);
    expect(policyEligibleProtectedSurface.success).toBe(false);
  });
}
