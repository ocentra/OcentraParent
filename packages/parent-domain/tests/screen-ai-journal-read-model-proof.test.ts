import { describe, expect, it } from 'vitest';
import {
  buildScreenAiJournalReadModelSnapshot,
  ScreenAiJournalReadModelInputSchema,
  ScreenAiJournalReadModelSnapshotSchema,
} from '../src/screen-ai-journal-read-model-proof';

const GeneratedAt = '2026-06-05T15:59:33.027Z';
const SourceRow = {
  analysisRowId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1',
  queueJobId: 'screen-service-queue-job-1780675160-1',
  localAiResultId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-local-ocr-result',
  modelRuntimeRef: 'windows-winrt-ocr-local-runtime',
  modelId: 'windows-winrt-ocr',
  promptOrTemplateVersion: 'screen-ocr-worker-winrt-v1',
  primaryCategory: 'school',
  confidence: 0.91,
  imageDigest: 'FEpy0os_b5vD0hNYS3E0OQmvPIRoczSOyNDExPDFD-I',
  imageDeletionState: 'deleted',
  rawImageRetained: false,
  custodyState: 'child-device-journal',
  evidenceReferenceIds: [
    'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-activity-row',
    'screen-service-queue-job-1780675160-1-encrypted-queue',
    'FEpy0os_b5vD0hNYS3E0OQmvPIRoczSOyNDExPDFD-I-screen-summary',
    'screen-service-adapter-evidence-screen-service-queue-job-1780675160-1-journal',
  ],
  policyDecisionRef: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-policy-dry-run',
  policyAction: 'allow',
  policyReasonCodes: ['screen-service-winrt-ocr-school-allow'],
  policyDryRun: true,
  enforcementHandoffState: 'disabled',
  parentRuleRefs: ['screen-service-winrt-ocr-school-rule'],
  readModelRowId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1',
  readModelRawImageRetained: false,
  readModelImageDeletionState: 'deleted',
} as const;

describe('screen AI journal read-model proof contracts', () => {
  it('builds a journal/read-model snapshot that preserves real screen AI refs and custody', () => {
    expectJournalReadModelSnapshotPreservesRefs();
  });

  it('rejects source rows that are not deleted dry-run local screen AI read-model rows', () => {
    expectInvalidSourceRowsRejected();
  });

  it('rejects read-model snapshots that claim retention, remote AI, portal runtime, policy authority, or enforcement', () => {
    expectInvalidReadModelClaimsRejected();
  });

  it('rejects stale summary counts that no longer match journal/read-model rows', () => {
    expectStaleSummaryCountsRejected();
  });
});

function expectJournalReadModelSnapshotPreservesRefs() {
  const snapshot = buildScreenAiJournalReadModelSnapshot(readModelInput());
  const [row] = snapshot.rows;

  expect(snapshot.summary).toEqual({
    rowCount: 1,
    journaledRowCount: 1,
    sqliteProjectedRowCount: 1,
    deletedImageRowCount: 1,
    dryRunPolicyRowCount: 1,
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    enforcementClaimed: false,
  });
  expect(row.analysisRowRef).toBe(SourceRow.analysisRowId);
  expect(row.queueJobRef).toBe(SourceRow.queueJobId);
  expect(row.localAiResultRef).toBe(SourceRow.localAiResultId);
  expect(row.journalEntryRef).toBe(`${SourceRow.analysisRowId}-journal`);
  expect(row.sqliteRowRef).toBe(`${SourceRow.readModelRowId}-sqlite-read-model`);
  expect(row.modelRuntimeRef).toBe(SourceRow.modelRuntimeRef);
  expect(row.modelId).toBe(SourceRow.modelId);
  expect(row.promptOrTemplateVersion).toBe(SourceRow.promptOrTemplateVersion);
  expect(row.evidenceReferenceIds).toEqual(SourceRow.evidenceReferenceIds);
  expect(row.policyDecisionRef).toBe(SourceRow.policyDecisionRef);
  expect(row.parentRuleRefs).toEqual(SourceRow.parentRuleRefs);
  expect(row.imageDeletionState).toBe('deleted');
  expect(row.rawImageRetained).toBe(false);
  expect(row.claimBoundaries).toEqual(noClaims());
}

function expectInvalidSourceRowsRejected() {
  const invalidRows = [
    { imageDeletionState: 'retained' },
    { rawImageRetained: true },
    { custodyState: 'ocentra-hosted-non-activity' },
    { evidenceReferenceIds: [] },
    { policyReasonCodes: [] },
    { policyDryRun: false },
    { enforcementHandoffState: 'handed-off' },
    { parentRuleRefs: [] },
    { readModelRowId: 'different-row' },
    { readModelRawImageRetained: true },
    { readModelImageDeletionState: 'retained' },
  ];

  for (const invalidRow of invalidRows) {
    expect(
      ScreenAiJournalReadModelInputSchema.safeParse({
        ...readModelInput(),
        sourceRows: [{ ...SourceRow, ...invalidRow }],
      }).success
    ).toBe(false);
  }
}

function expectInvalidReadModelClaimsRejected() {
  const snapshot = buildScreenAiJournalReadModelSnapshot(readModelInput());
  const invalidClaims = [
    { rawImageRetained: true },
    { remoteAiUsed: true },
    { apiAiUsed: true },
    { policyAuthorityClaimed: true },
    { portalRuntimeClaimed: true },
    { enforcementClaimed: true },
    { runtimeSqliteWriterClaimed: true },
  ];

  for (const claim of invalidClaims) {
    expect(
      ScreenAiJournalReadModelSnapshotSchema.safeParse({
        ...snapshot,
        rows: [{ ...snapshot.rows[0], claimBoundaries: { ...noClaims(), ...claim } }],
      }).success
    ).toBe(false);
    expect(
      ScreenAiJournalReadModelSnapshotSchema.safeParse({
        ...snapshot,
        claimBoundaries: { ...noClaims(), ...claim },
      }).success
    ).toBe(false);
  }
}

function expectStaleSummaryCountsRejected() {
  const snapshot = buildScreenAiJournalReadModelSnapshot(readModelInput());

  expect(
    ScreenAiJournalReadModelSnapshotSchema.safeParse({
      ...snapshot,
      summary: { ...snapshot.summary, rowCount: 2 },
    }).success
  ).toBe(false);
  expect(
    ScreenAiJournalReadModelSnapshotSchema.safeParse({
      ...snapshot,
      summary: { ...snapshot.summary, sqliteProjectedRowCount: 0 },
    }).success
  ).toBe(false);
}

function readModelInput() {
  return {
    schemaVersion: 'v0.6',
    snapshotId: 'screen-ai-journal-read-model-snapshot',
    generatedAt: GeneratedAt,
    sourceProof: 'output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json',
    sourceRows: [SourceRow],
    claimBoundaries: noClaims(),
  };
}

function noClaims() {
  return {
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
    runtimeSqliteWriterClaimed: false,
  };
}
