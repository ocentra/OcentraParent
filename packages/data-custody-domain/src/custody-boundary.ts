import {
  FamilyReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';

export const DataCustodyRecordIdSchema = brandedNonEmptyStringSchema('DataCustodyRecordId');
export const DataCustodyStoreRefSchema = brandedNonEmptyStringSchema('DataCustodyStoreRef');
export const DataCustodyRetentionPolicyIdSchema = brandedNonEmptyStringSchema('DataCustodyRetentionPolicyId');

export const DataCustodyStateLiteral = {
  LocalOnly: 'local-only',
  FamilyShared: 'family-shared',
  ExportReady: 'export-ready',
} as const;

export const DataCustodyRawPayloadStateLiteral = {
  Excluded: 'excluded',
  RedactedOnly: 'redacted-only',
} as const;

export const DataCustodyRetentionDispositionLiteral = {
  Retain: 'retain',
  DeleteEligible: 'delete-eligible',
  DeleteRequested: 'delete-requested',
} as const;

export const DataCustodyStateSchema = withParser(
  Schema.Literal(
    DataCustodyStateLiteral.LocalOnly,
    DataCustodyStateLiteral.FamilyShared,
    DataCustodyStateLiteral.ExportReady
  )
);

export const DataCustodyRawPayloadStateSchema = withParser(
  Schema.Literal(
    DataCustodyRawPayloadStateLiteral.Excluded,
    DataCustodyRawPayloadStateLiteral.RedactedOnly
  )
);

export const DataCustodyRetentionDispositionSchema = withParser(
  Schema.Literal(
    DataCustodyRetentionDispositionLiteral.Retain,
    DataCustodyRetentionDispositionLiteral.DeleteEligible,
    DataCustodyRetentionDispositionLiteral.DeleteRequested
  )
);

export const DataCustodyBoundarySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    evidence: ParentEvidenceReferenceSchema,
    recordId: DataCustodyRecordIdSchema,
    storeRef: DataCustodyStoreRefSchema,
    custodyState: DataCustodyStateSchema,
    rawPayloadState: DataCustodyRawPayloadStateSchema,
    retentionPolicyId: DataCustodyRetentionPolicyIdSchema,
    retentionDisposition: DataCustodyRetentionDispositionSchema,
  })
);

export type DataCustodyState = Infer<typeof DataCustodyStateSchema>;
export type DataCustodyRawPayloadState = Infer<typeof DataCustodyRawPayloadStateSchema>;
export type DataCustodyRetentionDisposition = Infer<typeof DataCustodyRetentionDispositionSchema>;
export type DataCustodyBoundary = Infer<typeof DataCustodyBoundarySchema>;

export const DataCustodyState = {
  LocalOnly: DataCustodyStateSchema.parse(DataCustodyStateLiteral.LocalOnly),
  FamilyShared: DataCustodyStateSchema.parse(DataCustodyStateLiteral.FamilyShared),
  ExportReady: DataCustodyStateSchema.parse(DataCustodyStateLiteral.ExportReady),
} as const;

export const DataCustodyRawPayloadState = {
  Excluded: DataCustodyRawPayloadStateSchema.parse(DataCustodyRawPayloadStateLiteral.Excluded),
  RedactedOnly: DataCustodyRawPayloadStateSchema.parse(DataCustodyRawPayloadStateLiteral.RedactedOnly),
} as const;

export const DataCustodyRetentionDisposition = {
  Retain: DataCustodyRetentionDispositionSchema.parse(DataCustodyRetentionDispositionLiteral.Retain),
  DeleteEligible: DataCustodyRetentionDispositionSchema.parse(
    DataCustodyRetentionDispositionLiteral.DeleteEligible
  ),
  DeleteRequested: DataCustodyRetentionDispositionSchema.parse(
    DataCustodyRetentionDispositionLiteral.DeleteRequested
  ),
} as const;

export function parseDataCustodyBoundary(input: unknown): DataCustodyBoundary {
  return DataCustodyBoundarySchema.parse(input);
}
