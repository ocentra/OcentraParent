import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenStatusBackendExecutionQueueDataClasses,
  ProductionSupportStatusBackendExecutionQueueDataClassSchema,
  ProductionSupportStatusBackendExecutionQueueNonClaimSchema,
  ProductionSupportStatusBackendExecutionQueueSchemaVersionSchema,
  ProductionSupportStatusBackendExecutionQueueSourceProofSchema,
  ProductionSupportStatusBackendExecutionQueueStateSchema,
  ProductionSupportStatusBackendExecutionQueueTargetSchema,
  RequiredStatusBackendExecutionQueueNonClaims,
  RequiredStatusBackendExecutionQueueStates,
  RequiredStatusBackendExecutionQueueTargets,
  StatusBackendExecutionQueueManualRequirementSchema,
  StatusBackendExecutionQueueReferenceSchema,
} from './production-support-status-backend-execution-queue-values';

export * from './production-support-status-backend-execution-queue-values';

type StatusBackendExecutionQueueProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly queueState: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly statusBackendExecutionClaim: string;
  readonly publicRuntimeExecutionClaim: string;
  readonly providerExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportStatusBackendExecutionQueueRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendExecutionQueueSchemaVersionSchema,
    target: ProductionSupportStatusBackendExecutionQueueTargetSchema,
    sourceProof: ProductionSupportStatusBackendExecutionQueueSourceProofSchema,
    queueState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    sourceContractState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    statusContractState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    authorizationState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    queueAdapterState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    backendExecutionState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    retryState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    auditState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    publicRuntimeExecutionState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    providerExecutionState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendExecutionQueueStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendExecutionQueueDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendExecutionQueueDataClassSchema),
    queueReference: StatusBackendExecutionQueueReferenceSchema,
    retryReference: StatusBackendExecutionQueueReferenceSchema,
    auditReference: StatusBackendExecutionQueueReferenceSchema,
    manualRequirement: StatusBackendExecutionQueueManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredStatusBackendExecutionQueueStates.includes(row.queueState as never) ||
        'Expected status backend execution queue rows to use required queue states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected status backend execution queue rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        (row.backendExecutionState !== 'implemented' && row.backendExecutionState !== 'executed') ||
        'Expected status backend execution queue rows to avoid real backend execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeExecutionState !== 'implemented' && row.publicRuntimeExecutionState !== 'executed') ||
        'Expected status backend execution queue rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.providerExecutionState !== 'implemented' && row.providerExecutionState !== 'executed') ||
        'Expected status backend execution queue rows to avoid provider execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected status backend execution queue rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenStatusBackendExecutionQueueDataClasses.includes(dataClass as never)
        ) || 'Expected status backend execution queue rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenStatusBackendExecutionQueueDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected status backend execution queue rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendExecutionQueueProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendExecutionQueueSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportStatusBackendExecutionQueueRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendExecutionQueueNonClaimSchema),
    statusBackendExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    providerExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendExecutionQueueStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendExecutionQueueProofIsHonest(proof) ||
        'Expected status backend execution queue proof to cover rows and preserve non-claims'
    )
  )
);

export type ProductionSupportStatusBackendExecutionQueueRow = Infer<
  typeof ProductionSupportStatusBackendExecutionQueueRowSchema
>;
export type ProductionSupportStatusBackendExecutionQueueProof = Infer<
  typeof ProductionSupportStatusBackendExecutionQueueProofSchema
>;
export type ProductionSupportStatusBackendExecutionQueueTarget = Infer<
  typeof ProductionSupportStatusBackendExecutionQueueTargetSchema
>;
export type ProductionSupportStatusBackendExecutionQueueState =
  (typeof RequiredStatusBackendExecutionQueueStates)[number];

export const decodeProductionSupportStatusBackendExecutionQueueProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendExecutionQueueProofSchema
);

export function summarizeProductionSupportStatusBackendExecutionQueueRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendExecutionQueueRow>
): Record<
  ProductionSupportStatusBackendExecutionQueueTarget,
  Record<ProductionSupportStatusBackendExecutionQueueState, number>
> {
  return RequiredStatusBackendExecutionQueueTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredStatusBackendExecutionQueueStates.reduce(
        (targetSummary, queueState) => ({
          ...targetSummary,
          [queueState]: rows.filter((row) => row.target === target && row.queueState === queueState).length,
        }),
        {} as Record<ProductionSupportStatusBackendExecutionQueueState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendExecutionQueueTarget,
      Record<ProductionSupportStatusBackendExecutionQueueState, number>
    >
  );
}

function productionSupportStatusBackendExecutionQueueProofIsHonest(
  proof: StatusBackendExecutionQueueProofCandidate
): boolean {
  return (
    RequiredStatusBackendExecutionQueueTargets.every((target) =>
      RequiredStatusBackendExecutionQueueStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.queueState === state)
      )
    ) &&
    RequiredStatusBackendExecutionQueueNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.statusBackendExecutionClaim === 'manual-required' &&
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
