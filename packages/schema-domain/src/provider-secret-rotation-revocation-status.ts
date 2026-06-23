import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';

import { providerSecretRotationRevocationStatusEntryIsSafe } from './provider-secret-rotation-revocation-status-guards.js';

const providerSecretRotationRevocationText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const ProviderSecretRotationRevocationReadModelIdSchema = providerSecretRotationRevocationText(
  'ProviderSecretRotationRevocationReadModelId'
);
export const ProviderSecretRotationRevocationStatusIdSchema = providerSecretRotationRevocationText(
  'ProviderSecretRotationRevocationStatusId'
);
export const ProviderSecretRotationRevocationReferenceSchema = providerSecretRotationRevocationText(
  'ProviderSecretRotationRevocationReference'
);
export const ProviderSecretRotationRevocationRequirementSchema = providerSecretRotationRevocationText(
  'ProviderSecretRotationRevocationRequirement'
);
export const ProviderSecretRotationRevocationTimestampSchema = providerSecretRotationRevocationText(
  'ProviderSecretRotationRevocationTimestamp'
);

export const ProviderSecretRotationRevocationStatusStateSchema = withParser(
  Schema.Literal(
    'rotation-requested',
    'rotation-preflight-ready',
    'rotation-manual-required',
    'revocation-requested',
    'revocation-preflight-ready',
    'revocation-manual-required',
    'audit-export-ready'
  )
);

export const ProviderSecretRotationRevocationExecutionStateSchema = withParser(
  Schema.Literal('not-implemented', 'not-applicable', 'preflight-ready', 'manual-required')
);
export const ProviderSecretRotationRevocationPayloadStateSchema = withParser(
  Schema.Literal('support-safe-status-refs-only')
);
export const ProviderSecretRotationRevocationDestinationSchema = withParser(
  Schema.Literal('support-safe-status-boundary', 'manual-security-runbook')
);

export const ProviderSecretRotationRevocationDataClassSchema = withParser(
  Schema.Literal(
    'provider-secret-custody-status-ref',
    'provider-secret-execution-readiness-ref',
    'backend-secret-store-preflight-ref',
    'provider-secret-rotation-status-ref',
    'provider-secret-revocation-status-ref',
    'operator-approval-ref',
    'manual-proof-ref',
    'audit-export-ref'
  )
);

export const ProviderSecretRotationRevocationRequiredDataClasses = [
  'provider-secret-custody-status-ref',
  'provider-secret-execution-readiness-ref',
  'backend-secret-store-preflight-ref',
  'provider-secret-rotation-status-ref',
  'provider-secret-revocation-status-ref',
  'operator-approval-ref',
  'manual-proof-ref',
  'audit-export-ref',
] as const satisfies ReadonlyArray<ProviderSecretRotationRevocationDataClass>;

const ProviderSecretRotationRevocationStatusEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  statusId: ProviderSecretRotationRevocationStatusIdSchema,
  rotationRevocationStatus: ProviderSecretRotationRevocationStatusStateSchema,
  backendSecretStoreState: ProviderSecretRotationRevocationExecutionStateSchema,
  rotationState: ProviderSecretRotationRevocationExecutionStateSchema,
  revocationState: ProviderSecretRotationRevocationExecutionStateSchema,
  operatorApprovalState: ProviderSecretRotationRevocationExecutionStateSchema,
  payloadState: ProviderSecretRotationRevocationPayloadStateSchema,
  disclosedDataClasses: Schema.Array(ProviderSecretRotationRevocationDataClassSchema),
  allowedDestinations: Schema.Array(ProviderSecretRotationRevocationDestinationSchema),
  custodyStatusRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  executionReadinessRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  backendSecretStoreRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  rotationRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  revocationRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  operatorApprovalRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  auditRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
  manualProofRequirements: Schema.Array(ProviderSecretRotationRevocationRequirementSchema),
  containsProviderSecrets: Schema.Boolean,
  containsPaymentProviderTokens: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundlePayloads: Schema.Boolean,
  containsAccountLookupResults: Schema.Boolean,
  containsBillingProviderContactRecords: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  backendSecretStoreExecuted: Schema.Boolean,
  rotationExecuted: Schema.Boolean,
  revocationExecuted: Schema.Boolean,
  providerSecretDelivered: Schema.Boolean,
  supportBackendUploadExecuted: Schema.Boolean,
  accountLookupExecuted: Schema.Boolean,
  billingProviderContactExecuted: Schema.Boolean,
  remoteSupportSessionExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  lastCheckedAt: ProviderSecretRotationRevocationTimestampSchema,
});

export type ProviderSecretRotationRevocationStatusEntryCandidate = Infer<
  typeof ProviderSecretRotationRevocationStatusEntryBaseSchema
>;

const providerSecretRotationRevocationStatusEntrySafetyMessage =
  'Expected provider secret rotation/revocation rows to be support-safe, manual-required before execution, and free of secrets, child activity, support payloads, account/billing records, remote transcripts, execution, SLA, or hosted family data claims';

export const ProviderSecretRotationRevocationStatusEntrySchema = withParser(
  ProviderSecretRotationRevocationStatusEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        providerSecretRotationRevocationStatusEntryIsSafe(entry) ||
        providerSecretRotationRevocationStatusEntrySafetyMessage
    )
  )
);

export const ProviderSecretRotationRevocationStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: ProviderSecretRotationRevocationReadModelIdSchema,
    generatedAt: ProviderSecretRotationRevocationTimestampSchema,
    sourceContractRefs: Schema.Array(ProviderSecretRotationRevocationReferenceSchema),
    entries: Schema.Array(ProviderSecretRotationRevocationStatusEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusId)).size === readModel.entries.length ||
        'Expected provider secret rotation/revocation status ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        providerSecretRotationRevocationStatusCoversRequiredStates(readModel.entries) ||
        'Expected provider secret rotation/revocation proof to cover rotation, revocation, and audit export states'
    )
  )
);

export type ProviderSecretRotationRevocationStatusState = Infer<
  typeof ProviderSecretRotationRevocationStatusStateSchema
>;
export type ProviderSecretRotationRevocationExecutionState = Infer<
  typeof ProviderSecretRotationRevocationExecutionStateSchema
>;
export type ProviderSecretRotationRevocationDestination = Infer<
  typeof ProviderSecretRotationRevocationDestinationSchema
>;
export type ProviderSecretRotationRevocationDataClass = Infer<typeof ProviderSecretRotationRevocationDataClassSchema>;
export type ProviderSecretRotationRevocationStatusEntry = Infer<
  typeof ProviderSecretRotationRevocationStatusEntrySchema
>;
export type ProviderSecretRotationRevocationStatusReadModel = Infer<
  typeof ProviderSecretRotationRevocationStatusReadModelSchema
>;

export function providerSecretRotationRevocationStatusCoversRequiredStates(
  entries: readonly ProviderSecretRotationRevocationStatusEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.rotationRevocationStatus));
  return [
    'rotation-requested',
    'rotation-preflight-ready',
    'rotation-manual-required',
    'revocation-requested',
    'revocation-preflight-ready',
    'revocation-manual-required',
    'audit-export-ready',
  ].every((state) => states.has(state as ProviderSecretRotationRevocationStatusState));
}
