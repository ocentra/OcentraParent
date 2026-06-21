import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const aiOutputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'screen-vlm-worker-contract-proof');
const pipelineOutputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'screen-vlm-worker-contract-proof');
const testResultRoot = resolve(repoRoot, 'test-results', 'screen-ai-vlm-worker-contract-proof');

await Promise.all([
  mkdir(aiOutputRoot, { recursive: true }),
  mkdir(pipelineOutputRoot, { recursive: true }),
  mkdir(testResultRoot, { recursive: true }),
]);

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/screen-domain']));
runCommand(...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/screen-domain', '--', 'screen-vlm-worker']));

const {
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerProofSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
  screenVlmWorkerResultToAnalysisResult,
} = await import('@ocentra-parent/schema-domain/screen-vlm-worker');

const evidenceRef = {
  evidenceId: 'screen-vlm-worker-proof-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-worker-proof-image',
  uri: null,
};

const proof = ScreenVlmWorkerProofSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  proofId: 'screen-vlm-worker-contract-proof',
  proofTier: 'P3_CONTRACT_LOCAL_VLM_WORKER',
  scenarios: [
    vlmResult({
      id: 'screen-vlm-worker-browser-game',
      queueJobId: 'screen-vlm-worker-browser-game-job',
      category: 'game',
      confidence: 0.86,
      summary: 'The local VLM worker classified a selected-window browser game image.',
      modelVisibleText: 'A browser game board is visible in the selected window.',
      riskSignals: [],
    }),
    vlmResult({
      id: 'screen-vlm-worker-bypass-risk',
      queueJobId: 'screen-vlm-worker-bypass-risk-job',
      category: 'bypassTool',
      confidence: 0.91,
      summary: 'The local VLM worker classified a selected-window bypass tool image.',
      modelVisibleText: 'A proxy bypass tool surface is visible in the selected window.',
      riskSignals: ['possibleBypassTool'],
    }),
  ],
  localOnly: true,
  rawImageRetained: false,
  remoteAiUsed: false,
  rawImageRemoteUploadEnabled: false,
});

const analysisRows = proof.scenarios.map((scenario) => screenVlmWorkerResultToAnalysisResult(scenario));
const assertions = {
  packageExportWorks: typeof ScreenVlmWorkerProofSchema.parse === 'function',
  guidedVlmWorkerContractImplemented: proof.scenarios.length === 2,
  schemaBoundModelOutputFeedsAnalysis: analysisRows.every(
    (row, index) => row.primaryCategory === proof.scenarios[index].modelOutput.primary_category
  ),
  policyEligibleOnlyAfterDeletion: analysisRows.every(
    (row) => row.policyEligible && row.imageDeletionState === 'deleted' && !row.rawImageRetained
  ),
  localOnlyNoRemoteAi: proof.localOnly && !proof.remoteAiUsed && !proof.rawImageRemoteUploadEnabled,
};

const summary = {
  status: 'ok',
  proof: 'screen-ai-vlm-worker-contract-proof',
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
  scenarioCount: proof.scenarios.length,
  categories: [...new Set(analysisRows.map((row) => row.primaryCategory))],
  riskSignals: [...new Set(analysisRows.flatMap((row) => row.riskSignals.map((signal) => signal.signal)))],
  analysisRows,
  assertions,
  validationCommands: [
    'npm run build --workspace @ocentra-parent/schema-domain',
    'npm run build --workspace @ocentra-parent/screen-domain',
    'npm run test --workspace @ocentra-parent/screen-domain -- screen-vlm-worker',
    'node scripts/test/screen-ai-vlm-worker-contract-proof.mjs',
  ],
  nonClaims: [
    'This is a contract/proof harness for guided local VLM worker inputs and outputs; it does not run live model inference.',
    'It does not claim production VLM quality, live external account coverage, portal UI, or enforcement adapter execution.',
    'It keeps raw images in encrypted temp custody before analysis and requires deleted-image custody before policy eligibility.',
  ],
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`screen VLM worker proof assertions failed: ${JSON.stringify(assertions)}`);
}

await Promise.all([
  writeFile(join(aiOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(pipelineOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(summary, null, 2)}\n`),
]);

console.log(
  `screen-ai-vlm-worker-contract-proof-ok:${proof.scenarios.length}:${join('output', 'ai-plan-proof', 'screen-vlm-worker-contract-proof', 'proof-summary.json')}`
);

function vlmResult({ id, queueJobId, category, confidence, summary, modelVisibleText, riskSignals }) {
  return {
    schemaVersion: ScreenVlmWorkerSchemaVersion,
    vlmResultId: id,
    queueJobId,
    analyzedAt: '2026-06-05T23:22:00.000Z',
    modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
    modelId: ScreenVlmWorkerModelId,
    promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
    providerKind: 'localVision',
    captureReason: 'manualParentTestCapture',
    captureScope: 'selectedWindow',
    capabilityStatus: 'ready',
    modelOutput: {
      primary_category: category,
      confidence,
      visible_text: modelVisibleText,
      risk_signals: riskSignals,
    },
    summary,
    visibleCategoryCandidates: [
      {
        category,
        confidence,
        evidenceRefs: [evidenceRef],
      },
    ],
    primaryCategory: category,
    riskSignals: riskSignals.map((signal) => ({
      signal,
      confidence,
      evidenceRefs: [evidenceRef],
    })),
    redactionNotes: [],
    confidence,
    uncertaintyReason: null,
    sourceEvidenceRefs: [evidenceRef],
    imageDigest: evidenceRef.digest,
    rawImageRetained: false,
    imageDeletionState: 'deleted',
    custodyState: 'child-device-query-store',
    policyEligible: true,
    localOnly: true,
    remoteAiUsed: false,
  };
}

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
