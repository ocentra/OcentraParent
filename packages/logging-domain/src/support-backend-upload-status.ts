import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

import {
  supportBackendUploadStatusCoversRequiredStates,
  supportBackendUploadStatusEntryIsSafe,
} from './support-backend-upload-status-guards.js';

const NonEmptySupportBackendUploadStatusText = Schema.String.pipe(Schema.minLength(1));

const supportBackendUploadStatusText = <Brand extends string>(brand: Brand) =>
  NonEmptySupportBackendUploadStatusText.pipe(Schema.brand(brand));

export const SupportBackendUploadStatusReadModelIdSchema = supportBackendUploadStatusText(
  'SupportBackendUploadStatusReadModelId'
);
export const SupportBackendUploadStatusUploadIdSchema = supportBackendUploadStatusText(
  'SupportBackendUploadStatusUploadId'
);
export const SupportBackendUploadStatusReferenceSchema = supportBackendUploadStatusText(
  'SupportBackendUploadStatusReference'
);
export const SupportBackendUploadStatusRequirementSchema = supportBackendUploadStatusText(
  'SupportBackendUploadStatusRequirement'
);
export const SupportBackendUploadStatusTimestampSchema = supportBackendUploadStatusText(
  'SupportBackendUploadStatusTimestamp'
);

export const SupportBackendUploadStatusStateSchema = withParser(
  Schema.Literal(
    'upload-queued',
    'upload-running',
    'upload-succeeded',
    'upload-failed',
    'upload-manual-required',
    'backend-unavailable',
    'provider-unavailable'
  )
);

export const SupportBackendUploadParentInitiationStateSchema = withParser(Schema.Literal('parent-initiated'));

export const SupportBackendUploadParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);

export const SupportBackendUploadAvailabilityStateSchema = withParser(
  Schema.Literal('available', 'unavailable', 'manual-required', 'not-applicable')
);

export const SupportBackendUploadRetryStateSchema = withParser(
  Schema.Literal('not-needed', 'retry-queued', 'retry-exhausted', 'manual-required')
);

export const SupportBackendUploadAbandonStateSchema = withParser(
  Schema.Literal('not-requested', 'abandoned', 'not-applicable')
);

export const SupportBackendUploadExecutionClaimStateSchema = withParser(Schema.Literal('status-boundary-only'));
export const SupportBackendUploadPayloadStateSchema = withParser(Schema.Literal('redacted-status-and-audit-refs-only'));
export const SupportBackendUploadCustodyStateSchema = withParser(Schema.Literal('no-ocentra-hosted-family-data'));

export const SupportBackendUploadDataClassSchema = withParser(
  Schema.Literal(
    'upload-status',
    'parent-consent-artifact-ref',
    'redaction-summary-ref',
    'support-bundle-ref',
    'audit-event-ref',
    'retry-policy-ref',
    'abandon-decision-ref',
    'failure-status-ref',
    'manual-proof-ref',
    'release-package-runtime-ref'
  )
);

export const SupportBackendUploadDestinationSchema = withParser(
  Schema.Literal('support-safe-upload-status-boundary', 'manual-support-backend', 'none')
);

export const SupportBackendUploadRequiredDataClasses = [
  'upload-status',
  'parent-consent-artifact-ref',
  'redaction-summary-ref',
  'support-bundle-ref',
  'audit-event-ref',
  'retry-policy-ref',
  'abandon-decision-ref',
  'failure-status-ref',
  'manual-proof-ref',
  'release-package-runtime-ref',
] as const satisfies ReadonlyArray<SupportBackendUploadDataClass>;

const SupportBackendUploadStatusEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  uploadId: SupportBackendUploadStatusUploadIdSchema,
  uploadStatus: SupportBackendUploadStatusStateSchema,
  parentInitiationState: SupportBackendUploadParentInitiationStateSchema,
  parentConsentState: SupportBackendUploadParentConsentStateSchema,
  executionClaimState: SupportBackendUploadExecutionClaimStateSchema,
  backendAvailabilityState: SupportBackendUploadAvailabilityStateSchema,
  providerAvailabilityState: SupportBackendUploadAvailabilityStateSchema,
  retryState: SupportBackendUploadRetryStateSchema,
  abandonState: SupportBackendUploadAbandonStateSchema,
  payloadState: SupportBackendUploadPayloadStateSchema,
  custodyState: SupportBackendUploadCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportBackendUploadDataClassSchema),
  allowedDestinations: Schema.Array(SupportBackendUploadDestinationSchema),
  consentRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  redactionRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  auditRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  backendRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  providerRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  retryRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  abandonRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  failureRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
  manualProofRequirements: Schema.Array(SupportBackendUploadStatusRequirementSchema),
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
  realSupportBackendUploadExecuted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderExecuted: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: SupportBackendUploadStatusTimestampSchema,
});

export type SupportBackendUploadStatusEntryCandidate = Infer<typeof SupportBackendUploadStatusEntryBaseSchema>;

export const SupportBackendUploadStatusEntrySchema = withParser(
  SupportBackendUploadStatusEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportBackendUploadStatusEntryIsSafe(entry, SupportBackendUploadRequiredDataClasses) ||
        'Expected support backend upload status rows to be parent-initiated and consented, support-safe, redaction/audit-backed, retry/abandon aware, and free of child activity custody, provider secrets, remote transcripts, account lookup, billing provider, real backend execution, or default Ocentra-hosted family data claims'
    )
  )
);

export const SupportBackendUploadStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportBackendUploadStatusReadModelIdSchema,
    generatedAt: SupportBackendUploadStatusTimestampSchema,
    sourceContractRefs: Schema.Array(SupportBackendUploadStatusReferenceSchema),
    entries: Schema.Array(SupportBackendUploadStatusEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.uploadId)).size === readModel.entries.length ||
        'Expected support backend upload status ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportBackendUploadStatusCoversRequiredStates(readModel.entries) ||
        'Expected support backend upload status proof to cover queued, running, succeeded, failed, manual-required, backend-unavailable, and provider-unavailable rows'
    )
  )
);

export type SupportBackendUploadStatusState = Infer<typeof SupportBackendUploadStatusStateSchema>;
export type SupportBackendUploadParentInitiationState = Infer<typeof SupportBackendUploadParentInitiationStateSchema>;
export type SupportBackendUploadParentConsentState = Infer<typeof SupportBackendUploadParentConsentStateSchema>;
export type SupportBackendUploadAvailabilityState = Infer<typeof SupportBackendUploadAvailabilityStateSchema>;
export type SupportBackendUploadRetryState = Infer<typeof SupportBackendUploadRetryStateSchema>;
export type SupportBackendUploadAbandonState = Infer<typeof SupportBackendUploadAbandonStateSchema>;
export type SupportBackendUploadExecutionClaimState = Infer<typeof SupportBackendUploadExecutionClaimStateSchema>;
export type SupportBackendUploadPayloadState = Infer<typeof SupportBackendUploadPayloadStateSchema>;
export type SupportBackendUploadCustodyState = Infer<typeof SupportBackendUploadCustodyStateSchema>;
export type SupportBackendUploadDataClass = Infer<typeof SupportBackendUploadDataClassSchema>;
export type SupportBackendUploadDestination = Infer<typeof SupportBackendUploadDestinationSchema>;
export type SupportBackendUploadStatusEntry = Infer<typeof SupportBackendUploadStatusEntrySchema>;
export type SupportBackendUploadStatusReadModel = Infer<typeof SupportBackendUploadStatusReadModelSchema>;

export const decodeSupportBackendUploadStatusEntry = Schema.decodeUnknownSync(SupportBackendUploadStatusEntrySchema);
export const decodeSupportBackendUploadStatusReadModel = Schema.decodeUnknownSync(
  SupportBackendUploadStatusReadModelSchema
);
