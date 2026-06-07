import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

import {
  statusBackendDeleteExecutorCoversRequiredStates,
  statusBackendDeleteExecutorEntryIsSafe,
} from './status-backend-delete-executor-guards.js';

const NonEmptyStatusBackendDeleteExecutorText = Schema.String.pipe(Schema.minLength(1));

const statusBackendDeleteExecutorText = <Brand extends string>(brand: Brand) =>
  NonEmptyStatusBackendDeleteExecutorText.pipe(Schema.brand(brand));

export const StatusBackendDeleteExecutorReadModelIdSchema = statusBackendDeleteExecutorText(
  'StatusBackendDeleteExecutorReadModelId'
);
export const StatusBackendDeleteExecutorIdSchema = statusBackendDeleteExecutorText('StatusBackendDeleteExecutorId');
export const StatusBackendDeleteExecutorReferenceSchema = statusBackendDeleteExecutorText(
  'StatusBackendDeleteExecutorReference'
);
export const StatusBackendDeleteExecutorRequirementSchema = statusBackendDeleteExecutorText(
  'StatusBackendDeleteExecutorRequirement'
);
export const StatusBackendDeleteExecutorTimestampSchema = statusBackendDeleteExecutorText(
  'StatusBackendDeleteExecutorTimestamp'
);

export const StatusBackendDeleteExecutorStateSchema = withParser(
  Schema.Literal(
    'delete-request-recorded',
    'delete-executor-authorized',
    'delete-executor-queued',
    'delete-executor-running',
    'deletion-manual-required',
    'delete-executor-failed',
    'audit-export-ready',
    'backend-unavailable'
  )
);

export const StatusBackendDeleteExecutorParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const StatusBackendDeleteExecutorExecutionClaimStateSchema = withParser(
  Schema.Literal('status-backend-delete-executor-boundary-only')
);
export const StatusBackendDeleteExecutorPayloadStateSchema = withParser(
  Schema.Literal('redacted-delete-status-refs-only')
);
export const StatusBackendDeleteExecutorExecutionStateSchema = withParser(
  Schema.Literal('manual-required', 'not-executed')
);
export const StatusBackendDeleteExecutorPayloadDeletionStateSchema = withParser(
  Schema.Literal('manual-required', 'not-requested')
);
export const StatusBackendDeleteExecutorAuditExportStateSchema = withParser(
  Schema.Literal('support-safe-export-ready', 'manual-required')
);

export const StatusBackendDeleteExecutorDataClassSchema = withParser(
  Schema.Literal(
    'parent-consent-ref',
    'status-backend-target-ref',
    'status-backend-queue-ref',
    'status-backend-audit-ref',
    'redaction-summary-ref',
    'custody-boundary-ref',
    'delete-request-ref',
    'delete-executor-ref',
    'delete-manual-proof-ref',
    'manual-runbook-ref'
  )
);

export const StatusBackendDeleteExecutorRequiredDataClasses = [
  'parent-consent-ref',
  'status-backend-target-ref',
  'status-backend-queue-ref',
  'status-backend-audit-ref',
  'redaction-summary-ref',
  'custody-boundary-ref',
  'delete-request-ref',
  'delete-executor-ref',
  'delete-manual-proof-ref',
  'manual-runbook-ref',
] as const satisfies ReadonlyArray<StatusBackendDeleteExecutorDataClass>;

const StatusBackendDeleteExecutorEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  executorId: StatusBackendDeleteExecutorIdSchema,
  deleteExecutorState: StatusBackendDeleteExecutorStateSchema,
  parentConsentState: StatusBackendDeleteExecutorParentConsentStateSchema,
  executionClaimState: StatusBackendDeleteExecutorExecutionClaimStateSchema,
  payloadState: StatusBackendDeleteExecutorPayloadStateSchema,
  executorExecutionState: StatusBackendDeleteExecutorExecutionStateSchema,
  payloadDeletionState: StatusBackendDeleteExecutorPayloadDeletionStateSchema,
  auditExportState: StatusBackendDeleteExecutorAuditExportStateSchema,
  disclosedDataClasses: Schema.Array(StatusBackendDeleteExecutorDataClassSchema),
  consentRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  targetRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  queueRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  auditRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  redactionRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  custodyRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  deleteRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  executorRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  failureRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
  manualProofRequirements: Schema.Array(StatusBackendDeleteExecutorRequirementSchema),
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
  statusBackendDeleteExecutorExecuted: Schema.Boolean,
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
  lastCheckedAt: StatusBackendDeleteExecutorTimestampSchema,
});

export type StatusBackendDeleteExecutorEntryCandidate = Infer<typeof StatusBackendDeleteExecutorEntryBaseSchema>;

export const StatusBackendDeleteExecutorEntrySchema = withParser(
  StatusBackendDeleteExecutorEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        statusBackendDeleteExecutorEntryIsSafe(entry, StatusBackendDeleteExecutorRequiredDataClasses) ||
        'Expected status backend delete executor rows to be parent-consented, redaction-backed, queue-linked, audit-linked, executor-boundary-only, and free of status backend execution, durable payload storage, delete executor execution, payload deletion, retry worker execution, audit persistence, public runtime, provider, support upload, account lookup, billing contact, remote support, production SLA, default hosted family data, or child activity custody claims'
    )
  )
);

export const StatusBackendDeleteExecutorReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: StatusBackendDeleteExecutorReadModelIdSchema,
    generatedAt: StatusBackendDeleteExecutorTimestampSchema,
    sourceContractRefs: Schema.Array(StatusBackendDeleteExecutorReferenceSchema),
    entries: Schema.Array(StatusBackendDeleteExecutorEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.executorId)).size === readModel.entries.length ||
        'Expected status backend delete executor ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        statusBackendDeleteExecutorCoversRequiredStates(readModel.entries) ||
        'Expected status backend delete executor proof to cover request, authorization, queue, running, manual deletion, failure, audit export, and backend unavailable rows'
    )
  )
);

export type StatusBackendDeleteExecutorState = Infer<typeof StatusBackendDeleteExecutorStateSchema>;
export type StatusBackendDeleteExecutorExecutionState = Infer<typeof StatusBackendDeleteExecutorExecutionStateSchema>;
export type StatusBackendDeleteExecutorPayloadDeletionState = Infer<
  typeof StatusBackendDeleteExecutorPayloadDeletionStateSchema
>;
export type StatusBackendDeleteExecutorAuditExportState = Infer<
  typeof StatusBackendDeleteExecutorAuditExportStateSchema
>;
export type StatusBackendDeleteExecutorDataClass = Infer<typeof StatusBackendDeleteExecutorDataClassSchema>;
export type StatusBackendDeleteExecutorEntry = Infer<typeof StatusBackendDeleteExecutorEntrySchema>;
export type StatusBackendDeleteExecutorReadModel = Infer<typeof StatusBackendDeleteExecutorReadModelSchema>;

export const decodeStatusBackendDeleteExecutorEntry = Schema.decodeUnknownSync(StatusBackendDeleteExecutorEntrySchema);
export const decodeStatusBackendDeleteExecutorReadModel = Schema.decodeUnknownSync(
  StatusBackendDeleteExecutorReadModelSchema
);
