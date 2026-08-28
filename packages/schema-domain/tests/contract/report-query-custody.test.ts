import { describe, expect, it } from 'vitest';
import {
  GeneratedReportQueryCustodyContractProof as generatedProof,
  GeneratedReportQueryCustodyMaxPageSize,
  GeneratedReportQueryCustodyStates,
  type GeneratedReportQueryCustodyContractProof,
  type GeneratedReportQueryCustodyRow,
} from '../../src/generated-report-query-custody-contracts';
import {
  reportQueryCustodyProofIsHonestGenerated,
  reportQueryCustodyRequestIsHonestGenerated,
  reportQueryCustodyRowIsHonestGenerated,
} from '../../src/generated-report-query-custody-contract-rules';
import {
  ReportQueryCustodyProofReadModel,
  ReportQueryCustodyProofSchema,
  summarizeReportQueryCustodyStates,
} from '../../src/report-query-custody';

describe('Rust-owned report/query custody contract edge', () => {
  it('parses the generated proof and preserves every required outcome state', () => {
    expect(ReportQueryCustodyProofSchema.parse(generatedProof)).toEqual(
      ReportQueryCustodyProofReadModel
    );
    expect(reportQueryCustodyProofIsHonestGenerated(generatedProof)).toBe(true);
    expect(summarizeReportQueryCustodyStates(ReportQueryCustodyProofReadModel.rows)).toEqual({
      derivedFresh: 1,
      derivedStale: 1,
      partiallyRedacted: 1,
      deletedSource: 1,
      syncConflict: 1,
      cursorExpired: 1,
      rateLimited: 1,
    });
    expect(generatedProof.rows).toHaveLength(GeneratedReportQueryCustodyStates.length);
  });

  it('rejects requested scopes that are not contained by the allowed scope', () => {
    const request = {
      ...generatedProof.request,
      requestedDataClasses: ['generated-summary'] as const,
      allowedSourceDataClasses: ['sqlite-query-row'] as const,
    };

    expect(reportQueryCustodyRequestIsHonestGenerated(request)).toBe(false);
  });

  it('binds each row source class to both request scopes', () => {
    const row = generatedProof.rows[0];
    expect(row?.state).toBe('derivedFresh');
    expect(row?.sourceDataClass).toBe('sqlite-query-row');

    const unboundRow: GeneratedReportQueryCustodyRow = {
      ...row!,
      sourceDataClass: 'generated-summary',
      requestedDataClasses: ['sqlite-query-row'],
      allowedSourceDataClasses: ['sqlite-query-row'],
    };

    expect(reportQueryCustodyRowIsHonestGenerated(unboundRow)).toBe(false);
    expect(
      reportQueryCustodyProofIsHonestGenerated({
        ...generatedProof,
        rows: [unboundRow, ...generatedProof.rows.slice(1)],
      })
    ).toBe(false);
  });

  it('keeps row scopes, citation arrays, and authority generation exactly bound to the request', () => {
    const row = generatedProof.rows[0]!;
    const scopeMismatch = {
      ...generatedProof,
      rows: [
        {
          ...row,
          requestedDataClasses: [...row.requestedDataClasses].reverse(),
          allowedSourceDataClasses: [...row.allowedSourceDataClasses].reverse(),
        },
        ...generatedProof.rows.slice(1),
      ],
    } as unknown as GeneratedReportQueryCustodyContractProof;
    const citationMismatch = {
      ...generatedProof,
      rows: [
        {
          ...row,
          sourceCitationRefs: row.sourceCitationRefs.map((citation, index) =>
            index === 0
              ? {
                  ...citation,
                  sourceReference:
                    'unbound-source-reference' as unknown as typeof citation.sourceReference,
                }
              : citation
          ),
        },
        ...generatedProof.rows.slice(1),
      ],
    } as unknown as GeneratedReportQueryCustodyContractProof;
    const assistantCitationMismatch = {
      ...generatedProof,
      rows: [
        {
          ...row,
          assistantCitationRefs: row.assistantCitationRefs.map((citation, index) =>
            index === 0
              ? {
                  ...citation,
                  sourceReference:
                    'unbound-assistant-reference' as unknown as typeof citation.sourceReference,
                }
              : citation
          ),
        },
        ...generatedProof.rows.slice(1),
      ],
    } as unknown as GeneratedReportQueryCustodyContractProof;
    const generationMismatch = {
      ...generatedProof,
      rows: [
        {
          ...row,
          parentAuthority: { ...row.parentAuthority, authorityGeneration: 2 },
        },
        ...generatedProof.rows.slice(1),
      ],
    } as unknown as GeneratedReportQueryCustodyContractProof;

    expect(reportQueryCustodyProofIsHonestGenerated(scopeMismatch)).toBe(false);
    expect(reportQueryCustodyProofIsHonestGenerated(citationMismatch)).toBe(false);
    expect(reportQueryCustodyProofIsHonestGenerated(assistantCitationMismatch)).toBe(false);
    expect(reportQueryCustodyProofIsHonestGenerated(generationMismatch)).toBe(false);
  });

  it('rejects proofs whose result count exceeds the request page size or omits a required state', () => {
    const pageBoundRows = generatedProof.rows.map((row) => ({
      ...row,
      pageSize: 6,
    }));
    const pageBoundProof: GeneratedReportQueryCustodyContractProof = {
      ...generatedProof,
      request: { ...generatedProof.request, pageSize: 6 },
      rows: pageBoundRows,
    };
    const missingStateProof = {
      ...generatedProof,
      rows: generatedProof.rows.slice(0, -1),
    };

    expect(pageBoundProof.rows.length).toBeGreaterThan(pageBoundProof.request.pageSize);
    expect(reportQueryCustodyProofIsHonestGenerated(pageBoundProof)).toBe(false);
    expect(reportQueryCustodyProofIsHonestGenerated(missingStateProof)).toBe(false);
    expect(GeneratedReportQueryCustodyMaxPageSize).toBe(100);
  });

  it('rejects raw child evidence and dishonest notification boundaries', () => {
    const rawChildEvidenceProof = {
      ...generatedProof,
      request: { ...generatedProof.request, rawChildEvidenceRequested: true },
    } as unknown as GeneratedReportQueryCustodyContractProof;
    const dishonestBoundaryProof = {
      ...generatedProof,
      request: {
        ...generatedProof.request,
        notificationPayloadBoundary:
          'not-parent-owned' as unknown as typeof generatedProof.request.notificationPayloadBoundary,
      },
    } as unknown as GeneratedReportQueryCustodyContractProof;

    expect(reportQueryCustodyProofIsHonestGenerated(rawChildEvidenceProof)).toBe(false);
    expect(reportQueryCustodyProofIsHonestGenerated(dishonestBoundaryProof)).toBe(false);
  });
});
