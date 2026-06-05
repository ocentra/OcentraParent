import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../src/kinds';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenAnalysisQueueJobSchema,
  ScreenAnalysisResultSchema,
  ScreenEvidenceRemoteBoundarySettingSchema,
  ScreenEvidenceSchemaVersion,
} from '../src/screen-evidence';

const JournalEvidence = {
  evidenceId: 'journal-entry-screen-retention-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:screen-retention-summary-digest',
  uri: null,
} as const;

const StrictPolicyParentSetting = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisEnabled: true,
  analysisMode: 'policyDryRun',
  cadenceCaptureEnabled: true,
  cadenceSeconds: 60,
  strictModeEnabled: true,
  triggerCaptureEnabled: true,
  enabledTriggers: ['foregroundAppChange', 'policyAmbiguity'],
  allowedCaptureScope: 'activeWindow',
  ocrTextEnabled: true,
  ocrTextSnippetLimit: 4,
  redactionMode: 'localSensitiveText',
  temporaryImageTtlSeconds: 300,
  maxRetryCount: 2,
  deleteAfterSuccess: true,
  deleteAfterExpiry: true,
  retainRawImage: false,
  policyUseEnabled: true,
  changedByParentRef: 'parent-setting-screen-retention-1',
  changedAt: '2026-05-21T06:50:00Z',
  settingVersion: 2,
  reason: 'parent enabled strict screen analysis dry run',
} as const;

const QueueJob = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  queueJobId: 'screen-queue-job-retention-1',
  createdAt: '2026-05-21T06:51:00Z',
  notBefore: '2026-05-21T06:51:00Z',
  expiresAt: '2026-05-21T06:56:00Z',
  lastAttemptAt: null,
  captureReason: 'policyAmbiguity',
  captureScope: 'activeWindow',
  sourceId: 'screen-capture-scheduler',
  adapterId: 'windows-screen-capture',
  deviceRef: 'child-device-1',
  localUserRef: 'local-user-1',
  parentSettingRef: 'parent-setting-screen-retention-1',
  settingVersion: 2,
  relatedEvidenceRefs: [JournalEvidence],
  encryptedImageRef: 'screen-queue-ref-retention-1',
  imageDigest: 'sha256:screen-retention-image-digest',
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

const DeletedQueueJob = {
  ...QueueJob,
  status: 'deleted',
  attemptCount: 1,
  lastAttemptAt: '2026-05-21T06:52:00Z',
  deletedAt: '2026-05-21T06:52:30Z',
  deletionStatus: 'deleted',
  deletionProofRef: 'screen-delete-proof-1',
} as const;

const ExpiredQueueJob = {
  ...QueueJob,
  status: 'expired',
  attemptCount: 2,
  lastAttemptAt: '2026-05-21T06:55:00Z',
  deletedAt: '2026-05-21T06:56:10Z',
  deletionStatus: 'expiredDeleted',
  deletionProofRef: 'screen-delete-proof-expired-1',
} as const;

const DeleteFailedQueueJob = {
  ...QueueJob,
  status: 'failed',
  attemptCount: 2,
  lastAttemptAt: '2026-05-21T06:55:00Z',
  failureReason: 'temporary image deletion failed',
  deletionStatus: 'deleteFailed',
} as const;

const AnalysisResult = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisResultId: 'screen-analysis-result-retention-1',
  queueJobId: 'screen-queue-job-retention-1',
  analyzedAt: '2026-05-21T06:52:00Z',
  modelRuntimeRef: 'local-vision-runtime-1',
  modelId: 'local-vision-model',
  providerKind: 'localVision',
  promptOrTemplateVersion: 'screen-summary-v1',
  captureReason: 'policyAmbiguity',
  captureScope: 'activeWindow',
  capabilityStatus: 'ready',
  summary: 'A study page is visible in the active window.',
  visibleCategoryCandidates: [{ category: 'school', confidence: 0.88, evidenceRefs: [JournalEvidence] }],
  primaryCategory: 'school',
  riskSignals: [],
  ocrTextSnippets: [],
  redactionNotes: ['credentialLikeTextRedacted'],
  confidence: 0.88,
  uncertaintyReason: null,
  sourceEvidenceRefs: [JournalEvidence],
  imageDigest: 'sha256:screen-retention-image-digest',
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-journal',
  policyEligible: true,
} as const;

const UnknownAnalysisResult = {
  ...AnalysisResult,
  screenAnalysisResultId: 'screen-analysis-result-retention-unknown-1',
  summary: 'The visible activity could not be classified confidently.',
  visibleCategoryCandidates: [{ category: 'unknown', confidence: 0.34, evidenceRefs: [JournalEvidence] }],
  primaryCategory: 'unknown',
  confidence: 0.34,
  uncertaintyReason: 'lowConfidence',
  policyEligible: false,
} as const;

const DisabledRemoteBoundarySetting = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  parentSettingRef: 'parent-setting-screen-retention-1',
  settingVersion: 2,
  rawScreenshotRetentionMode: 'disabled',
  liveViewMode: 'disabled',
  rawScreenshotRemoteUploadEnabled: false,
  remoteSummaryMode: 'disabled',
  remoteSummaryRedactedOnly: true,
  parentApprovedRemoteSummary: false,
  remoteSummaryApprovalRef: null,
  remoteSummaryDestinationCustodyState: 'unavailable',
  changedByParentRef: 'parent-setting-screen-retention-1',
  changedAt: '2026-05-21T06:50:00Z',
  reason: 'default local screen evidence boundary',
} as const;

const ParentApprovedSummaryBoundarySetting = {
  ...DisabledRemoteBoundarySetting,
  settingVersion: 3,
  remoteSummaryMode: 'parentApprovedRedactedSummary',
  parentApprovedRemoteSummary: true,
  remoteSummaryApprovalRef: 'screen-remote-summary-approval-1',
  remoteSummaryDestinationCustodyState: 'parent-owned-export',
  reason: 'parent approved redacted screen summary export',
} as const;

describe('screen evidence retention contracts', () => {
  specifyParentOptInSettings();
  specifyQueueDeletionStates();
  specifyLowConfidenceEvidence();
  specifyUnsafeRetentionRejections();
  specifyRemoteBoundaryDefaults();
  specifyRemoteBoundaryRejections();
});

function specifyParentOptInSettings() {
  it('parses parent opt-in, strict cadence, trigger capture, and policy dry-run intent', () => {
    const setting = ScreenAnalysisParentSettingSchema.parse(StrictPolicyParentSetting);

    expect(setting.screenAnalysisEnabled).toBe(true);
    expect(setting.analysisMode).toBe('policyDryRun');
    expect(setting.cadenceSeconds).toBe(60);
    expect(setting.enabledTriggers).toEqual(['foregroundAppChange', 'policyAmbiguity']);
    expect(setting.policyUseEnabled).toBe(true);
    expect(setting.retainRawImage).toBe(false);
  });
}

function specifyQueueDeletionStates() {
  it('parses deleted, expired-deleted, and delete-failed queue states', () => {
    const deleted = ScreenAnalysisQueueJobSchema.parse(DeletedQueueJob);
    const expired = ScreenAnalysisQueueJobSchema.parse(ExpiredQueueJob);
    const deleteFailed = ScreenAnalysisQueueJobSchema.parse(DeleteFailedQueueJob);

    expect(deleted.deletionProofRef).toBe('screen-delete-proof-1');
    expect(expired.deletionStatus).toBe('expiredDeleted');
    expect(deleteFailed.deletionStatus).toBe('deleteFailed');
    expect(deleteFailed.deletedAt).toBeNull();
  });
}

function specifyLowConfidenceEvidence() {
  it('parses unknown low-confidence summaries as policy-ineligible evidence', () => {
    const result = ScreenAnalysisResultSchema.parse(UnknownAnalysisResult);

    expect(result.primaryCategory).toBe('unknown');
    expect(result.confidence).toBe(0.34);
    expect(result.uncertaintyReason).toBe('lowConfidence');
    expect(result.policyEligible).toBe(false);
    expect(result.rawImageRetained).toBe(false);
  });
}

function specifyUnsafeRetentionRejections() {
  it('rejects unsafe settings, queue bounds, deletion proof, and policy eligibility', () => {
    expect(
      ScreenAnalysisParentSettingSchema.safeParse({ ...StrictPolicyParentSetting, analysisMode: 'observeOnly' }).success
    ).toBe(false);
    expect(
      ScreenAnalysisParentSettingSchema.safeParse({ ...StrictPolicyParentSetting, cadenceSeconds: 300 }).success
    ).toBe(false);
    expect(
      ScreenAnalysisParentSettingSchema.safeParse({ ...StrictPolicyParentSetting, enabledTriggers: [] }).success
    ).toBe(false);
    expect(ScreenAnalysisQueueJobSchema.safeParse({ ...QueueJob, attemptCount: 3, maxRetryCount: 2 }).success).toBe(
      false
    );
    expect(ScreenAnalysisQueueJobSchema.safeParse({ ...QueueJob, expiresAt: '2026-05-21T06:50:00Z' }).success).toBe(
      false
    );
    expect(
      ScreenAnalysisQueueJobSchema.safeParse({
        ...DeleteFailedQueueJob,
        deletionStatus: 'deleted',
        deletedAt: '2026-05-21T06:56:30Z',
        deletionProofRef: 'screen-delete-proof-false-success',
      }).success
    ).toBe(false);
    expect(
      ScreenAnalysisQueueJobSchema.safeParse({
        ...ExpiredQueueJob,
        status: 'failed',
      }).success
    ).toBe(false);
    expect(ScreenAnalysisResultSchema.safeParse({ ...AnalysisResult, rawImageRetained: true }).success).toBe(false);
    expect(ScreenAnalysisResultSchema.safeParse({ ...UnknownAnalysisResult, policyEligible: true }).success).toBe(
      false
    );
  });
}

function specifyRemoteBoundaryDefaults() {
  it('keeps raw screenshot retention, live view, and raw remote upload outside the default summary boundary', () => {
    const disabled = ScreenEvidenceRemoteBoundarySettingSchema.parse(DisabledRemoteBoundarySetting);
    const approvedSummary = ScreenEvidenceRemoteBoundarySettingSchema.parse(ParentApprovedSummaryBoundarySetting);

    expect(disabled.rawScreenshotRetentionMode).toBe('disabled');
    expect(disabled.liveViewMode).toBe('disabled');
    expect(disabled.rawScreenshotRemoteUploadEnabled).toBe(false);
    expect(disabled.remoteSummaryMode).toBe('disabled');
    expect(approvedSummary.remoteSummaryMode).toBe('parentApprovedRedactedSummary');
    expect(approvedSummary.remoteSummaryDestinationCustodyState).toBe('parent-owned-export');
    expect(approvedSummary.remoteSummaryRedactedOnly).toBe(true);
  });
}

function specifyRemoteBoundaryRejections() {
  it('rejects remote screen boundary settings without parent approval and redacted-summary custody', () => {
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...DisabledRemoteBoundarySetting,
        rawScreenshotRetentionMode: 'retainRawScreenshot',
      }).success
    ).toBe(false);
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...DisabledRemoteBoundarySetting,
        liveViewMode: 'relayBackedLiveView',
      }).success
    ).toBe(false);
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...DisabledRemoteBoundarySetting,
        rawScreenshotRemoteUploadEnabled: true,
      }).success
    ).toBe(false);
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...ParentApprovedSummaryBoundarySetting,
        remoteSummaryApprovalRef: null,
      }).success
    ).toBe(false);
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...ParentApprovedSummaryBoundarySetting,
        remoteSummaryDestinationCustodyState: 'ocentra-hosted-non-activity',
      }).success
    ).toBe(false);
    expect(
      ScreenEvidenceRemoteBoundarySettingSchema.safeParse({
        ...DisabledRemoteBoundarySetting,
        remoteSummaryMode: 'disabled',
        parentApprovedRemoteSummary: true,
      }).success
    ).toBe(false);
  });
}
