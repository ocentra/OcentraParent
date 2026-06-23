import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import { supportProofHasAnyClaimUpgrade, supportProofRequiredValuesArePresent } from './support-proof-contract.js';

const providerSecretExecutionText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const ProviderSecretExecutionReadModelIdSchema = providerSecretExecutionText(
  'ProviderSecretExecutionReadModelId'
);
export const ProviderSecretExecutionStatusIdSchema = providerSecretExecutionText('ProviderSecretExecutionStatusId');
export const ProviderSecretExecutionReferenceSchema = providerSecretExecutionText('ProviderSecretExecutionReference');
export const ProviderSecretExecutionRequirementSchema = providerSecretExecutionText(
  'ProviderSecretExecutionRequirement'
);
export const ProviderSecretExecutionTimestampSchema = providerSecretExecutionText('ProviderSecretExecutionTimestamp');

export const ProviderSecretExecutionStatusStateSchema = withParser(
  Schema.Literal(
    'execution-boundary-recorded',
    'backend-secret-store-preflight-required',
    'rotation-preflight-required',
    'revocation-preflight-required',
    'operator-approval-required',
    'execution-manual-required',
    'audit-export-ready'
  )
);

export const ProviderSecretExecutionClaimStateSchema = withParser(
  Schema.Literal('readiness-only', 'manual-required', 'not-implemented')
);
export const ProviderSecretExecutionPayloadStateSchema = withParser(Schema.Literal('support-safe-status-refs-only'));
export const ProviderSecretExecutionDestinationSchema = withParser(
  Schema.Literal('manual-security-runbook', 'support-safe-audit-export', 'none')
);

export const ProviderSecretExecutionDataClassSchema = withParser(
  Schema.Literal(
    'provider-execution-boundary-status',
    'provider-secret-custody-status-ref',
    'backend-secret-store-preflight-ref',
    'rotation-preflight-ref',
    'revocation-preflight-ref',
    'operator-approval-ref',
    'manual-proof-ref',
    'audit-export-ref'
  )
);

export const ProviderSecretExecutionRequiredDataClasses = [
  'provider-execution-boundary-status',
  'provider-secret-custody-status-ref',
  'backend-secret-store-preflight-ref',
  'rotation-preflight-ref',
  'revocation-preflight-ref',
  'operator-approval-ref',
  'manual-proof-ref',
  'audit-export-ref',
] as const satisfies ReadonlyArray<ProviderSecretExecutionDataClass>;

const ProviderSecretExecutionReadinessEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  statusId: ProviderSecretExecutionStatusIdSchema,
  readinessStatus: ProviderSecretExecutionStatusStateSchema,
  backendSecretStoreState: ProviderSecretExecutionClaimStateSchema,
  rotationState: ProviderSecretExecutionClaimStateSchema,
  revocationState: ProviderSecretExecutionClaimStateSchema,
  operatorApprovalState: ProviderSecretExecutionClaimStateSchema,
  executionState: ProviderSecretExecutionClaimStateSchema,
  payloadState: ProviderSecretExecutionPayloadStateSchema,
  disclosedDataClasses: Schema.Array(ProviderSecretExecutionDataClassSchema),
  allowedDestinations: Schema.Array(ProviderSecretExecutionDestinationSchema),
  custodyStatusRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  backendSecretStoreRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  rotationRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  revocationRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  operatorApprovalRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  auditRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
  manualProofRequirements: Schema.Array(ProviderSecretExecutionRequirementSchema),
  containsProviderSecrets: Schema.Boolean,
  containsPaymentProviderTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundlePayloads: Schema.Boolean,
  containsAccountLookupResults: Schema.Boolean,
  containsBillingProviderContactRecords: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  backendSecretStoreExecuted: Schema.Boolean,
  providerSecretRotationExecuted: Schema.Boolean,
  providerSecretRevocationExecuted: Schema.Boolean,
  providerSecretExecutionDelivered: Schema.Boolean,
  supportBackendUploadExecuted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: ProviderSecretExecutionTimestampSchema,
});

export type ProviderSecretExecutionReadinessEntryCandidate = Infer<
  typeof ProviderSecretExecutionReadinessEntryBaseSchema
>;

export const ProviderSecretExecutionReadinessEntrySchema = withParser(
  ProviderSecretExecutionReadinessEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        providerSecretExecutionReadinessEntryIsSafe(entry) ||
        'Expected provider secret execution readiness rows to be support-safe, preflight/manual-required, custody-status linked, and free of provider secrets, execution, child activity, billing, remote support, SLA, or hosted family data claims'
    )
  )
);

export const ProviderSecretExecutionReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: ProviderSecretExecutionReadModelIdSchema,
    generatedAt: ProviderSecretExecutionTimestampSchema,
    sourceContractRefs: Schema.Array(ProviderSecretExecutionReferenceSchema),
    entries: Schema.Array(ProviderSecretExecutionReadinessEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusId)).size === readModel.entries.length ||
        'Expected provider secret execution readiness status ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        providerSecretExecutionReadinessCoversRequiredStates(readModel.entries) ||
        'Expected provider secret execution readiness proof to cover execution boundary, secret-store, rotation, revocation, operator approval, manual execution, and audit export states'
    )
  )
);

export type ProviderSecretExecutionStatusState = Infer<typeof ProviderSecretExecutionStatusStateSchema>;
export type ProviderSecretExecutionClaimState = Infer<typeof ProviderSecretExecutionClaimStateSchema>;
export type ProviderSecretExecutionDestination = Infer<typeof ProviderSecretExecutionDestinationSchema>;
export type ProviderSecretExecutionDataClass = Infer<typeof ProviderSecretExecutionDataClassSchema>;
export type ProviderSecretExecutionReadinessEntry = Infer<typeof ProviderSecretExecutionReadinessEntrySchema>;
export type ProviderSecretExecutionReadinessReadModel = Infer<typeof ProviderSecretExecutionReadinessReadModelSchema>;

export const decodeProviderSecretExecutionReadinessEntry = Schema.decodeUnknownSync(
  ProviderSecretExecutionReadinessEntrySchema
);
export const decodeProviderSecretExecutionReadinessReadModel = Schema.decodeUnknownSync(
  ProviderSecretExecutionReadinessReadModelSchema
);

export function providerSecretExecutionReadinessCoversRequiredStates(
  entries: readonly ProviderSecretExecutionReadinessEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.readinessStatus));
  return [
    'execution-boundary-recorded',
    'backend-secret-store-preflight-required',
    'rotation-preflight-required',
    'revocation-preflight-required',
    'operator-approval-required',
    'execution-manual-required',
    'audit-export-ready',
  ].every((state) => states.has(state as ProviderSecretExecutionStatusState));
}

function providerSecretExecutionReadinessEntryIsSafe(entry: ProviderSecretExecutionReadinessEntryCandidate): boolean {
  return (
    !providerSecretExecutionReadinessHasClaimUpgrade(entry) &&
    supportProofRequiredValuesArePresent(entry.disclosedDataClasses, ProviderSecretExecutionRequiredDataClasses) &&
    providerSecretExecutionRefsArePresent(entry) &&
    providerSecretExecutionStatesAreCoherent(entry)
  );
}

function providerSecretExecutionReadinessHasClaimUpgrade(
  entry: ProviderSecretExecutionReadinessEntryCandidate
): boolean {
  return supportProofHasAnyClaimUpgrade([
    entry.containsProviderSecrets,
    entry.containsPaymentProviderTokens,
    entry.containsRawChildActivity,
    entry.containsRawSupportBundlePayloads,
    entry.containsAccountLookupResults,
    entry.containsBillingProviderContactRecords,
    entry.containsRemoteSupportTranscripts,
    entry.backendSecretStoreExecuted,
    entry.providerSecretRotationExecuted,
    entry.providerSecretRevocationExecuted,
    entry.providerSecretExecutionDelivered,
    entry.supportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ]);
}

function providerSecretExecutionRefsArePresent(entry: ProviderSecretExecutionReadinessEntryCandidate): boolean {
  return (
    entry.custodyStatusRefs.length > 0 &&
    entry.backendSecretStoreRefs.length > 0 &&
    entry.rotationRefs.length > 0 &&
    entry.revocationRefs.length > 0 &&
    entry.operatorApprovalRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function providerSecretExecutionStatesAreCoherent(entry: ProviderSecretExecutionReadinessEntryCandidate): boolean {
  return (
    entry.payloadState === 'support-safe-status-refs-only' &&
    entry.allowedDestinations.includes('manual-security-runbook') &&
    entry.executionState !== 'readiness-only' &&
    providerSecretExecutionPreflightStatesAreCoherent(entry)
  );
}

function providerSecretExecutionPreflightStatesAreCoherent(
  entry: ProviderSecretExecutionReadinessEntryCandidate
): boolean {
  if (entry.readinessStatus === 'execution-boundary-recorded') {
    return entry.executionState === 'not-implemented';
  }

  if (entry.readinessStatus === 'audit-export-ready') {
    return (
      entry.executionState === 'manual-required' && entry.allowedDestinations.includes('support-safe-audit-export')
    );
  }

  return entry.executionState === 'manual-required';
}
