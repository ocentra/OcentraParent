import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenPublicationExecutionStatusDataClasses,
  ProductionSupportPublicationExecutionStatusDataClassSchema,
  ProductionSupportPublicationExecutionStatusNonClaimSchema,
  ProductionSupportPublicationExecutionStatusSchemaVersionSchema,
  ProductionSupportPublicationExecutionStatusSourceProofSchema,
  ProductionSupportPublicationExecutionStatusStateSchema,
  ProductionSupportPublicationExecutionStatusTargetSchema,
  PublicationExecutionStatusManualRequirementSchema,
  PublicationExecutionStatusReferenceSchema,
  RequiredPublicationExecutionStatusLifecycleStates,
  RequiredPublicationExecutionStatusNonClaims,
  RequiredPublicationExecutionStatusTargets,
} from './production-support-publication-execution-status-values';

export * from './production-support-publication-execution-status-values';

type PublicationExecutionStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly lifecycleStatus: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeExecutionClaim: string;
  readonly publicationRunnerExecutionClaim: string;
  readonly statusBackendExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportPublicationExecutionStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationExecutionStatusSchemaVersionSchema,
    target: ProductionSupportPublicationExecutionStatusTargetSchema,
    sourceProof: ProductionSupportPublicationExecutionStatusSourceProofSchema,
    lifecycleStatus: ProductionSupportPublicationExecutionStatusStateSchema,
    sourceContractState: ProductionSupportPublicationExecutionStatusStateSchema,
    statusContractState: ProductionSupportPublicationExecutionStatusStateSchema,
    publicRuntimeState: ProductionSupportPublicationExecutionStatusStateSchema,
    publicationRunnerState: ProductionSupportPublicationExecutionStatusStateSchema,
    statusBackendState: ProductionSupportPublicationExecutionStatusStateSchema,
    supportBackendUploadState: ProductionSupportPublicationExecutionStatusStateSchema,
    legalExecutionState: ProductionSupportPublicationExecutionStatusStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportPublicationExecutionStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportPublicationExecutionStatusDataClassSchema),
    statusReference: PublicationExecutionStatusReferenceSchema,
    manualRequirement: PublicationExecutionStatusManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredPublicationExecutionStatusLifecycleStates.includes(row.lifecycleStatus as never) ||
        'Expected publication execution status rows to use the required lifecycle states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected publication execution status rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeState !== 'implemented' && row.publicRuntimeState !== 'executed') ||
        'Expected publication execution status rows to avoid real public runtime claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicationRunnerState !== 'implemented' && row.publicationRunnerState !== 'executed') ||
        'Expected publication execution status rows to keep publication runner execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        (row.statusBackendState !== 'implemented' && row.statusBackendState !== 'executed') ||
        'Expected publication execution status rows to keep status backend execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected publication execution status rows to keep support backend upload execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        (row.legalExecutionState !== 'implemented' && row.legalExecutionState !== 'executed') ||
        'Expected publication execution status rows to keep legal disclosure execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicationExecutionStatusDataClasses.includes(dataClass as never)
        ) || 'Expected publication execution status rows to exclude forbidden support data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicationExecutionStatusDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected publication execution status rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportPublicationExecutionStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationExecutionStatusSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportPublicationExecutionStatusRowSchema),
    nonClaims: Schema.Array(ProductionSupportPublicationExecutionStatusNonClaimSchema),
    publicRuntimeExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    publicationRunnerExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    statusBackendExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    accountLookupExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    billingProviderContactClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    productionSlaClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    childActivityCustodyClaim: ProductionSupportPublicationExecutionStatusStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportPublicationExecutionStatusProofIsHonest(proof) ||
        'Expected publication execution status proof to cover all targets/states and preserve non-claims'
    )
  )
);

export type ProductionSupportPublicationExecutionStatusRow = Infer<
  typeof ProductionSupportPublicationExecutionStatusRowSchema
>;
export type ProductionSupportPublicationExecutionStatusProof = Infer<
  typeof ProductionSupportPublicationExecutionStatusProofSchema
>;
export type ProductionSupportPublicationExecutionStatusTarget = Infer<
  typeof ProductionSupportPublicationExecutionStatusTargetSchema
>;
export type ProductionSupportPublicationExecutionLifecycleStatus =
  (typeof RequiredPublicationExecutionStatusLifecycleStates)[number];

export const decodeProductionSupportPublicationExecutionStatusProof = Schema.decodeUnknownSync(
  ProductionSupportPublicationExecutionStatusProofSchema
);

export function summarizeProductionSupportPublicationExecutionStatusRows(
  rows: ReadonlyArray<ProductionSupportPublicationExecutionStatusRow>
): Record<
  ProductionSupportPublicationExecutionStatusTarget,
  Record<ProductionSupportPublicationExecutionLifecycleStatus, number>
> {
  return RequiredPublicationExecutionStatusTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredPublicationExecutionStatusLifecycleStates.reduce(
        (targetSummary, lifecycleStatus) => ({
          ...targetSummary,
          [lifecycleStatus]: rows.filter((row) => row.target === target && row.lifecycleStatus === lifecycleStatus)
            .length,
        }),
        {} as Record<ProductionSupportPublicationExecutionLifecycleStatus, number>
      ),
    }),
    {} as Record<
      ProductionSupportPublicationExecutionStatusTarget,
      Record<ProductionSupportPublicationExecutionLifecycleStatus, number>
    >
  );
}

function productionSupportPublicationExecutionStatusProofIsHonest(
  proof: PublicationExecutionStatusProofCandidate
): boolean {
  return (
    RequiredPublicationExecutionStatusTargets.every((target) =>
      RequiredPublicationExecutionStatusLifecycleStates.every((status) =>
        proof.rows.some((row) => row.target === target && row.lifecycleStatus === status)
      )
    ) &&
    RequiredPublicationExecutionStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.publicationRunnerExecutionClaim === 'manual-required' &&
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
