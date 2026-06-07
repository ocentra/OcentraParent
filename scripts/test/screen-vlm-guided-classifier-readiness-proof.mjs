import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const screenOutputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '36-small-vlm-guided-classifier-evaluation');
const screenProofPath = join(screenOutputRoot, 'proof-summary.json');

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/activity-domain',
  '--',
  'screen-vlm-execution-readiness',
]);

const {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
  screenVlmWorkerPromptIsOpenEnded,
} = await import('@ocentra-parent/activity-domain/screen-vlm-worker');
const {
  ScreenVlmExecutionReadinessProofSchema,
  ScreenVlmExecutionReadinessProofTier,
  ScreenVlmExecutionReadinessSchemaVersion,
  screenVlmCompletedStatusFromResult,
  screenVlmManualRequiredStatus,
  screenVlmQueueHandoffFromJob,
  screenVlmQueuedStatusFromHandoff,
} = await import('@ocentra-parent/activity-domain/screen-vlm-execution-readiness');

const evidenceRef = {
  evidenceId: 'screen-vlm-guided-classifier-readiness-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-guided-classifier-readiness-image',
  uri: null,
};
const job = ScreenVlmWorkerJobSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-guided-classifier-readiness-job',
  createdAt: '2026-06-06T23:45:00.000Z',
  captureReason: 'manualParentTestCapture',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: evidenceRef.digest,
  encryptedImageRef: 'encrypted-temp-screen-vlm-guided-classifier-readiness-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Use the guided screen classifier template. Return schema-valid category JSON only.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
});
const result = ScreenVlmWorkerResultSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-guided-classifier-readiness-result',
  queueJobId: job.queueJobId,
  analyzedAt: '2026-06-06T23:45:05.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: job.captureReason,
  captureScope: job.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'productivity',
    confidence: 0.8,
    visible_text: 'Guided classifier readiness row for a productivity screen.',
    risk_signals: [],
  },
  summary: 'Guided classifier readiness row for a productivity screen.',
  visibleCategoryCandidates: [{ category: 'productivity', confidence: 0.8, evidenceRefs: [evidenceRef] }],
  primaryCategory: 'productivity',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.8,
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
  handoffId: 'screen-vlm-guided-classifier-readiness-handoff',
  queuedAt: '2026-06-06T23:45:01.000Z',
  acceptedAt: '2026-06-06T23:45:02.000Z',
  statusReason: 'The encrypted selected-window capture is ready for guided local VLM classification.',
});
const queuedStatus = screenVlmQueuedStatusFromHandoff({
  handoff,
  statusId: 'screen-vlm-guided-classifier-readiness-queued',
  updatedAt: '2026-06-06T23:45:02.000Z',
});
const completedStatus = screenVlmCompletedStatusFromResult({
  result,
  statusId: 'screen-vlm-guided-classifier-readiness-completed',
});
const manualStatus = screenVlmManualRequiredStatus({
  queueJobId: 'screen-vlm-guided-classifier-readiness-manual-job',
  statusId: 'screen-vlm-guided-classifier-readiness-manual',
  updatedAt: '2026-06-06T23:45:03.000Z',
  statusReason: 'The local VLM runtime is unavailable, so manual review is required.',
  degradedReasons: ['local-vlm-runtime-unavailable'],
});
const openEndedPromptRejected =
  ScreenVlmWorkerJobSchema.safeParse({
    ...job,
    queueJobId: 'screen-vlm-guided-classifier-open-ended-job',
    prompt: 'Describe the screen in detail.',
  }).success === false;
const readiness = ScreenVlmExecutionReadinessProofSchema.parse({
  schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
  proofId: 'screen-vlm-guided-classifier-readiness-proof',
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
const statusRows = readiness.statusRows.map((row) => ({
  statusId: row.statusId,
  queueJobId: row.queueJobId,
  status: row.status,
  custodyState: row.custodyState,
  queueAccepted: row.queueAccepted,
  hasResult: row.result !== null,
  degradedReasons: row.degradedReasons,
  nonClaims: row.nonClaims,
}));
const assertions = {
  sourceReadinessProofPassed: true,
  localOnly: readiness.localOnly && !readiness.remoteAiUsed && !readiness.rawImageRetained,
  noLiveModelExecutionClaim: statusRows.every((row) => row.nonClaims?.liveModelExecutionClaimed === false),
  manualRequiredCovered: statusRows.some((row) => row.status === 'manual-required' && row.degradedReasons.length > 0),
  completedRowsRequireDeletedQueryStore: readiness.statusRows.some(
    (row) =>
      row.status === 'completed' &&
      row.result?.imageDeletionState === 'deleted' &&
      row.custodyState === 'child-device-query-store'
  ),
  maxImagePixelsBounded: job.maxImagePixels <= ScreenVlmWorkerMaxImagePixels,
  guidedTemplateVersionPinned: job.promptOrTemplateVersion === ScreenVlmWorkerTemplateVersion,
  openEndedPromptsRejected: openEndedPromptRejected && !screenVlmWorkerPromptIsOpenEnded(job.prompt),
};
const localProviderRuntimeProbe = {
  ollama: probeCommand('ollama', ['list']),
  lmStudioCliVersion: probeCommand('lms', ['version']),
  lmStudioCliStatus: probeCommand('lms', ['status']),
  lmStudioServerStatus: probeCommand('lms', ['server', 'status']),
  lmStudioLoadedModels: probeCommand('lms', ['ps']),
  legacyLmStudioCommand: probeCommand('lmstudio', ['--version']),
  llamaServer: probeCommand('llama-server', ['--version']),
};
const providerCommandAvailable = Object.values(localProviderRuntimeProbe).some((probe) => probe.available);
const lmStudioCliDetected = localProviderRuntimeProbe.lmStudioCliVersion.available;
const lmStudioStatusText =
  `${localProviderRuntimeProbe.lmStudioCliStatus.output} ${localProviderRuntimeProbe.lmStudioServerStatus.output}`.toLowerCase();
const lmStudioServerRunning =
  localProviderRuntimeProbe.lmStudioServerStatus.available &&
  !lmStudioStatusText.includes('not running') &&
  !lmStudioStatusText.includes('server: off');
const loadedLmStudioModelsAvailable = localProviderRuntimeProbe.lmStudioLoadedModels.available;
const providerRuntimeAvailable =
  localProviderRuntimeProbe.ollama.available ||
  localProviderRuntimeProbe.llamaServer.available ||
  (lmStudioCliDetected && lmStudioServerRunning && loadedLmStudioModelsAvailable);

const screenProof = {
  proof: 'screen-vlm-guided-classifier-readiness-proof',
  generatedAt: new Date().toISOString(),
  proofTier: 'P3_CONTRACT_LOCAL_VLM_READINESS',
  modelConstants: {
    modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
    modelId: ScreenVlmWorkerModelId,
    promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
    maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  },
  statusRows,
  localProviderRuntimeProbe,
  providerRuntimeState: {
    providerCommandAvailable,
    providerRuntimeAvailable,
    lmStudioCliDetected,
    lmStudioServerRunning,
    loadedLmStudioModelsAvailable,
    liveVlmInferenceReady: providerRuntimeAvailable,
    note:
      lmStudioCliDetected && !providerRuntimeAvailable
        ? 'LM Studio CLI is installed, but this lane did not find a running server or loaded local VLM model; live VLM quality proof remains open.'
        : 'No local VLM provider runtime is ready for live inference in this proof.',
  },
  assertions,
  validationCommands: [
    'npm run build --workspace @ocentra-parent/activity-domain',
    'npm run test --workspace @ocentra-parent/activity-domain -- screen-vlm-execution-readiness',
    'node scripts/test/screen-vlm-guided-classifier-readiness-proof.mjs',
  ],
  completedChecklistClaims: [
    'guided VLM worker template/version is pinned',
    'image pixel budget is bounded for local worker handoff',
    'open-ended screen description prompts are rejected before worker handoff',
    'manual-required behavior is represented when runtime is unavailable',
    'completed status rows require deleted query-store custody before policy eligibility',
  ],
  openChecklistClaims: [
    providerRuntimeAvailable
      ? 'a local VLM provider runtime appears available, but live model execution is not run by this proof'
      : lmStudioCliDetected
        ? 'LM Studio lms CLI is detected, but the local server/model runtime is not ready for live inference'
        : 'no local VLM provider command was detected on PATH in this Windows lane',
    'detector-specific prompt-pack quality is not measured by this proof',
    'real crop extraction and visual classifier quality are not measured by this proof',
    'CPU/GPU/memory/runtime measurements remain unclaimed',
  ],
  nonClaims: [
    'This screen-plan proof reuses the VLM execution-readiness contract proof and does not run live VLM inference.',
    'This proof does not claim production model quality, portal runtime rendering, enforcement, or final screen-AI pipeline completion.',
    'This proof does not upload raw screenshots or retain raw images.',
  ],
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`screen VLM guided classifier readiness assertions failed: ${JSON.stringify(assertions)}`);
}

await mkdir(screenOutputRoot, { recursive: true });
await writeFile(screenProofPath, `${JSON.stringify(screenProof, null, 2)}\n`);
console.log(`screen-vlm-guided-classifier-readiness-proof-ok:${screenProofPath}`);

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

function probeCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  return {
    command,
    available: result.status === 0,
    status: result.status ?? 1,
    output: oneLine(result.stdout || result.stderr),
  };
}

function oneLine(value) {
  return value.replace(/\s+/g, ' ').trim();
}
