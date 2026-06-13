import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

const supportBundleText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const SupportBundleRedactionReadModelIdSchema = supportBundleText('SupportBundleRedactionReadModelId');
export const SupportBundleRedactionIncidentIdSchema = supportBundleText('SupportBundleRedactionIncidentId');
export const SupportBundleRedactionReferenceSchema = supportBundleText('SupportBundleRedactionReference');
export const SupportBundleRedactionRequirementSchema = supportBundleText('SupportBundleRedactionRequirement');
export const SupportBundleRedactionTimestampSchema = supportBundleText('SupportBundleRedactionTimestamp');

export const SupportBundleIncidentStatusSchema = withParser(
  Schema.Literal(
    'parent-consent-required',
    'support-bundle-ready',
    'manual-review-required',
    'backend-upload-manual-required',
    'status-backend-redaction-ready',
    'status-backend-redaction-manual-required',
    'billing-escalation-manual-required',
    'account-lookup-manual-required'
  )
);

export const SupportBundleParentConsentStateSchema = withParser(
  Schema.Literal('not-requested', 'required', 'parent-approved', 'revoked')
);

export const SupportBundleManualBoundaryStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'not-applicable')
);

export const SupportBundleRedactionStateSchema = withParser(Schema.Literal('support-safe-metadata-only'));
export const SupportBundleCustodyStateSchema = withParser(Schema.Literal('no-child-activity-custody'));

export const SupportBundleDataClassSchema = withParser(
  Schema.Literal(
    'release-version',
    'commit',
    'platform',
    'package-runtime',
    'service-health',
    'route-state',
    'capability-state',
    'degraded-state',
    'redaction-summary',
    'manual-proof-ref',
    'incident-status',
    'status-backend-target',
    'status-backend-redaction-manifest',
    'status-backend-queue-ref',
    'billing-status-ref',
    'account-status-ref'
  )
);

export const SupportBundlePayloadFieldSchema = withParser(
  Schema.Literal(
    'incident-id-ref',
    'release-version',
    'commit',
    'platform',
    'package-runtime-state',
    'service-health-state',
    'route-state',
    'capability-state',
    'degraded-state',
    'redaction-summary-ref',
    'status-backend-target-ref',
    'status-backend-redaction-manifest-ref',
    'status-backend-queue-ref',
    'billing-status-ref',
    'account-status-ref',
    'manual-proof-ref'
  )
);

export const SupportBundleDiagnosticReferenceKindSchema = withParser(
  Schema.Literal(
    'proof-json-ref',
    'workflow-ref',
    'redaction-summary-ref',
    'status-backend-redaction-manifest-ref',
    'manual-runbook-ref',
    'status-row-ref'
  )
);

export const SupportBundleRequiredDataClasses = [
  'release-version',
  'commit',
  'platform',
  'package-runtime',
  'service-health',
  'route-state',
  'capability-state',
  'degraded-state',
  'redaction-summary',
  'manual-proof-ref',
  'incident-status',
  'status-backend-target',
  'status-backend-redaction-manifest',
  'status-backend-queue-ref',
  'billing-status-ref',
  'account-status-ref',
] as const satisfies ReadonlyArray<SupportBundleDataClass>;

export const SupportBundleRequiredPayloadFields = [
  'incident-id-ref',
  'release-version',
  'commit',
  'platform',
  'package-runtime-state',
  'service-health-state',
  'route-state',
  'capability-state',
  'degraded-state',
  'redaction-summary-ref',
  'status-backend-target-ref',
  'status-backend-redaction-manifest-ref',
  'status-backend-queue-ref',
  'billing-status-ref',
  'account-status-ref',
  'manual-proof-ref',
] as const satisfies ReadonlyArray<SupportBundlePayloadField>;

export const SupportBundleRequiredDiagnosticReferenceKinds = [
  'proof-json-ref',
  'workflow-ref',
  'redaction-summary-ref',
  'status-backend-redaction-manifest-ref',
  'manual-runbook-ref',
  'status-row-ref',
] as const satisfies ReadonlyArray<SupportBundleDiagnosticReferenceKind>;

const SupportBundleRedactionEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  incidentId: SupportBundleRedactionIncidentIdSchema,
  incidentStatus: SupportBundleIncidentStatusSchema,
  parentConsentState: SupportBundleParentConsentStateSchema,
  backendUploadState: SupportBundleManualBoundaryStateSchema,
  billingEscalationState: SupportBundleManualBoundaryStateSchema,
  accountLookupState: SupportBundleManualBoundaryStateSchema,
  remoteSupportState: SupportBundleManualBoundaryStateSchema,
  productionSlaState: SupportBundleManualBoundaryStateSchema,
  payloadRedactionState: SupportBundleRedactionStateSchema,
  childActivityCustodyState: SupportBundleCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportBundleDataClassSchema),
  diagnosticReferenceKinds: Schema.Array(SupportBundleDiagnosticReferenceKindSchema),
  redactionSafePayloadFields: Schema.Array(SupportBundlePayloadFieldSchema),
  incidentRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  releaseRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  diagnosticRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  statusBackendRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  billingRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  accountRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
  manualProofRequirements: Schema.Array(SupportBundleRedactionRequirementSchema),
  containsTokens: Schema.Boolean,
  containsChildActivity: Schema.Boolean,
  containsRawUrls: Schema.Boolean,
  containsScreenshots: Schema.Boolean,
  containsJournals: Schema.Boolean,
  containsSqliteSnapshots: Schema.Boolean,
  containsPrivatePaths: Schema.Boolean,
  containsCommandLines: Schema.Boolean,
  containsKeystrokes: Schema.Boolean,
  containsClipboardData: Schema.Boolean,
  containsMessageContents: Schema.Boolean,
  containsStatusBackendPayload: Schema.Boolean,
  publicRuntimePayloadIncluded: Schema.Boolean,
  providerSecretPresent: Schema.Boolean,
  backendUploadExecuted: Schema.Boolean,
  statusBackendExecutionClaimed: Schema.Boolean,
  billingProviderContacted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  remoteSupportSessionStarted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  lastCheckedAt: SupportBundleRedactionTimestampSchema,
});

type SupportBundleRedactionEntryCandidate = Infer<typeof SupportBundleRedactionEntryBaseSchema>;

export const SupportBundleRedactionEntrySchema = withParser(
  SupportBundleRedactionEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportBundleRedactionEntryIsSafe(entry) ||
        'Expected support bundle incident rows to disclose only support-safe metadata, require parent consent, keep status backend, billing, and account support manual-required, and exclude secrets, child activity, raw URLs, screenshots, journals, SQLite snapshots, private paths, commands, keystrokes, clipboard data, message contents, status backend payloads, and public runtime payloads'
    )
  )
);

export const SupportBundleRedactionReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportBundleRedactionReadModelIdSchema,
    generatedAt: SupportBundleRedactionTimestampSchema,
    sourceContractRefs: Schema.Array(SupportBundleRedactionReferenceSchema),
    entries: Schema.Array(SupportBundleRedactionEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.incidentId)).size === readModel.entries.length ||
        'Expected support bundle incident ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportBundleRedactionCoversIncidentStatuses(readModel.entries) ||
        'Expected support bundle redaction proof to cover consent, ready, review, backend-upload, status-backend redaction, billing-escalation, and account-lookup states'
    )
  )
);

function supportBundleRedactionEntryIsSafe(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    !supportBundleRedactionHasClaimUpgrade(entry) &&
    supportBundleRedactionHasRequiredRefs(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, SupportBundleRequiredDataClasses) &&
    requiredValuesArePresent(entry.redactionSafePayloadFields, SupportBundleRequiredPayloadFields) &&
    requiredValuesArePresent(entry.diagnosticReferenceKinds, SupportBundleRequiredDiagnosticReferenceKinds) &&
    supportBundleRedactionStatesAreCoherent(entry)
  );
}

function supportBundleRedactionHasClaimUpgrade(entry: SupportBundleRedactionEntryCandidate): boolean {
  return [
    entry.containsTokens,
    entry.containsChildActivity,
    entry.containsRawUrls,
    entry.containsScreenshots,
    entry.containsJournals,
    entry.containsSqliteSnapshots,
    entry.containsPrivatePaths,
    entry.containsCommandLines,
    entry.containsKeystrokes,
    entry.containsClipboardData,
    entry.containsMessageContents,
    entry.containsStatusBackendPayload,
    entry.publicRuntimePayloadIncluded,
    entry.providerSecretPresent,
    entry.backendUploadExecuted,
    entry.statusBackendExecutionClaimed,
    entry.billingProviderContacted,
    entry.accountLookupExecuted,
    entry.remoteSupportSessionStarted,
    entry.productionSlaClaimed,
  ].some(Boolean);
}

function supportBundleRedactionHasRequiredRefs(entry: SupportBundleRedactionEntryCandidate): boolean {
  return entry.incidentRefs.length > 0 && entry.releaseRefs.length > 0 && entry.diagnosticRefs.length > 0;
}

function supportBundleRedactionStatesAreCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    supportBundleRedactionConsentStateIsCoherent(entry) &&
    supportBundleRedactionBackendStateIsCoherent(entry) &&
    supportBundleRedactionStatusBackendStateIsCoherent(entry) &&
    supportBundleRedactionBillingStateIsCoherent(entry) &&
    supportBundleRedactionAccountStateIsCoherent(entry)
  );
}

function supportBundleRedactionConsentStateIsCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    (entry.incidentStatus !== 'parent-consent-required' || entry.parentConsentState === 'required') &&
    (entry.incidentStatus !== 'support-bundle-ready' || entry.parentConsentState === 'parent-approved')
  );
}

function supportBundleRedactionBackendStateIsCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    entry.incidentStatus !== 'backend-upload-manual-required' ||
    (entry.backendUploadState === 'manual-required' && entry.manualProofRequirements.length > 0)
  );
}

function supportBundleRedactionStatusBackendStateIsCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    (entry.incidentStatus !== 'status-backend-redaction-ready' ||
      (entry.parentConsentState === 'parent-approved' && entry.statusBackendRefs.length > 0)) &&
    (entry.incidentStatus !== 'status-backend-redaction-manual-required' ||
      (entry.statusBackendRefs.length > 0 && entry.manualProofRequirements.length > 0))
  );
}

function supportBundleRedactionBillingStateIsCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    entry.incidentStatus !== 'billing-escalation-manual-required' ||
    (entry.billingEscalationState === 'manual-required' &&
      entry.billingRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function supportBundleRedactionAccountStateIsCoherent(entry: SupportBundleRedactionEntryCandidate): boolean {
  return (
    entry.incidentStatus !== 'account-lookup-manual-required' ||
    (entry.accountLookupState === 'manual-required' &&
      entry.accountRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function supportBundleRedactionCoversIncidentStatuses(entries: readonly SupportBundleRedactionEntry[]): boolean {
  const statuses = new Set(entries.map((entry) => entry.incidentStatus));
  return [
    'parent-consent-required',
    'support-bundle-ready',
    'manual-review-required',
    'backend-upload-manual-required',
    'status-backend-redaction-ready',
    'status-backend-redaction-manual-required',
    'billing-escalation-manual-required',
    'account-lookup-manual-required',
  ].every((status) => statuses.has(status as SupportBundleIncidentStatus));
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}

export type SupportBundleIncidentStatus = Infer<typeof SupportBundleIncidentStatusSchema>;
export type SupportBundleParentConsentState = Infer<typeof SupportBundleParentConsentStateSchema>;
export type SupportBundleManualBoundaryState = Infer<typeof SupportBundleManualBoundaryStateSchema>;
export type SupportBundleDataClass = Infer<typeof SupportBundleDataClassSchema>;
export type SupportBundlePayloadField = Infer<typeof SupportBundlePayloadFieldSchema>;
export type SupportBundleDiagnosticReferenceKind = Infer<typeof SupportBundleDiagnosticReferenceKindSchema>;
export type SupportBundleRedactionEntry = Infer<typeof SupportBundleRedactionEntrySchema>;
export type SupportBundleRedactionReadModel = Infer<typeof SupportBundleRedactionReadModelSchema>;

export const decodeSupportBundleRedactionEntry = Schema.decodeUnknownSync(SupportBundleRedactionEntrySchema);
export const decodeSupportBundleRedactionReadModel = Schema.decodeUnknownSync(SupportBundleRedactionReadModelSchema);

