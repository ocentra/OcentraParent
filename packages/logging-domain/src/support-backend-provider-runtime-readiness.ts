import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptySupportBackendProviderRuntimeReadinessText = Schema.String.pipe(Schema.minLength(1));

const supportBackendProviderRuntimeReadinessText = <Brand extends string>(brand: Brand) =>
  NonEmptySupportBackendProviderRuntimeReadinessText.pipe(Schema.brand(brand));

export const SupportBackendProviderRuntimeReadinessReadModelIdSchema = supportBackendProviderRuntimeReadinessText(
  'SupportBackendProviderRuntimeReadinessReadModelId'
);
export const SupportBackendProviderRuntimeReadinessStatusIdSchema = supportBackendProviderRuntimeReadinessText(
  'SupportBackendProviderRuntimeReadinessStatusId'
);
export const SupportBackendProviderRuntimeReadinessReferenceSchema = supportBackendProviderRuntimeReadinessText(
  'SupportBackendProviderRuntimeReadinessReference'
);
export const SupportBackendProviderRuntimeReadinessRequirementSchema = supportBackendProviderRuntimeReadinessText(
  'SupportBackendProviderRuntimeReadinessRequirement'
);
export const SupportBackendProviderRuntimeReadinessTimestampSchema = supportBackendProviderRuntimeReadinessText(
  'SupportBackendProviderRuntimeReadinessTimestamp'
);

export const SupportBackendProviderRuntimeReadinessStateSchema = withParser(
  Schema.Literal(
    'upload-runtime-linked',
    'provider-secret-preflight-linked',
    'billing-provider-manual-required',
    'account-lookup-manual-required',
    'legal-disclosure-manual-required',
    'remote-support-manual-required',
    'sla-manual-required',
    'audit-export-ready'
  )
);
export const SupportBackendProviderRuntimeReadinessClaimStateSchema = withParser(
  Schema.Literal('readiness-only', 'manual-required', 'not-implemented')
);
export const SupportBackendProviderRuntimeReadinessPayloadStateSchema = withParser(
  Schema.Literal('support-safe-status-refs-only')
);
export const SupportBackendProviderRuntimeReadinessCustodyStateSchema = withParser(
  Schema.Literal('no-ocentra-hosted-family-data')
);

export const SupportBackendProviderRuntimeReadinessDataClassSchema = withParser(
  Schema.Literal(
    'support-upload-runtime-status-ref',
    'support-upload-custody-audit-ref',
    'provider-secret-readiness-ref',
    'account-billing-status-ref',
    'privacy-legal-status-ref',
    'support-case-status-ref',
    'manual-proof-ref',
    'support-safe-audit-export-ref'
  )
);

export const SupportBackendProviderRuntimeReadinessRequiredDataClasses = [
  'support-upload-runtime-status-ref',
  'support-upload-custody-audit-ref',
  'provider-secret-readiness-ref',
  'account-billing-status-ref',
  'privacy-legal-status-ref',
  'support-case-status-ref',
  'manual-proof-ref',
  'support-safe-audit-export-ref',
] as const satisfies ReadonlyArray<SupportBackendProviderRuntimeReadinessDataClass>;

const SupportBackendProviderRuntimeReadinessEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  statusId: SupportBackendProviderRuntimeReadinessStatusIdSchema,
  readinessState: SupportBackendProviderRuntimeReadinessStateSchema,
  uploadRuntimeState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  providerSecretState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  billingProviderState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  accountLookupState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  legalDisclosureState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  remoteSupportState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  productionSlaState: SupportBackendProviderRuntimeReadinessClaimStateSchema,
  payloadState: SupportBackendProviderRuntimeReadinessPayloadStateSchema,
  custodyState: SupportBackendProviderRuntimeReadinessCustodyStateSchema,
  disclosedDataClasses: Schema.Array(SupportBackendProviderRuntimeReadinessDataClassSchema),
  uploadRuntimeRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  custodyAuditRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  providerSecretRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  accountBillingRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  privacyLegalRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  caseStatusRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  auditRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
  manualProofRequirements: Schema.Array(SupportBackendProviderRuntimeReadinessRequirementSchema),
  containsProviderSecrets: Schema.Boolean,
  containsPaymentProviderTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundlePayloads: Schema.Boolean,
  containsAccountLookupResults: Schema.Boolean,
  containsBillingProviderContactRecords: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  supportBackendUploadExecuted: Schema.Boolean,
  providerSecretDelivered: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  legalDisclosureExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: SupportBackendProviderRuntimeReadinessTimestampSchema,
});

export type SupportBackendProviderRuntimeReadinessEntryCandidate = Infer<
  typeof SupportBackendProviderRuntimeReadinessEntryBaseSchema
>;

export const SupportBackendProviderRuntimeReadinessEntrySchema = withParser(
  SupportBackendProviderRuntimeReadinessEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        supportBackendProviderRuntimeReadinessEntryIsSafe(entry) ||
        'Expected support backend provider runtime readiness rows to be support-safe, manual or readiness-only, linked to upload/custody/provider/account/legal/case refs, and free of backend/provider/legal/SLA execution or child custody claims'
    )
  )
);

export const SupportBackendProviderRuntimeReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: SupportBackendProviderRuntimeReadinessReadModelIdSchema,
    generatedAt: SupportBackendProviderRuntimeReadinessTimestampSchema,
    sourceContractRefs: Schema.Array(SupportBackendProviderRuntimeReadinessReferenceSchema),
    entries: Schema.Array(SupportBackendProviderRuntimeReadinessEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusId)).size === readModel.entries.length ||
        'Expected support backend provider runtime readiness ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        supportBackendProviderRuntimeReadinessCoversRequiredStates(readModel.entries) ||
        'Expected support backend provider runtime readiness proof to cover upload runtime, provider secret, billing, account lookup, legal, remote support, SLA, and audit export rows'
    )
  )
);

export type SupportBackendProviderRuntimeReadinessState = Infer<
  typeof SupportBackendProviderRuntimeReadinessStateSchema
>;
export type SupportBackendProviderRuntimeReadinessClaimState = Infer<
  typeof SupportBackendProviderRuntimeReadinessClaimStateSchema
>;
export type SupportBackendProviderRuntimeReadinessDataClass = Infer<
  typeof SupportBackendProviderRuntimeReadinessDataClassSchema
>;
export type SupportBackendProviderRuntimeReadinessEntry = Infer<
  typeof SupportBackendProviderRuntimeReadinessEntrySchema
>;
export type SupportBackendProviderRuntimeReadinessReadModel = Infer<
  typeof SupportBackendProviderRuntimeReadinessReadModelSchema
>;

export const decodeSupportBackendProviderRuntimeReadinessEntry = Schema.decodeUnknownSync(
  SupportBackendProviderRuntimeReadinessEntrySchema
);
export const decodeSupportBackendProviderRuntimeReadinessReadModel = Schema.decodeUnknownSync(
  SupportBackendProviderRuntimeReadinessReadModelSchema
);

function supportBackendProviderRuntimeReadinessCoversRequiredStates(
  entries: readonly SupportBackendProviderRuntimeReadinessEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.readinessState));
  return [
    'upload-runtime-linked',
    'provider-secret-preflight-linked',
    'billing-provider-manual-required',
    'account-lookup-manual-required',
    'legal-disclosure-manual-required',
    'remote-support-manual-required',
    'sla-manual-required',
    'audit-export-ready',
  ].every((state) => states.has(state as SupportBackendProviderRuntimeReadinessState));
}

function supportBackendProviderRuntimeReadinessEntryIsSafe(
  entry: SupportBackendProviderRuntimeReadinessEntryCandidate
): boolean {
  return (
    !supportBackendProviderRuntimeReadinessHasClaimUpgrade(entry) &&
    supportBackendProviderRuntimeReadinessRequiredValuesArePresent(entry.disclosedDataClasses) &&
    supportBackendProviderRuntimeReadinessRefsArePresent(entry) &&
    supportBackendProviderRuntimeReadinessStatesAreCoherent(entry)
  );
}

function supportBackendProviderRuntimeReadinessHasClaimUpgrade(
  entry: SupportBackendProviderRuntimeReadinessEntryCandidate
): boolean {
  return [
    entry.containsProviderSecrets,
    entry.containsPaymentProviderTokens,
    entry.containsRawChildActivity,
    entry.containsRawSupportBundlePayloads,
    entry.containsAccountLookupResults,
    entry.containsBillingProviderContactRecords,
    entry.containsRemoteSupportTranscripts,
    entry.supportBackendUploadExecuted,
    entry.providerSecretDelivered,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.legalDisclosureExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function supportBackendProviderRuntimeReadinessRefsArePresent(
  entry: SupportBackendProviderRuntimeReadinessEntryCandidate
): boolean {
  return (
    entry.uploadRuntimeRefs.length > 0 &&
    entry.custodyAuditRefs.length > 0 &&
    entry.providerSecretRefs.length > 0 &&
    entry.accountBillingRefs.length > 0 &&
    entry.privacyLegalRefs.length > 0 &&
    entry.caseStatusRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function supportBackendProviderRuntimeReadinessStatesAreCoherent(
  entry: SupportBackendProviderRuntimeReadinessEntryCandidate
): boolean {
  return (
    entry.payloadState === 'support-safe-status-refs-only' &&
    entry.custodyState === 'no-ocentra-hosted-family-data' &&
    entry.uploadRuntimeState !== 'not-implemented' &&
    supportBackendProviderRuntimeReadinessManualStatesAreCoherent(entry)
  );
}

function supportBackendProviderRuntimeReadinessManualStatesAreCoherent(
  entry: SupportBackendProviderRuntimeReadinessEntryCandidate
): boolean {
  if (entry.readinessState === 'upload-runtime-linked') {
    return entry.uploadRuntimeState === 'readiness-only';
  }

  if (entry.readinessState === 'provider-secret-preflight-linked') {
    return entry.providerSecretState === 'manual-required';
  }

  if (entry.readinessState === 'audit-export-ready') {
    return entry.uploadRuntimeState === 'readiness-only' && entry.providerSecretState === 'manual-required';
  }

  return [
    entry.billingProviderState,
    entry.accountLookupState,
    entry.legalDisclosureState,
    entry.remoteSupportState,
    entry.productionSlaState,
  ].includes('manual-required');
}

function supportBackendProviderRuntimeReadinessRequiredValuesArePresent(
  actualValues: ReadonlyArray<SupportBackendProviderRuntimeReadinessDataClass>
): boolean {
  const actual = new Set(actualValues);
  return (
    actual.size === actualValues.length &&
    SupportBackendProviderRuntimeReadinessRequiredDataClasses.every((value) => actual.has(value))
  );
}
