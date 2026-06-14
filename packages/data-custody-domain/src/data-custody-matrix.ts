import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import {
  DataCustodyAuthority,
  DataCustodyAuthoritySchema,
  DataCustodyClassId,
  DataCustodyClassIdSchema,
  DataCustodyDefaultLocation,
  DataCustodyDefaultLocationSchema,
  DataCustodyHostingPolicySchema,
  DataCustodyOcentraHostingMode,
  DataCustodySourceOfTruth,
  DataCustodySourceOfTruthSchema,
  SeededDataCustodyClassIds,
} from './custody-boundary';

export const DataCustodySourceOfTruthMatrixIdSchema = brandedNonEmptyStringSchema(
  'DataCustodySourceOfTruthMatrixId'
);
export const DataCustodySourceOfTruthMatrixRowIdSchema = brandedNonEmptyStringSchema(
  'DataCustodySourceOfTruthMatrixRowId'
);

const DataCustodySourceOfTruthMatrixRowBaseSchema = Schema.Struct({
  rowId: DataCustodySourceOfTruthMatrixRowIdSchema,
  classId: DataCustodyClassIdSchema,
  sourceOfTruth: DataCustodySourceOfTruthSchema,
  defaultLocation: DataCustodyDefaultLocationSchema,
  parentActionRequired: Schema.Boolean,
  ocentraHostedByDefault: Schema.Boolean,
  rawChildEvidenceAllowed: Schema.Boolean,
  custodyAuthority: DataCustodyAuthoritySchema,
  hostingPolicy: DataCustodyHostingPolicySchema,
});

type DataCustodySourceOfTruthMatrixRowCandidate = Infer<
  typeof DataCustodySourceOfTruthMatrixRowBaseSchema
>;
type DataCustodyClassIdValue = Infer<typeof DataCustodyClassIdSchema>;

const OcentraHostedDefaultDeniedClassIds: ReadonlySet<DataCustodyClassIdValue> = new Set([
  DataCustodyClassId.EncryptedJournalSegment,
  DataCustodyClassId.SqliteQueryRow,
  DataCustodyClassId.NotificationHistory,
  DataCustodyClassId.AuditEvent,
  DataCustodyClassId.GeneratedSummary,
]);

const RawChildEvidenceDefaultDeniedClassIds: ReadonlySet<DataCustodyClassIdValue> = new Set([
  DataCustodyClassId.NotificationHistory,
  DataCustodyClassId.AuditEvent,
  DataCustodyClassId.GeneratedSummary,
]);

export const DataCustodySourceOfTruthMatrixRowSchema = withParser(
  DataCustodySourceOfTruthMatrixRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        dataCustodySourceAlignmentIsCanonical(row) ||
        'Expected encrypted journal rows to be self-owned, SQLite rows to derive from journal, generated summaries to derive from SQLite, and other rows to remain locally authoritative'
    ),
    Schema.filter(
      (row) =>
        dataCustodyHostedDefaultIsAllowed(row) ||
        'Expected journal, query, notification, audit, and generated summary rows to stay local or parent-owned by default instead of Ocentra-hosted'
    ),
    Schema.filter(
      (row) =>
        dataCustodyRawEvidenceDefaultIsAllowed(row) ||
        'Expected notification, audit, and generated summary rows to exclude raw child evidence by default'
    ),
    Schema.filter(
      (row) =>
        dataCustodyHostedModeMatchesDefault(row) ||
        'Expected hosted modes to stay limited when Ocentra is not the default custody location'
    )
  )
);

const DataCustodySourceOfTruthMatrixBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  matrixId: DataCustodySourceOfTruthMatrixIdSchema,
  rows: Schema.Array(DataCustodySourceOfTruthMatrixRowSchema),
});

export const DataCustodySourceOfTruthMatrixSchema = withParser(
  DataCustodySourceOfTruthMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.rows.map((row) => row.rowId)).size === matrix.rows.length ||
        'Expected data custody matrix row ids to be unique'
    ),
    Schema.filter(
      (matrix) =>
        matrix.rows.length === SeededDataCustodyClassIds.length &&
        new Set(matrix.rows.map((row) => row.classId)).size === SeededDataCustodyClassIds.length ||
        'Expected exactly one matrix row for each seeded data custody class'
    )
  )
);

export type DataCustodySourceOfTruthMatrixId = Infer<typeof DataCustodySourceOfTruthMatrixIdSchema>;
export type DataCustodySourceOfTruthMatrixRowId = Infer<
  typeof DataCustodySourceOfTruthMatrixRowIdSchema
>;
export type DataCustodySourceOfTruthMatrixRow = Infer<typeof DataCustodySourceOfTruthMatrixRowSchema>;
export type DataCustodySourceOfTruthMatrix = Infer<typeof DataCustodySourceOfTruthMatrixSchema>;

export const CanonicalDataCustodySourceOfTruthMatrix =
  DataCustodySourceOfTruthMatrixSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    matrixId: 'data-custody-source-of-truth-wp01',
    rows: [
      {
        rowId: 'custody-row-encrypted-journal-segment',
        classId: DataCustodyClassId.EncryptedJournalSegment,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.ChildDeviceEncryptedJournal,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: true,
        custodyAuthority: DataCustodyAuthority.ChildDevice,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-sqlite-query-row',
        classId: DataCustodyClassId.SqliteQueryRow,
        sourceOfTruth: DataCustodySourceOfTruth.derivedFromDataClass(
          DataCustodyClassId.EncryptedJournalSegment
        ),
        defaultLocation: DataCustodyDefaultLocation.ChildDeviceLocalQueryStore,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.ChildDevice,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-parent-rule',
        classId: DataCustodyClassId.ParentRule,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.HouseholdLocalRuleStore,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.HouseholdLocalDevices,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-approval-decision',
        classId: DataCustodyClassId.ApprovalDecision,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.HouseholdLocalApprovalStore,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.HouseholdLocalDevices,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-device-registry-entry',
        classId: DataCustodyClassId.DeviceRegistryEntry,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.HouseholdLocalDeviceRegistry,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.HouseholdLocalDevices,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-notification-history',
        classId: DataCustodyClassId.NotificationHistory,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.ParentDeviceNotificationHistoryCache,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.ParentDevice,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.MinimalRoutingMetadataOnly,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: true,
        },
      },
      {
        rowId: 'custody-row-audit-event',
        classId: DataCustodyClassId.AuditEvent,
        sourceOfTruth: DataCustodySourceOfTruth.self(),
        defaultLocation: DataCustodyDefaultLocation.HouseholdLocalAuditStore,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.HouseholdLocalDevices,
        hostingPolicy: {
          ocentraHostingMode: DataCustodyOcentraHostingMode.Forbidden,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
      {
        rowId: 'custody-row-generated-summary',
        classId: DataCustodyClassId.GeneratedSummary,
        sourceOfTruth: DataCustodySourceOfTruth.derivedFromDataClass(
          DataCustodyClassId.SqliteQueryRow
        ),
        defaultLocation: DataCustodyDefaultLocation.ParentDeviceGeneratedSummaryCache,
        parentActionRequired: true,
        ocentraHostedByDefault: false,
        rawChildEvidenceAllowed: false,
        custodyAuthority: DataCustodyAuthority.ParentDevice,
        hostingPolicy: {
          ocentraHostingMode:
            DataCustodyOcentraHostingMode.ParentAuthorizedStatelessDerivationOnly,
          parentOwnedStorageAllowed: true,
          providerMetadataAllowed: false,
        },
      },
    ],
  });

export function parseDataCustodySourceOfTruthMatrix(input: unknown): DataCustodySourceOfTruthMatrix {
  return DataCustodySourceOfTruthMatrixSchema.parse(input);
}

export function getDataCustodySourceOfTruthMatrixRow(
  classId: DataCustodyClassIdValue
): DataCustodySourceOfTruthMatrixRow {
  const row = CanonicalDataCustodySourceOfTruthMatrix.rows.find(
    (candidate) => candidate.classId === classId
  );

  if (!row) {
    throw new Error(`Missing canonical data custody row for ${classId}`);
  }

  return row;
}

function dataCustodySourceAlignmentIsCanonical(
  row: DataCustodySourceOfTruthMatrixRowCandidate
): boolean {
  if (row.classId === DataCustodyClassId.EncryptedJournalSegment) {
    return row.sourceOfTruth.kind === 'self' && row.sourceOfTruth.sourceClassId === null;
  }

  if (row.classId === DataCustodyClassId.SqliteQueryRow) {
    return (
      row.sourceOfTruth.kind === 'derived-from-data-class' &&
      row.sourceOfTruth.sourceClassId === DataCustodyClassId.EncryptedJournalSegment
    );
  }

  if (row.classId === DataCustodyClassId.GeneratedSummary) {
    return (
      row.sourceOfTruth.kind === 'derived-from-data-class' &&
      row.sourceOfTruth.sourceClassId === DataCustodyClassId.SqliteQueryRow
    );
  }

  return row.sourceOfTruth.kind === 'self' && row.sourceOfTruth.sourceClassId === null;
}

function dataCustodyHostedDefaultIsAllowed(
  row: DataCustodySourceOfTruthMatrixRowCandidate
): boolean {
  if (OcentraHostedDefaultDeniedClassIds.has(row.classId)) {
    return !row.ocentraHostedByDefault;
  }

  return true;
}

function dataCustodyRawEvidenceDefaultIsAllowed(
  row: DataCustodySourceOfTruthMatrixRowCandidate
): boolean {
  if (RawChildEvidenceDefaultDeniedClassIds.has(row.classId)) {
    return !row.rawChildEvidenceAllowed;
  }

  return true;
}

function dataCustodyHostedModeMatchesDefault(
  row: DataCustodySourceOfTruthMatrixRowCandidate
): boolean {
  if (row.ocentraHostedByDefault) {
    return row.hostingPolicy.ocentraHostingMode !== DataCustodyOcentraHostingMode.Forbidden;
  }

  if (row.rawChildEvidenceAllowed) {
    return row.hostingPolicy.ocentraHostingMode === DataCustodyOcentraHostingMode.Forbidden;
  }

  return true;
}
