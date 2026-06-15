import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const aiOutputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'screen-vlm-journal-read-model-proof');
const pipelineOutputRoot = resolve(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'screen-vlm-journal-read-model-proof'
);
const testResultRoot = resolve(repoRoot, 'test-results', 'screen-ai-vlm-journal-read-model-proof');

await Promise.all([
  mkdir(aiOutputRoot, { recursive: true }),
  mkdir(pipelineOutputRoot, { recursive: true }),
  mkdir(testResultRoot, { recursive: true }),
]);

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'screen-vlm-journal-read-model',
  ])
);

const { ActivitySurfaceSchemaVersion } = await import('@ocentra-parent/activity-domain/activity-surface');
const {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
} = await import('@ocentra-parent/screen-domain/screen-vlm-worker');
const { screenVlmCompletedStatusFromResult, screenVlmQueueHandoffFromJob, screenVlmQueuedStatusFromHandoff } =
  await import('@ocentra-parent/screen-domain/screen-vlm-execution-readiness');
const {
  ScreenVlmJournalReadModelProofSchema,
  ScreenVlmJournalReadModelProofTier,
  ScreenVlmJournalReadModelSchemaVersion,
  screenVlmJournalLineFromCompletedStatus,
  screenVlmJournalReadModelProjection,
  screenVlmReadModelFromCompletedStatus,
} = await import('@ocentra-parent/screen-domain/screen-vlm-journal-read-model');

const evidenceRef = {
  evidenceId: 'screen-vlm-journal-read-model-proof-source',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-journal-read-model-proof-image',
  uri: null,
};
const request = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-vlm-journal-proof',
  },
  requestedAt: '2026-06-06T01:00:00.000Z',
  rangeStart: '2026-06-06T00:00:00.000Z',
  rangeEnd: '2026-06-06T01:00:00.000Z',
};
const job = ScreenVlmWorkerJobSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-journal-read-model-proof-job',
  createdAt: '2026-06-06T00:58:00.000Z',
  captureReason: 'timedCadence',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: evidenceRef.digest,
  encryptedImageRef: 'encrypted-temp-screen-vlm-journal-read-model-proof-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Classify this encrypted local selected-window capture for parent-visible screen history.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
});
const result = ScreenVlmWorkerResultSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-journal-read-model-proof-result',
  queueJobId: job.queueJobId,
  analyzedAt: '2026-06-06T00:58:06.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: job.captureReason,
  captureScope: job.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'school',
    confidence: 0.88,
    visible_text: 'A school assignment page is visible in the selected window.',
    risk_signals: [],
  },
  summary: 'The local VLM result classified the selected-window capture as school.',
  visibleCategoryCandidates: [{ category: 'school', confidence: 0.88, evidenceRefs: [evidenceRef] }],
  primaryCategory: 'school',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.88,
  uncertaintyReason: null,
  sourceEvidenceRefs: [evidenceRef],
  imageDigest: evidenceRef.digest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  localOnly: true,
  remoteAiUsed: false,
});
const handoff = screenVlmQueueHandoffFromJob({
  job,
  handoffId: 'screen-vlm-journal-read-model-proof-handoff',
  queuedAt: '2026-06-06T00:58:01.000Z',
  acceptedAt: '2026-06-06T00:58:02.000Z',
  statusReason: 'The encrypted selected-window capture is ready for local VLM journal projection.',
});
const queuedStatus = screenVlmQueuedStatusFromHandoff({
  handoff,
  statusId: 'screen-vlm-journal-read-model-proof-queued',
  updatedAt: '2026-06-06T00:58:02.000Z',
});
const completedStatus = screenVlmCompletedStatusFromResult({
  result,
  statusId: 'screen-vlm-journal-read-model-proof-completed',
});
const journalLine = screenVlmJournalLineFromCompletedStatus({
  statusRow: completedStatus,
  entryId: 'screen-vlm-journal-read-model-proof-entry',
  segmentId: 'screen-vlm-journal-read-model-proof-segment',
  writtenAt: '2026-06-06T00:58:07.000Z',
  nonce: 'screen-vlm-journal-read-model-proof-nonce',
  ciphertext: 'encrypted-screen-vlm-journal-read-model-proof-status-row',
});
const readModel = screenVlmReadModelFromCompletedStatus({
  statusRow: completedStatus,
  request,
  generatedAt: '2026-06-06T00:58:08.000Z',
  rowId: 'screen-vlm-journal-read-model-proof-row',
  label: 'School window classified by local VLM',
  deviceId: 'child-device-vlm-journal-proof',
  journalEntryId: journalLine.entryId,
  policyDecisionRef: 'screen-vlm-journal-read-model-proof-policy-dry-run',
  policyAction: 'allow',
  policyReasonCodes: ['school-content-allowed'],
  parentRuleRefs: ['parent-rule-school-hours'],
  parentExplanationRefs: ['parent-explanation-school-window'],
  explanationReasons: ['journal-entry-cited', 'deleted-image-custody-cited'],
});
const projection = screenVlmJournalReadModelProjection({
  projectionId: 'screen-vlm-journal-read-model-proof-projection',
  statusRow: completedStatus,
  journalLine,
  readModel,
});
const proof = ScreenVlmJournalReadModelProofSchema.parse({
  schemaVersion: ScreenVlmJournalReadModelSchemaVersion,
  proofId: 'screen-vlm-journal-read-model-proof',
  proofTier: ScreenVlmJournalReadModelProofTier,
  projections: [projection],
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
  portalRuntimeClaimed: false,
  enforcementClaimed: false,
});

const assertions = {
  packageExportWorks: typeof ScreenVlmJournalReadModelProofSchema.parse === 'function',
  queuedRowsDoNotProject: queuedStatus.result === null && queuedStatus.custodyState === 'child-device-temp-queue',
  completedStatusRequiresDeletedCustody:
    completedStatus.result?.imageDeletionState === 'deleted' &&
    completedStatus.custodyState === 'child-device-query-store',
  journalCitesCompletedStatus:
    String(journalLine.eventId) === String(completedStatus.statusId) &&
    String(journalLine.activityDigest) === String(result.imageDigest),
  readModelPreservesRuntimeRefs:
    readModel.rows[0]?.queueJobId === completedStatus.queueJobId &&
    readModel.rows[0]?.modelRuntimeRef === ScreenVlmWorkerRuntimeRef &&
    readModel.rows[0]?.modelId === ScreenVlmWorkerModelId &&
    readModel.rows[0]?.promptOrTemplateVersion === ScreenVlmWorkerTemplateVersion,
  readModelRejectsRawRetention: readModel.rows.every((row) => row.rawImageRetained === false),
  proofMakesNoPortalOrEnforcementClaim: !proof.portalRuntimeClaimed && !proof.enforcementClaimed,
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`screen VLM journal read-model proof assertions failed: ${JSON.stringify(assertions)}`);
}

const summary = {
  status: 'ok',
  proof: 'screen-ai-vlm-journal-read-model-proof',
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
  },
  projectionCount: proof.projections.length,
  journalLines: proof.projections.map((row) => ({
    entryId: row.journalLine.entryId,
    eventId: row.journalLine.eventId,
    cipher: row.journalLine.cipher,
    activityDigest: row.journalLine.activityDigest,
  })),
  readModelRows: readModel.rows.map((row) => ({
    rowId: row.rowId,
    queueJobId: row.queueJobId,
    modelRuntimeRef: row.modelRuntimeRef,
    modelId: row.modelId,
    promptOrTemplateVersion: row.promptOrTemplateVersion,
    providerKind: row.providerKind,
    primaryCategory: row.primaryCategory,
    policyEligible: row.policyEligible,
    policyDecisionRef: row.policyDecisionRef,
    imageDeletionState: row.imageDeletionState,
    rawImageRetained: row.rawImageRetained,
    custodyState: row.custodyState,
    evidenceKinds: row.evidence.map((ref) => ref.kind),
  })),
  assertions,
  validationCommands: [
    'npm run build --workspace @ocentra-parent/activity-domain',
    'npm run test --workspace @ocentra-parent/activity-domain -- screen-vlm-journal-read-model',
    'node scripts/test/screen-ai-vlm-journal-read-model-proof.mjs',
  ],
  nonClaims: [
    'This proof uses schema-backed VLM readiness status rows; it does not execute a live VLM model.',
    'It proves encrypted journal and Activity Screen read-model projection only; it does not render production portal UI.',
    'It does not claim policy authority, enforcement adapter execution, production VLM quality, remote/API AI, or raw image retention.',
  ],
};

await Promise.all([
  writeFile(join(aiOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(pipelineOutputRoot, 'proof-summary.json'), `${JSON.stringify(summary, null, 2)}\n`),
  writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(summary, null, 2)}\n`),
]);

console.log(
  `screen-ai-vlm-journal-read-model-proof-ok:${proof.projections.length}:${join('output', 'ai-plan-proof', 'screen-vlm-journal-read-model-proof', 'proof-summary.json')}`
);

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
