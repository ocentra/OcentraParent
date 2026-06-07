import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', 'screen-plan-closure-audit');
const proofPath = join(outputRoot, 'proof-summary.json');
const checklistPath = join(repoRoot, 'docs', 'plans', 'screen-plan', 'implementation-checklist.md');
const windowsOcrSelectionPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'windows-ocr-candidate-selection',
  'proof-summary.json'
);

const checklist = readText(checklistPath);
const windowsOcrSelection = readJson(windowsOcrSelectionPath);
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
    requiredProof: 'output/screen-plan-proof/live-view-parent-ui-persistence/proof-summary.json',
    gate: 'Fail-closed platform permission gate, local loopback live-frame transport proof, parent UI persistence proof, service-session readiness boundary proof, and Rust service runtime decision proof exist; real platform live-view prompt screenshots, production worker startup, relay/cache execution, physical-device parity, and privacy/legal approval remain.',
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
    gate: 'Local Tesseract extraction, CPU/memory measurement, and derived failure-mode capture are proved; Tesseract is retained as a measured fallback while the current Windows service OCR route is WinRT. Cross-platform OCR parity, broad language coverage, and final production quality remain open.',
  },
  {
    id: '35',
    label: 'OCR PaddleOCR/PP-OCR evaluation',
    status: workpackStatus('35 OCR PaddleOCR/PP-OCR evaluation'),
    requiredProof: 'output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/proof-summary.json',
    gate: 'Current PP-OCRv5 inference now runs locally with oneDNN/MKLDNN disabled but extracts zero text from the real proof image; an isolated pinned PaddleOCR 2.x fallback extracts comparable text locally. PaddleOCR is not selected; current Windows service OCR route selection is WinRT, while PP-OCRv5 quality/resource resolution and broad quality remain open.',
  },
  {
    id: '36',
    label: 'Small VLM guided classifier evaluation',
    status: workpackStatus('36 Small VLM guided classifier evaluation'),
    requiredProof: 'output/screen-plan-proof/36-small-vlm-guided-classifier-evaluation/proof-summary.json',
    gate: 'Current proof detects the local llama.cpp/Qwen2-VL runtime, retained controlled local VLM matrix, retained nine-scenario live operator matrix, bounded retained VLM inputs, managed-browser CDP crop capture path, retained proof-image VLM wall/CPU/RSS measurement, public-live CDP crop quality, current Windows local VLM route selection, and measured rollout/fallback gate; authenticated-account social proof and broader hardware rollout thresholds remain open.',
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
assert(
  windowsOcrSelection.assertions?.windowsServiceOcrSelected === true,
  'Closure audit expects the Windows OCR route selection artifact to select WinRT OCR.'
);
assert(
  windowsOcrSelection.selectedCurrentRoute?.modelId === 'windows-winrt-ocr',
  'Closure audit expects Windows OCR selection to name windows-winrt-ocr.'
);

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
    'output/screen-plan-proof/live-view-session-transport/proof-summary.json',
    'output/screen-plan-proof/live-view-parent-ui-persistence/proof-summary.json',
    'output/screen-plan-proof/live-view-service-session/proof-summary.json',
    'output/screen-plan-proof/live-view-runtime/proof-summary.json',
    'output/screen-plan-proof/windows-ocr-candidate-selection/proof-summary.json',
    'output/screen-plan-proof/36-vlm-resource-crop-readiness/proof-summary.json',
    'output/screen-plan-proof/36-vlm-runtime-resource-measurement/proof-summary.json',
    'output/screen-plan-proof/36-vlm-live-crop-quality/proof-summary.json',
    'output/screen-plan-proof/36-vlm-model-selection/proof-summary.json',
    'output/screen-plan-proof/36-vlm-rollout-fallback-gate/proof-summary.json',
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
    'This audit does not complete macOS, Linux, Android parity, iOS, live-view platform prompt screenshots/production worker startup/relay-cache execution/physical-device parity, current PP-OCRv5 quality resolution, cross-platform OCR parity, authenticated-account social proof, or broader VLM hardware rollout-threshold gates.',
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

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function relativePath(path) {
  return path.replace(`${repoRoot}\\`, '').replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
