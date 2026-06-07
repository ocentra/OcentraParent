import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', 'screen-plan-closure-audit');
const proofPath = join(outputRoot, 'proof-summary.json');
const checklistPath = join(repoRoot, 'docs', 'plans', 'screen-plan', 'implementation-checklist.md');

const checklist = readText(checklistPath);
const workpacks = [
  {
    id: '10',
    label: 'macOS capture adapter plan/proof',
    status: workpackStatus('10 macOS capture adapter plan/proof'),
    requiredProof: 'output/screen-plan-proof/macos-screen-capture/proof-summary.json',
    gate: 'Requires real macOS ScreenCaptureKit/permission proof on macOS hardware.',
  },
  {
    id: '11',
    label: 'Linux capture adapter plan/proof',
    status: workpackStatus('11 Linux capture adapter plan/proof'),
    requiredProof: 'output/screen-plan-proof/linux-screen-capture/proof-summary.json',
    gate: 'Requires real Linux X11/Wayland portal proof on a Linux desktop session.',
  },
  {
    id: '12',
    label: 'Android MediaProjection adapter plan/proof',
    status: workpackStatus('12 Android MediaProjection adapter plan/proof'),
    requiredProof: 'output/screen-plan-proof/android-mediaprojection/proof-summary.json',
    gate: 'Needs physical/emulator parity beyond the existing consent/capture proof before broad Android product claim.',
  },
  {
    id: '13',
    label: 'iOS ReplayKit adapter plan/proof',
    status: workpackStatus('13 iOS ReplayKit adapter plan/proof'),
    requiredProof: 'output/screen-plan-proof/ios-replaykit/proof-summary.json',
    gate: 'Requires real iOS ReplayKit entitlement/device proof.',
  },
  {
    id: '28',
    label: 'Live view optional mode',
    status: workpackStatus('28 Live view optional mode'),
    requiredProof: 'output/screen-plan-proof/live-view-platform-permission/proof-summary.json',
    gate: 'Requires real platform permission and live transport proof; current proof is contract/preflight only.',
  },
  {
    id: '30',
    label: 'Test suite, Playwright, rollout, PR gate',
    status: workpackStatus('30 Test suite, Playwright, rollout, PR gate'),
    requiredProof: null,
    gate: 'Final closure waits for partial platform/model gates or explicit product non-claim handoff.',
  },
  {
    id: '34',
    label: 'OCR Tesseract baseline',
    status: workpackStatus('34 OCR Tesseract baseline'),
    requiredProof: 'output/screen-plan-proof/34-ocr-tesseract-baseline/proof-summary.json',
    gate: 'Local tesseract binary and language data are not installed/proved in this Windows lane.',
  },
  {
    id: '35',
    label: 'OCR PaddleOCR/PP-OCR evaluation',
    status: workpackStatus('35 OCR PaddleOCR/PP-OCR evaluation'),
    requiredProof: 'output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/proof-summary.json',
    gate: 'Current proof is packaging/version/local-only gate only; no package install, model cache, runtime, or quality comparison claim.',
  },
  {
    id: '36',
    label: 'Small VLM guided classifier evaluation',
    status: workpackStatus('36 Small VLM guided classifier evaluation'),
    requiredProof: 'output/screen-plan-proof/36-small-vlm-guided-classifier-evaluation/proof-summary.json',
    gate: 'Current proof is local VLM readiness/manual-required only; no live VLM runtime or quality measurement claim.',
  },
];

const completeRows = [...checklist.matchAll(/\| \[x\]\s+\|\s+([^|]+?)\s+\|/g)].map((match) => match[1].trim());
const partialRows = [...checklist.matchAll(/\| \[~\]\s+\|\s+([^|]+?)\s+\|/g)].map((match) => match[1].trim());
const openRows = [...checklist.matchAll(/\| \[ \]\s+\|\s+([^|]+?)\s+\|/g)].map((match) => match[1].trim());
const missingProofs = workpacks
  .filter((workpack) => workpack.requiredProof !== null)
  .filter((workpack) => !existsSync(join(repoRoot, workpack.requiredProof)));

assert(
  completeRows.includes('19 Sensitive text and redaction model'),
  'WP19 must remain closed after selected-policy proof.'
);
assert(
  partialRows.includes('28 Live view optional mode'),
  'Live view must remain partial until real platform transport proof exists.'
);
assert(
  openRows.includes('13 iOS ReplayKit adapter plan/proof'),
  'iOS ReplayKit must remain open without real iOS proof.'
);
assert(missingProofs.length > 0, 'Closure audit expects at least one missing external proof gate.');

const summary = {
  proof: 'screen-plan-closure-audit',
  generatedAt: new Date().toISOString(),
  branchScope: 'codex/screen-ai-full-scope-b',
  checklist: {
    path: relativePath(checklistPath),
    completeCount: completeRows.length,
    partialCount: partialRows.length,
    openCount: openRows.length,
    completeRows,
    partialRows,
    openRows,
  },
  remainingExternalProofGates: workpacks.map((workpack) => ({
    id: workpack.id,
    label: workpack.label,
    status: workpack.status,
    requiredProof: workpack.requiredProof,
    proofPresent: workpack.requiredProof === null ? null : existsSync(join(repoRoot, workpack.requiredProof)),
    gate: workpack.gate,
  })),
  currentWindowsEvidence: [
    'output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/proof-summary.json',
    'output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/portal-screen-analysis-redaction.png',
    'output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/parent-redaction-policy.json',
    'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  ].map((artifact) => ({
    artifact,
    present: existsSync(join(repoRoot, artifact)),
  })),
  assertions: {
    wp19Closed: completeRows.includes('19 Sensitive text and redaction model'),
    remainingGatesExplicit: partialRows.length + openRows.length > 0,
    externalRuntimeProofsNotInvented: missingProofs.length > 0,
    noProductCompleteClaim: true,
  },
  nonClaims: [
    'This audit does not complete macOS, Linux, Android parity, iOS, live-view transport, Tesseract runtime, PaddleOCR runtime, or live VLM quality gates.',
    'This audit does not replace real device/runtime proof for remaining partial rows.',
    'This audit exists to prevent product-complete wording before the remaining external proof gates are satisfied.',
  ],
};

mkdirSync(outputRoot, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-plan-closure-audit-proof-ok:${proofPath}`);

function workpackStatus(label) {
  const escaped = label.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const match = checklist.match(new RegExp(`\\| \\[([^\\]]*)\\]\\s+\\|\\s+${escaped}\\s+\\|`));
  assert(match, `Missing checklist row: ${label}`);
  return match[1].trim() || 'open';
}

function readText(path) {
  return readFileSync(path, 'utf8');
}

function relativePath(path) {
  return path.replace(`${repoRoot}\\`, '').replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
