import { describe, expect, it } from 'vitest';
import {
  ReportQueryCustodyKnownGaps,
  ReportQueryCustodyProofReadModel,
  ReportQueryCustodyProofSchema,
  ReportQueryCustodyRequestSchema,
  ReportQueryCustodyRowSchema,
  RequiredReportQueryCustodyStates,
  summarizeReportQueryCustodyStates,
} from '@ocentra-parent/schema-domain/report-query-custody';

describe('report query custody contracts', () => {
  derivedSourceMatrixProof();
  deletedExpiredNoLeakProof();
  queryCursorPaginationProof();
  queryRateLimitProof();
  notificationPayloadAllowDenyProof();
  assistantAllowedCitationProof();
  staleConflictStateProof();
});

function derivedSourceMatrixProof(): void {
  it('covers the derived source matrix without a second truth store', () => {
    const proof = ReportQueryCustodyProofSchema.parse(ReportQueryCustodyProofReadModel);

    expect(summarizeReportQueryCustodyStates(proof.rows)).toEqual({
      derivedFresh: 1,
      derivedStale: 1,
      partiallyRedacted: 1,
      deletedSource: 1,
      syncConflict: 1,
      cursorExpired: 1,
      rateLimited: 1,
    });
    expect(proof.rows.map((row) => row.state)).toEqual(RequiredReportQueryCustodyStates);
    expect(proof.request.allowedSourceDataClasses).toEqual([
      'sqlite-query-row',
      'notification-history',
      'audit-event',
      'generated-summary',
    ]);
    expect(proof.rows.map((row) => row.sourceDataClass)).toEqual([
      'sqlite-query-row',
      'generated-summary',
      'notification-history',
      'audit-event',
      'sqlite-query-row',
      'generated-summary',
      'notification-history',
    ]);
    expect(ReportQueryCustodyKnownGaps).toContain('No uncontrolled second truth store is claimed for report or query custody.');
  });
}

function deletedExpiredNoLeakProof(): void {
  it('rejects deleted or expired rows that would leak raw child evidence or lose explicit tombstone state', () => {
    const deletedRow = rowFor('deletedSource');
    const expiredRow = rowFor('cursorExpired');

    expect(deletedRow.deletedSourceAt).not.toBeNull();
    expect(deletedRow.deletedSourceRef).not.toBeNull();
    expect(deletedRow.tombstoneState).toBe('written');
    expect(deletedRow.rawChildEvidenceIncluded).toBe(false);
    expect(expiredRow.cursorExpiredAt).not.toBeNull();
    expect(expiredRow.rawChildEvidenceIncluded).toBe(false);
    expect(ReportQueryCustodyRowSchema.safeParse({ ...deletedRow, deletedSourceAt: null }).success).toBe(false);
    expect(ReportQueryCustodyRowSchema.safeParse({ ...expiredRow, cursorExpiredAt: null }).success).toBe(false);
  });
}

function queryCursorPaginationProof(): void {
  it('keeps pagination cursors stable and page ordering explicit', () => {
    const proof = ReportQueryCustodyProofSchema.parse(ReportQueryCustodyProofReadModel);

    expect(proof.request.pageSize).toBe(25);
    expect(proof.rows.map((row) => row.pageIndex)).toEqual([1, 2, 3, 4, 5, 6, 7]);
    expect(new Set(proof.rows.map((row) => row.cursorRef)).size).toBe(proof.rows.length);
    expect(proof.rows.filter((row) => row.nextCursorRef !== null).length).toBe(4);
    expect(proof.rows.filter((row) => row.nextCursorRef === null).length).toBe(3);
  });
}

function queryRateLimitProof(): void {
  it('rejects rate-limited rows that claim a second truth store or drop the rate limit boundary', () => {
    const rateLimitedRow = rowFor('rateLimited');

    expect(rateLimitedRow.rateLimitedUntilAt).not.toBeNull();
    expect(rateLimitedRow.secondTruthStoreClaimed).toBe(false);
    expect(ReportQueryCustodyRowSchema.safeParse({ ...rateLimitedRow, secondTruthStoreClaimed: true }).success).toBe(
      false
    );
    expect(ReportQueryCustodyRowSchema.safeParse({ ...rateLimitedRow, rateLimitedUntilAt: null }).success).toBe(false);
  });
}

function notificationPayloadAllowDenyProof(): void {
  it('keeps notification payloads inside the parent-owned citation boundary', () => {
    const proof = ReportQueryCustodyProofReadModel;
    const partiallyRedactedRow = rowFor('partiallyRedacted');

    expect(proof.rows.every((row) => row.notificationPayloadBoundary === 'parent-owned-citations-only')).toBe(true);
    expect(partiallyRedactedRow.payloadRedactionState).toBe('partially-redacted');
    expect(ReportQueryCustodyRowSchema.safeParse({ ...partiallyRedactedRow, rawChildEvidenceIncluded: true }).success).toBe(
      false
    );
    expect(
      ReportQueryCustodyRowSchema.safeParse({
        ...partiallyRedactedRow,
        notificationPayloadBoundary: 'parent-owned-citations-only',
        payloadRedactionState: 'fully-redacted',
      }).success
    ).toBe(false);
  });
}

function assistantAllowedCitationProof(): void {
  it('accepts only query-store-summary citations for assistant and report evidence contexts', () => {
    const proof = ReportQueryCustodyProofReadModel;

    expect(
      proof.request.sourceCitationRefs.every((citation) => citation.kind === 'query-store-summary')
    ).toBe(true);
    expect(
      proof.request.assistantCitationRefs.every((citation) => citation.kind === 'query-store-summary')
    ).toBe(true);
    expect(
      ReportQueryCustodyRequestSchema.safeParse({
        ...proof.request,
        sourceCitationRefs: [
          {
            evidenceReferenceId: 'bad-citation',
            kind: 'journal-event',
            observedAt: proof.request.sourceCitationRefs[0].observedAt,
          },
        ],
      }).success
    ).toBe(false);
  });
}

function staleConflictStateProof(): void {
  it('keeps derived stale and conflict states explicit and claim-safe', () => {
    const staleRow = rowFor('derivedStale');
    const conflictRow = rowFor('syncConflict');

    expect(staleRow.sourceFreshness).toBe('stale');
    expect(conflictRow.conflictRef).not.toBeNull();
    expect(conflictRow.claimSafe).toBe(true);
    expect(conflictRow.tombstoneState).toBe('not-required');
    expect(ReportQueryCustodyRowSchema.safeParse({ ...staleRow, sourceFreshness: 'fresh' }).success).toBe(false);
    expect(ReportQueryCustodyRowSchema.safeParse({ ...conflictRow, conflictRef: null }).success).toBe(false);
  });
}

function rowFor(state: (typeof RequiredReportQueryCustodyStates)[number]) {
  const row = ReportQueryCustodyProofReadModel.rows.find((candidate) => candidate.state === state);
  if (row === undefined) {
    throw new Error(`missing report query custody row: ${state}`);
  }
  return row;
}
