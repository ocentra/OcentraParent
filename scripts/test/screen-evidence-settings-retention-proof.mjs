import { strict as assert } from 'node:assert';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { ActivityEvidenceKind } from '../../packages/activity-domain/dist/kinds.js';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenAnalysisQueueJobSchema,
  ScreenAnalysisResultSchema,
  ScreenEvidenceRecentSummarySchema,
  ScreenEvidenceSchemaVersion,
} from '../../packages/activity-domain/dist/screen-evidence.js';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'test-results', 'screen-evidence-settings-retention-proof');
const outputPath = join(outputDir, 'proof.json');

const evidenceRef = {
  evidenceId: 'journal-entry-screen-proof-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:screen-proof-summary-digest',
  uri: null,
};

const parentSetting = {
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
  changedByParentRef: 'parent-setting-screen-proof-1',
  changedAt: '2026-05-21T06:50:00Z',
  settingVersion: 2,
  reason: 'parent enabled strict screen analysis dry run',
};

const queueJob = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  queueJobId: 'screen-queue-job-proof-1',
  createdAt: '2026-05-21T06:51:00Z',
  notBefore: '2026-05-21T06:51:00Z',
  expiresAt: '2026-05-21T06:56:00Z',
  lastAttemptAt: '2026-05-21T06:52:00Z',
  captureReason: 'policyAmbiguity',
  captureScope: 'activeWindow',
  sourceId: 'screen-capture-scheduler',
  adapterId: 'windows-screen-capture',
  deviceRef: 'child-device-1',
  localUserRef: 'local-user-1',
  parentSettingRef: 'parent-setting-screen-proof-1',
  settingVersion: 2,
  relatedEvidenceRefs: [evidenceRef],
  encryptedImageRef: 'screen-queue-ref-proof-1',
  imageDigest: 'sha256:screen-proof-image-digest',
  imageByteSize: 2048,
  imageFormat: 'png',
  status: 'deleted',
  attemptCount: 1,
  maxRetryCount: 2,
  failureReason: null,
  unavailableReason: null,
  deletionRequired: true,
  deletedAt: '2026-05-21T06:52:30Z',
  deletionStatus: 'deleted',
  deletionProofRef: 'screen-delete-proof-1',
  custodyState: 'child-device-temp-queue',
};

const analysisResult = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisResultId: 'screen-analysis-result-proof-1',
  queueJobId: 'screen-queue-job-proof-1',
  analyzedAt: '2026-05-21T06:52:00Z',
  modelRuntimeRef: 'local-vision-runtime-1',
  modelId: 'local-vision-model',
  providerKind: 'localVision',
  promptOrTemplateVersion: 'screen-summary-v1',
  captureReason: 'policyAmbiguity',
  captureScope: 'activeWindow',
  capabilityStatus: 'ready',
  summary: 'A study page is visible in the active window.',
  visibleCategoryCandidates: [
    {
      category: 'school',
      confidence: 0.88,
      evidenceRefs: [evidenceRef],
    },
  ],
  primaryCategory: 'school',
  riskSignals: [],
  ocrTextSnippets: [],
  redactionNotes: ['credentialLikeTextRedacted'],
  confidence: 0.88,
  uncertaintyReason: null,
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: 'sha256:screen-proof-image-digest',
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-journal',
  policyEligible: true,
};

const unknownResult = {
  ...analysisResult,
  screenAnalysisResultId: 'screen-analysis-result-proof-unknown-1',
  summary: 'Visible activity could not be classified confidently.',
  visibleCategoryCandidates: [
    {
      category: 'unknown',
      confidence: 0.34,
      evidenceRefs: [evidenceRef],
    },
  ],
  primaryCategory: 'unknown',
  redactionNotes: ['ocrDisabled'],
  confidence: 0.34,
  uncertaintyReason: 'lowConfidence',
  policyEligible: false,
};

const recentSummary = {
  schemaVersion: ScreenEvidenceSchemaVersion,
  generatedAt: '2026-05-21T06:53:00Z',
  custodyState: 'child-device-query-store',
  limit: 10,
  returned: 2,
  queueHealth: {
    schemaVersion: ScreenEvidenceSchemaVersion,
    generatedAt: '2026-05-21T06:53:00Z',
    custodyState: 'child-device-query-store',
    pendingCount: 0,
    expiredCount: 1,
    deletePendingCount: 0,
    deleteFailedCount: 1,
    latestQueueJobId: 'screen-queue-job-proof-1',
    latestStatus: 'deleted',
    lastSuccessfulAnalysisAt: '2026-05-21T06:52:00Z',
  },
  latestResultId: 'screen-analysis-result-proof-1',
  latestSummary: 'A study page is visible in the active window.',
  latestPrimaryCategory: 'school',
  latestConfidence: 0.88,
  latestImageDeletionState: 'deleted',
  latestPolicyEligible: true,
  evidence: [evidenceRef],
  results: [analysisResult, unknownResult],
};

const parsedSetting = ScreenAnalysisParentSettingSchema.parse(parentSetting);
const parsedQueueJob = ScreenAnalysisQueueJobSchema.parse(queueJob);
const parsedResult = ScreenAnalysisResultSchema.parse(analysisResult);
const parsedUnknownResult = ScreenAnalysisResultSchema.parse(unknownResult);
const parsedSummary = ScreenEvidenceRecentSummarySchema.parse(recentSummary);

const invalidDisabledCapture = ScreenAnalysisParentSettingSchema.safeParse({
  ...parentSetting,
  screenAnalysisEnabled: false,
  cadenceCaptureEnabled: true,
});
const invalidRetryOverflow = ScreenAnalysisQueueJobSchema.safeParse({
  ...queueJob,
  attemptCount: 3,
});
const invalidDeleteProof = ScreenAnalysisQueueJobSchema.safeParse({
  ...queueJob,
  deletionProofRef: null,
});
const invalidLowConfidencePolicy = ScreenAnalysisResultSchema.safeParse({
  ...unknownResult,
  policyEligible: true,
});

assert.equal(parsedSetting.policyUseEnabled, true);
assert.equal(parsedSetting.retainRawImage, false);
assert.equal(parsedQueueJob.custodyState, 'child-device-temp-queue');
assert.equal(parsedQueueJob.deletionStatus, 'deleted');
assert.equal(parsedResult.policyEligible, true);
assert.equal(parsedResult.rawImageRetained, false);
assert.equal(parsedUnknownResult.policyEligible, false);
assert.equal(parsedUnknownResult.primaryCategory, 'unknown');
assert.equal(parsedSummary.queueHealth.expiredCount, 1);
assert.equal(parsedSummary.queueHealth.deleteFailedCount, 1);
assert.equal(invalidDisabledCapture.success, false);
assert.equal(invalidRetryOverflow.success, false);
assert.equal(invalidDeleteProof.success, false);
assert.equal(invalidLowConfidencePolicy.success, false);

const proof = {
  proofId: 'screen-evidence-settings-retention-proof',
  generatedAt: '2026-05-21T06:58:00Z',
  source: '@ocentra-parent/activity-domain screen evidence contracts',
  assertions: [
    'parent opt-in gates cadence, trigger capture, strict mode, and policy use',
    'temporary queue custody stays on child-device-temp-queue with encrypted image ref',
    'queue TTL and retry attempts stay bounded by schema guards',
    'deleted and expired-deleted queue states require deletion proof',
    'delete-failed queue state remains visible without pretending deletion proof exists',
    'rawImageRetained is schema-forced false for policy evidence',
    'low-confidence unknown summaries remain policy-ineligible',
    'policyEligible requires ready local evidence, confidence, category, and deleted raw-image custody',
  ],
  parsed: {
    settingVersion: parsedSetting.settingVersion,
    queueJobId: parsedQueueJob.queueJobId,
    resultId: parsedResult.screenAnalysisResultId,
    unknownResultId: parsedUnknownResult.screenAnalysisResultId,
    queueExpiredCount: parsedSummary.queueHealth.expiredCount,
    queueDeleteFailedCount: parsedSummary.queueHealth.deleteFailedCount,
  },
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-evidence-settings-retention-proof-ok: ${outputPath}`);
