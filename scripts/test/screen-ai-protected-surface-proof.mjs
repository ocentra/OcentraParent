import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'protected-surface');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');

await mkdir(outputRoot, { recursive: true });
runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));

const { ScreenAnalysisResultSchema, ScreenCapabilitySnapshotSchema, ScreenEvidenceSchemaVersion } =
  await import('../../packages/activity-domain/dist/screen-evidence.js');

const observedAt = '2026-06-03T21:10:00.000Z';
const evidenceRef = {
  evidenceId: 'screen-protected-surface-evidence',
  kind: 'journal-entry',
  digest: 'sha256:protected-surface-no-capture',
  uri: null,
};

const capabilitySnapshot = ScreenCapabilitySnapshotSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  observedAt,
  capabilityStatus: 'protectedSurface',
  captureScope: 'selectedWindow',
  parentSettingRef: 'screen-protected-surface-setting',
  settingVersion: 1,
  unavailableReason: 'protected surface blocked capture',
  custodyState: 'unavailable',
});

const protectedSkipResult = ScreenAnalysisResultSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisResultId: 'screen-analysis-protected-surface-skip',
  queueJobId: 'screen-queue-protected-surface-skip',
  analyzedAt: observedAt,
  modelRuntimeRef: 'local-model-unavailable-protected-surface',
  modelId: 'unavailable-protected-surface',
  providerKind: 'unavailable',
  promptOrTemplateVersion: 'screen-protected-surface-v1',
  captureReason: 'manualParentTestCapture',
  captureScope: 'selectedWindow',
  capabilityStatus: 'protectedSurface',
  summary: 'Protected surface blocked local capture and analysis.',
  visibleCategoryCandidates: [],
  primaryCategory: null,
  riskSignals: [],
  ocrTextSnippets: [],
  redactionNotes: ['protectedRegionSkipped'],
  confidence: 0,
  uncertaintyReason: 'protectedSurface',
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: 'sha256:protected-surface-no-capture',
  rawImageRetained: false,
  imageDeletionState: 'unavailableNoImage',
  custodyState: 'unavailable',
  policyEligible: false,
});

const policyEligibleProtectedRejected = !ScreenAnalysisResultSchema.safeParse({
  ...protectedSkipResult,
  primaryCategory: 'unknown',
  policyEligible: true,
}).success;

if (!policyEligibleProtectedRejected) {
  throw new Error('Expected protected-surface result to reject policy eligibility');
}

const summary = {
  status: 'ok',
  proofKind: 'screen-ai-protected-surface-skip',
  artifact: artifactSummaryPath,
  capabilitySnapshot,
  protectedSkipResult,
  captureAttempted: false,
  aiAnalysisAttempted: false,
  policyDecisionCreated: false,
  policyEligibleProtectedRejected,
  assertions: [
    'A protected surface is recorded as capabilityStatus protectedSurface with unavailable custody.',
    'No raw image custody, OCR/VLM provider, or policy decision is claimed.',
    'A protected-surface result cannot become policy eligible.',
  ],
  nonClaims: [
    'This is a protected-surface degraded-state contract proof.',
    'It does not claim a live OS permission prompt, protected-app bypass, or final enforcement execution.',
  ],
};

await writeFile(artifactSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ai-protected-surface-proof-ok ${artifactSummaryPath}`);

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
