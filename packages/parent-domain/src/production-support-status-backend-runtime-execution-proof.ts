import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenRuntimeExecutionDataClasses,
  ProductionSupportStatusBackendRuntimeExecutionDataClassSchema,
  ProductionSupportStatusBackendRuntimeExecutionNonClaimSchema,
  ProductionSupportStatusBackendRuntimeExecutionSchemaVersionSchema,
  ProductionSupportStatusBackendRuntimeExecutionSourceProofSchema,
  ProductionSupportStatusBackendRuntimeExecutionStateSchema,
  ProductionSupportStatusBackendRuntimeExecutionTargetSchema,
  RequiredRuntimeExecutionNonClaims,
  RequiredRuntimeExecutionStates,
  RequiredRuntimeExecutionTargets,
  RuntimeExecutionManualRequirementSchema,
  RuntimeExecutionReferenceSchema,
} from './production-support-status-backend-runtime-execution-values';

export * from './production-support-status-backend-runtime-execution-values';

type RuntimeExecutionProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly readinessState: string }>;
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
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

type RuntimeExecutionInfrastructureStates = {
  readonly durableQueueStorageState: string;
  readonly retryWorkerState: string;
  readonly auditPersistenceState: string;
  readonly deadLetterState: string;
  readonly runtimeEvidenceState: string;
  readonly backendExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly supportBackendUploadState: string;
};

export const ProductionSupportStatusBackendRuntimeExecutionRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendRuntimeExecutionSchemaVersionSchema,
    target: ProductionSupportStatusBackendRuntimeExecutionTargetSchema,
    sourceProof: ProductionSupportStatusBackendRuntimeExecutionSourceProofSchema,
    readinessState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    sourceContractState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    statusContractState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    durableQueueStorageState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    retryWorkerState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    deadLetterState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    runtimeEvidenceState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    backendExecutionState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    providerExecutionState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendRuntimeExecutionDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendRuntimeExecutionDataClassSchema),
    queueReference: RuntimeExecutionReferenceSchema,
    retryReference: RuntimeExecutionReferenceSchema,
    auditReference: RuntimeExecutionReferenceSchema,
    deadLetterReference: RuntimeExecutionReferenceSchema,
    runtimeEvidenceReference: RuntimeExecutionReferenceSchema,
    manualRequirement: RuntimeExecutionManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredRuntimeExecutionStates.includes(row.readinessState as never) ||
        'Expected runtime execution rows to use required readiness states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected runtime execution rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        runtimeInfrastructureRemainsUnclaimed(row) ||
        'Expected runtime execution rows to avoid durable queue retry audit dead-letter and backend execution claims'
    ),
    Schema.filter(
      (row) =>
        publicAndProviderRuntimeRemainsUnclaimed(row) ||
        'Expected runtime execution rows to avoid public runtime provider and upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenRuntimeExecutionDataClasses.includes(dataClass as never)
        ) || 'Expected runtime execution rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenRuntimeExecutionDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected runtime execution rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendRuntimeExecutionProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendRuntimeExecutionSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportStatusBackendRuntimeExecutionRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendRuntimeExecutionNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    deadLetterPayloadCustodyClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendRuntimeExecutionStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendRuntimeExecutionProofIsHonest(proof) ||
        'Expected runtime execution proof to cover rows and preserve non-claims'
    )
  )
);

export type ProductionSupportStatusBackendRuntimeExecutionRow = Infer<
  typeof ProductionSupportStatusBackendRuntimeExecutionRowSchema
>;
export type ProductionSupportStatusBackendRuntimeExecutionProof = Infer<
  typeof ProductionSupportStatusBackendRuntimeExecutionProofSchema
>;
export type ProductionSupportStatusBackendRuntimeExecutionTarget = Infer<
  typeof ProductionSupportStatusBackendRuntimeExecutionTargetSchema
>;
export type ProductionSupportStatusBackendRuntimeExecutionState = (typeof RequiredRuntimeExecutionStates)[number];

export const decodeProductionSupportStatusBackendRuntimeExecutionProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendRuntimeExecutionProofSchema
);

export function summarizeProductionSupportStatusBackendRuntimeExecutionRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendRuntimeExecutionRow>
): Record<
  ProductionSupportStatusBackendRuntimeExecutionTarget,
  Record<ProductionSupportStatusBackendRuntimeExecutionState, number>
> {
  return RequiredRuntimeExecutionTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredRuntimeExecutionStates.reduce(
        (targetSummary, readinessState) => ({
          ...targetSummary,
          [readinessState]: rows.filter((row) => row.target === target && row.readinessState === readinessState).length,
        }),
        {} as Record<ProductionSupportStatusBackendRuntimeExecutionState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendRuntimeExecutionTarget,
      Record<ProductionSupportStatusBackendRuntimeExecutionState, number>
    >
  );
}

function runtimeInfrastructureRemainsUnclaimed(row: RuntimeExecutionInfrastructureStates): boolean {
  return (
    durableQueueRemainsUnclaimed(row) &&
    retryAuditAndDeadLetterRemainUnclaimed(row) &&
    backendAndEvidenceRemainUnclaimed(row)
  );
}

function durableQueueRemainsUnclaimed(row: RuntimeExecutionInfrastructureStates): boolean {
  return (
    row.durableQueueStorageState !== 'implemented' &&
    row.durableQueueStorageState !== 'executed' &&
    row.durableQueueStorageState !== 'persisted'
  );
}

function retryAuditAndDeadLetterRemainUnclaimed(row: RuntimeExecutionInfrastructureStates): boolean {
  return (
    row.retryWorkerState !== 'implemented' &&
    row.retryWorkerState !== 'executed' &&
    row.auditPersistenceState !== 'implemented' &&
    row.auditPersistenceState !== 'executed' &&
    row.auditPersistenceState !== 'persisted' &&
    row.deadLetterState !== 'implemented' &&
    row.deadLetterState !== 'executed' &&
    row.deadLetterState !== 'persisted'
  );
}

function backendAndEvidenceRemainUnclaimed(row: RuntimeExecutionInfrastructureStates): boolean {
  return (
    row.backendExecutionState !== 'implemented' &&
    row.backendExecutionState !== 'executed' &&
    row.runtimeEvidenceState !== 'executed'
  );
}

function publicAndProviderRuntimeRemainsUnclaimed(row: RuntimeExecutionInfrastructureStates): boolean {
  return (
    row.publicRuntimeExecutionState !== 'implemented' &&
    row.publicRuntimeExecutionState !== 'executed' &&
    row.providerExecutionState !== 'implemented' &&
    row.providerExecutionState !== 'executed' &&
    row.supportBackendUploadState !== 'implemented' &&
    row.supportBackendUploadState !== 'executed'
  );
}

function productionSupportStatusBackendRuntimeExecutionProofIsHonest(proof: RuntimeExecutionProofCandidate): boolean {
  return (
    RequiredRuntimeExecutionTargets.every((target) =>
      RequiredRuntimeExecutionStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.readinessState === state)
      )
    ) &&
    RequiredRuntimeExecutionNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    runtimeExecutionClaimsRemainManual(proof)
  );
}

function runtimeExecutionClaimsRemainManual(proof: RuntimeExecutionProofCandidate): boolean {
  return (
    backendQueueAndAuditClaimsRemainManual(proof) &&
    externalRuntimeClaimsRemainManual(proof) &&
    custodyAndDisclosureClaimsRemainManual(proof)
  );
}

function backendQueueAndAuditClaimsRemainManual(proof: RuntimeExecutionProofCandidate): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
    proof.deadLetterPayloadCustodyClaim === 'manual-required'
  );
}

function externalRuntimeClaimsRemainManual(proof: RuntimeExecutionProofCandidate): boolean {
  return (
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.providerExecutionClaim === 'not-implemented' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented'
  );
}

function custodyAndDisclosureClaimsRemainManual(proof: RuntimeExecutionProofCandidate): boolean {
  return (
    proof.legalDisclosureExecutionClaim === 'manual-required' && proof.childActivityCustodyClaim === 'not-implemented'
  );
}
