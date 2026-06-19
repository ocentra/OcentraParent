import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ForbiddenRuntimeClosureDataClasses,
  ProductionSupportStatusBackendRuntimeClosureDataClassSchema,
  ProductionSupportStatusBackendRuntimeClosureNonClaimSchema,
  ProductionSupportStatusBackendRuntimeClosureSchemaVersionSchema,
  ProductionSupportStatusBackendRuntimeClosureSourceProofSchema,
  ProductionSupportStatusBackendRuntimeClosureStateSchema,
  ProductionSupportStatusBackendRuntimeClosureTargetSchema,
  RequiredRuntimeClosureNonClaims,
  RequiredRuntimeClosureSourceProofs,
  RequiredRuntimeClosureStates,
  RequiredRuntimeClosureTargets,
  RuntimeClosureManualRequirementSchema,
  RuntimeClosureReferenceSchema,
} from './production-support-status-backend-runtime-closure-values';

type RuntimeClosureProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly closureState: string }>;
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
  readonly childActivityCustodyClaim: string;
};

type RuntimeClosureExecutionStates = {
  readonly durableQueueStorageState: string;
  readonly retryWorkerState: string;
  readonly auditPersistenceState: string;
  readonly deadLetterPayloadCustodyState: string;
  readonly statusBackendPayloadCustodyState: string;
  readonly redactionManifestExecutionState: string;
  readonly statusBackendExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly supportBackendUploadState: string;
  readonly legalDisclosureExecutionState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportStatusBackendRuntimeClosureRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendRuntimeClosureSchemaVersionSchema,
    target: ProductionSupportStatusBackendRuntimeClosureTargetSchema,
    closureState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    runtimeExecutionRef: RuntimeClosureReferenceSchema,
    queueAuditPersistenceRef: RuntimeClosureReferenceSchema,
    deadLetterRef: RuntimeClosureReferenceSchema,
    payloadCustodyRef: RuntimeClosureReferenceSchema,
    redactionManifestRef: RuntimeClosureReferenceSchema,
    sourceProofRefs: Schema.Array(ProductionSupportStatusBackendRuntimeClosureSourceProofSchema),
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendRuntimeClosureDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendRuntimeClosureDataClassSchema),
    durableQueueStorageState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    retryWorkerState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    deadLetterPayloadCustodyState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    statusBackendPayloadCustodyState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    redactionManifestExecutionState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    statusBackendExecutionState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    providerExecutionState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    legalDisclosureExecutionState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    childActivityCustodyState: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    manualRequirement: RuntimeClosureManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredRuntimeClosureStates.includes(row.closureState as never) ||
        'Expected runtime closure rows to use required closure states'
    ),
    Schema.filter(
      (row) =>
        RequiredRuntimeClosureSourceProofs.every((sourceProof) => row.sourceProofRefs.includes(sourceProof)) ||
        'Expected runtime closure rows to link runtime, queue, dead-letter, payload custody, redaction manifest, and public follow-through proofs'
    ),
    Schema.filter(
      (row) =>
        runtimeClosureExecutionRemainsUnclaimed(row) ||
        'Expected runtime closure rows to preserve backend execution, durable queue, audit, payload custody, redaction execution, public runtime, provider, legal, and child-custody non-claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenRuntimeClosureDataClasses.includes(dataClass as never)
        ) || 'Expected runtime closure rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenRuntimeClosureDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected runtime closure rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendRuntimeClosureProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendRuntimeClosureSchemaVersionSchema,
    sourceContractRefs: Schema.Array(ProductionSupportStatusBackendRuntimeClosureSourceProofSchema),
    rows: Schema.Array(ProductionSupportStatusBackendRuntimeClosureRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendRuntimeClosureNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    deadLetterPayloadCustodyClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    statusBackendPayloadCustodyClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    redactionManifestExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    remoteSupportSessionClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    providerSecretCustodyClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendRuntimeClosureStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendRuntimeClosureProofIsHonest(proof) ||
        'Expected runtime closure proof to cover every target/state and preserve all backend/public/provider/legal/SLA/custody non-claims'
    )
  )
);

export type ProductionSupportStatusBackendRuntimeClosureRow = Infer<
  typeof ProductionSupportStatusBackendRuntimeClosureRowSchema
>;
export type ProductionSupportStatusBackendRuntimeClosureProof = Infer<
  typeof ProductionSupportStatusBackendRuntimeClosureProofSchema
>;
export type ProductionSupportStatusBackendRuntimeClosureTarget = Infer<
  typeof ProductionSupportStatusBackendRuntimeClosureTargetSchema
>;
export type ProductionSupportStatusBackendRuntimeClosureState = (typeof RequiredRuntimeClosureStates)[number];

export const decodeProductionSupportStatusBackendRuntimeClosureProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendRuntimeClosureProofSchema
);

export function summarizeProductionSupportStatusBackendRuntimeClosureRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendRuntimeClosureRow>
): Record<
  ProductionSupportStatusBackendRuntimeClosureTarget,
  Record<ProductionSupportStatusBackendRuntimeClosureState, number>
> {
  return RequiredRuntimeClosureTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredRuntimeClosureStates.reduce(
        (targetSummary, closureState) => ({
          ...targetSummary,
          [closureState]: rows.filter((row) => row.target === target && row.closureState === closureState).length,
        }),
        {} as Record<ProductionSupportStatusBackendRuntimeClosureState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendRuntimeClosureTarget,
      Record<ProductionSupportStatusBackendRuntimeClosureState, number>
    >
  );
}

function runtimeClosureExecutionRemainsUnclaimed(row: RuntimeClosureExecutionStates): boolean {
  return (
    durableRuntimeStorageRemainsManual(row) &&
    payloadAndRedactionRemainManual(row) &&
    externalRuntimeAndCustodyRemainUnclaimed(row)
  );
}

function durableRuntimeStorageRemainsManual(row: RuntimeClosureExecutionStates): boolean {
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

function payloadAndRedactionRemainManual(row: RuntimeClosureExecutionStates): boolean {
  return (
    row.statusBackendPayloadCustodyState !== 'implemented' &&
    row.statusBackendPayloadCustodyState !== 'executed' &&
    row.statusBackendPayloadCustodyState !== 'persisted' &&
    row.redactionManifestExecutionState !== 'implemented' &&
    row.redactionManifestExecutionState !== 'executed' &&
    row.statusBackendExecutionState !== 'implemented' &&
    row.statusBackendExecutionState !== 'executed'
  );
}

function externalRuntimeAndCustodyRemainUnclaimed(row: RuntimeClosureExecutionStates): boolean {
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

function productionSupportStatusBackendRuntimeClosureProofIsHonest(proof: RuntimeClosureProofCandidate): boolean {
  return (
    RequiredRuntimeClosureTargets.every((target) =>
      RequiredRuntimeClosureStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.closureState === state)
      )
    ) &&
    RequiredRuntimeClosureSourceProofs.every((sourceProof) => proof.sourceContractRefs.includes(sourceProof)) &&
    RequiredRuntimeClosureNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    runtimeClosureClaimsRemainManual(proof)
  );
}

function runtimeClosureClaimsRemainManual(proof: RuntimeClosureProofCandidate): boolean {
  return (
    backendRuntimeClaimsRemainManual(proof) &&
    payloadAndRedactionClaimsRemainManual(proof) &&
    externalAndCustodyClaimsRemainManual(proof)
  );
}

function backendRuntimeClaimsRemainManual(proof: RuntimeClosureProofCandidate): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
    proof.deadLetterPayloadCustodyClaim === 'manual-required'
  );
}

function payloadAndRedactionClaimsRemainManual(proof: RuntimeClosureProofCandidate): boolean {
  return (
    proof.statusBackendPayloadCustodyClaim === 'manual-required' &&
    proof.redactionManifestExecutionClaim === 'manual-required'
  );
}

function externalAndCustodyClaimsRemainManual(proof: RuntimeClosureProofCandidate): boolean {
  return (
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.providerExecutionClaim === 'not-implemented' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.providerSecretCustodyClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
