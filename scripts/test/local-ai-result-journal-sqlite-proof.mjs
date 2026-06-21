import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-result-journal-sqlite-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-result-journal-sqlite-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build:contracts']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/ai-domain',
    '--',
    'local-ai-result-journal-sqlite-proof',
  ])
);

const journalModule = await import('@ocentra-parent/schema-domain/local-ai-result-journal-sqlite-proof');
const textModule = await import('@ocentra-parent/schema-domain/local-ai-text-inference-dry-run-proof');

const readyInput = localAiTextInferenceDryRunInput();
const readyResult = textModule.runLocalAiTextInferenceDryRun(readyInput).result;
const unavailableResult = textModule.runLocalAiTextInferenceDryRun({
  ...readyInput,
  modelRuntime: {
    ...readyInput.modelRuntime,
    executionState: 'disabled',
    loadState: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-provider-unconfigured',
  },
}).result;
const missingEvidenceResult = textModule.runLocalAiTextInferenceDryRun({
  ...readyInput,
  evaluationInput: {
    ...readyInput.evaluationInput,
    evidenceReferences: [],
  },
}).result;

const snapshot = journalModule.buildLocalAiResultReadModelSnapshot({
  generatedAt,
  readModelId: 'local-ai-result-read-model:screen-summary-text',
  sourceProofRefs: ['output/ai-plan-proof/local-ai-text-inference-dry-run/proof-summary.json'],
  results: [readyResult, unavailableResult, missingEvidenceResult],
});

const rejectionChecks = [
  {
    name: 'journal-raw-prompt-retention',
    rejected: !journalModule.LocalAiResultJournalEntrySchema.safeParse({
      ...snapshot.journalEntries[0],
      rawPromptRetained: true,
    }).success,
  },
  {
    name: 'sqlite-remote-api-overclaim',
    rejected: !journalModule.LocalAiResultSqliteRowSchema.safeParse({
      ...snapshot.sqliteRows[0],
      remoteApiClaimed: true,
    }).success,
  },
  {
    name: 'snapshot-count-mismatch',
    rejected: !journalModule.LocalAiResultReadModelSnapshotSchema.safeParse({
      ...snapshot,
      readyResultCount: 3,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-result-journal-sqlite-proof',
  generatedAt,
  output: relativePath(ProofPath),
  sourceProofRefs: snapshot.sourceProofRefs,
  readModelId: snapshot.readModelId,
  counts: {
    journalEntries: snapshot.journalEntries.length,
    sqliteRows: snapshot.sqliteRows.length,
    readyResultCount: snapshot.readyResultCount,
    degradedResultCount: snapshot.degradedResultCount,
    unavailableResultCount: snapshot.unavailableResultCount,
  },
  journalRows: snapshot.journalEntries.map(journalProofRow),
  sqliteRows: snapshot.sqliteRows.map(sqliteProofRow),
  assertions: {
    readyResultJournaledAndIngested:
      snapshot.journalEntries[0].journalState === 'journaled' && snapshot.sqliteRows[0].ingestState === 'ingested',
    unavailableResultVisibleWithoutPromotion:
      snapshot.journalEntries[1].journalState === 'unavailable' && snapshot.sqliteRows[1].ingestState === 'unavailable',
    missingEvidenceResultManualRequired:
      snapshot.journalEntries[2].journalState === 'manual-required' &&
      snapshot.sqliteRows[2].ingestState === 'manual-required',
    runtimeRefsPreserved: snapshot.journalEntries.every(
      (entry, index) => entry.runtimeReferenceId === snapshot.sqliteRows[index].runtimeReferenceId
    ),
    promptVersionsPreserved: snapshot.journalEntries.every(
      (entry, index) => entry.promptVersion === snapshot.sqliteRows[index].promptVersion
    ),
    noRawPromptRetention: snapshot.sqliteRows.every((row) => !row.rawPromptRetained),
    noRawModelOutputRetention: snapshot.sqliteRows.every((row) => !row.rawModelOutputRetained),
    noRemoteApiClaim: snapshot.sqliteRows.every((row) => !row.remoteApiClaimed),
    noPolicyAuthorityClaim: snapshot.sqliteRows.every((row) => !row.policyAuthorityClaimed),
    noEnforcementClaim: snapshot.sqliteRows.every((row) => !row.enforcementClaimed),
    malformedRowsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: snapshot.nonClaims,
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- local-ai-result-journal-sqlite-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-result-journal-sqlite-proof-ok:${snapshot.journalEntries.length}:${snapshot.sqliteRows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function journalProofRow(row) {
  return {
    journalEntryId: row.journalEntryId,
    sourceResultId: row.sourceResultId,
    action: row.action,
    confidence: row.confidence,
    journalState: row.journalState,
    evidenceReferenceCount: row.evidenceReferences.length,
    parentRuleReferenceCount: row.parentRuleReferences.length,
    runtimeReferenceId: row.runtimeReferenceId,
    providerId: row.providerId,
    modelId: row.modelId,
    promptVersion: row.promptVersion,
    rawPromptRetained: row.rawPromptRetained,
    rawModelOutputRetained: row.rawModelOutputRetained,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
  };
}

function sqliteProofRow(row) {
  return {
    sqliteRowId: row.sqliteRowId,
    journalEntryId: row.journalEntryId,
    sourceResultId: row.sourceResultId,
    action: row.action,
    confidence: row.confidence,
    ingestState: row.ingestState,
    evidenceReferenceCount: row.evidenceReferenceCount,
    parentRuleReferenceCount: row.parentRuleReferenceCount,
    runtimeReferenceId: row.runtimeReferenceId,
    providerId: row.providerId,
    modelId: row.modelId,
    promptVersion: row.promptVersion,
  };
}

function localAiTextInferenceDryRunInput() {
  return {
    schemaVersion: 'v0.6',
    dryRunId: 'local-ai-text-dry-run:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    modelRuntime: {
      runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
      providerId: 'local-provider-llama-cli',
      modelId: 'gemma-4-e2b-it-q4-k-m',
      modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
      privacyMode: 'local-only',
      adapterBoundary: 'local-adapter-ready',
      executionState: 'dry-run-ready',
      providerSource: 'local-model-cache',
      loadState: 'loaded',
      capabilityFlags: ['classification', 'safety-decision'],
      resourceClass: 'cpu',
      degradedState: 'none',
      lastCheckedAt: '2026-06-06T04:00:00.000Z',
      unavailableReason: null,
    },
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: 'local-ai-eval:screen-summary-wiki-ocr',
      childProfile: {
        childProfileId: 'child:maya',
        displayName: 'Maya',
      },
      device: {
        deviceId: 'device:maya-windows',
        childProfileId: 'child:maya',
        label: 'Maya Windows laptop',
        platform: 'windows',
      },
      currentObservation: {
        observationReferenceId: 'observation:screen-summary-wiki-ocr',
        contextKind: 'page',
        evidence: {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: 'local-provider-llama-cli',
        modelId: 'gemma-4-e2b-it-q4-k-m',
        promptVersion: 'prompt:screen-safety:v1',
      },
    },
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
