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

export const SeededDataCustodyClassIds = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const;

export const DataCustodyClassIdSchema = withParser(Schema.Literal(...SeededDataCustodyClassIds));
export type DataCustodyClassId = Infer<typeof DataCustodyClassIdSchema>;

export const DataCustodyClassId = {
  EncryptedJournalSegment: DataCustodyClassIdSchema.parse('encrypted-journal-segment'),
  SqliteQueryRow: DataCustodyClassIdSchema.parse('sqlite-query-row'),
  ParentRule: DataCustodyClassIdSchema.parse('parent-rule'),
  ApprovalDecision: DataCustodyClassIdSchema.parse('approval-decision'),
  DeviceRegistryEntry: DataCustodyClassIdSchema.parse('device-registry-entry'),
  NotificationHistory: DataCustodyClassIdSchema.parse('notification-history'),
  AuditEvent: DataCustodyClassIdSchema.parse('audit-event'),
  GeneratedSummary: DataCustodyClassIdSchema.parse('generated-summary'),
} as const;

const DataCustodySourceOfTruthKinds = ['self', 'derived-from-data-class'] as const;

export const DataCustodySourceOfTruthKindSchema = withParser(
  Schema.Literal(...DataCustodySourceOfTruthKinds)
);

const DataCustodySourceOfTruthBaseSchema = Schema.Struct({
  kind: DataCustodySourceOfTruthKindSchema,
  sourceClassId: Schema.Union(DataCustodyClassIdSchema, Schema.Null),
});

type DataCustodySourceOfTruthCandidate = Infer<typeof DataCustodySourceOfTruthBaseSchema>;

export const DataCustodySourceOfTruthSchema = withParser(
  DataCustodySourceOfTruthBaseSchema.pipe(
    Schema.filter(
      (source) =>
        dataCustodySourceOfTruthIsUnambiguous(source) ||
        'Expected source-of-truth to be self or derived from exactly one data class'
    )
  )
);

export type DataCustodySourceOfTruthKind = Infer<typeof DataCustodySourceOfTruthKindSchema>;
export type DataCustodySourceOfTruth = Infer<typeof DataCustodySourceOfTruthSchema>;

export const DataCustodySourceOfTruth = {
  self(): DataCustodySourceOfTruth {
    return DataCustodySourceOfTruthSchema.parse({
      kind: 'self',
      sourceClassId: null,
    });
  },
  derivedFromDataClass(sourceClassId: DataCustodyClassId): DataCustodySourceOfTruth {
    return DataCustodySourceOfTruthSchema.parse({
      kind: 'derived-from-data-class',
      sourceClassId,
    });
  },
} as const;

const DataCustodyDefaultLocations = [
  'child-device-encrypted-journal',
  'child-device-local-query-store',
  'household-local-rule-store',
  'household-local-approval-store',
  'household-local-device-registry',
  'parent-device-notification-history-cache',
  'household-local-audit-store',
  'parent-device-generated-summary-cache',
] as const;

export const DataCustodyDefaultLocationSchema = withParser(
  Schema.Literal(...DataCustodyDefaultLocations)
);
export type DataCustodyDefaultLocation = Infer<typeof DataCustodyDefaultLocationSchema>;

export const DataCustodyDefaultLocation = {
  ChildDeviceEncryptedJournal: DataCustodyDefaultLocationSchema.parse(
    'child-device-encrypted-journal'
  ),
  ChildDeviceLocalQueryStore: DataCustodyDefaultLocationSchema.parse(
    'child-device-local-query-store'
  ),
  HouseholdLocalRuleStore: DataCustodyDefaultLocationSchema.parse(
    'household-local-rule-store'
  ),
  HouseholdLocalApprovalStore: DataCustodyDefaultLocationSchema.parse(
    'household-local-approval-store'
  ),
  HouseholdLocalDeviceRegistry: DataCustodyDefaultLocationSchema.parse(
    'household-local-device-registry'
  ),
  ParentDeviceNotificationHistoryCache: DataCustodyDefaultLocationSchema.parse(
    'parent-device-notification-history-cache'
  ),
  HouseholdLocalAuditStore: DataCustodyDefaultLocationSchema.parse(
    'household-local-audit-store'
  ),
  ParentDeviceGeneratedSummaryCache: DataCustodyDefaultLocationSchema.parse(
    'parent-device-generated-summary-cache'
  ),
} as const;

const DataCustodyAuthorities = [
  'child-device',
  'household-local-devices',
  'parent-device',
] as const;

export const DataCustodyAuthoritySchema = withParser(Schema.Literal(...DataCustodyAuthorities));
export type DataCustodyAuthority = Infer<typeof DataCustodyAuthoritySchema>;

export const DataCustodyAuthority = {
  ChildDevice: DataCustodyAuthoritySchema.parse('child-device'),
  HouseholdLocalDevices: DataCustodyAuthoritySchema.parse('household-local-devices'),
  ParentDevice: DataCustodyAuthoritySchema.parse('parent-device'),
} as const;

const DataCustodyOcentraHostingModes = [
  'forbidden',
  'minimal-routing-metadata-only',
  'parent-authorized-stateless-derivation-only',
] as const;

export const DataCustodyOcentraHostingModeSchema = withParser(
  Schema.Literal(...DataCustodyOcentraHostingModes)
);
export type DataCustodyOcentraHostingMode = Infer<typeof DataCustodyOcentraHostingModeSchema>;

export const DataCustodyOcentraHostingMode = {
  Forbidden: DataCustodyOcentraHostingModeSchema.parse('forbidden'),
  MinimalRoutingMetadataOnly: DataCustodyOcentraHostingModeSchema.parse(
    'minimal-routing-metadata-only'
  ),
  ParentAuthorizedStatelessDerivationOnly: DataCustodyOcentraHostingModeSchema.parse(
    'parent-authorized-stateless-derivation-only'
  ),
} as const;

const DataCustodyHostingPolicyBaseSchema = Schema.Struct({
  ocentraHostingMode: DataCustodyOcentraHostingModeSchema,
  parentOwnedStorageAllowed: Schema.Boolean,
  providerMetadataAllowed: Schema.Boolean,
});

type DataCustodyHostingPolicyCandidate = Infer<typeof DataCustodyHostingPolicyBaseSchema>;

export const DataCustodyHostingPolicySchema = withParser(
  DataCustodyHostingPolicyBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        dataCustodyHostingPolicyIsCoherent(policy) ||
        'Expected hosting policy to allow parent-owned storage or declare limited hosted metadata or stateless derivation'
    )
  )
);

export type DataCustodyHostingPolicy = Infer<typeof DataCustodyHostingPolicySchema>;

function dataCustodySourceOfTruthIsUnambiguous(source: DataCustodySourceOfTruthCandidate): boolean {
  if (source.kind === 'self') {
    return source.sourceClassId === null;
  }

  return source.sourceClassId !== null;
}

function dataCustodyHostingPolicyIsCoherent(policy: DataCustodyHostingPolicyCandidate): boolean {
  return (
    policy.parentOwnedStorageAllowed ||
    policy.providerMetadataAllowed ||
    policy.ocentraHostingMode !== DataCustodyOcentraHostingMode.Forbidden
  );
}
