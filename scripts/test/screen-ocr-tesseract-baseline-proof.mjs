import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '34-ocr-tesseract-baseline');
const proofSummaryPath = join(outputRoot, 'proof-summary.json');

await mkdir(outputRoot, { recursive: true });

const whereTesseract =
  process.platform === 'win32' ? runOptional('where.exe', ['tesseract']) : runOptional('which', ['tesseract']);
const tesseractVersion = runOptional('tesseract', ['--version']);
const tesseractInstalled = tesseractVersion.status === 0;

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
    tesseractVersionStatus: tesseractVersion.status,
    tesseractVersionOutput: oneLine(tesseractVersion.stdout || tesseractVersion.stderr),
    tesseractInstalled,
  },
  baselineReadiness: {
    status: tesseractInstalled ? 'runtime-available' : 'runtime-unavailable',
    windowsPackagingProofComplete: false,
    localExtractionProofComplete: false,
    cpuMemoryRuntimeMeasured: false,
    failureModesRecorded: false,
    comparedAgainstPaddleOcr: false,
    reason: tesseractInstalled
      ? 'Tesseract is available on PATH; a follow-up proof must run real extraction and measurement.'
      : 'Tesseract is not available on PATH in this Windows lane; install/package proof must happen before extraction or quality claims.',
  },
  nonClaims: [
    'This proof does not install Tesseract or language data.',
    'This proof does not run OCR extraction or measure quality, CPU, memory, or latency.',
    'This proof does not select Tesseract as the production OCR runtime.',
    'This proof does not compare Tesseract against PaddleOCR/PP-OCR.',
  ],
  validationCommands: [
    'node --check scripts/test/screen-ocr-tesseract-baseline-proof.mjs',
    'node scripts/test/screen-ocr-tesseract-baseline-proof.mjs',
    process.platform === 'win32' ? 'where.exe tesseract' : 'which tesseract',
    'tesseract --version',
  ],
};

await writeFile(proofSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ocr-tesseract-baseline-proof-ok:${summary.baselineReadiness.status}`);
console.log(`artifact=${proofSummaryPath}`);

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

function oneLine(value) {
  return value.replace(/\s+/g, ' ').trim();
}
