import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';

import {
  statusBackendRedactionManifestCoversRequiredStates,
  statusBackendRedactionManifestEntryIsSafe,
} from './status-backend-redaction-manifest-guards.js';

const statusBackendRedactionManifestText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const StatusBackendRedactionManifestReadModelIdSchema = statusBackendRedactionManifestText(
  'StatusBackendRedactionManifestReadModelId'
);
export const StatusBackendRedactionManifestIdSchema = statusBackendRedactionManifestText(
  'StatusBackendRedactionManifestId'
);
export const StatusBackendRedactionManifestReferenceSchema = statusBackendRedactionManifestText(
  'StatusBackendRedactionManifestReference'
);
export const StatusBackendRedactionManifestRequirementSchema = statusBackendRedactionManifestText(
  'StatusBackendRedactionManifestRequirement'
);
export const StatusBackendRedactionManifestTimestampSchema = statusBackendRedactionManifestText(
  'StatusBackendRedactionManifestTimestamp'
);

export const StatusBackendRedactionManifestStateSchema = withParser(
  Schema.Literal(
    'redaction-manifest-ready',
    'redaction-manifest-manual-required',
    'redaction-review-queued',
    'redaction-review-running',
    'redaction-review-failed',
    'backend-unavailable'
  )
);

export const StatusBackendRedactionManifestParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const StatusBackendRedactionManifestExecutionClaimStateSchema = withParser(
  Schema.Literal('status-backend-redaction-manifest-boundary-only')
);
export const StatusBackendRedactionManifestPayloadStateSchema = withParser(Schema.Literal('redacted-status-refs-only'));
export const StatusBackendRedactionManifestReadinessStateSchema = withParser(
  Schema.Literal('support-safe-manifest-ready', 'manual-required')
);
export const StatusBackendRedactionManifestReviewStateSchema = withParser(
  Schema.Literal('queued', 'running', 'reviewed', 'failed', 'manual-required')
);

export const StatusBackendRedactionManifestDataClassSchema = withParser(
  Schema.Literal(
    'parent-consent-ref',
    'status-backend-target-ref',
    'status-backend-queue-ref',
    'status-backend-audit-ref',
    'redaction-manifest-ref',
    'redaction-summary-ref',
    'redaction-review-ref',
    'manual-runbook-ref',
    'failure-ref'
  )
);

export const StatusBackendRedactionManifestRequiredDataClasses = [
  'parent-consent-ref',
  'status-backend-target-ref',
  'status-backend-queue-ref',
  'status-backend-audit-ref',
  'redaction-manifest-ref',
  'redaction-summary-ref',
  'redaction-review-ref',
  'manual-runbook-ref',
  'failure-ref',
] as const satisfies ReadonlyArray<StatusBackendRedactionManifestDataClass>;

const StatusBackendRedactionManifestEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  manifestId: StatusBackendRedactionManifestIdSchema,
  manifestState: StatusBackendRedactionManifestStateSchema,
  parentConsentState: StatusBackendRedactionManifestParentConsentStateSchema,
  executionClaimState: StatusBackendRedactionManifestExecutionClaimStateSchema,
  payloadState: StatusBackendRedactionManifestPayloadStateSchema,
  redactionManifestState: StatusBackendRedactionManifestReadinessStateSchema,
  redactionReviewState: StatusBackendRedactionManifestReviewStateSchema,
  disclosedDataClasses: Schema.Array(StatusBackendRedactionManifestDataClassSchema),
  consentRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  targetRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  queueRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  auditRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  redactionManifestRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  redactionSummaryRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  redactionReviewRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  failureRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
  manualProofRequirements: Schema.Array(StatusBackendRedactionManifestRequirementSchema),
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
  statusBackendPayloadCustodyClaimed: Schema.Boolean,
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
  lastCheckedAt: StatusBackendRedactionManifestTimestampSchema,
});

export type StatusBackendRedactionManifestEntryCandidate = Infer<typeof StatusBackendRedactionManifestEntryBaseSchema>;

export const StatusBackendRedactionManifestEntrySchema = withParser(
  StatusBackendRedactionManifestEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        statusBackendRedactionManifestEntryIsSafe(entry, StatusBackendRedactionManifestRequiredDataClasses) ||
        'Expected status backend redaction manifest rows to be parent-consented, support-safe, queue-linked, audit-linked, redaction-manifest-only, and free of status backend execution, payload custody, durable payload storage, payload deletion, retry worker execution, audit persistence execution, public runtime, provider, support upload, account lookup, billing contact, remote support, production SLA, default hosted family data, or child activity custody claims'
    )
  )
);

export const StatusBackendRedactionManifestReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: StatusBackendRedactionManifestReadModelIdSchema,
    generatedAt: StatusBackendRedactionManifestTimestampSchema,
    sourceContractRefs: Schema.Array(StatusBackendRedactionManifestReferenceSchema),
    entries: Schema.Array(StatusBackendRedactionManifestEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.manifestId)).size === readModel.entries.length ||
        'Expected status backend redaction manifest ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        statusBackendRedactionManifestCoversRequiredStates(readModel.entries) ||
        'Expected status backend redaction manifest proof to cover ready, manual-required, review queued, review running, review failed, and backend unavailable rows'
    )
  )
);

export type StatusBackendRedactionManifestState = Infer<typeof StatusBackendRedactionManifestStateSchema>;
export type StatusBackendRedactionManifestReadinessState = Infer<
  typeof StatusBackendRedactionManifestReadinessStateSchema
>;
export type StatusBackendRedactionManifestReviewState = Infer<typeof StatusBackendRedactionManifestReviewStateSchema>;
export type StatusBackendRedactionManifestDataClass = Infer<typeof StatusBackendRedactionManifestDataClassSchema>;
export type StatusBackendRedactionManifestEntry = Infer<typeof StatusBackendRedactionManifestEntrySchema>;
export type StatusBackendRedactionManifestReadModel = Infer<typeof StatusBackendRedactionManifestReadModelSchema>;

export const decodeStatusBackendRedactionManifestEntry = Schema.decodeUnknownSync(
  StatusBackendRedactionManifestEntrySchema
);
export const decodeStatusBackendRedactionManifestReadModel = Schema.decodeUnknownSync(
  StatusBackendRedactionManifestReadModelSchema
);
