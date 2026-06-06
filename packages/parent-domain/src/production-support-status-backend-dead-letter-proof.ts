import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenStatusBackendDeadLetterDataClasses,
  ProductionSupportStatusBackendDeadLetterDataClassSchema,
  ProductionSupportStatusBackendDeadLetterNonClaimSchema,
  ProductionSupportStatusBackendDeadLetterSchemaVersionSchema,
  ProductionSupportStatusBackendDeadLetterSourceProofSchema,
  ProductionSupportStatusBackendDeadLetterStateSchema,
  ProductionSupportStatusBackendDeadLetterTargetSchema,
  RequiredStatusBackendDeadLetterNonClaims,
  RequiredStatusBackendDeadLetterStates,
  RequiredStatusBackendDeadLetterTargets,
  StatusBackendDeadLetterManualRequirementSchema,
  StatusBackendDeadLetterReferenceSchema,
} from './production-support-status-backend-dead-letter-values';

export * from './production-support-status-backend-dead-letter-values';

type StatusBackendDeadLetterProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly deadLetterState: string }>;
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

export const ProductionSupportStatusBackendDeadLetterRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendDeadLetterSchemaVersionSchema,
    target: ProductionSupportStatusBackendDeadLetterTargetSchema,
    sourceProof: ProductionSupportStatusBackendDeadLetterSourceProofSchema,
    deadLetterState: ProductionSupportStatusBackendDeadLetterStateSchema,
    sourceContractState: ProductionSupportStatusBackendDeadLetterStateSchema,
    statusContractState: ProductionSupportStatusBackendDeadLetterStateSchema,
    durableQueueStorageState: ProductionSupportStatusBackendDeadLetterStateSchema,
    retryWorkerState: ProductionSupportStatusBackendDeadLetterStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendDeadLetterStateSchema,
    backendExecutionState: ProductionSupportStatusBackendDeadLetterStateSchema,
    deadLetterPayloadCustodyState: ProductionSupportStatusBackendDeadLetterStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendDeadLetterStateSchema,
    providerExecutionState: ProductionSupportStatusBackendDeadLetterStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendDeadLetterStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendDeadLetterDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendDeadLetterDataClassSchema),
    queueReference: StatusBackendDeadLetterReferenceSchema,
    deadLetterReference: StatusBackendDeadLetterReferenceSchema,
    retryReference: StatusBackendDeadLetterReferenceSchema,
    auditReference: StatusBackendDeadLetterReferenceSchema,
    manualRequirement: StatusBackendDeadLetterManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredStatusBackendDeadLetterStates.includes(row.deadLetterState as never) ||
        'Expected status backend dead-letter rows to use required dead-letter states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected status backend dead-letter rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        (row.durableQueueStorageState !== 'implemented' &&
          row.durableQueueStorageState !== 'executed' &&
          row.durableQueueStorageState !== 'persisted') ||
        'Expected status backend dead-letter rows to avoid durable queue storage claims'
    ),
    Schema.filter(
      (row) =>
        (row.retryWorkerState !== 'implemented' && row.retryWorkerState !== 'executed') ||
        'Expected status backend dead-letter rows to avoid retry worker execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.auditPersistenceState !== 'implemented' &&
          row.auditPersistenceState !== 'executed' &&
          row.auditPersistenceState !== 'persisted') ||
        'Expected status backend dead-letter rows to avoid audit persistence claims'
    ),
    Schema.filter(
      (row) =>
        (row.backendExecutionState !== 'implemented' && row.backendExecutionState !== 'executed') ||
        'Expected status backend dead-letter rows to avoid backend execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.deadLetterPayloadCustodyState !== 'implemented' &&
          row.deadLetterPayloadCustodyState !== 'executed' &&
          row.deadLetterPayloadCustodyState !== 'persisted') ||
        'Expected status backend dead-letter rows to avoid dead-letter payload custody claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeExecutionState !== 'implemented' && row.publicRuntimeExecutionState !== 'executed') ||
        'Expected status backend dead-letter rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.providerExecutionState !== 'implemented' && row.providerExecutionState !== 'executed') ||
        'Expected status backend dead-letter rows to avoid provider execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected status backend dead-letter rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenStatusBackendDeadLetterDataClasses.includes(dataClass as never)
        ) || 'Expected status backend dead-letter rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenStatusBackendDeadLetterDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected status backend dead-letter rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendDeadLetterProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendDeadLetterSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportStatusBackendDeadLetterRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendDeadLetterNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    deadLetterPayloadCustodyClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendDeadLetterStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendDeadLetterProofIsHonest(proof) ||
        'Expected status backend dead-letter proof to cover rows and preserve non-claims'
    )
  )
);

export type ProductionSupportStatusBackendDeadLetterRow = Infer<
  typeof ProductionSupportStatusBackendDeadLetterRowSchema
>;
export type ProductionSupportStatusBackendDeadLetterProof = Infer<
  typeof ProductionSupportStatusBackendDeadLetterProofSchema
>;
export type ProductionSupportStatusBackendDeadLetterTarget = Infer<
  typeof ProductionSupportStatusBackendDeadLetterTargetSchema
>;
export type ProductionSupportStatusBackendDeadLetterState = (typeof RequiredStatusBackendDeadLetterStates)[number];

export const decodeProductionSupportStatusBackendDeadLetterProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendDeadLetterProofSchema
);

export function summarizeProductionSupportStatusBackendDeadLetterRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendDeadLetterRow>
): Record<
  ProductionSupportStatusBackendDeadLetterTarget,
  Record<ProductionSupportStatusBackendDeadLetterState, number>
> {
  return RequiredStatusBackendDeadLetterTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredStatusBackendDeadLetterStates.reduce(
        (targetSummary, deadLetterState) => ({
          ...targetSummary,
          [deadLetterState]: rows.filter((row) => row.target === target && row.deadLetterState === deadLetterState)
            .length,
        }),
        {} as Record<ProductionSupportStatusBackendDeadLetterState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendDeadLetterTarget,
      Record<ProductionSupportStatusBackendDeadLetterState, number>
    >
  );
}

function productionSupportStatusBackendDeadLetterProofIsHonest(proof: StatusBackendDeadLetterProofCandidate): boolean {
  return (
    RequiredStatusBackendDeadLetterTargets.every((target) =>
      RequiredStatusBackendDeadLetterStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.deadLetterState === state)
      )
    ) &&
    RequiredStatusBackendDeadLetterNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    statusBackendDeadLetterClaimsRemainManual(proof)
  );
}

function statusBackendDeadLetterClaimsRemainManual(proof: StatusBackendDeadLetterProofCandidate): boolean {
  return (
    statusBackendDeadLetterInfrastructureClaimsRemainManual(proof) &&
    statusBackendDeadLetterExternalClaimsRemainManual(proof)
  );
}

function statusBackendDeadLetterInfrastructureClaimsRemainManual(
  proof: StatusBackendDeadLetterProofCandidate
): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
    proof.deadLetterPayloadCustodyClaim === 'manual-required'
  );
}

function statusBackendDeadLetterExternalClaimsRemainManual(proof: StatusBackendDeadLetterProofCandidate): boolean {
  return (
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.providerExecutionClaim === 'not-implemented' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
