import { describe, expect, it } from 'vitest';
import {
  buildScreenSummaryParentExplanationReadModelSnapshot,
  ScreenSummaryParentExplanationReadModelInputSchema,
  ScreenSummaryParentExplanationReadModelSnapshotSchema,
} from '../src/local-ai-screen-summary-parent-explanation-read-model';

const GeneratedAt = '2026-06-05T11:24:30.710Z';
const SourceRow = {
  ocrResultId: 'screen-winrt-ocr-result-live-wikipedia-browser-ocr',
  sourceQueueJobId: 'screen-winrt-ocr-job-live-wikipedia-browser-ocr',
  primaryCategory: 'school',
  imageDigest: '7d92d5f2e039dd9eef4e077c056476a6388b2b7fa1eae24f0bf3775d781e48f6',
  sourceImageDeletionState: 'deleted',
  sourceCustodyState: 'child-device-query-store',
  sourceRawImageRetained: false,
  contextState: 'ready',
  readiness: 'ready-for-parent-audit',
  screenSummaryRefs: ['screen-winrt-ocr-result-live-wikipedia-browser-ocr-screen-summary-ref'],
  auditEvidenceReferences: ['screen-winrt-ocr-result-live-wikipedia-browser-ocr-query-store-summary'],
  policyDecisionRef: 'screen-winrt-ocr-policy-live-wikipedia-browser-ocr',
  policyAction: 'allow',
  policyReasonCodes: ['screen-winrt-ocr-school'],
  policyDryRun: true,
  enforcementHandoffState: 'disabled',
  parentRuleRefs: ['screen-winrt-ocr-rule-school'],
  localModelRuntimeRefs: ['windows-winrt-ocr-local-runtime-parent-explanation'],
  custodyLabels: ['child-device-query-store'],
  deletionReasons: ['screen-image-deleted'],
  explanationReasons: [
    'screen-summary-evidence-cited',
    'parent-rule-cited',
    'dry-run-policy-cited',
    'image-deleted',
    'local-only-custody',
    'remote-ai-not-used',
    'enforcement-not-claimed',
  ],
  claimBoundaries: noSourceClaims(),
} as const;

describe('local AI screen-summary parent explanation read-model contracts', () => {
  it('builds parent-visible read-model rows while preserving evidence, policy, runtime, custody, and deletion refs', () => {
    expectReadModelSnapshotPreservesRefs();
  });

  it('rejects source rows that are not deleted local-only screen-summary explanation rows', () => {
    expectInvalidSourceRowsRejected();
  });

  it('rejects read-model snapshots that claim rendered raw images, remote AI, portal runtime, or enforcement', () => {
    expectInvalidReadModelClaimsRejected();
  });

  it('rejects stale summary counts that do not match rendered read-model rows', () => {
    expectStaleSummaryCountsRejected();
  });
});

function expectReadModelSnapshotPreservesRefs() {
  const snapshot = buildScreenSummaryParentExplanationReadModelSnapshot(readModelInput());
  const [row] = snapshot.rows;

  expect(snapshot.summary).toEqual({
    rowCount: 1,
    readyRowCount: 1,
    screenSummaryRefCount: 1,
    localOnly: true,
    rawImageShown: false,
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
  });
  expect(row.displayState).toBe('ready-for-parent-explanation');
  expect(row.sourceOcrResultRef).toBe(SourceRow.ocrResultId);
  expect(row.sourceQueueJobRef).toBe(SourceRow.sourceQueueJobId);
  expect(row.screenSummaryRefs).toEqual(SourceRow.screenSummaryRefs);
  expect(row.auditEvidenceRefs).toEqual(SourceRow.auditEvidenceReferences);
  expect(row.policyDecisionRef).toBe(SourceRow.policyDecisionRef);
  expect(row.policyReasonCodes).toEqual(SourceRow.policyReasonCodes);
  expect(row.parentRuleRefs).toEqual(SourceRow.parentRuleRefs);
  expect(row.localModelRuntimeRefs).toEqual(SourceRow.localModelRuntimeRefs);
  expect(row.custodyLabels).toEqual(['child-device-query-store']);
  expect(row.deletionReasons).toEqual(['screen-image-deleted']);
  expect(row.claimBoundaries).toEqual(noReadModelClaims());
}

function expectInvalidSourceRowsRejected() {
  const invalidRows = [
    { sourceImageDeletionState: 'temporary' },
    { sourceCustodyState: 'ocentra-hosted-non-activity' },
    { sourceRawImageRetained: true },
    { contextState: 'partial' },
    { readiness: 'manual-required' },
    { screenSummaryRefs: [] },
    { auditEvidenceReferences: [] },
    { policyDryRun: false },
    { enforcementHandoffState: 'handed-off' },
    { custodyLabels: ['ocentra-hosted-non-activity'] },
    { deletionReasons: ['screen-deletion-unconfirmed'] },
  ];

  for (const invalidRow of invalidRows) {
    expect(
      ScreenSummaryParentExplanationReadModelInputSchema.safeParse({
        ...readModelInput(),
        sourceRows: [{ ...SourceRow, ...invalidRow }],
      }).success
    ).toBe(false);
  }
}

function expectInvalidReadModelClaimsRejected() {
  const snapshot = buildScreenSummaryParentExplanationReadModelSnapshot(readModelInput());
  const invalidClaims = [
    { rawImageShown: true },
    { rawImageRetained: true },
    { remoteAiUsed: true },
    { apiAiUsed: true },
    { policyAuthorityClaimed: true },
    { portalRuntimeClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const claim of invalidClaims) {
    expect(
      ScreenSummaryParentExplanationReadModelSnapshotSchema.safeParse({
        ...snapshot,
        rows: [{ ...snapshot.rows[0], claimBoundaries: { ...noReadModelClaims(), ...claim } }],
      }).success
    ).toBe(false);
    expect(
      ScreenSummaryParentExplanationReadModelSnapshotSchema.safeParse({
        ...snapshot,
        claimBoundaries: { ...noReadModelClaims(), ...claim },
      }).success
    ).toBe(false);
  }
}

function expectStaleSummaryCountsRejected() {
  const snapshot = buildScreenSummaryParentExplanationReadModelSnapshot(readModelInput());

  expect(
    ScreenSummaryParentExplanationReadModelSnapshotSchema.safeParse({
      ...snapshot,
      summary: { ...snapshot.summary, rowCount: 2 },
    }).success
  ).toBe(false);
  expect(
    ScreenSummaryParentExplanationReadModelSnapshotSchema.safeParse({
      ...snapshot,
      summary: { ...snapshot.summary, screenSummaryRefCount: 0 },
    }).success
  ).toBe(false);
}

function readModelInput() {
  return {
    schemaVersion: 'v0.6',
    snapshotId: 'screen-summary-parent-explanation-read-model-snapshot',
    generatedAt: GeneratedAt,
    sourceProof: 'output/ai-plan-proof/screen-summary-parent-explanation/proof-summary.json',
    sourceRows: [SourceRow],
    claimBoundaries: noReadModelClaims(),
  };
}

function noSourceClaims() {
  return {
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalRuntimeClaimed: false,
  };
}

function noReadModelClaims() {
  return {
    rawImageShown: false,
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
  };
}
