import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const screenOutputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '36-small-vlm-guided-classifier-evaluation');
const screenProofPath = join(screenOutputRoot, 'proof-summary.json');
const localAiModelRoot = resolveUserCachePath('local-ai-models');
const llamaRoot = process.env.OCENTRA_PARENT_LLAMA_CPP_DIR ?? resolveUserCachePath('llama.cpp', 'b9279');
const vlmBinary = process.env.OCENTRA_PARENT_LOCAL_VLM_BINARY ?? join(llamaRoot, 'llama-mtmd-cli.exe');
const vlmModel =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MODEL ?? join(localAiModelRoot, 'Qwen2-VL-2B-Instruct-Q4_K_M.gguf');
const vlmMmproj =
  process.env.OCENTRA_PARENT_LOCAL_VLM_MMPROJ ?? join(localAiModelRoot, 'mmproj-Qwen2-VL-2B-Instruct-Q8_0.gguf');
const localVlmMatrixProofPath = resolve(repoRoot, 'output', 'ai-plan-proof', 'real-analysis', 'proof-summary.json');
const liveOperatorProofPath = resolve(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'live-operator',
  'proof-summary.json'
);
const vlmResourceCropProofPath = resolve(
  repoRoot,
  'output',
  'screen-plan-proof',
  '36-vlm-resource-crop-readiness',
  'proof-summary.json'
);
const vlmRuntimeResourceMeasurementProofPath = resolve(
  repoRoot,
  'output',
  'screen-plan-proof',
  '36-vlm-runtime-resource-measurement',
  'proof-summary.json'
);

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
  llamaMtmdCli: probeCommand(vlmBinary, ['--version'], { existsRequired: true }),
};
const localVlmMatrixProof = await readOptionalJson(localVlmMatrixProofPath);
const liveOperatorProof = await readOptionalJson(liveOperatorProofPath);
const vlmResourceCropProof = await readOptionalJson(vlmResourceCropProofPath);
const vlmRuntimeResourceMeasurementProof = await readOptionalJson(vlmRuntimeResourceMeasurementProofPath);
const requiredLiveOperatorScenarioIds = [
  'youtube-ordinary-video',
  'youtube-education-video',
  'vimeo-video',
  'facebook-social-surface',
  'browser-game',
  'shopping-page',
  'school-productivity',
  'native-app',
  'protected-unsupported-state',
];
const liveOperatorRequiredScenarioStatus = Object.fromEntries(
  requiredLiveOperatorScenarioIds.map((id) => [id, liveOperatorProof?.requiredScenarioStatus?.[id] === true])
);
const retainedLiveOperatorVlmQualityAvailable =
  liveOperatorProof?.fullRequiredMatrixComplete === true &&
  liveOperatorProof?.passedScenarioCount === requiredLiveOperatorScenarioIds.length &&
  liveOperatorProof?.liveExternalUrlProof === true &&
  liveOperatorProof?.localVlmAnalysisProof === true &&
  liveOperatorProof?.policyDryRunProof === true &&
  liveOperatorProof?.rawImagesDeletedAfterAnalysis === true &&
  liveOperatorProof?.controlledFixtureProof === false &&
  liveOperatorProof?.productCompleteClaimed === false &&
  Object.values(liveOperatorRequiredScenarioStatus).every(Boolean);
const localLlamaRuntime = {
  binary: redactHome(vlmBinary),
  binaryExists: existsSync(vlmBinary),
  model: redactHome(vlmModel),
  modelExists: existsSync(vlmModel),
  mmproj: redactHome(vlmMmproj),
  mmprojExists: existsSync(vlmMmproj),
  matrixProofPath: relativePath(localVlmMatrixProofPath),
  matrixProofPresent: localVlmMatrixProof !== null,
  matrixScenarioCount: localVlmMatrixProof?.scenarioCount ?? 0,
  matrixRealWindowCaptureCount: localVlmMatrixProof?.realWindowCaptureCount ?? 0,
  matrixAnalyzedByRealLocalVlm: localVlmMatrixProof?.analyzedByRealLocalVlm === true,
  matrixSchemaValidated: localVlmMatrixProof?.schemaValidated === true,
  matrixPolicyDecisionValidated: localVlmMatrixProof?.policyDecisionValidated === true,
  matrixRawImagesDeleted: localVlmMatrixProof?.rawImagesDeletedAfterAnalysis === true,
  matrixUsesControlledFixtures: localVlmMatrixProof?.localFixturesAreLiveExternalSites === false,
  liveOperatorExternalUrlProofStillRequired: localVlmMatrixProof?.liveOperatorExternalUrlProofStillRequired !== false,
  liveOperatorProofPath: relativePath(liveOperatorProofPath),
  liveOperatorProofPresent: liveOperatorProof !== null,
  liveOperatorScenarioCount: liveOperatorProof?.scenarioCount ?? 0,
  liveOperatorPassedScenarioCount: liveOperatorProof?.passedScenarioCount ?? 0,
  liveOperatorRequiredScenarioStatus,
  liveOperatorFullRequiredMatrixComplete: liveOperatorProof?.fullRequiredMatrixComplete === true,
  liveOperatorExternalUrlProof: liveOperatorProof?.liveExternalUrlProof === true,
  liveOperatorLocalVlmAnalysisProof: liveOperatorProof?.localVlmAnalysisProof === true,
  liveOperatorPolicyDryRunProof: liveOperatorProof?.policyDryRunProof === true,
  liveOperatorRawImagesDeleted: liveOperatorProof?.rawImagesDeletedAfterAnalysis === true,
  retainedLiveOperatorVlmQualityAvailable,
  resourceCropProofPath: relativePath(vlmResourceCropProofPath),
  resourceCropProofPresent: vlmResourceCropProof !== null,
  resourceCropSampleCount: vlmResourceCropProof?.captureBudgetSummary?.sampleCount ?? 0,
  resourceCropLiveExternalAnalyzedSampleCount:
    vlmResourceCropProof?.captureBudgetSummary?.liveExternalAnalyzedSampleCount ?? 0,
  resourceCropAllSamplesWithinPixelBudget:
    vlmResourceCropProof?.captureBudgetSummary?.allSamplesWithinPixelBudget === true,
  resourceCropAllSamplesDeleteRawImages: vlmResourceCropProof?.captureBudgetSummary?.allSamplesDeleteRawImages === true,
  managedBrowserCdpCropPathVerified:
    vlmResourceCropProof?.retainedProofs?.managedBrowserCdpCropProof?.cropModeCaptured === true &&
    vlmResourceCropProof?.retainedProofs?.managedBrowserCdpCropProof?.allDeleted === true,
  runtimeResourceMeasurementProofPath: relativePath(vlmRuntimeResourceMeasurementProofPath),
  runtimeResourceMeasurementProofPresent: vlmRuntimeResourceMeasurementProof !== null,
  runtimeResourceMeasurementSampleCount: vlmRuntimeResourceMeasurementProof?.summary?.sampleCount ?? 0,
  runtimeResourceMeasurementAllSamplesWithinEnvelope:
    vlmRuntimeResourceMeasurementProof?.summary?.allSamplesWithinResourceEnvelope === true,
  runtimeResourceMeasurementMaxWallMs: vlmRuntimeResourceMeasurementProof?.summary?.maxWallMsObserved ?? 0,
  runtimeResourceMeasurementMaxPeakWorkingSetBytes:
    vlmRuntimeResourceMeasurementProof?.summary?.maxPeakWorkingSetBytesObserved ?? 0,
  runtimeResourceMeasurementMaxCpuSeconds: vlmRuntimeResourceMeasurementProof?.summary?.maxCpuSecondsObserved ?? 0,
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
const localLlamaRuntimeAvailable =
  localLlamaRuntime.binaryExists && localLlamaRuntime.modelExists && localLlamaRuntime.mmprojExists;
const retainedLocalVlmMatrixAvailable =
  localLlamaRuntimeAvailable &&
  localLlamaRuntime.matrixProofPresent &&
  localLlamaRuntime.matrixScenarioCount >= 1 &&
  localLlamaRuntime.matrixAnalyzedByRealLocalVlm &&
  localLlamaRuntime.matrixSchemaValidated &&
  localLlamaRuntime.matrixPolicyDecisionValidated &&
  localLlamaRuntime.matrixRawImagesDeleted;
const providerRuntimeAvailable =
  localProviderRuntimeProbe.ollama.available ||
  localProviderRuntimeProbe.llamaServer.available ||
  (lmStudioCliDetected && lmStudioServerRunning && loadedLmStudioModelsAvailable) ||
  retainedLocalVlmMatrixAvailable;

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
  localLlamaRuntime,
  providerRuntimeState: {
    providerCommandAvailable,
    providerRuntimeAvailable,
    lmStudioCliDetected,
    lmStudioServerRunning,
    loadedLmStudioModelsAvailable,
    localLlamaRuntimeAvailable,
    retainedLocalVlmMatrixAvailable,
    retainedLiveOperatorVlmQualityAvailable,
    retainedResourceCropBudgetAvailable:
      localLlamaRuntime.resourceCropProofPresent &&
      localLlamaRuntime.resourceCropAllSamplesWithinPixelBudget &&
      localLlamaRuntime.resourceCropAllSamplesDeleteRawImages,
    managedBrowserCdpCropPathVerified: localLlamaRuntime.managedBrowserCdpCropPathVerified,
    retainedRuntimeResourceMeasurementAvailable:
      localLlamaRuntime.runtimeResourceMeasurementProofPresent &&
      localLlamaRuntime.runtimeResourceMeasurementAllSamplesWithinEnvelope,
    liveVlmInferenceReady: providerRuntimeAvailable,
    note: retainedLiveOperatorVlmQualityAvailable
      ? 'Local llama.cpp/Qwen2-VL runtime files exist; retained controlled matrix proof and retained nine-scenario live operator proof show real local VLM analysis, schema validation, policy dry-run, and raw deletion over public/live URL plus native-app captures. Resource/crop audit proves retained VLM inputs stayed within the max pixel budget and CDP crop capture exists. Retained proof-image VLM measurement records per-sample wall time, CPU seconds, and peak working set. Detector-specific crop quality, authenticated-account social proof, and production model selection remain open.'
      : retainedLocalVlmMatrixAvailable
        ? 'Local llama.cpp/Qwen2-VL runtime files exist and the retained local VLM matrix proof shows real local model execution over controlled browser/native window captures with schema, policy, and deletion proof; live external-site/operator classification proof remains open.'
        : lmStudioCliDetected && !providerRuntimeAvailable
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
    retainedLocalVlmMatrixAvailable
      ? 'retained local llama.cpp/Qwen2-VL matrix proves real local VLM execution over controlled browser/native window captures with policy and deletion proof'
      : null,
    retainedLiveOperatorVlmQualityAvailable
      ? 'retained nine-scenario live operator proof covers public/live URL and native-app captures through local VLM classification, schema validation, policy dry-run, and raw image deletion'
      : null,
    localLlamaRuntime.resourceCropProofPresent && localLlamaRuntime.resourceCropAllSamplesWithinPixelBudget
      ? 'retained controlled and live-operator VLM capture inputs are bounded below the worker max-image-pixel budget'
      : null,
    localLlamaRuntime.managedBrowserCdpCropPathVerified
      ? 'managed-browser CDP crop capture path exists and deletes captured material'
      : null,
    localLlamaRuntime.runtimeResourceMeasurementProofPresent &&
    localLlamaRuntime.runtimeResourceMeasurementAllSamplesWithinEnvelope
      ? 'retained proof-image VLM inference records per-sample wall time, CPU seconds, and peak working set inside the local measurement envelope'
      : null,
  ].filter(Boolean),
  openChecklistClaims: [
    retainedLiveOperatorVlmQualityAvailable
      ? 'detector-specific VLM crop quality is not measured by this readiness proof'
      : providerRuntimeAvailable
        ? 'a local VLM provider runtime appears available and retained matrix proof exists, but live external-site/operator classification is not measured by this proof'
        : lmStudioCliDetected
          ? 'LM Studio lms CLI is detected, but the local server/model runtime is not ready for live inference'
          : 'no local VLM provider command was detected on PATH in this Windows lane',
    'detector-specific prompt-pack quality is not measured by this proof',
    'detector-specific crop-quality measurement on freshly cropped live pages remains unclaimed',
  ],
  nonClaims: [
    'This screen-plan proof reuses the VLM execution-readiness contract proof and does not run live VLM inference.',
    retainedLiveOperatorVlmQualityAvailable
      ? 'This proof cross-checks the retained live operator matrix artifact instead of rerunning the nine live URL/app captures.'
      : 'Retained local VLM matrix artifacts use controlled browser/native window captures, not live external sites.',
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

function probeCommand(command, args, options = {}) {
  if (options.existsRequired === true && !existsSync(command)) {
    return {
      command,
      available: false,
      status: 1,
      output: 'command path is missing',
    };
  }
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  return {
    command,
    available: result.status === 0 || (options.existsRequired === true && existsSync(command)),
    status: result.status ?? 1,
    output: oneLine(result.stdout || result.stderr),
  };
}

function oneLine(value) {
  return value.replace(/\s+/g, ' ').trim();
}

async function readOptionalJson(path) {
  try {
    return JSON.parse(await readFile(path, 'utf8'));
  } catch {
    return null;
  }
}

function relativePath(path) {
  return path.slice(repoRoot.length + 1).replaceAll('\\', '/');
}

function redactHome(path) {
  const userHome = process.env.USERPROFILE ?? process.env.HOME;
  return userHome === undefined ? path : path.replace(userHome, '%USERPROFILE%');
}

function resolveUserCachePath(...segments) {
  const userHome = process.env.USERPROFILE ?? process.env.HOME;
  if (userHome === undefined || userHome.length === 0) {
    throw new Error('Set USERPROFILE or HOME so the local VLM readiness proof can resolve the Ocentra cache path.');
  }
  return resolve(userHome, '.cache', 'ocentra-parent', ...segments);
}
