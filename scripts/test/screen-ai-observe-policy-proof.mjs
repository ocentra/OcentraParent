import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'observe-policy');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');

await mkdir(outputRoot, { recursive: true });
runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));

const { ScreenAnalysisParentSettingSchema, ScreenAnalysisResultSchema, ScreenEvidenceSchemaVersion } =
  await import('../../packages/activity-domain/dist/screen-evidence.js');

const observedAt = '2026-06-03T21:05:00.000Z';
const evidenceRef = {
  evidenceId: 'screen-observe-policy-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-observe-policy',
  uri: null,
};

const observeOnlySetting = ScreenAnalysisParentSettingSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisEnabled: true,
  analysisMode: 'observeOnly',
  cadenceCaptureEnabled: false,
  cadenceSeconds: 300,
  strictModeEnabled: false,
  triggerCaptureEnabled: true,
  enabledTriggers: ['manualParentTestCapture'],
  allowedCaptureScope: 'activeWindow',
  ocrTextEnabled: true,
  ocrTextSnippetLimit: 3,
  redactionMode: 'localSensitiveText',
  temporaryImageTtlSeconds: 300,
  maxRetryCount: 2,
  deleteAfterSuccess: true,
  deleteAfterExpiry: true,
  retainRawImage: false,
  policyUseEnabled: false,
  changedByParentRef: 'screen-observe-policy-setting',
  changedAt: observedAt,
  settingVersion: 1,
  reason: 'parent selected observe only mode',
});

const policyEnabledObserveRejected = !ScreenAnalysisParentSettingSchema.safeParse({
  ...observeOnlySetting,
  policyUseEnabled: true,
}).success;

const observeResult = ScreenAnalysisResultSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisResultId: 'screen-analysis-observe-only',
  queueJobId: 'screen-queue-observe-only',
  analyzedAt: observedAt,
  modelRuntimeRef: 'local-vision-runtime-observe-proof',
  modelId: 'local-vision-observe-proof',
  providerKind: 'localVision',
  promptOrTemplateVersion: 'screen-observe-policy-v1',
  captureReason: 'manualParentTestCapture',
  captureScope: 'activeWindow',
  capabilityStatus: 'ready',
  summary: 'A school activity surface is visible, but parent setting is observe only.',
  visibleCategoryCandidates: [
    {
      category: 'school',
      confidence: 0.91,
      evidenceRefs: [evidenceRef],
    },
  ],
  primaryCategory: 'school',
  riskSignals: [],
  ocrTextSnippets: [
    {
      text: 'school lesson visible',
      confidence: 0.91,
      evidenceRefs: [evidenceRef],
    },
  ],
  redactionNotes: [],
  confidence: 0.91,
  uncertaintyReason: null,
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: 'sha256:screen-observe-policy',
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: false,
});

if (!policyEnabledObserveRejected) {
  throw new Error('Expected observe-only mode to reject policyUseEnabled true');
}

const summary = {
  status: 'ok',
  proofKind: 'screen-ai-observe-only-policy-result',
  artifact: artifactSummaryPath,
  observeOnlySetting,
  observeResult,
  policyDecisionCreated: false,
  policyEnabledObserveRejected,
  assertions: [
    'Observe-only settings can allow local screen analysis while keeping policyUseEnabled false.',
    'Observe-only settings reject policyUseEnabled true, preventing policy decision handoff.',
    'The analysis result remains policyEligible false even with a known category and sufficient confidence.',
  ],
  nonClaims: [
    'This is an observe-only policy handoff guard proof.',
    'It does not claim final policy enforcement, parent notification UX, or live external account proof.',
  ],
};

await writeFile(artifactSummaryPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-ai-observe-policy-proof-ok ${artifactSummaryPath}`);

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
