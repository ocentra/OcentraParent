import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const repoRoot = process.cwd();
const aiOutputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'activity-screen-ai-degraded-surface-proof');
const testResultRoot = resolve(repoRoot, 'test-results', 'activity-screen-ai-degraded-surface-proof');

await Promise.all([mkdir(aiOutputRoot, { recursive: true }), mkdir(testResultRoot, { recursive: true })]);

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/activity-domain',
  '--',
  'activity-surface',
]);

const { ActivityScreenReadModelSchema, ActivitySurfaceSchemaVersion } =
  await import('@ocentra-parent/activity-domain/activity-surface');

const evidenceRef = {
  evidenceId: 'activity-screen-ai-degraded-surface-proof-journal-ref',
  kind: 'journal-entry',
  digest: 'sha256:activity-screen-ai-degraded-surface-proof',
  uri: null,
};
const request = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-activity-screen-ai-proof',
  },
  requestedAt: '2026-06-06T15:50:00.000Z',
  rangeStart: '2026-06-06T15:00:00.000Z',
  rangeEnd: '2026-06-06T15:50:00.000Z',
};
const readModel = ActivityScreenReadModelSchema.parse({
  schemaVersion: ActivitySurfaceSchemaVersion,
  request,
  state: 'stale',
  generatedAt: '2026-06-06T15:50:02.000Z',
  summary: 'Activity Screen read model exposes local OCR unavailable and local VLM degraded states.',
  rows: [
    {
      rowId: 'activity-screen-ai-proof-ocr-unavailable',
      label: 'Local OCR unavailable for encrypted screen capture',
      deviceId: 'child-device-activity-screen-ai-proof',
      state: 'unavailable',
      totalMs: 0,
      foregroundMs: 0,
      backgroundMs: 0,
      captureReason: 'timedCadence',
      captureScope: 'activeWindow',
      capabilityStatus: 'modelUnavailable',
      queueJobId: 'activity-screen-ai-proof-ocr-job',
      modelRuntimeRef: 'windows-winrt-ocr-local-runtime',
      modelId: 'windows-winrt-ocr',
      providerKind: 'localOcr',
      promptOrTemplateVersion: 'screen-ocr-worker-winrt-v1',
      primaryCategory: null,
      confidence: 0,
      imageDeletionState: 'unavailableNoImage',
      rawImageRetained: false,
      policyEligible: false,
      imageDigest: 'sha256:activity-screen-ai-proof-ocr-unavailable-no-image',
      custodyState: 'unavailable',
      evidence: [evidenceRef],
      policyDecisionRef: null,
      policyAction: null,
      policyReasonCodes: [],
      parentRuleRefs: [],
      localModelRuntimeRefs: ['windows-winrt-ocr-local-runtime'],
      parentExplanationRefs: [],
      explanationReasons: ['local-ocr-unavailable'],
      deletionReasons: ['no-image-retained'],
    },
    {
      rowId: 'activity-screen-ai-proof-vlm-degraded',
      label: 'Local VLM degraded before policy eligibility',
      deviceId: 'child-device-activity-screen-ai-proof',
      state: 'ready',
      totalMs: 0,
      foregroundMs: 0,
      backgroundMs: 0,
      captureReason: 'timedCadence',
      captureScope: 'activeWindow',
      capabilityStatus: 'degraded',
      queueJobId: 'activity-screen-ai-proof-vlm-job',
      modelRuntimeRef: 'screen-vlm-worker-runtime',
      modelId: 'screen-vlm-worker-model',
      providerKind: 'localVision',
      promptOrTemplateVersion: 'screen-vlm-worker-v1',
      primaryCategory: 'unknown',
      confidence: 0.31,
      imageDeletionState: 'deleted',
      rawImageRetained: false,
      policyEligible: false,
      imageDigest: 'sha256:activity-screen-ai-proof-vlm-degraded-image',
      custodyState: 'child-device-query-store',
      evidence: [evidenceRef],
      policyDecisionRef: null,
      policyAction: null,
      policyReasonCodes: [],
      parentRuleRefs: ['screen-parent-rule-review'],
      localModelRuntimeRefs: ['screen-vlm-worker-runtime'],
      parentExplanationRefs: ['screen-vlm-degraded-explanation'],
      explanationReasons: ['low-confidence-local-vlm'],
      deletionReasons: ['screen-image-deleted'],
    },
  ],
});

const assertions = {
  exportedSchemaParsed: readModel.schemaVersion === ActivitySurfaceSchemaVersion,
  exposesOcrUnavailableRow:
    readModel.rows[0]?.providerKind === 'localOcr' &&
    readModel.rows[0]?.capabilityStatus === 'modelUnavailable' &&
    readModel.rows[0]?.state === 'unavailable' &&
    readModel.rows[0]?.primaryCategory === null,
  exposesVlmDegradedRow:
    readModel.rows[1]?.providerKind === 'localVision' &&
    readModel.rows[1]?.capabilityStatus === 'degraded' &&
    readModel.rows[1]?.modelId === 'screen-vlm-worker-model' &&
    readModel.rows[1]?.promptOrTemplateVersion === 'screen-vlm-worker-v1',
  noRawImagesRetained: readModel.rows.every((row) => row.rawImageRetained === false),
  noPolicyEligibilityForDegradedRows: readModel.rows.every((row) => row.policyEligible === false),
  deletionCustodyVisible:
    readModel.rows[0]?.imageDeletionState === 'unavailableNoImage' &&
    readModel.rows[1]?.imageDeletionState === 'deleted' &&
    readModel.rows[1]?.deletionReasons.includes('screen-image-deleted'),
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`Activity Screen AI degraded surface proof failed: ${JSON.stringify(assertions)}`);
}

const proof = {
  status: 'ok',
  proof: 'activity-screen-ai-degraded-surface-proof',
  generatedAt: new Date().toISOString(),
  artifactRoots: {
    aiPlan: aiOutputRoot,
    testResults: testResultRoot,
  },
  rowCount: readModel.rows.length,
  rowStates: readModel.rows.map((row) => ({
    rowId: row.rowId,
    providerKind: row.providerKind,
    capabilityStatus: row.capabilityStatus,
    state: row.state,
    modelRuntimeRef: row.modelRuntimeRef,
    modelId: row.modelId,
    promptOrTemplateVersion: row.promptOrTemplateVersion,
    primaryCategory: row.primaryCategory,
    imageDeletionState: row.imageDeletionState,
    rawImageRetained: row.rawImageRetained,
    policyEligible: row.policyEligible,
  })),
  assertions,
  nonClaims: [
    'This proof validates Activity Screen read-model contract visibility for degraded and unavailable local AI rows.',
    'It does not execute OCR or VLM inference, prove production model quality, render portal UI, grant policy authority, or dispatch enforcement.',
    'It does not retain raw screen images by default or claim live view/raw retention product behavior.',
  ],
};

await Promise.all([
  writeJson(resolve(aiOutputRoot, 'proof-summary.json'), proof),
  writeJson(resolve(testResultRoot, 'proof.json'), proof),
]);

console.log(`activity-screen-ai-degraded-surface-proof-ok:${resolve(aiOutputRoot, 'proof-summary.json')}`);

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    stdio: 'inherit',
    shell: false,
  });

  if (result.status !== 0) {
    throw new Error(`Command failed: ${[command, ...args].join(' ')}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
