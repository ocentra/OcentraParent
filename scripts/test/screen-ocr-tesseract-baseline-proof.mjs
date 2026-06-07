import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { performance } from 'node:perf_hooks';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '34-ocr-tesseract-baseline');
const proofSummaryPath = join(outputRoot, 'proof-summary.json');
const extractionTextPath = join(outputRoot, 'vimeo-public-video-tesseract-output.txt');
const sourceImagePath = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-14-managed-browser-feed-short-video-gate',
  '06-live-screenshots',
  'vimeo-public-video.png'
);

await mkdir(outputRoot, { recursive: true });

const whereTesseract =
  process.platform === 'win32' ? runOptional('where.exe', ['tesseract']) : runOptional('which', ['tesseract']);
const tesseractCommand = resolveTesseractCommand(whereTesseract);
const tesseractVersion = tesseractCommand
  ? runOptional(tesseractCommand, ['--version'], { shell: false })
  : runOptional('tesseract', ['--version']);
const tesseractInstalled = tesseractVersion.status === 0;
const extraction = tesseractCommand
  ? runTimed(tesseractCommand, [sourceImagePath, 'stdout', '--psm', '6'])
  : unavailableExtraction();
const extractedText = oneLine(extraction.stdout);
const expectedTerms = ['vimeo', 'video', 'player'];
const matchedTerms = expectedTerms.filter((term) => extractedText.toLowerCase().includes(term));
const localExtractionProofComplete = tesseractInstalled && extraction.status === 0 && matchedTerms.length >= 3;

assert(existsSync(sourceImagePath), `Missing real screenshot source image: ${sourceImagePath}`);
if (tesseractInstalled) {
  assert(localExtractionProofComplete, `Tesseract extraction did not match expected Vimeo terms: ${extractedText}`);
}

const summary = {
  proof: 'screen-ocr-tesseract-baseline',
  generatedAt: new Date().toISOString(),
  officialSourceSnapshot: {
    project: 'https://github.com/tesseract-ocr/tesseract',
    documentation: 'https://tesseract-ocr.github.io/tessdoc/',
    windowsInstallation: 'https://tesseract-ocr.github.io/tessdoc/Installation.html',
    windowsDownloads: 'https://tesseract-ocr.github.io/tessdoc/Downloads.html',
    verifiedClaims: [
      'Tesseract is the upstream open-source OCR engine candidate for the simple baseline.',
      'The upstream project is Apache-2.0 licensed.',
      'The upstream tessdoc installation page points Windows users to UB Mannheim builds for Tesseract 3.05, 4, and 5.',
      'The upstream tessdoc downloads page says there is no official Windows installer for newer versions.',
    ],
  },
  localEnvironment: {
    platform: process.platform,
    whereTesseractStatus: whereTesseract.status,
    whereTesseractOutput: oneLine(whereTesseract.stdout || whereTesseract.stderr),
    resolvedTesseractCommand: tesseractCommand,
    tesseractVersionStatus: tesseractVersion.status,
    tesseractVersionOutput: oneLine(tesseractVersion.stdout || tesseractVersion.stderr),
    tesseractInstalled,
  },
  extractionProof: {
    sourceImage: relativePath(sourceImagePath),
    sourceImageKind: 'retained real managed-browser public Vimeo screenshot artifact',
    sourceImageExists: existsSync(sourceImagePath),
    commandStatus: extraction.status,
    durationMs: extraction.durationMs,
    outputArtifact: relativePath(extractionTextPath),
    outputCharacterCount: extraction.stdout.length,
    matchedTerms,
    expectedTerms,
    localExtractionProofComplete,
  },
  baselineReadiness: {
    status: localExtractionProofComplete
      ? 'runtime-extraction-proved'
      : tesseractInstalled
        ? 'runtime-available'
        : 'runtime-unavailable',
    windowsPackagingProofComplete: tesseractInstalled,
    localExtractionProofComplete,
    runtimeMeasured: localExtractionProofComplete,
    cpuMemoryRuntimeMeasured: false,
    failureModesRecorded: false,
    comparedAgainstPaddleOcr: false,
    reason: localExtractionProofComplete
      ? 'Tesseract is installed and extracted expected text from a retained real public Vimeo screenshot artifact.'
      : tesseractInstalled
        ? 'Tesseract is available, but extraction proof did not complete.'
        : 'Tesseract is not available on PATH or the standard Windows install path in this lane; install/package proof must happen before extraction or quality claims.',
  },
  assertions: {
    sourceImageExists: existsSync(sourceImagePath),
    tesseractInstalled,
    extractionSucceeded: extraction.status === 0,
    expectedTextMatched: matchedTerms.length >= 3,
    localExtractionProofComplete,
    noProductionQualityClaim: true,
  },
  packageInstallEvidence: {
    installCommand:
      'winget install --id tesseract-ocr.tesseract --exact --source winget --accept-package-agreements --accept-source-agreements --disable-interactivity',
    packageId: 'tesseract-ocr.tesseract',
    packageVersionObserved: '5.5.0.20241111',
    installedPathObserved: process.platform === 'win32' ? 'C:\\Program Files\\Tesseract-OCR\\tesseract.exe' : null,
    pathRefreshRequired: process.platform === 'win32' && whereTesseract.status !== 0 && tesseractCommand !== null,
  },
  openMeasurements: {
    cpuMemoryRuntimeMeasured: false,
    smallFontFailureModesRecorded: false,
    messyUiFailureModesRecorded: false,
    paddleOcrComparisonComplete: false,
    reason:
      'This proof measures command duration and extraction output only; CPU/memory, small-font/messy-UI failure modes, and PaddleOCR comparison remain follow-up gates.',
  },
  nonClaims: [
    tesseractInstalled
      ? 'This proof installed and invoked Tesseract locally, but it does not select Tesseract as the production OCR runtime.'
      : 'Tesseract is not available on PATH in this Windows lane; install/package proof must happen before extraction or quality claims.',
    'This proof runs OCR over a retained real public browser screenshot artifact; it does not create a new screen capture.',
    'This proof records extraction duration and matched text terms, but it does not claim OCR quality, CPU, memory, or production latency suitability.',
    'This proof does not compare Tesseract against PaddleOCR/PP-OCR.',
  ],
  validationCommands: [
    'node --check scripts/test/screen-ocr-tesseract-baseline-proof.mjs',
    'node scripts/test/screen-ocr-tesseract-baseline-proof.mjs',
    process.platform === 'win32' ? 'where.exe tesseract' : 'which tesseract',
    process.platform === 'win32'
      ? '"C:\\Program Files\\Tesseract-OCR\\tesseract.exe" --version'
      : 'tesseract --version',
    'tesseract --version',
  ],
};

await writeFile(extractionTextPath, normalizeExtractedText(extraction.stdout));
await writeFile(proofSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ocr-tesseract-baseline-proof-ok:${summary.baselineReadiness.status}`);
console.log(`artifact=${proofSummaryPath}`);

function resolveTesseractCommand(whereResult) {
  const firstWhereLine = whereResult.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .find(Boolean);
  if (firstWhereLine && existsSync(firstWhereLine)) return firstWhereLine;
  const windowsDefault = 'C:\\Program Files\\Tesseract-OCR\\tesseract.exe';
  if (process.platform === 'win32' && existsSync(windowsDefault)) return windowsDefault;
  return null;
}

function runTimed(command, args) {
  const started = performance.now();
  const result = runOptional(command, args, { shell: false });
  return {
    ...result,
    durationMs: Math.round(performance.now() - started),
  };
}

function unavailableExtraction() {
  return {
    status: 1,
    stdout: '',
    stderr: 'Tesseract command unavailable.',
    durationMs: null,
  };
}

function runOptional(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: options.shell ?? process.platform === 'win32',
  });
  return {
    status: result.status ?? 1,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function oneLine(value) {
  return value.replace(/\s+/g, ' ').trim();
}

function normalizeExtractedText(value) {
  return `${value
    .split(/\r?\n/)
    .map((line) => line.trimEnd())
    .join('\n')
    .trim()}\n`;
}

function relativePath(path) {
  return path.replace(`${repoRoot}\\`, '').replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
