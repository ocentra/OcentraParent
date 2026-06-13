import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import {
  supportBackendUploadExecutionRuntimeCoversRequiredStates,
  supportBackendUploadExecutionRuntimeEntryIsSafe,
} from './support-backend-upload-execution-runtime-guards.js';

const supportBackendUploadExecutionRuntimeText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const SupportBackendUploadExecutionRuntimeReadModelIdSchema = supportBackendUploadExecutionRuntimeText(
  'SupportBackendUploadExecutionRuntimeReadModelId'
);
export const SupportBackendUploadExecutionRuntimeIdSchema = supportBackendUploadExecutionRuntimeText(
  'SupportBackendUploadExecutionRuntimeId'
);
export const SupportBackendUploadExecutionRuntimeReferenceSchema = supportBackendUploadExecutionRuntimeText(
  'SupportBackendUploadExecutionRuntimeReference'
);
export const SupportBackendUploadExecutionRuntimeRequirementSchema = supportBackendUploadExecutionRuntimeText(
  'SupportBackendUploadExecutionRuntimeRequirement'
);
export const SupportBackendUploadExecutionRuntimeTimestampSchema = supportBackendUploadExecutionRuntimeText(
  'SupportBackendUploadExecutionRuntimeTimestamp'
);

export const SupportBackendUploadExecutionRuntimeStateSchema = withParser(
  Schema.Literal(
    'execution-request-recorded',
    'redaction-preflight-ready',
    'dispatch-manual-required',
    'backend-unavailable',
    'provider-unavailable',
    'retry-scheduled',
    'operator-abandoned'
  )
);

export const SupportBackendUploadExecutionRuntimeParentInitiationStateSchema = withParser(
  Schema.Literal('parent-initiated')
);
export const SupportBackendUploadExecutionRuntimeParentConsentStateSchema = withParser(
  Schema.Literal('parent-approved', 'required', 'revoked')
);
export const SupportBackendUploadExecutionRuntimeAvailabilityStateSchema = withParser(
  Schema.Literal('available', 'unavailable', 'manual-required', 'not-applicable')
);
export const SupportBackendUploadExecutionRuntimeRetryStateSchema = withParser(
  Schema.Literal('not-needed', 'retry-scheduled', 'retry-exhausted', 'manual-required')
);
export const SupportBackendUploadExecutionRuntimeAbandonStateSchema = withParser(
  Schema.Literal('not-requested', 'abandoned', 'not-applicable')
);
export const SupportBackendUploadExecutionRuntimeExecutionClaimStateSchema = withParser(
  Schema.Literal('runtime-boundary-only')
);
export const SupportBackendUploadExecutionRuntimePayloadStateSchema = withParser(
  Schema.Literal('redacted-runtime-refs-only')
);
export const SupportBackendUploadExecutionRuntimeCustodyStateSchema = withParser(
  Schema.Literal('no-ocentra-hosted-family-data')
);

export const SupportBackendUploadExecutionRuntimeDataClassSchema = withParser(
  Schema.Literal(
    'runtime-request-status',
    'parent-consent-artifact-ref',
    'redaction-preflight-ref',
    'support-bundle-manifest-ref',
    'audit-event-ref',
    'status-row-ref',
    'runtime-dispatch-ref',
    'retry-schedule-ref',
    'abandon-decision-ref',
    'manual-proof-ref'
  )
);

export const SupportBackendUploadExecutionRuntimeRequiredDataClasses = [
  'runtime-request-status',
  'parent-consent-artifact-ref',
  'redaction-preflight-ref',
  'support-bundle-manifest-ref',
  'audit-event-ref',
  'status-row-ref',
  'runtime-dispatch-ref',
  'retry-schedule-ref',
  'abandon-decision-ref',
  'manual-proof-ref',
] as const satisfies ReadonlyArray<SupportBackendUploadExecutionRuntimeDataClass>;

const SupportBackendUploadExecutionRuntimeEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  runtimeId: SupportBackendUploadExecutionRuntimeIdSchema,
  runtimeState: SupportBackendUploadExecutionRuntimeStateSchema,
  parentInitiationState: SupportBackendUploadExecutionRuntimeParentInitiationStateSchema,
  parentConsentState: SupportBackendUploadExecutionRuntimeParentConsentStateSchema,
  executionClaimState: SupportBackendUploadExecutionRuntimeExecutionClaimStateSchema,
  backendAvailabilityState: SupportBackendUploadExecutionRuntimeAvailabilityStateSchema,
  providerAvailabilityState: SupportBackendUploadExecutionRuntimeAvailabilityStateSchema,
  retryState: SupportBackendUploadExecutionRuntimeRetryStateSchema,
  abandonState: SupportBackendUploadExecutionRuntimeAbandonStateSchema,
  payloadState: SupportBackendUploadExecutionRuntimePayloadStateSchema,
  custodyState: SupportBackendUploadExecutionRuntimeCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportBackendUploadExecutionRuntimeDataClassSchema),
  consentRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  redactionRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  auditRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  statusRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  runtimeRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  backendRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  providerRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  retryRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  abandonRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  failureRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
  manualProofRequirements: Schema.Array(SupportBackendUploadExecutionRuntimeRequirementSchema),
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
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: SupportBackendUploadExecutionRuntimeTimestampSchema,
});

export type SupportBackendUploadExecutionRuntimeEntryCandidate = Infer<
  typeof SupportBackendUploadExecutionRuntimeEntryBaseSchema
>;

export const SupportBackendUploadExecutionRuntimeEntrySchema = withParser(
  SupportBackendUploadExecutionRuntimeEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportBackendUploadExecutionRuntimeEntryIsSafe(
          entry,
          SupportBackendUploadExecutionRuntimeRequiredDataClasses
        ) ||
        'Expected support backend upload execution runtime rows to be parent-consented, redaction-backed, audit-backed, status-linked, runtime-boundary-only, and free of backend execution, child activity custody, provider secrets, account lookup, billing contact, remote support session, production SLA, or default Ocentra-hosted family data claims'
    )
  )
);

export const SupportBackendUploadExecutionRuntimeReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportBackendUploadExecutionRuntimeReadModelIdSchema,
    generatedAt: SupportBackendUploadExecutionRuntimeTimestampSchema,
    sourceContractRefs: Schema.Array(SupportBackendUploadExecutionRuntimeReferenceSchema),
    entries: Schema.Array(SupportBackendUploadExecutionRuntimeEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.runtimeId)).size === readModel.entries.length ||
        'Expected support backend upload execution runtime ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportBackendUploadExecutionRuntimeCoversRequiredStates(readModel.entries) ||
        'Expected support backend upload execution runtime proof to cover request, preflight, manual, backend unavailable, provider unavailable, retry, and abandon rows'
    )
  )
);

export type SupportBackendUploadExecutionRuntimeState = Infer<typeof SupportBackendUploadExecutionRuntimeStateSchema>;
export type SupportBackendUploadExecutionRuntimeAvailabilityState = Infer<
  typeof SupportBackendUploadExecutionRuntimeAvailabilityStateSchema
>;
export type SupportBackendUploadExecutionRuntimeRetryState = Infer<
  typeof SupportBackendUploadExecutionRuntimeRetryStateSchema
>;
export type SupportBackendUploadExecutionRuntimeAbandonState = Infer<
  typeof SupportBackendUploadExecutionRuntimeAbandonStateSchema
>;
export type SupportBackendUploadExecutionRuntimeDataClass = Infer<
  typeof SupportBackendUploadExecutionRuntimeDataClassSchema
>;
export type SupportBackendUploadExecutionRuntimeEntry = Infer<typeof SupportBackendUploadExecutionRuntimeEntrySchema>;
export type SupportBackendUploadExecutionRuntimeReadModel = Infer<
  typeof SupportBackendUploadExecutionRuntimeReadModelSchema
>;

export const decodeSupportBackendUploadExecutionRuntimeEntry = Schema.decodeUnknownSync(
  SupportBackendUploadExecutionRuntimeEntrySchema
);
export const decodeSupportBackendUploadExecutionRuntimeReadModel = Schema.decodeUnknownSync(
  SupportBackendUploadExecutionRuntimeReadModelSchema
);

