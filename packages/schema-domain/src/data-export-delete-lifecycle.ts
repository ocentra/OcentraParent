import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from './effect';

import {
  dataExportDeleteLifecycleCoversRequiredStates,
  dataExportDeleteLifecycleEntryIsSafe,
} from './data-export-delete-lifecycle-guards.js';

const dataExportDeleteLifecycleText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const DataExportDeleteLifecycleReadModelIdSchema = dataExportDeleteLifecycleText(
  'DataExportDeleteLifecycleReadModelId'
);
export const DataExportDeleteLifecycleIdSchema = dataExportDeleteLifecycleText('DataExportDeleteLifecycleId');
export const DataExportDeleteLifecycleReferenceSchema = dataExportDeleteLifecycleText(
  'DataExportDeleteLifecycleReference'
);
export const DataExportDeleteLifecycleRequirementSchema = dataExportDeleteLifecycleText(
  'DataExportDeleteLifecycleRequirement'
);
export const DataExportDeleteLifecycleTimestampSchema = dataExportDeleteLifecycleText(
  'DataExportDeleteLifecycleTimestamp'
);

export const DataExportDeleteLifecycleOperationSchema = withParser(Schema.Literal('export', 'delete'));
export const DataExportDeleteLifecycleStateSchema = withParser(
  Schema.Literal('requested', 'authorized', 'queued', 'running', 'succeeded', 'failed', 'manual-required')
);
export const DataExportDeleteLifecycleParentInitiationStateSchema = withParser(Schema.Literal('parent-initiated'));
export const DataExportDeleteLifecycleParentAuthorizationStateSchema = withParser(Schema.Literal('parent-authorized'));
export const DataExportDeleteLifecyclePayloadStateSchema = withParser(Schema.Literal('redacted-runtime-status-only'));
export const DataExportDeleteLifecycleCustodyStateSchema = withParser(Schema.Literal('parent-owned-local-output-only'));

export const DataExportDeleteLifecycleDataClassSchema = withParser(
  Schema.Literal(
    'parent-request-ref',
    'parent-authorization-ref',
    'local-queue-ref',
    'local-runtime-ref',
    'local-output-ref',
    'local-delete-ref',
    'redaction-audit-ref',
    'manual-proof-ref'
  )
);

export const DataExportDeleteLifecycleRequiredDataClasses = [
  'parent-request-ref',
  'parent-authorization-ref',
  'local-queue-ref',
  'local-runtime-ref',
  'local-output-ref',
  'local-delete-ref',
  'redaction-audit-ref',
  'manual-proof-ref',
] as const satisfies ReadonlyArray<DataExportDeleteLifecycleDataClass>;

const DataExportDeleteLifecycleEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  lifecycleId: DataExportDeleteLifecycleIdSchema,
  operation: DataExportDeleteLifecycleOperationSchema,
  lifecycleState: DataExportDeleteLifecycleStateSchema,
  parentInitiationState: DataExportDeleteLifecycleParentInitiationStateSchema,
  parentAuthorizationState: DataExportDeleteLifecycleParentAuthorizationStateSchema,
  payloadState: DataExportDeleteLifecyclePayloadStateSchema,
  custodyState: DataExportDeleteLifecycleCustodyStateSchema,
  disclosedDataClasses: Schema.Array(DataExportDeleteLifecycleDataClassSchema),
  requestRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  authorizationRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  queueRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  runtimeRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  outputRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  deleteRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  auditRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  custodyRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
  manualProofRequirements: Schema.Array(DataExportDeleteLifecycleRequirementSchema),
  containsTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawUrls: Schema.Boolean,
  containsScreenshots: Schema.Boolean,
  containsJournals: Schema.Boolean,
  containsSqliteSnapshots: Schema.Boolean,
  containsPrivatePaths: Schema.Boolean,
  containsCommandLines: Schema.Boolean,
  containsKeystrokes: Schema.Boolean,
  containsClipboardData: Schema.Boolean,
  containsMessageContents: Schema.Boolean,
  containsProviderSecrets: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  realBackendUploadExecuted: Schema.Boolean,
  publicRuntimeExecuted: Schema.Boolean,
  providerExecutionOccurred: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  childActivityCustodyClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: DataExportDeleteLifecycleTimestampSchema,
});

export type DataExportDeleteLifecycleEntryCandidate = Infer<typeof DataExportDeleteLifecycleEntryBaseSchema>;

export const DataExportDeleteLifecycleEntrySchema = withParser(
  DataExportDeleteLifecycleEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        dataExportDeleteLifecycleEntryIsSafe(entry, DataExportDeleteLifecycleRequiredDataClasses) ||
        'Expected data export/delete lifecycle rows to be parent-authorized, redacted, local-output scoped, custody-safe, and free of backend upload, public runtime, provider execution, SLA, remote support, or child activity custody claims'
    )
  )
);

export const DataExportDeleteLifecycleReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: DataExportDeleteLifecycleReadModelIdSchema,
    generatedAt: DataExportDeleteLifecycleTimestampSchema,
    sourceContractRefs: Schema.Array(DataExportDeleteLifecycleReferenceSchema),
    entries: Schema.Array(DataExportDeleteLifecycleEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.lifecycleId)).size === readModel.entries.length ||
        'Expected data export/delete lifecycle ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        dataExportDeleteLifecycleCoversRequiredStates(readModel.entries) ||
        'Expected data export/delete lifecycle proof to cover requested, authorized, queued, running, succeeded, failed, and manual-required states'
    )
  )
);

export type DataExportDeleteLifecycleOperation = Infer<typeof DataExportDeleteLifecycleOperationSchema>;
export type DataExportDeleteLifecycleState = Infer<typeof DataExportDeleteLifecycleStateSchema>;
export type DataExportDeleteLifecycleDataClass = Infer<typeof DataExportDeleteLifecycleDataClassSchema>;
export type DataExportDeleteLifecycleEntry = Infer<typeof DataExportDeleteLifecycleEntrySchema>;
export type DataExportDeleteLifecycleReadModel = Infer<typeof DataExportDeleteLifecycleReadModelSchema>;

export const decodeDataExportDeleteLifecycleEntry = Schema.decodeUnknownSync(DataExportDeleteLifecycleEntrySchema);
export const decodeDataExportDeleteLifecycleReadModel = Schema.decodeUnknownSync(
  DataExportDeleteLifecycleReadModelSchema
);

