import { describe, expect, it } from 'vitest';
import {
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  DataCustodyBoundarySchema,
  DataCustodyRawPayloadState,
  DataCustodyRetentionDisposition,
  DataCustodyState,
  parseDataCustodyBoundary,
} from '../../src/custody-boundary';

const Boundary = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  family: {
    familyId: 'family-local-1',
  },
  evidence: {
    evidenceReferenceId: 'tracking-evidence-1',
    kind: ParentEvidenceReferenceKind.ActivityEvent,
    observedAt: '2026-06-01T00:00:00Z',
  },
  recordId: 'custody-record-1',
  storeRef: 'sqlite://tracking/evidence/1',
  custodyState: DataCustodyState.LocalOnly,
  rawPayloadState: DataCustodyRawPayloadState.Excluded,
  retentionPolicyId: 'tracking-retention-policy-default',
  retentionDisposition: DataCustodyRetentionDisposition.Retain,
} as const;

describe('data custody boundary contracts', () => {
  it('parses a parent-safe custody boundary with evidence refs only', () => {
    const parsed = parseDataCustodyBoundary(Boundary);

    expect(parsed.recordId).toBe('custody-record-1');
    expect(parsed.rawPayloadState).toBe(DataCustodyRawPayloadState.Excluded);
  });

  it('rejects missing custody record identity', () => {
    expect(
      DataCustodyBoundarySchema.safeParse({
        ...Boundary,
        recordId: '',
      }).success
    ).toBe(false);
  });
});
