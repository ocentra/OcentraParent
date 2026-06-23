import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';

import {
  statusBackendPayloadCustodyCoversRequiredStates,
  statusBackendPayloadCustodyEntryIsSafe,
} from './status-backend-payload-custody-guards.js';

const statusBackendPayloadCustodyText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const StatusBackendPayloadCustodyReadModelIdSchema = statusBackendPayloadCustodyText(
  'StatusBackendPayloadCustodyReadModelId'
);
export const StatusBackendPayloadCustodyIdSchema = statusBackendPayloadCustodyText('StatusBackendPayloadCustodyId');
export const StatusBackendPayloadCustodyReferenceSchema = statusBackendPayloadCustodyText(
  'StatusBackendPayloadCustodyReference'
);
export const StatusBackendPayloadCustodyRequirementSchema = statusBackendPayloadCustodyText(
  'StatusBackendPayloadCustodyRequirement'
);
export const StatusBackendPayloadCustodyTimestampSchema = statusBackendPayloadCustodyText(
  'StatusBackendPayloadCustodyTimestamp'
);

export const StatusBackendPayloadCustodyStateSchema = withParser(
  Schema.Literal(
    'custody-boundary-recorded',
    'retention-manual-required',
    'delete-request-recorded',
    'deletion-manual-required',
    'audit-export-ready',
    'backend-unavailable'
  )
);

export const StatusBackendPayloadCustodyParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const StatusBackendPayloadCustodyExecutionClaimStateSchema = withParser(
  Schema.Literal('status-backend-payload-custody-boundary-only')
);
export const StatusBackendPayloadCustodyPayloadStateSchema = withParser(Schema.Literal('redacted-status-refs-only'));
export const StatusBackendPayloadCustodyStorageStateSchema = withParser(
  Schema.Literal('manual-required', 'not-retained')
);
export const StatusBackendPayloadCustodyDeleteStateSchema = withParser(
  Schema.Literal('manual-required', 'not-requested')
);
export const StatusBackendPayloadCustodyAuditExportStateSchema = withParser(
  Schema.Literal('support-safe-export-ready', 'manual-required')
);

export const StatusBackendPayloadCustodyDataClassSchema = withParser(
  Schema.Literal(
    'parent-consent-ref',
    'status-backend-target-ref',
    'status-backend-queue-ref',
    'status-backend-audit-ref',
    'redaction-summary-ref',
    'custody-boundary-ref',
    'retention-manual-proof-ref',
    'delete-request-ref',
    'delete-manual-proof-ref',
    'manual-runbook-ref'
  )
);

export const StatusBackendPayloadCustodyRequiredDataClasses = [
  'parent-consent-ref',
  'status-backend-target-ref',
  'status-backend-queue-ref',
  'status-backend-audit-ref',
  'redaction-summary-ref',
  'custody-boundary-ref',
  'retention-manual-proof-ref',
  'delete-request-ref',
  'delete-manual-proof-ref',
  'manual-runbook-ref',
] as const satisfies ReadonlyArray<StatusBackendPayloadCustodyDataClass>;

const StatusBackendPayloadCustodyEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  custodyId: StatusBackendPayloadCustodyIdSchema,
  custodyState: StatusBackendPayloadCustodyStateSchema,
  parentConsentState: StatusBackendPayloadCustodyParentConsentStateSchema,
  executionClaimState: StatusBackendPayloadCustodyExecutionClaimStateSchema,
  payloadState: StatusBackendPayloadCustodyPayloadStateSchema,
  storageState: StatusBackendPayloadCustodyStorageStateSchema,
  deleteState: StatusBackendPayloadCustodyDeleteStateSchema,
  auditExportState: StatusBackendPayloadCustodyAuditExportStateSchema,
  disclosedDataClasses: Schema.Array(StatusBackendPayloadCustodyDataClassSchema),
  consentRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  targetRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  queueRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  auditRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  redactionRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  custodyRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  retentionRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  deleteRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
  manualProofRequirements: Schema.Array(StatusBackendPayloadCustodyRequirementSchema),
  containsTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundles: Schema.Boolean,
  containsProviderSecrets: Schema.Boolean,
  containsAccountLookupResults: Schema.Boolean,
  containsBillingContactRecords: Schema.Boolean,
  containsBackendUploadPayloads: Schema.Boolean,
  containsStatusBackendPayloads: Schema.Boolean,
  containsPublicRuntimePayloads: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  realStatusBackendExecution: Schema.Boolean,
  durableStatusBackendPayloadStorage: Schema.Boolean,
  statusBackendPayloadDeletionExecuted: Schema.Boolean,
  retryWorkerExecution: Schema.Boolean,
  auditPersistenceExecuted: Schema.Boolean,
  publicRuntimeExecution: Schema.Boolean,
  supportBackendUploadExecution: Schema.Boolean,
  providerExecution: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: StatusBackendPayloadCustodyTimestampSchema,
});

export type StatusBackendPayloadCustodyEntryCandidate = Infer<typeof StatusBackendPayloadCustodyEntryBaseSchema>;

export const StatusBackendPayloadCustodyEntrySchema = withParser(
  StatusBackendPayloadCustodyEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        statusBackendPayloadCustodyEntryIsSafe(entry, StatusBackendPayloadCustodyRequiredDataClasses) ||
        'Expected status backend payload custody rows to be parent-consented, redaction-backed, queue-linked, audit-linked, custody-boundary-only, and free of status backend execution, durable payload storage, payload deletion, retry worker execution, audit persistence, public runtime, provider, support upload, account lookup, billing contact, remote support, production SLA, default hosted family data, or child activity custody claims'
    )
  )
);

export const StatusBackendPayloadCustodyReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: StatusBackendPayloadCustodyReadModelIdSchema,
    generatedAt: StatusBackendPayloadCustodyTimestampSchema,
    sourceContractRefs: Schema.Array(StatusBackendPayloadCustodyReferenceSchema),
    entries: Schema.Array(StatusBackendPayloadCustodyEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.custodyId)).size === readModel.entries.length ||
        'Expected status backend payload custody ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        statusBackendPayloadCustodyCoversRequiredStates(readModel.entries) ||
        'Expected status backend payload custody proof to cover custody boundary, retention manual, delete request, deletion manual, audit export, and backend unavailable rows'
    )
  )
);

export type StatusBackendPayloadCustodyState = Infer<typeof StatusBackendPayloadCustodyStateSchema>;
export type StatusBackendPayloadCustodyStorageState = Infer<typeof StatusBackendPayloadCustodyStorageStateSchema>;
export type StatusBackendPayloadCustodyDeleteState = Infer<typeof StatusBackendPayloadCustodyDeleteStateSchema>;
export type StatusBackendPayloadCustodyAuditExportState = Infer<
  typeof StatusBackendPayloadCustodyAuditExportStateSchema
>;
export type StatusBackendPayloadCustodyDataClass = Infer<typeof StatusBackendPayloadCustodyDataClassSchema>;
export type StatusBackendPayloadCustodyEntry = Infer<typeof StatusBackendPayloadCustodyEntrySchema>;
export type StatusBackendPayloadCustodyReadModel = Infer<typeof StatusBackendPayloadCustodyReadModelSchema>;

export const decodeStatusBackendPayloadCustodyEntry = Schema.decodeUnknownSync(StatusBackendPayloadCustodyEntrySchema);
export const decodeStatusBackendPayloadCustodyReadModel = Schema.decodeUnknownSync(
  StatusBackendPayloadCustodyReadModelSchema
);
