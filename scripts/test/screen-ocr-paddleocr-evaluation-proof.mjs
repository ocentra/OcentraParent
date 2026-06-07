import { existsSync, writeFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { performance } from 'node:perf_hooks';
import { spawnSync } from 'node:child_process';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '35-ocr-paddleocr-ppocr-evaluation');
const proofSummaryPath = join(outputRoot, 'proof-summary.json');
const runtimeLogPath = join(outputRoot, 'paddleocr-runtime-attempt.log');
const sourceImagePath = resolve(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-14-managed-browser-feed-short-video-gate',
  '06-live-screenshots',
  'vimeo-public-video.png'
);
const tesseractTextPath = resolve(
  repoRoot,
  'output',
  'screen-plan-proof',
  '34-ocr-tesseract-baseline',
  'vimeo-public-video-tesseract-output.txt'
);

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
const paddleVersions = runOptional('python', [
  '-c',
  'import paddle, paddleocr; print(f"paddle={paddle.__version__}"); print(f"paddleocr={getattr(paddleocr, \'__version__\', \'unknown\')}")',
]);
const tesseract = resolveTesseractVersion();

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
const runtimeAttempt = runtimeExecutionAllowed ? runPaddleOcrRuntimeAttempt() : null;
const tesseractText = await readOptionalText(tesseractTextPath);
const tesseractTerms = ['vimeo', 'video', 'player'].filter((term) => tesseractText.toLowerCase().includes(term));
const modelCache = [
  'PP-LCNet_x1_0_doc_ori',
  'UVDoc',
  'PP-LCNet_x1_0_textline_ori',
  'PP-OCRv5_server_det',
  'PP-OCRv5_mobile_det',
  'en_PP-OCRv5_mobile_rec',
].map((name) => ({
  name,
  path: join(process.env.USERPROFILE ?? '', '.paddlex', 'official_models', name),
  exists: existsSync(join(process.env.USERPROFILE ?? '', '.paddlex', 'official_models', name)),
}));

const summary = {
  proof: 'screen-ocr-paddleocr-ppocr-evaluation',
  generatedAt: new Date().toISOString(),
  sourceEvidence: {
    sourceImagePath,
    sourceImageExists: existsSync(sourceImagePath),
    sourceImageKind: 'retained real public Vimeo managed-browser screenshot artifact',
    tesseractBaselinePath: tesseractTextPath,
    tesseractBaselineExists: Boolean(tesseractText),
    tesseractMatchedTerms: tesseractTerms,
  },
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
    installedVersions: parseInstalledVersions(paddleVersions.stdout),
    packagingRisk:
      'pip install succeeded, but pip reported an environment conflict: mediapipe requires protobuf<5 while this Python environment has protobuf 5.29.5.',
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
      ? 'Explicit local runtime execution was requested and local packages are importable; inference is attempted against the retained real Vimeo screenshot artifact.'
      : 'The gate records packaging/current-version facts only; it does not download models or call remote OCR services by default.',
  },
  modelCacheCustody: {
    cacheRoot: join(process.env.USERPROFILE ?? '', '.paddlex', 'official_models'),
    checkedModels: modelCache,
    allCheckedModelsCached: modelCache.every((entry) => entry.exists),
    note: 'Runtime execution downloads official model files into the local user cache. This is not a hosted OCR call, but production custody still needs explicit model-cache policy.',
  },
  runtimeAndQualityComparison: {
    status:
      runtimeAttempt?.status === 0
        ? 'runtime-comparison-complete'
        : runtimeExecutionAllowed
          ? 'runtime-blocked'
          : 'not-run',
    paddleOcrRuntimeReady: localRuntimeReady,
    tesseractRuntimeReady: tesseractInstalled,
    comparedAgainstTesseract: runtimeAttempt?.status === 0,
    tesseractMatchedTerms: tesseractTerms,
    paddleOcrRuntimeAttempt: runtimeAttempt,
    reason: runtimeAttempt
      ? runtimeAttempt.status === 0
        ? 'PaddleOCR completed local inference and can be compared against the Tesseract baseline.'
        : 'PaddleOCR packages and models are present, but local inference fails before OCR text extraction; Tesseract remains the only runtime-proved OCR baseline in this lane.'
      : tesseractComparisonReady
        ? 'Both runtimes are importable/available, but this proof run did not request runtime execution.'
        : 'This Windows lane lacks one or more local OCR runtimes, so quality comparison remains a follow-up proof.',
  },
  placementDecision: {
    selectedForProduction: false,
    preferredNextHost:
      runtimeAttempt?.status === 0
        ? 'child-device-or-household-mesh-after-resource-measurement'
        : 'not-selected-runtime-blocked',
    decision:
      'Do not select PaddleOCR/PP-OCR for production screen OCR until local package install, model-cache custody, no-upload inference, Tesseract comparison, and CPU/GPU/memory/runtime proof pass.',
  },
  nonClaims: [
    runtimeExecutionAllowed
      ? 'This proof attempts local PaddleOCR inference only because OCENTRA_RUN_PADDLEOCR_LOCAL=1 was set.'
      : 'This proof does not install PaddleOCR, download OCR models, or run PaddleOCR inference by default.',
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
  });
  return {
    status: result.status ?? 1,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function runPaddleOcrRuntimeAttempt() {
  const code = String.raw`
from paddleocr import PaddleOCR
from pathlib import Path
import json
import time
image = Path(r"${sourceImagePath}")
start = time.perf_counter()
ocr = PaddleOCR(
    text_detection_model_name="PP-OCRv5_mobile_det",
    text_recognition_model_name="en_PP-OCRv5_mobile_rec",
    use_doc_orientation_classify=False,
    use_doc_unwarping=False,
    use_textline_orientation=False,
)
init_seconds = time.perf_counter() - start
start = time.perf_counter()
result = ocr.predict(str(image))
predict_seconds = time.perf_counter() - start
texts = []
raw_items = []
for item in result:
    data = item.json if hasattr(item, "json") else item.to_json() if hasattr(item, "to_json") else item
    raw_items.append(data)
    if isinstance(data, dict):
        for key in ("rec_texts", "texts"):
            values = data.get(key)
            if isinstance(values, list):
                texts.extend(str(value) for value in values)
print(json.dumps({
    "initSeconds": round(init_seconds, 3),
    "predictSeconds": round(predict_seconds, 3),
    "texts": texts,
    "itemCount": len(raw_items),
}, ensure_ascii=False))
`;
  const start = performance.now();
  const result = runOptional('python', ['-c', code]);
  const durationMs = Math.round(performance.now() - start);
  const combinedLog = normalizeLog(stripAnsi(`${result.stdout}${result.stderr}`));
  writeFileSync(runtimeLogPath, combinedLog);
  const parsed = parseJsonLine(result.stdout);
  const error = result.status === 0 ? null : summarizeRuntimeError(combinedLog);
  return {
    status: result.status,
    durationMs,
    logPath: runtimeLogPath,
    mode: 'PP-OCRv5_mobile_det + en_PP-OCRv5_mobile_rec with orientation/unwarping/textline disabled',
    extractedTexts: parsed?.texts ?? [],
    extractedTextCount: parsed?.texts?.length ?? 0,
    initSeconds: parsed?.initSeconds ?? null,
    predictSeconds: parsed?.predictSeconds ?? null,
    error,
  };
}

function stripAnsi(value) {
  return value.replace(/\u001b\[[0-9;]*m/g, '');
}

function normalizeLog(value) {
  return `${value
    .split(/\r?\n/)
    .map((line) => line.trimEnd())
    .join('\n')
    .trimEnd()}\n`;
}

function resolveTesseractVersion() {
  const direct = runOptional('tesseract', ['--version']);
  if (direct.status === 0) {
    return direct;
  }
  const windowsPath = 'C:\\Program Files\\Tesseract-OCR\\tesseract.exe';
  if (existsSync(windowsPath)) {
    return runOptional(windowsPath, ['--version']);
  }
  return direct;
}

async function readOptionalText(path) {
  try {
    return await readFile(path, 'utf8');
  } catch {
    return '';
  }
}

function parseInstalledVersions(stdout) {
  return Object.fromEntries(
    stdout
      .split(/\r?\n/)
      .map((line) => line.trim().split('='))
      .filter((parts) => parts.length === 2)
  );
}

function parseJsonLine(stdout) {
  for (const line of stdout.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed.startsWith('{')) {
      continue;
    }
    try {
      return JSON.parse(trimmed);
    } catch {
      return null;
    }
  }
  return null;
}

function summarizeRuntimeError(log) {
  const lines = log
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
  const notImplemented = lines.find((line) => line.includes('ConvertPirAttribute2RuntimeAttribute'));
  const traceback = lines.find((line) => line.startsWith('NotImplementedError:'));
  return notImplemented ?? traceback ?? lines.at(-1) ?? 'unknown PaddleOCR runtime failure';
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
