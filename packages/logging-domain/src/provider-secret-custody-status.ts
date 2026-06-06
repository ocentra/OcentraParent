import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyProviderSecretCustodyText = Schema.String.pipe(Schema.minLength(1));

const providerSecretCustodyText = <Brand extends string>(brand: Brand) =>
  NonEmptyProviderSecretCustodyText.pipe(Schema.brand(brand));

export const ProviderSecretCustodyReadModelIdSchema = providerSecretCustodyText('ProviderSecretCustodyReadModelId');
export const ProviderSecretCustodyStatusIdSchema = providerSecretCustodyText('ProviderSecretCustodyStatusId');
export const ProviderSecretCustodyReferenceSchema = providerSecretCustodyText('ProviderSecretCustodyReference');
export const ProviderSecretCustodyRequirementSchema = providerSecretCustodyText('ProviderSecretCustodyRequirement');
export const ProviderSecretCustodyTimestampSchema = providerSecretCustodyText('ProviderSecretCustodyTimestamp');

export const ProviderSecretCustodyStatusStateSchema = withParser(
  Schema.Literal(
    'custody-boundary-recorded',
    'provider-secret-absent',
    'backend-secret-store-manual-required',
    'rotation-manual-required',
    'revocation-manual-required',
    'audit-export-ready'
  )
);

export const ProviderSecretCustodyExecutionStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'not-applicable')
);
export const ProviderSecretCustodyPayloadStateSchema = withParser(Schema.Literal('support-safe-status-refs-only'));
export const ProviderSecretCustodyBoundaryStateSchema = withParser(Schema.Literal('no-provider-secret-custody'));
export const ProviderSecretCustodyDestinationSchema = withParser(
  Schema.Literal('support-safe-status-boundary', 'manual-security-runbook', 'none')
);

export const ProviderSecretCustodyDataClassSchema = withParser(
  Schema.Literal(
    'provider-boundary-status',
    'legal-provider-readiness-ref',
    'billing-support-ref',
    'redaction-summary-ref',
    'custody-audit-ref',
    'rotation-status-ref',
    'revocation-status-ref',
    'manual-proof-ref',
    'audit-export-ref'
  )
);

export const ProviderSecretCustodyRequiredDataClasses = [
  'provider-boundary-status',
  'legal-provider-readiness-ref',
  'billing-support-ref',
  'redaction-summary-ref',
  'custody-audit-ref',
  'rotation-status-ref',
  'revocation-status-ref',
  'manual-proof-ref',
  'audit-export-ref',
] as const satisfies ReadonlyArray<ProviderSecretCustodyDataClass>;

const ProviderSecretCustodyStatusEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  statusId: ProviderSecretCustodyStatusIdSchema,
  custodyStatus: ProviderSecretCustodyStatusStateSchema,
  providerSecretCustodyState: ProviderSecretCustodyExecutionStateSchema,
  backendSecretStoreState: ProviderSecretCustodyExecutionStateSchema,
  rotationState: ProviderSecretCustodyExecutionStateSchema,
  revocationState: ProviderSecretCustodyExecutionStateSchema,
  payloadState: ProviderSecretCustodyPayloadStateSchema,
  custodyBoundaryState: ProviderSecretCustodyBoundaryStateSchema,
  disclosedDataClasses: Schema.Array(ProviderSecretCustodyDataClassSchema),
  allowedDestinations: Schema.Array(ProviderSecretCustodyDestinationSchema),
  legalProviderRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  billingSupportRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  redactionRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  auditRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  custodyRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  rotationRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  revocationRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
  manualProofRequirements: Schema.Array(ProviderSecretCustodyRequirementSchema),
  containsProviderSecrets: Schema.Boolean,
  containsPaymentProviderTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundlePayloads: Schema.Boolean,
  containsAccountLookupResults: Schema.Boolean,
  containsBillingProviderContactRecords: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  providerSecretCustodyExecuted: Schema.Boolean,
  backendSecretStoreImplemented: Schema.Boolean,
  rotationExecuted: Schema.Boolean,
  revocationExecuted: Schema.Boolean,
  supportBackendUploadExecuted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: ProviderSecretCustodyTimestampSchema,
});

export type ProviderSecretCustodyStatusEntryCandidate = Infer<typeof ProviderSecretCustodyStatusEntryBaseSchema>;

export const ProviderSecretCustodyStatusEntrySchema = withParser(
  ProviderSecretCustodyStatusEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        providerSecretCustodyStatusEntryIsSafe(entry) ||
        'Expected provider secret custody status rows to be support-safe, manual-required where provider custody would be needed, and free of provider secrets, tokens, child activity, support payloads, account/billing records, remote transcripts, execution, SLA, or hosted family data claims'
    )
  )
);

export const ProviderSecretCustodyStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: ProviderSecretCustodyReadModelIdSchema,
    generatedAt: ProviderSecretCustodyTimestampSchema,
    sourceContractRefs: Schema.Array(ProviderSecretCustodyReferenceSchema),
    entries: Schema.Array(ProviderSecretCustodyStatusEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusId)).size === readModel.entries.length ||
        'Expected provider secret custody status ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        providerSecretCustodyStatusCoversRequiredStates(readModel.entries) ||
        'Expected provider secret custody status proof to cover boundary, absence, backend store, rotation, revocation, and audit export states'
    )
  )
);

export type ProviderSecretCustodyStatusState = Infer<typeof ProviderSecretCustodyStatusStateSchema>;
export type ProviderSecretCustodyExecutionState = Infer<typeof ProviderSecretCustodyExecutionStateSchema>;
export type ProviderSecretCustodyDestination = Infer<typeof ProviderSecretCustodyDestinationSchema>;
export type ProviderSecretCustodyDataClass = Infer<typeof ProviderSecretCustodyDataClassSchema>;
export type ProviderSecretCustodyStatusEntry = Infer<typeof ProviderSecretCustodyStatusEntrySchema>;
export type ProviderSecretCustodyStatusReadModel = Infer<typeof ProviderSecretCustodyStatusReadModelSchema>;

export const decodeProviderSecretCustodyStatusEntry = Schema.decodeUnknownSync(ProviderSecretCustodyStatusEntrySchema);
export const decodeProviderSecretCustodyStatusReadModel = Schema.decodeUnknownSync(
  ProviderSecretCustodyStatusReadModelSchema
);

export function providerSecretCustodyStatusCoversRequiredStates(
  entries: readonly ProviderSecretCustodyStatusEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.custodyStatus));
  return [
    'custody-boundary-recorded',
    'provider-secret-absent',
    'backend-secret-store-manual-required',
    'rotation-manual-required',
    'revocation-manual-required',
    'audit-export-ready',
  ].every((state) => states.has(state as ProviderSecretCustodyStatusState));
}

function providerSecretCustodyStatusEntryIsSafe(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  return (
    !providerSecretCustodyStatusHasClaimUpgrade(entry) &&
    providerSecretCustodyRequiredValuesArePresent(entry.disclosedDataClasses) &&
    providerSecretCustodyRefsArePresent(entry) &&
    providerSecretCustodyStatesAreCoherent(entry)
  );
}

function providerSecretCustodyStatusHasClaimUpgrade(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  return [
    entry.containsProviderSecrets,
    entry.containsPaymentProviderTokens,
    entry.containsRawChildActivity,
    entry.containsRawSupportBundlePayloads,
    entry.containsAccountLookupResults,
    entry.containsBillingProviderContactRecords,
    entry.containsRemoteSupportTranscripts,
    entry.providerSecretCustodyExecuted,
    entry.backendSecretStoreImplemented,
    entry.rotationExecuted,
    entry.revocationExecuted,
    entry.supportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function providerSecretCustodyRefsArePresent(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  return (
    entry.legalProviderRefs.length > 0 &&
    entry.billingSupportRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.custodyRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function providerSecretCustodyStatesAreCoherent(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  return (
    entry.payloadState === 'support-safe-status-refs-only' &&
    entry.custodyBoundaryState === 'no-provider-secret-custody' &&
    entry.allowedDestinations.includes('support-safe-status-boundary') &&
    providerSecretCustodyManualStatesAreCoherent(entry) &&
    providerSecretCustodyRotationStatesAreCoherent(entry)
  );
}

function providerSecretCustodyManualStatesAreCoherent(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  if (entry.custodyStatus === 'provider-secret-absent') {
    return entry.providerSecretCustodyState === 'not-implemented' && entry.backendSecretStoreState === 'not-applicable';
  }

  return entry.providerSecretCustodyState !== 'not-applicable' && entry.backendSecretStoreState !== 'not-applicable';
}

function providerSecretCustodyRotationStatesAreCoherent(entry: ProviderSecretCustodyStatusEntryCandidate): boolean {
  if (entry.custodyStatus === 'rotation-manual-required') {
    return entry.rotationState === 'manual-required' && entry.rotationRefs.length > 0;
  }

  if (entry.custodyStatus === 'revocation-manual-required') {
    return entry.revocationState === 'manual-required' && entry.revocationRefs.length > 0;
  }

  return true;
}

function providerSecretCustodyRequiredValuesArePresent(
  actualValues: ReadonlyArray<ProviderSecretCustodyDataClass>
): boolean {
  const actual = new Set(actualValues);
  return (
    actual.size === actualValues.length && ProviderSecretCustodyRequiredDataClasses.every((value) => actual.has(value))
  );
}
