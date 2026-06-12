import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  DurableQueueRuntimeManualRequirementSchema,
  DurableQueueRuntimeReferenceSchema,
  ForbiddenDurableQueueRuntimeDataClasses,
  ProductionSupportStatusBackendDurableQueueRuntimeDataClassSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeNonClaimSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeSchemaVersionSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeSourceProofSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeTargetSchema,
  RequiredDurableQueueRuntimeNonClaims,
  RequiredDurableQueueRuntimeSourceProofs,
  RequiredDurableQueueRuntimeStates,
  RequiredDurableQueueRuntimeTargets,
} from './production-support-status-backend-durable-queue-runtime-values';

export * from './production-support-status-backend-durable-queue-runtime-values';

type DurableQueueRuntimeProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly runtimeBoundaryState: string }>;
  readonly sourceContractRefs: ReadonlyArray<string>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly statusBackendExecutionClaim: string;
  readonly durableQueueStorageClaim: string;
  readonly retryWorkerExecutionClaim: string;
  readonly auditPersistenceClaim: string;
  readonly deadLetterPayloadCustodyClaim: string;
  readonly publicRuntimeExecutionClaim: string;
  readonly providerExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly providerSecretCustodyClaim: string;
  readonly childActivityCustodyClaim: string;
};

type DurableQueueRuntimeExecutionStates = {
  readonly durableQueueStorageState: string;
  readonly retryWorkerState: string;
  readonly auditPersistenceState: string;
  readonly deadLetterPayloadCustodyState: string;
  readonly statusBackendExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly supportBackendUploadState: string;
  readonly legalDisclosureExecutionState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportStatusBackendDurableQueueRuntimeRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendDurableQueueRuntimeSchemaVersionSchema,
    target: ProductionSupportStatusBackendDurableQueueRuntimeTargetSchema,
    runtimeBoundaryState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    sourceProofRefs: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeSourceProofSchema),
    queueBoundaryRef: DurableQueueRuntimeReferenceSchema,
    retryWorkerRef: DurableQueueRuntimeReferenceSchema,
    auditPersistenceRef: DurableQueueRuntimeReferenceSchema,
    deadLetterRef: DurableQueueRuntimeReferenceSchema,
    runtimeExecutionRef: DurableQueueRuntimeReferenceSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeDataClassSchema),
    durableQueueStorageState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    retryWorkerState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    deadLetterPayloadCustodyState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    statusBackendExecutionState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    providerExecutionState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    legalDisclosureExecutionState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    childActivityCustodyState: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    manualRequirement: DurableQueueRuntimeManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredDurableQueueRuntimeStates.includes(row.runtimeBoundaryState as never) ||
        'Expected durable queue runtime rows to use required boundary states'
    ),
    Schema.filter(
      (row) =>
        RequiredDurableQueueRuntimeSourceProofs.every((sourceProof) => row.sourceProofRefs.includes(sourceProof)) ||
        'Expected durable queue runtime rows to link execution queue, audit persistence, dead-letter, runtime execution, and closure proofs'
    ),
    Schema.filter(
      (row) =>
        durableQueueRuntimeExecutionRemainsUnclaimed(row) ||
        'Expected durable queue runtime rows to preserve durable queue, retry worker, audit persistence, dead-letter, backend, public, provider, legal, and child-custody non-claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenDurableQueueRuntimeDataClasses.includes(dataClass as never)
        ) || 'Expected durable queue runtime rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenDurableQueueRuntimeDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected durable queue runtime rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendDurableQueueRuntimeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendDurableQueueRuntimeSchemaVersionSchema,
    sourceContractRefs: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeSourceProofSchema),
    rows: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendDurableQueueRuntimeNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    deadLetterPayloadCustodyClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    remoteSupportSessionClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    providerSecretCustodyClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendDurableQueueRuntimeStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendDurableQueueRuntimeProofIsHonest(proof) ||
        'Expected durable queue runtime proof to cover every target/state and preserve all runtime/storage/provider/legal/SLA/custody non-claims'
    )
  )
);

export type ProductionSupportStatusBackendDurableQueueRuntimeRow = Infer<
  typeof ProductionSupportStatusBackendDurableQueueRuntimeRowSchema
>;
export type ProductionSupportStatusBackendDurableQueueRuntimeProof = Infer<
  typeof ProductionSupportStatusBackendDurableQueueRuntimeProofSchema
>;
export type ProductionSupportStatusBackendDurableQueueRuntimeTarget = Infer<
  typeof ProductionSupportStatusBackendDurableQueueRuntimeTargetSchema
>;
export type ProductionSupportStatusBackendDurableQueueRuntimeState = (typeof RequiredDurableQueueRuntimeStates)[number];

export const decodeProductionSupportStatusBackendDurableQueueRuntimeProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendDurableQueueRuntimeProofSchema
);

export function summarizeProductionSupportStatusBackendDurableQueueRuntimeRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendDurableQueueRuntimeRow>
): Record<
  ProductionSupportStatusBackendDurableQueueRuntimeTarget,
  Record<ProductionSupportStatusBackendDurableQueueRuntimeState, number>
> {
  return RequiredDurableQueueRuntimeTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredDurableQueueRuntimeStates.reduce(
        (targetSummary, runtimeBoundaryState) => ({
          ...targetSummary,
          [runtimeBoundaryState]: rows.filter(
            (row) => row.target === target && row.runtimeBoundaryState === runtimeBoundaryState
          ).length,
        }),
        {} as Record<ProductionSupportStatusBackendDurableQueueRuntimeState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendDurableQueueRuntimeTarget,
      Record<ProductionSupportStatusBackendDurableQueueRuntimeState, number>
    >
  );
}

function durableQueueRuntimeExecutionRemainsUnclaimed(row: DurableQueueRuntimeExecutionStates): boolean {
  return (
    durableQueueRuntimeStorageRemainsManual(row) &&
    row.statusBackendExecutionState !== 'implemented' &&
    row.statusBackendExecutionState !== 'executed' &&
    externalRuntimeAndCustodyRemainUnclaimed(row)
  );
}

function durableQueueRuntimeStorageRemainsManual(row: DurableQueueRuntimeExecutionStates): boolean {
  return (
    row.durableQueueStorageState !== 'implemented' &&
    row.durableQueueStorageState !== 'executed' &&
    row.durableQueueStorageState !== 'persisted' &&
    row.retryWorkerState !== 'implemented' &&
    row.retryWorkerState !== 'executed' &&
    row.auditPersistenceState !== 'implemented' &&
    row.auditPersistenceState !== 'executed' &&
    row.auditPersistenceState !== 'persisted' &&
    row.deadLetterPayloadCustodyState !== 'implemented' &&
    row.deadLetterPayloadCustodyState !== 'executed' &&
    row.deadLetterPayloadCustodyState !== 'persisted'
  );
}

function externalRuntimeAndCustodyRemainUnclaimed(row: DurableQueueRuntimeExecutionStates): boolean {
  return (
    row.publicRuntimeExecutionState !== 'implemented' &&
    row.publicRuntimeExecutionState !== 'executed' &&
    row.providerExecutionState !== 'implemented' &&
    row.providerExecutionState !== 'executed' &&
    row.supportBackendUploadState !== 'implemented' &&
    row.supportBackendUploadState !== 'executed' &&
    row.legalDisclosureExecutionState !== 'implemented' &&
    row.legalDisclosureExecutionState !== 'executed' &&
    row.childActivityCustodyState !== 'implemented' &&
    row.childActivityCustodyState !== 'persisted'
  );
}

function productionSupportStatusBackendDurableQueueRuntimeProofIsHonest(
  proof: DurableQueueRuntimeProofCandidate
): boolean {
  return (
    RequiredDurableQueueRuntimeTargets.every((target) =>
      RequiredDurableQueueRuntimeStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.runtimeBoundaryState === state)
      )
    ) &&
    RequiredDurableQueueRuntimeSourceProofs.every((sourceProof) => proof.sourceContractRefs.includes(sourceProof)) &&
    RequiredDurableQueueRuntimeNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    durableQueueRuntimeClaimsRemainManual(proof)
  );
}

function durableQueueRuntimeClaimsRemainManual(proof: DurableQueueRuntimeProofCandidate): boolean {
  return (
    durableQueueRuntimeBackendClaimsRemainManual(proof) &&
    durableQueueRuntimeSupportClaimsRemainManual(proof) &&
    durableQueueRuntimeExternalClaimsRemainUnclaimed(proof)
  );
}

function durableQueueRuntimeBackendClaimsRemainManual(proof: DurableQueueRuntimeProofCandidate): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
    proof.deadLetterPayloadCustodyClaim === 'manual-required'
  );
}

function durableQueueRuntimeSupportClaimsRemainManual(proof: DurableQueueRuntimeProofCandidate): boolean {
  return (
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.legalDisclosureExecutionClaim === 'manual-required'
  );
}

function durableQueueRuntimeExternalClaimsRemainUnclaimed(proof: DurableQueueRuntimeProofCandidate): boolean {
  return (
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.providerExecutionClaim === 'not-implemented' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.providerSecretCustodyClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
