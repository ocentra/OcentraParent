import { strict as assert } from 'node:assert';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

import { ActivityEvidenceKind } from '../../packages/activity-domain/dist/kinds.js';
import {
  ScreenAnalysisParentSettingSchema,
} from '../../packages/schema-domain/dist/screen-evidence-settings.js';
import { ScreenAnalysisQueueJobSchema } from '../../packages/screen-domain/dist/screen-evidence-queue.js';
import { ScreenEvidenceRecentSummarySchema } from '../../packages/screen-domain/dist/screen-evidence-read-model.js';
import { ScreenEvidenceSchemaVersion } from '../../packages/schema-domain/dist/screen-evidence-primitives.js';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'deletion-retention-custody');

const evidenceRef = {
  evidenceId: 'screen-retention-journal-entry',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:screen-retention-summary',
  uri: null,
};

const parentSetting = ScreenAnalysisParentSettingSchema.parse({
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
  changedByParentRef: 'parent-setting-screen-retention-proof',
  changedAt: '2026-06-04T11:25:00Z',
  settingVersion: 5,
  reason: 'parent enabled local screen AI proof with no raw image retention',
});

const successfulDeletionJob = queueJob({
  queueJobId: 'screen-retention-job-success',
  status: 'deleted',
  attemptCount: 1,
  deletionStatus: 'deleted',
  deletedAt: '2026-06-04T11:26:20Z',
  deletionProofRef: 'screen-delete-proof-success',
});

const expiredDeletionJob = queueJob({
  queueJobId: 'screen-retention-job-expired',
  status: 'expired',
  attemptCount: 2,
  deletionStatus: 'expiredDeleted',
  deletedAt: '2026-06-04T11:32:10Z',
  deletionProofRef: 'screen-delete-proof-expired',
});

const deleteFailedJob = queueJob({
  queueJobId: 'screen-retention-job-delete-failed',
  status: 'failed',
  attemptCount: 2,
  deletionStatus: 'deleteFailed',
  deletedAt: null,
  deletionProofRef: null,
  failureReason: 'os-delete-denied-visible-to-parent',
});

const parsedJobs = [
  ScreenAnalysisQueueJobSchema.parse(successfulDeletionJob),
  ScreenAnalysisQueueJobSchema.parse(expiredDeletionJob),
  ScreenAnalysisQueueJobSchema.parse(deleteFailedJob),
];

const readModel = ScreenEvidenceRecentSummarySchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  generatedAt: '2026-06-04T11:33:00Z',
  custodyState: 'child-device-query-store',
  limit: 10,
  returned: 0,
  queueHealth: {
    schemaVersion: ScreenEvidenceSchemaVersion,
    generatedAt: '2026-06-04T11:33:00Z',
    custodyState: 'child-device-query-store',
    pendingCount: 0,
    expiredCount: 1,
    deletePendingCount: 0,
    deleteFailedCount: 1,
    latestQueueJobId: deleteFailedJob.queueJobId,
    latestStatus: 'failed',
    lastSuccessfulAnalysisAt: '2026-06-04T11:26:00Z',
  },
  latestResultId: null,
  latestSummary: null,
  latestPrimaryCategory: null,
  latestConfidence: null,
  latestImageDeletionState: 'deleteFailed',
  latestPolicyEligible: null,
  evidence: [evidenceRef],
  results: [],
});

const invalidCases = {
  retentionWithoutSupportedMode: ScreenAnalysisParentSettingSchema.safeParse({
    ...parentSetting,
    retainRawImage: true,
  }).success,
  expiredWithoutDeletionProof: ScreenAnalysisQueueJobSchema.safeParse({
    ...expiredDeletionJob,
    deletedAt: null,
    deletionProofRef: null,
  }).success,
  deleteFailedPretendingDeleted: ScreenAnalysisQueueJobSchema.safeParse({
    ...deleteFailedJob,
    deletionStatus: 'deleted',
    deletedAt: '2026-06-04T11:32:30Z',
    deletionProofRef: 'screen-delete-proof-false-success',
  }).success,
  retryPastBound: ScreenAnalysisQueueJobSchema.safeParse({
    ...deleteFailedJob,
    attemptCount: 3,
  }).success,
};

assert.equal(parentSetting.retainRawImage, false);
assert.equal(parentSetting.deleteAfterSuccess, true);
assert.equal(parentSetting.deleteAfterExpiry, true);
assert.equal(parsedJobs[0].deletionStatus, 'deleted');
assert.equal(parsedJobs[1].deletionStatus, 'expiredDeleted');
assert.equal(parsedJobs[2].deletionStatus, 'deleteFailed');
assert.equal(readModel.queueHealth.expiredCount, 1);
assert.equal(readModel.queueHealth.deleteFailedCount, 1);
assert.deepEqual(invalidCases, {
  retentionWithoutSupportedMode: false,
  expiredWithoutDeletionProof: false,
  deleteFailedPretendingDeleted: false,
  retryPastBound: false,
});

const proof = {
  proof: 'screen-ai-deletion-retention-custody-proof',
  proofTier: 'P2_CONTRACT_PIPELINE_PROOF',
  generatedAt: '2026-06-04T11:33:30Z',
  artifacts: {
    proofSummary: relative(repoRoot, join(outputDir, 'proof-summary.json')),
    queueJobs: relative(repoRoot, join(outputDir, 'queue-jobs.json')),
    readModel: relative(repoRoot, join(outputDir, 'read-model.json')),
    invalidCases: relative(repoRoot, join(outputDir, 'invalid-cases.json')),
  },
  assertions: {
    parentSettingKeepsRawRetentionDisabled: parentSetting.retainRawImage === false,
    deleteAfterSuccessRequired: parentSetting.deleteAfterSuccess === true,
    deleteAfterExpiryRequired: parentSetting.deleteAfterExpiry === true,
    successfulQueueDeletionRequiresProof: parsedJobs[0].deletionProofRef !== null,
    expiredQueueDeletionRequiresProof: parsedJobs[1].deletionProofRef !== null,
    deleteFailureRemainsVisible:
      parsedJobs[2].deletionStatus === 'deleteFailed' && parsedJobs[2].deletionProofRef === null,
    readModelSurfacesExpiredAndDeleteFailedCounts:
      readModel.queueHealth.expiredCount === 1 && readModel.queueHealth.deleteFailedCount === 1,
    unsupportedRawRetentionRejected: invalidCases.retentionWithoutSupportedMode === false,
    expiredWithoutDeletionProofRejected: invalidCases.expiredWithoutDeletionProof === false,
    deleteFailureCannotPretendDeleted: invalidCases.deleteFailedPretendingDeleted === false,
    retryOverflowRejected: invalidCases.retryPastBound === false,
  },
  nonClaims: [
    'This is a combined pipeline contract proof for TTL, delete-failure visibility, and no-retention custody state.',
    'It does not claim a production background TTL sweeper or parent UI retention controls.',
    'Raw screenshot retention remains unsupported unless a future explicit retention mode and consent flow are added.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeJson('proof-summary.json', proof);
writeJson('queue-jobs.json', parsedJobs);
writeJson('read-model.json', readModel);
writeJson('invalid-cases.json', invalidCases);

console.log(`screen-ai-deletion-retention-custody-proof-ok:${relative(repoRoot, outputDir)}`);

function queueJob(overrides) {
  return {
    schemaVersion: ScreenEvidenceSchemaVersion,
    queueJobId: overrides.queueJobId,
    createdAt: '2026-06-04T11:26:00Z',
    notBefore: '2026-06-04T11:26:00Z',
    expiresAt: '2026-06-04T11:31:00Z',
    lastAttemptAt: '2026-06-04T11:31:30Z',
    captureReason: 'policyAmbiguity',
    captureScope: 'activeWindow',
    sourceId: 'screen-ai-deletion-retention-proof',
    adapterId: 'windows-screen-capture',
    deviceRef: 'local-dev-agent',
    localUserRef: 'local-user-screen-retention-proof',
    parentSettingRef: parentSetting.changedByParentRef,
    settingVersion: parentSetting.settingVersion,
    relatedEvidenceRefs: [evidenceRef],
    encryptedImageRef: `encrypted-${overrides.queueJobId}`,
    imageDigest: `sha256:${overrides.queueJobId}`,
    imageByteSize: 4096,
    imageFormat: 'png',
    status: overrides.status,
    attemptCount: overrides.attemptCount,
    maxRetryCount: 2,
    failureReason: overrides.failureReason ?? null,
    unavailableReason: null,
    deletionRequired: true,
    deletedAt: overrides.deletedAt,
    deletionStatus: overrides.deletionStatus,
    deletionProofRef: overrides.deletionProofRef,
    custodyState: 'child-device-temp-queue',
  };
}

function writeJson(fileName, value) {
  writeFileSync(join(outputDir, fileName), `${JSON.stringify(value, null, 2)}\n`);
}
