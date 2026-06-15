import { describe, expect, it } from 'vitest';
import {
  buildLocalAiResultReadModelSnapshot,
  LocalAiResultJournalEntrySchema,
  LocalAiResultReadModelSnapshotSchema,
  LocalAiResultSqliteRowSchema,
} from '../../src/local-ai-result-journal-sqlite-proof';
import { runLocalAiTextInferenceDryRun } from '../../src/local-ai-text-inference-dry-run-proof';

describe('local AI result journal SQLite proof', () => {
  it('journals and ingests ready local AI safety results with typed refs', provesReadyJournalIngest);
  it('keeps unavailable and missing-evidence results visible without promotion', provesFallbackRows);
  it('rejects raw retention, remote AI, policy authority, and enforcement overclaims', rejectsOverclaims);
  it('rejects snapshots with mismatched journal and SQLite refs', rejectsMismatchedSnapshotRefs);
});

const generatedAt = '2026-06-06T05:20:00.000Z';
const sourceProofRefs = ['output/ai-plan-proof/local-ai-text-inference-dry-run/proof-summary.json'];

function provesReadyJournalIngest() {
  const snapshot = readModelSnapshot();

  expect(snapshot.journalEntries.map((entry) => entry.journalState)).toEqual([
    'journaled',
    'unavailable',
    'manual-required',
  ]);
  expect(snapshot.sqliteRows.map((row) => row.ingestState)).toEqual(['ingested', 'unavailable', 'manual-required']);
  expect(snapshot.readyResultCount).toBe(1);
  expect(snapshot.degradedResultCount).toBe(1);
  expect(snapshot.unavailableResultCount).toBe(1);
  expect(snapshot.sqliteRows[0].action).toBe('warn');
  expect(snapshot.sqliteRows[0].confidence).toBe(0.62);
  expect(snapshot.sqliteRows[0].evidenceReferenceCount).toBe(1);
  expect(snapshot.sqliteRows[0].parentRuleReferenceCount).toBe(1);
  expect(snapshot.sqliteRows[0].rawPromptRetained).toBe(false);
  expect(snapshot.sqliteRows[0].rawModelOutputRetained).toBe(false);
  expect(snapshot.sqliteRows[0].remoteApiClaimed).toBe(false);
  expect(snapshot.sqliteRows[0].policyAuthorityClaimed).toBe(false);
  expect(snapshot.sqliteRows[0].enforcementClaimed).toBe(false);
}

function provesFallbackRows() {
  const snapshot = readModelSnapshot();
  const unavailableRow = snapshot.sqliteRows[1];
  const missingEvidenceRow = snapshot.sqliteRows[2];

  expect(unavailableRow.action).toBe('ask-parent');
  expect(unavailableRow.ingestState).toBe('unavailable');
  expect(missingEvidenceRow.action).toBe('unknown');
  expect(missingEvidenceRow.ingestState).toBe('manual-required');
  expect(snapshot.journalEntries[1].runtimeReferenceId).toBe(snapshot.sqliteRows[1].runtimeReferenceId);
  expect(snapshot.journalEntries[2].promptVersion).toBe(snapshot.sqliteRows[2].promptVersion);
}

function rejectsOverclaims() {
  const snapshot = readModelSnapshot();
  const validEntry = snapshot.journalEntries[0];
  const validRow = snapshot.sqliteRows[0];
  const invalidEntries = [
    { ...validEntry, rawPromptRetained: true },
    { ...validEntry, rawModelOutputRetained: true },
    { ...validEntry, remoteApiClaimed: true },
    { ...validEntry, policyAuthorityClaimed: true },
    { ...validEntry, enforcementClaimed: true },
    { ...validEntry, sourceProofRefs: [] },
  ];
  const invalidRows = [
    { ...validRow, rawPromptRetained: true },
    { ...validRow, rawModelOutputRetained: true },
    { ...validRow, remoteApiClaimed: true },
    { ...validRow, policyAuthorityClaimed: true },
    { ...validRow, enforcementClaimed: true },
    { ...validRow, sourceProofRefs: [] },
  ];

  for (const invalid of invalidEntries) {
    expect(LocalAiResultJournalEntrySchema.safeParse(invalid).success).toBe(false);
  }
  for (const invalid of invalidRows) {
    expect(LocalAiResultSqliteRowSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsMismatchedSnapshotRefs() {
  const snapshot = readModelSnapshot();

  expect(
    LocalAiResultReadModelSnapshotSchema.safeParse({
      ...snapshot,
      sqliteRows: [{ ...snapshot.sqliteRows[0], journalEntryId: 'local-ai-result-journal:missing' }],
    }).success
  ).toBe(false);
  expect(LocalAiResultReadModelSnapshotSchema.safeParse({ ...snapshot, readyResultCount: 3 }).success).toBe(false);
}

function readModelSnapshot() {
  return buildLocalAiResultReadModelSnapshot({
    generatedAt,
    readModelId: 'local-ai-result-read-model:screen-summary-text',
    sourceProofRefs,
    results: [readyResult(), unavailableResult(), missingEvidenceResult()],
  });
}

function readyResult() {
  return runLocalAiTextInferenceDryRun(readyDryRunInput()).result;
}

function unavailableResult() {
  return runLocalAiTextInferenceDryRun({
    ...readyDryRunInput(),
    modelRuntime: {
      ...readyRuntime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-provider-unconfigured',
    },
  }).result;
}

function missingEvidenceResult() {
  const input = readyDryRunInput();
  return runLocalAiTextInferenceDryRun({
    ...input,
    evaluationInput: {
      ...input.evaluationInput,
      evidenceReferences: [],
    },
  }).result;
}

const readyRuntime = {
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
};

function readyDryRunInput() {
  return {
    schemaVersion: 'v0.6',
    dryRunId: 'local-ai-text-dry-run:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    modelRuntime: readyRuntime,
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
