import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '35-ocr-paddleocr-ppocr-evaluation');
const proofSummaryPath = join(outputRoot, 'proof-summary.json');

await mkdir(outputRoot, { recursive: true });

const python = runOptional('python', ['--version']);
const pip = runOptional('python', ['-m', 'pip', '--version']);
const paddleOcrIndex = runOptional('python', ['-m', 'pip', 'index', 'versions', 'paddleocr']);
const paddlePaddleIndex = runOptional('python', ['-m', 'pip', 'index', 'versions', 'paddlepaddle']);
const paddleOcrImport = runOptional('python', [
  '-c',
  'import importlib.util; print(importlib.util.find_spec("paddleocr") is not None)',
]);
const paddlePaddleImport = runOptional('python', [
  '-c',
  'import importlib.util; print(importlib.util.find_spec("paddle") is not None)',
]);
const tesseract = runOptional('tesseract', ['--version']);

const paddleOcrVersion = parsePipLatest(paddleOcrIndex.stdout);
const paddlePaddleVersion = parsePipLatest(paddlePaddleIndex.stdout);
const paddleOcrInstalled = parsePythonBool(paddleOcrImport.stdout);
const paddlePaddleInstalled = parsePythonBool(paddlePaddleImport.stdout);
const tesseractInstalled = tesseract.status === 0;

assert(python.status === 0, 'Python must be available for the OCR candidate evaluation.');
assert(pip.status === 0, 'pip must be available for the OCR candidate evaluation.');
assert(paddleOcrVersion, 'PyPI must report a current paddleocr version.');
assert(paddlePaddleVersion, 'PyPI must report a current paddlepaddle version.');

const localRuntimeReady = paddleOcrInstalled && paddlePaddleInstalled;
const tesseractComparisonReady = localRuntimeReady && tesseractInstalled;
const explicitRuntimeRunRequested = process.env.OCENTRA_RUN_PADDLEOCR_LOCAL === '1';
const runtimeExecutionAllowed = explicitRuntimeRunRequested && localRuntimeReady;

const summary = {
  proof: 'screen-ocr-paddleocr-ppocr-evaluation',
  generatedAt: new Date().toISOString(),
  officialSourceSnapshot: {
    paddleOcrInstallation: 'https://paddlepaddle.github.io/PaddleOCR/main/en/version3.x/installation.html',
    paddleOcrQuickStart: 'https://paddlepaddle.github.io/PaddleOCR/main/en/quick_start.html',
    ppOcrV5: 'https://paddlepaddle.github.io/PaddleOCR/main/en/version3.x/algorithm/PP-OCRv5/PP-OCRv5.html',
    verifiedClaims: [
      'PaddleOCR 3.x inference package is installed from PyPI as paddleocr.',
      'PaddleOCR 3.x depends on PaddlePaddle 3.0 or newer.',
      'PP-OCRv5 is the current 3.x OCR family to evaluate for general OCR.',
      'PaddleOCR supports Windows, Linux, and macOS deployment, but Ocentra still requires local runtime proof before product selection.',
    ],
  },
  localEnvironment: {
    python: oneLine(python.stdout || python.stderr),
    pip: oneLine(pip.stdout || pip.stderr),
    tesseractInstalled,
    paddleOcrInstalled,
    paddlePaddleInstalled,
  },
  pypiCandidateSnapshot: {
    paddleocrLatest: paddleOcrVersion,
    paddlepaddleLatest: paddlePaddleVersion,
    paddleocrIndexCommandPassed: paddleOcrIndex.status === 0,
    paddlepaddleIndexCommandPassed: paddlePaddleIndex.status === 0,
  },
  localOnlyGate: {
    remoteApiAllowed: false,
    remoteModelDownloadAllowedByDefault: false,
    runtimeExecutionAllowed,
    explicitRuntimeRunEnv: 'OCENTRA_RUN_PADDLEOCR_LOCAL=1',
    reason: runtimeExecutionAllowed
      ? 'Explicit local runtime execution was requested and local packages are importable.'
      : 'The gate records packaging/current-version facts only; it does not download models or call remote OCR services by default.',
  },
  runtimeAndQualityComparison: {
    status: tesseractComparisonReady ? 'ready-for-local-comparison' : 'not-run',
    paddleOcrRuntimeReady: localRuntimeReady,
    tesseractRuntimeReady: tesseractInstalled,
    comparedAgainstTesseract: false,
    reason: tesseractComparisonReady
      ? 'Both runtimes are importable/available, but this proof run did not request runtime execution.'
      : 'This Windows lane lacks one or more local OCR runtimes, so quality comparison remains a follow-up proof.',
  },
  placementDecision: {
    selectedForProduction: false,
    preferredNextHost: localRuntimeReady ? 'child-device' : 'family-hub-or-child-after-install-proof',
    decision:
      'Do not select PaddleOCR/PP-OCR for production screen OCR until local package install, model-cache custody, no-upload execution, and Tesseract comparison proof pass.',
  },
  nonClaims: [
    'This proof does not install PaddleOCR, download OCR models, or run PaddleOCR inference.',
    'This proof does not call PaddleOCR remote API or any hosted OCR endpoint.',
    'This proof does not claim PaddleOCR quality, latency, CPU, GPU, or memory suitability.',
    'This proof does not replace the existing typed OCR route proof.',
  ],
};

await writeFile(proofSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ocr-paddleocr-evaluation-proof-ok:${proofSummaryPath}`);

function runOptional(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  return {
    status: result.status ?? 1,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function parsePipLatest(stdout) {
  const match = stdout.match(/^\s*[A-Za-z0-9_-]+\s+\(([^)]+)\)/m);
  return match?.[1] ?? null;
}

function parsePythonBool(stdout) {
  return stdout.trim().toLowerCase() === 'true';
}

function oneLine(value) {
  return value.replace(/\s+/g, ' ').trim();
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
