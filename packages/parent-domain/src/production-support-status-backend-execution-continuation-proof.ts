import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ExecutionContinuationManualRequirementSchema,
  ExecutionContinuationReferenceSchema,
  ForbiddenExecutionContinuationDataClasses,
  ProductionSupportStatusBackendExecutionContinuationDataClassSchema,
  ProductionSupportStatusBackendExecutionContinuationNonClaimSchema,
  ProductionSupportStatusBackendExecutionContinuationSchemaVersionSchema,
  ProductionSupportStatusBackendExecutionContinuationSourceProofSchema,
  ProductionSupportStatusBackendExecutionContinuationStateSchema,
  ProductionSupportStatusBackendExecutionContinuationTargetSchema,
  RequiredExecutionContinuationNonClaims,
  RequiredExecutionContinuationSourceProofs,
  RequiredExecutionContinuationStates,
  RequiredExecutionContinuationTargets,
} from './production-support-status-backend-execution-continuation-values';

export * from './production-support-status-backend-execution-continuation-values';

type ExecutionContinuationProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly continuationState: string }>;
  readonly sourceContractRefs: ReadonlyArray<string>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly statusBackendExecutionClaim: string;
  readonly durableQueueStorageClaim: string;
  readonly retryWorkerExecutionClaim: string;
  readonly auditPersistenceClaim: string;
  readonly deadLetterPayloadCustodyClaim: string;
  readonly statusBackendPayloadCustodyClaim: string;
  readonly redactionManifestExecutionClaim: string;
  readonly publicRuntimeExecutionClaim: string;
  readonly providerExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly providerSecretCustodyClaim: string;
  readonly defaultHostedFamilyDataClaim: string;
  readonly childActivityCustodyClaim: string;
};

type ExecutionContinuationRowClaims = {
  readonly statusBackendExecutionState: string;
  readonly durableQueueStorageState: string;
  readonly retryWorkerExecutionState: string;
  readonly auditPersistenceState: string;
  readonly deadLetterPayloadCustodyState: string;
  readonly statusBackendPayloadCustodyState: string;
  readonly redactionManifestExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly supportBackendUploadExecutionState: string;
  readonly legalDisclosureExecutionState: string;
  readonly defaultHostedFamilyDataState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportStatusBackendExecutionContinuationRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendExecutionContinuationSchemaVersionSchema,
    target: ProductionSupportStatusBackendExecutionContinuationTargetSchema,
    continuationState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    sourceProofRefs: Schema.Array(ProductionSupportStatusBackendExecutionContinuationSourceProofSchema),
    durableQueueRuntimeRef: ExecutionContinuationReferenceSchema,
    payloadCustodyBoundaryRef: ExecutionContinuationReferenceSchema,
    redactionManifestRef: ExecutionContinuationReferenceSchema,
    manualProofRef: ExecutionContinuationReferenceSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendExecutionContinuationDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendExecutionContinuationDataClassSchema),
    statusBackendExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    durableQueueStorageState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    retryWorkerExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    deadLetterPayloadCustodyState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    statusBackendPayloadCustodyState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    redactionManifestExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    providerExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    supportBackendUploadExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    legalDisclosureExecutionState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    defaultHostedFamilyDataState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    childActivityCustodyState: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    manualRequirement: ExecutionContinuationManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredExecutionContinuationStates.includes(row.continuationState as never) ||
        'Expected execution continuation rows to use required continuation states'
    ),
    Schema.filter(
      (row) =>
        RequiredExecutionContinuationSourceProofs.every((sourceProof) => row.sourceProofRefs.includes(sourceProof)) ||
        'Expected execution continuation rows to link durable runtime, closure, payload custody, and redaction manifest proofs'
    ),
    Schema.filter(
      (row) =>
        executionContinuationClaimsRemainUnimplemented(row) ||
        'Expected execution continuation rows to preserve backend execution, storage, custody, redaction, provider, legal, SLA, hosted-data, and child-custody non-claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenExecutionContinuationDataClasses.includes(dataClass as never)
        ) || 'Expected execution continuation rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenExecutionContinuationDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected execution continuation rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendExecutionContinuationProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendExecutionContinuationSchemaVersionSchema,
    sourceContractRefs: Schema.Array(ProductionSupportStatusBackendExecutionContinuationSourceProofSchema),
    rows: Schema.Array(ProductionSupportStatusBackendExecutionContinuationRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendExecutionContinuationNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    deadLetterPayloadCustodyClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    statusBackendPayloadCustodyClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    redactionManifestExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    remoteSupportSessionClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    providerSecretCustodyClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    defaultHostedFamilyDataClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendExecutionContinuationStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendExecutionContinuationProofIsHonest(proof) ||
        'Expected execution continuation proof to cover every target/state and preserve all runtime/storage/custody/provider/legal/SLA non-claims'
    )
  )
);

export type ProductionSupportStatusBackendExecutionContinuationRow = Infer<
  typeof ProductionSupportStatusBackendExecutionContinuationRowSchema
>;
export type ProductionSupportStatusBackendExecutionContinuationProof = Infer<
  typeof ProductionSupportStatusBackendExecutionContinuationProofSchema
>;
export type ProductionSupportStatusBackendExecutionContinuationTarget = Infer<
  typeof ProductionSupportStatusBackendExecutionContinuationTargetSchema
>;
export type ProductionSupportStatusBackendExecutionContinuationState =
  (typeof RequiredExecutionContinuationStates)[number];

export const decodeProductionSupportStatusBackendExecutionContinuationProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendExecutionContinuationProofSchema
);

export function summarizeProductionSupportStatusBackendExecutionContinuationRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendExecutionContinuationRow>
): Record<
  ProductionSupportStatusBackendExecutionContinuationTarget,
  Record<ProductionSupportStatusBackendExecutionContinuationState, number>
> {
  return RequiredExecutionContinuationTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredExecutionContinuationStates.reduce(
        (targetSummary, continuationState) => ({
          ...targetSummary,
          [continuationState]: rows.filter(
            (row) => row.target === target && row.continuationState === continuationState
          ).length,
        }),
        {} as Record<ProductionSupportStatusBackendExecutionContinuationState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendExecutionContinuationTarget,
      Record<ProductionSupportStatusBackendExecutionContinuationState, number>
    >
  );
}

function executionContinuationClaimsRemainUnimplemented(row: ExecutionContinuationRowClaims): boolean {
  return (
    statusBackendRuntimeClaimsRemainManual(row) &&
    externalContinuationClaimsRemainUnimplemented(row) &&
    row.defaultHostedFamilyDataState !== 'implemented' &&
    row.childActivityCustodyState !== 'persisted'
  );
}

function statusBackendRuntimeClaimsRemainManual(row: ExecutionContinuationRowClaims): boolean {
  return (
    row.statusBackendExecutionState !== 'executed' &&
    row.durableQueueStorageState !== 'persisted' &&
    row.retryWorkerExecutionState !== 'executed' &&
    row.auditPersistenceState !== 'persisted' &&
    row.deadLetterPayloadCustodyState !== 'persisted' &&
    row.statusBackendPayloadCustodyState !== 'persisted' &&
    row.redactionManifestExecutionState !== 'executed'
  );
}

function externalContinuationClaimsRemainUnimplemented(row: ExecutionContinuationRowClaims): boolean {
  return (
    row.publicRuntimeExecutionState !== 'executed' &&
    row.providerExecutionState !== 'executed' &&
    row.supportBackendUploadExecutionState !== 'executed' &&
    row.legalDisclosureExecutionState !== 'executed'
  );
}

function productionSupportStatusBackendExecutionContinuationProofIsHonest(
  proof: ExecutionContinuationProofCandidate
): boolean {
  return (
    RequiredExecutionContinuationTargets.every((target) =>
      RequiredExecutionContinuationStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.continuationState === state)
      )
    ) &&
    RequiredExecutionContinuationSourceProofs.every((sourceProof) => proof.sourceContractRefs.includes(sourceProof)) &&
    RequiredExecutionContinuationNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    executionContinuationTopLevelClaimsRemainManual(proof)
  );
}

function executionContinuationTopLevelClaimsRemainManual(proof: ExecutionContinuationProofCandidate): boolean {
  return (
    executionContinuationBackendClaimsRemainManual(proof) &&
    executionContinuationSupportClaimsRemainManual(proof) &&
    executionContinuationExternalClaimsRemainUnimplemented(proof)
  );
}

function executionContinuationBackendClaimsRemainManual(proof: ExecutionContinuationProofCandidate): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
    proof.deadLetterPayloadCustodyClaim === 'manual-required' &&
    proof.statusBackendPayloadCustodyClaim === 'manual-required' &&
    proof.redactionManifestExecutionClaim === 'manual-required'
  );
}

function executionContinuationSupportClaimsRemainManual(proof: ExecutionContinuationProofCandidate): boolean {
  return (
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.legalDisclosureExecutionClaim === 'manual-required'
  );
}

function executionContinuationExternalClaimsRemainUnimplemented(proof: ExecutionContinuationProofCandidate): boolean {
  return (
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.providerExecutionClaim === 'not-implemented' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.providerSecretCustodyClaim === 'not-implemented' &&
    proof.defaultHostedFamilyDataClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
