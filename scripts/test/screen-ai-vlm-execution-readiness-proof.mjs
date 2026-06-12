import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const aiOutputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'screen-vlm-execution-readiness-proof');
const pipelineOutputRoot = resolve(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'screen-vlm-execution-readiness-proof'
);
const testResultRoot = resolve(repoRoot, 'test-results', 'screen-ai-vlm-execution-readiness-proof');

await Promise.all([
  mkdir(aiOutputRoot, { recursive: true }),
  mkdir(pipelineOutputRoot, { recursive: true }),
  mkdir(testResultRoot, { recursive: true }),
]);

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'screen-vlm-execution-readiness',
  ])
);

const {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
} = await import('@ocentra-parent/screen-domain/screen-vlm-worker');
const {
  ScreenVlmExecutionReadinessProofSchema,
  ScreenVlmExecutionReadinessProofTier,
  ScreenVlmExecutionReadinessSchemaVersion,
  screenVlmCompletedStatusFromResult,
  screenVlmManualRequiredStatus,
  screenVlmQueueHandoffFromJob,
  screenVlmQueuedStatusFromHandoff,
} = await import('@ocentra-parent/screen-domain/screen-vlm-execution-readiness');

const evidenceRef = {
  evidenceId: 'screen-vlm-execution-readiness-proof-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-execution-readiness-proof-image',
  uri: null,
};

const job = ScreenVlmWorkerJobSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-execution-readiness-proof-job',
  createdAt: '2026-06-06T00:25:00.000Z',
  captureReason: 'manualParentTestCapture',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: evidenceRef.digest,
  encryptedImageRef: 'encrypted-temp-screen-vlm-execution-readiness-proof-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Queue this encrypted local capture for a VLM worker and publish execution-readiness status only.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
});

const result = ScreenVlmWorkerResultSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-execution-readiness-proof-result',
  queueJobId: job.queueJobId,
  analyzedAt: '2026-06-06T00:25:05.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: job.captureReason,
  captureScope: job.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'productivity',
    confidence: 0.82,
    visible_text: 'A productivity document editor is visible in the selected window.',
    risk_signals: [],
  },
  summary: 'The local VLM worker result classified the selected-window capture as productivity.',
  visibleCategoryCandidates: [
    {
      category: 'productivity',
      confidence: 0.82,
      evidenceRefs: [evidenceRef],
    },
  ],
  primaryCategory: 'productivity',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.82,
  uncertaintyReason: null,
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: evidenceRef.digest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  localOnly: true,
  remoteAiUsed: false,
});

const handoff = screenVlmQueueHandoffFromJob({
  job,
  handoffId: 'screen-vlm-execution-readiness-proof-handoff',
  queuedAt: '2026-06-06T00:25:01.000Z',
  acceptedAt: '2026-06-06T00:25:02.000Z',
  statusReason: 'The encrypted selected-window capture is ready for local VLM worker handoff.',
});
const queuedStatus = screenVlmQueuedStatusFromHandoff({
  handoff,
  statusId: 'screen-vlm-execution-readiness-proof-queued',
  updatedAt: '2026-06-06T00:25:02.000Z',
});
const completedStatus = screenVlmCompletedStatusFromResult({
  result,
  statusId: 'screen-vlm-execution-readiness-proof-completed',
});
const manualStatus = screenVlmManualRequiredStatus({
  queueJobId: 'screen-vlm-execution-readiness-proof-manual-job',
  statusId: 'screen-vlm-execution-readiness-proof-manual',
  updatedAt: '2026-06-06T00:25:03.000Z',
  statusReason: 'The local VLM runtime is unavailable, so manual review is still required.',
  degradedReasons: ['local-vlm-runtime-unavailable'],
});

const proof = ScreenVlmExecutionReadinessProofSchema.parse({
  schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
  proofId: 'screen-vlm-execution-readiness-proof',
  proofTier: ScreenVlmExecutionReadinessProofTier,
  handoffs: [handoff],
  statusRows: [queuedStatus, completedStatus, manualStatus],
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
  liveModelExecutionClaimed: false,
  productionVlmQualityClaimed: false,
  portalRuntimeClaimed: false,
  enforcementClaimed: false,
});

const assertions = {
  packageExportWorks: typeof ScreenVlmExecutionReadinessProofSchema.parse === 'function',
  queueHandoffUsesEncryptedTempCustody: proof.handoffs.every(
    (row) => row.job.custodyState === 'child-device-temp-queue' && row.queueAccepted
  ),
  completedStatusRequiresDeletedQueryStoreResult: proof.statusRows.some(
    (row) =>
      row.status === 'completed' &&
      row.result?.imageDeletionState === 'deleted' &&
      row.custodyState === 'child-device-query-store'
  ),
  degradedStatusMakesNoExecutionClaim: proof.statusRows.some(
    (row) =>
      row.status === 'manual-required' &&
      row.result === null &&
      !row.nonClaims.liveModelExecutionClaimed &&
      row.degradedReasons.length > 0
  ),
  proofMakesOnlyLocalNonClaims:
    proof.localOnly &&
    !proof.remoteAiUsed &&
    !proof.rawImageRetained &&
    !proof.liveModelExecutionClaimed &&
    !proof.productionVlmQualityClaimed &&
    !proof.portalRuntimeClaimed &&
    !proof.enforcementClaimed,
};

const summary = {
  status: 'ok',
  proof: 'screen-ai-vlm-execution-readiness-proof',
  proofTier: proof.proofTier,
  generatedAt: new Date().toISOString(),
  artifactRoots: {
    aiPlan: aiOutputRoot,
    screenAiPipelinePlan: pipelineOutputRoot,
    testResults: testResultRoot,
  },
  constants: {
    modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
    modelId: ScreenVlmWorkerModelId,
    promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
    maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  },
  handoffCount: proof.handoffs.length,
  statusRows: proof.statusRows.map((row) => ({
    statusId: row.statusId,
    queueJobId: row.queueJobId,
    status: row.status,
    custodyState: row.custodyState,
    queueAccepted: row.queueAccepted,
    hasResult: row.result !== null,
    degradedReasons: row.degradedReasons,
    nonClaims: row.nonClaims,
  })),
  assertions,
  validationCommands: [
    'npm run build --workspace @ocentra-parent/activity-domain',
    'npm run test --workspace @ocentra-parent/activity-domain -- screen-vlm-execution-readiness',
    'node scripts/test/screen-ai-vlm-execution-readiness-proof.mjs',
  ],
  nonClaims: [
    'This proof models execution readiness, status rows, and queue handoff only; it does not run live VLM inference.',
    'It does not claim production VLM quality, portal runtime rendering, enforcement adapter execution, or end-to-end policy action.',
    'It requires local-only handling, encrypted temp queue custody before execution, and deleted query-store custody before completed status can feed policy.',
  ],
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`screen VLM execution readiness proof assertions failed: ${JSON.stringify(assertions)}`);
}

await Promise.all([
  writeFile(join(aiOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(pipelineOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(summary, null, 2)}\n`),
]);

console.log(
  `screen-ai-vlm-execution-readiness-proof-ok:${proof.statusRows.length}:${join('output', 'ai-plan-proof', 'screen-vlm-execution-readiness-proof', 'proof-summary.json')}`
);

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
