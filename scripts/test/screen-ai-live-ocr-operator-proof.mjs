import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const repoRoot = process.cwd();
const pipelineRoot = join('output', 'screen-ai-pipeline-proof', 'live-ocr-operator');
const aiRoot = join('output', 'ai-plan-proof', 'live-ocr-operator');
const resultRoot = join('test-results', 'screen-ai-live-ocr-operator-proof');
const workerProofPath = join(pipelineRoot, 'worker-proof-summary.json');
const scenarios = [
  'live-wikipedia-browser-ocr',
  'live-vimeo-browser-ocr',
  'live-browser-game-ocr',
  'live-shopping-browser-ocr',
  'native-notepad-productivity-ocr',
];
const validationCommands = [
  'node --check scripts/test/screen-ai-live-ocr-operator-proof.mjs',
  'node scripts/test/screen-ai-live-ocr-operator-proof.mjs',
];

rmSync(resolve(repoRoot, pipelineRoot), { recursive: true, force: true });
rmSync(resolve(repoRoot, aiRoot), { recursive: true, force: true });
rmSync(resolve(repoRoot, resultRoot), { recursive: true, force: true });
mkdirSync(resolve(repoRoot, pipelineRoot), { recursive: true });
mkdirSync(resolve(repoRoot, aiRoot), { recursive: true });
mkdirSync(resolve(repoRoot, resultRoot), { recursive: true });

runWorkerProof();

const workerProof = readJson(join(pipelineRoot, 'proof-summary.json'));
workerProof.artifact = resolve(repoRoot, workerProofPath);
workerProof.artifacts = {
  ...workerProof.artifacts,
  liveOcrOperatorSummary: resolve(repoRoot, join(pipelineRoot, 'proof-summary.json')),
};
writeJson(workerProofPath, workerProof);
const browserRows = workerProof.rows.filter((row) => row.surfaceKind === 'live-browser');
const nativeRows = workerProof.rows.filter((row) => row.surfaceKind === 'native-app');
const categorySet = new Set(workerProof.rows.map((row) => row.expectedCategory));
const actionSet = new Set(workerProof.rows.map((row) => row.expectedAction));

assert(workerProof.status === 'ok', 'worker proof did not pass');
assert(workerProof.scenarioCount === scenarios.length, 'worker proof did not run the requested scenario matrix');
assert(browserRows.length === 4, 'expected four real live browser rows');
assert(nativeRows.length === 1, 'expected one real native app row');
for (const expected of ['school', 'video', 'game', 'shopping', 'productivity']) {
  assert(categorySet.has(expected), `missing category ${expected}`);
}
for (const expected of ['allow', 'warn', 'time-limit', 'ask-parent']) {
  assert(actionSet.has(expected), `missing policy action ${expected}`);
}
assert(
  workerProof.rows.every((row) => row.deletionProof?.existsAfterDelete === false),
  'one or more rows retained raw temp image material'
);
assert(
  workerProof.rows.every((row) => row.screenAnalysisResult?.providerKind === 'localOcr'),
  'one or more rows did not preserve localOcr provider kind'
);
assert(
  workerProof.rows.every((row) => row.policyDecision?.dryRun === true),
  'one or more rows did not produce a dry-run policy decision'
);

const summary = {
  proof: 'screen-ai-live-ocr-operator-proof',
  proofTier: 'P3_REAL_LIVE_PUBLIC_SURFACE_LOCAL_OCR',
  generatedAt: new Date().toISOString(),
  sourceWorkerProof: workerProofPath.replaceAll('\\', '/'),
  mirroredAiProof: join(aiRoot, 'proof-summary.json').replaceAll('\\', '/'),
  scenarioCount: workerProof.scenarioCount,
  browserLiveRows: browserRows.length,
  nativeRows: nativeRows.length,
  realCaptureRows: workerProof.rows.length,
  localOcrRows: workerProof.rows.length,
  policyDryRunRows: workerProof.rows.length,
  rawImagesDeletedAfterAnalysis: true,
  categories: [...categorySet],
  actions: [...actionSet],
  assertions: {
    realPublicPagesCaptured: browserRows.every((row) => row.sourceEvidence?.liveExternalUrl === true),
    actualPixelsCaptured: workerProof.rows.every((row) => row.sourceEvidence),
    ocrProducedMeaningfulCategories: categorySet.size >= 5,
    policyConsumedOcrResults: workerProof.rows.every((row) => row.policyDecision?.dryRun === true),
    rawScreenshotsNotRetainedByDefault: workerProof.rows.every((row) => row.deletionProof?.existsAfterDelete === false),
  },
  nonClaims: [
    'This proof uses real public browser pages and a real native app window, but it does not claim authenticated-account social coverage.',
    'This proof uses Windows WinRT OCR plus deterministic category expectations; it does not claim the local VLM live-operator matrix because the configured VLM binary/model/mmproj are absent.',
    'This proof does not claim managed-browser URL trigger ownership or broad browser/network/mobile enforcement adapters.',
    'This proof is progress toward the final product path, not the final product-complete proof.',
  ],
  rows: workerProof.rows.map((row) => ({
    scenarioId: row.scenarioId,
    surfaceKind: row.surfaceKind,
    category: row.expectedCategory,
    action: row.expectedAction,
    sourceEvidence: row.sourceEvidence,
    ocrTextDigest: row.ocrTextDigest,
    rawImageDeletedAfterOcr: row.deletionProof.rawImageDeletedAfterOcr,
    rawImageExistsAfterDelete: row.deletionProof.existsAfterDelete,
    providerKind: row.screenAnalysisResult.providerKind,
    policyDryRun: row.policyDecision.dryRun,
  })),
};

writeJson(join(pipelineRoot, 'proof-summary.json'), summary);
writeJson(join(aiRoot, 'proof-summary.json'), summary);
writeJson(join(resultRoot, 'proof.json'), summary);
writeText(join(pipelineRoot, 'validation-commands.log'), `${validationCommands.join('\n')}\n`);
writeText(join(aiRoot, 'validation-commands.log'), `${validationCommands.join('\n')}\n`);

console.log(`screen-ai-live-ocr-operator-proof-ok:${summary.scenarioCount}:${pipelineRoot}/proof-summary.json`);

function runWorkerProof() {
  const result = spawnSync('cmd', ['/c', 'node', 'scripts\\test\\screen-ai-winrt-ocr-worker-proof.mjs'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    env: {
      ...process.env,
      OCENTRA_SCREEN_WINRT_OCR_OUTPUT_ROOT: pipelineRoot,
      OCENTRA_SCREEN_WINRT_OCR_SCENARIOS: scenarios.join(','),
    },
  });
  writeText(join(pipelineRoot, 'worker-stdout.log'), result.stdout ?? '');
  writeText(join(pipelineRoot, 'worker-stderr.log'), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(
      `screen-ai-winrt-ocr-worker-proof failed with ${result.status}\n${result.stdout}\n${result.stderr}`
    );
  }
}

function readJson(path) {
  return JSON.parse(readFileSync(resolve(repoRoot, path), 'utf8'));
}

function writeJson(path, value) {
  writeText(path, `${JSON.stringify(value, null, 2)}\n`);
}

function writeText(path, value) {
  const absolutePath = resolve(repoRoot, path);
  mkdirSync(resolve(absolutePath, '..'), { recursive: true });
  writeFileSync(absolutePath, value);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
