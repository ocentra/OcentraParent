import { describe, expect, it } from 'vitest';
import { DataCustodyClassId } from '../../src/custody-boundary';
import {
  CanonicalDataCustodySourceOfTruthMatrix,
  DataCustodySourceOfTruthMatrixRowSchema,
  DataCustodySourceOfTruthMatrixSchema,
  getDataCustodySourceOfTruthMatrixRow,
  parseDataCustodySourceOfTruthMatrix,
} from '../../src/data-custody-matrix';

describe('data custody source-of-truth matrix', () => {
  it('parses the seeded matrix and assigns exactly one row per seeded class', () => {
    const parsed = parseDataCustodySourceOfTruthMatrix(CanonicalDataCustodySourceOfTruthMatrix);
    const expectedClassIds = Object.values(DataCustodyClassId);

    expect(parsed.rows).toHaveLength(expectedClassIds.length);

    for (const classId of expectedClassIds) {
      expect(parsed.rows.filter((row) => row.classId === classId)).toHaveLength(1);
      expect(getDataCustodySourceOfTruthMatrixRow(classId).classId).toBe(classId);
    }
  });

  it('rejects missing classes', () => {
    const result = DataCustodySourceOfTruthMatrixSchema.safeParse({
      ...CanonicalDataCustodySourceOfTruthMatrix,
      rows: CanonicalDataCustodySourceOfTruthMatrix.rows.filter(
        (row) => row.classId !== DataCustodyClassId.GeneratedSummary
      ),
    });

    expect(result.success).toBe(false);
  });

  it('rejects empty ids', () => {
    const emptyMatrixId = DataCustodySourceOfTruthMatrixSchema.safeParse({
      ...CanonicalDataCustodySourceOfTruthMatrix,
      matrixId: '',
    });
    const emptyRowId = DataCustodySourceOfTruthMatrixRowSchema.safeParse({
      ...getDataCustodySourceOfTruthMatrixRow(DataCustodyClassId.EncryptedJournalSegment),
      rowId: '',
    });

    expect(emptyMatrixId.success).toBe(false);
    expect(emptyRowId.success).toBe(false);
  });

  it('rejects journal, query, report, notification, and audit rows as Ocentra-hosted defaults', () => {
    const hostedDefaultDeniedClassIds = [
      DataCustodyClassId.EncryptedJournalSegment,
      DataCustodyClassId.SqliteQueryRow,
      DataCustodyClassId.GeneratedSummary,
      DataCustodyClassId.NotificationHistory,
      DataCustodyClassId.AuditEvent,
    ] as const;

    for (const classId of hostedDefaultDeniedClassIds) {
      expect(
        DataCustodySourceOfTruthMatrixRowSchema.safeParse({
          ...getDataCustodySourceOfTruthMatrixRow(classId),
          ocentraHostedByDefault: true,
        }).success
      ).toBe(false);
    }
  });

  it('defaults notification, audit, and generated summary rows to no raw child evidence', () => {
    const noRawDefaultClassIds = [
      DataCustodyClassId.NotificationHistory,
      DataCustodyClassId.AuditEvent,
      DataCustodyClassId.GeneratedSummary,
    ] as const;

    for (const classId of noRawDefaultClassIds) {
      const row = getDataCustodySourceOfTruthMatrixRow(classId);

      expect(row.rawChildEvidenceAllowed).toBe(false);
      expect(
        DataCustodySourceOfTruthMatrixRowSchema.safeParse({
          ...row,
          rawChildEvidenceAllowed: true,
        }).success
      ).toBe(false);
    }
  });
});
