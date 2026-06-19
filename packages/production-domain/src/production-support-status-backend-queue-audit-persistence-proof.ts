import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ForbiddenQueueAuditPersistenceDataClasses,
  ProductionSupportStatusBackendQueueAuditPersistenceDataClassSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceNonClaimSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceSchemaVersionSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceSourceProofSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceTargetSchema,
  QueueAuditPersistenceManualRequirementSchema,
  QueueAuditPersistenceReferenceSchema,
  RequiredQueueAuditPersistenceNonClaims,
  RequiredQueueAuditPersistenceStates,
  RequiredQueueAuditPersistenceTargets,
} from './production-support-status-backend-queue-audit-persistence-values';

type QueueAuditPersistenceProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly readinessState: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly statusBackendExecutionClaim: string;
  readonly durableQueueStorageClaim: string;
  readonly retryWorkerExecutionClaim: string;
  readonly auditPersistenceClaim: string;
  readonly publicRuntimeExecutionClaim: string;
  readonly providerExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportStatusBackendQueueAuditPersistenceRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendQueueAuditPersistenceSchemaVersionSchema,
    target: ProductionSupportStatusBackendQueueAuditPersistenceTargetSchema,
    sourceProof: ProductionSupportStatusBackendQueueAuditPersistenceSourceProofSchema,
    readinessState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    sourceContractState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    statusContractState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    durableQueueStorageState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    retryWorkerState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    auditPersistenceState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    backendExecutionState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    providerExecutionState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendQueueAuditPersistenceDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendQueueAuditPersistenceDataClassSchema),
    queueReference: QueueAuditPersistenceReferenceSchema,
    retryReference: QueueAuditPersistenceReferenceSchema,
    auditReference: QueueAuditPersistenceReferenceSchema,
    manualRequirement: QueueAuditPersistenceManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredQueueAuditPersistenceStates.includes(row.readinessState as never) ||
        'Expected queue audit persistence rows to use required readiness states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected queue audit persistence rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        (row.durableQueueStorageState !== 'implemented' &&
          row.durableQueueStorageState !== 'executed' &&
          row.durableQueueStorageState !== 'persisted') ||
        'Expected queue audit persistence rows to avoid durable queue storage claims'
    ),
    Schema.filter(
      (row) =>
        (row.retryWorkerState !== 'implemented' && row.retryWorkerState !== 'executed') ||
        'Expected queue audit persistence rows to avoid retry worker execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.auditPersistenceState !== 'implemented' &&
          row.auditPersistenceState !== 'executed' &&
          row.auditPersistenceState !== 'persisted') ||
        'Expected queue audit persistence rows to avoid audit persistence claims'
    ),
    Schema.filter(
      (row) =>
        (row.backendExecutionState !== 'implemented' && row.backendExecutionState !== 'executed') ||
        'Expected queue audit persistence rows to avoid backend execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeExecutionState !== 'implemented' && row.publicRuntimeExecutionState !== 'executed') ||
        'Expected queue audit persistence rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.providerExecutionState !== 'implemented' && row.providerExecutionState !== 'executed') ||
        'Expected queue audit persistence rows to avoid provider execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected queue audit persistence rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenQueueAuditPersistenceDataClasses.includes(dataClass as never)
        ) || 'Expected queue audit persistence rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenQueueAuditPersistenceDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected queue audit persistence rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendQueueAuditPersistenceProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendQueueAuditPersistenceSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportStatusBackendQueueAuditPersistenceRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendQueueAuditPersistenceNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    durableQueueStorageClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    retryWorkerExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    auditPersistenceClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendQueueAuditPersistenceStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendQueueAuditPersistenceProofIsHonest(proof) ||
        'Expected queue audit persistence proof to cover rows and preserve non-claims'
    )
  )
);

export type ProductionSupportStatusBackendQueueAuditPersistenceRow = Infer<
  typeof ProductionSupportStatusBackendQueueAuditPersistenceRowSchema
>;
export type ProductionSupportStatusBackendQueueAuditPersistenceProof = Infer<
  typeof ProductionSupportStatusBackendQueueAuditPersistenceProofSchema
>;
export type ProductionSupportStatusBackendQueueAuditPersistenceTarget = Infer<
  typeof ProductionSupportStatusBackendQueueAuditPersistenceTargetSchema
>;
export type ProductionSupportStatusBackendQueueAuditPersistenceState =
  (typeof RequiredQueueAuditPersistenceStates)[number];

export const decodeProductionSupportStatusBackendQueueAuditPersistenceProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendQueueAuditPersistenceProofSchema
);

export function summarizeProductionSupportStatusBackendQueueAuditPersistenceRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendQueueAuditPersistenceRow>
): Record<
  ProductionSupportStatusBackendQueueAuditPersistenceTarget,
  Record<ProductionSupportStatusBackendQueueAuditPersistenceState, number>
> {
  return RequiredQueueAuditPersistenceTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredQueueAuditPersistenceStates.reduce(
        (targetSummary, readinessState) => ({
          ...targetSummary,
          [readinessState]: rows.filter((row) => row.target === target && row.readinessState === readinessState).length,
        }),
        {} as Record<ProductionSupportStatusBackendQueueAuditPersistenceState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendQueueAuditPersistenceTarget,
      Record<ProductionSupportStatusBackendQueueAuditPersistenceState, number>
    >
  );
}

function productionSupportStatusBackendQueueAuditPersistenceProofIsHonest(
  proof: QueueAuditPersistenceProofCandidate
): boolean {
  return (
    RequiredQueueAuditPersistenceTargets.every((target) =>
      RequiredQueueAuditPersistenceStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.readinessState === state)
      )
    ) &&
    RequiredQueueAuditPersistenceNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    queueAuditPersistenceClaimsRemainManual(proof)
  );
}

function queueAuditPersistenceClaimsRemainManual(proof: QueueAuditPersistenceProofCandidate): boolean {
  return (
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.durableQueueStorageClaim === 'manual-required' &&
    proof.retryWorkerExecutionClaim === 'manual-required' &&
    proof.auditPersistenceClaim === 'manual-required' &&
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
