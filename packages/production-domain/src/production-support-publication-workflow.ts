import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenPublicationWorkflowDataClasses,
  ProductionSupportPublicationWorkflowDataClassSchema,
  ProductionSupportPublicationWorkflowItemSchema,
  ProductionSupportPublicationWorkflowNonClaimSchema,
  ProductionSupportPublicationWorkflowSchemaVersionSchema,
  ProductionSupportPublicationWorkflowSourceProofSchema,
  ProductionSupportPublicationWorkflowStateSchema,
  PublicationWorkflowReferenceSchema,
  PublicationWorkflowRequirementSchema,
  RequiredPublicationWorkflowItems,
  RequiredPublicationWorkflowNonClaims,
} from './production-support-publication-workflow-values';

export * from './production-support-publication-workflow-values';

type PublicationWorkflowProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly item: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeClaim: string;
  readonly legalExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportPublicationWorkflowRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationWorkflowSchemaVersionSchema,
    item: ProductionSupportPublicationWorkflowItemSchema,
    sourceProof: ProductionSupportPublicationWorkflowSourceProofSchema,
    sourceContractState: ProductionSupportPublicationWorkflowStateSchema,
    publicPublicationState: ProductionSupportPublicationWorkflowStateSchema,
    legalExecutionState: ProductionSupportPublicationWorkflowStateSchema,
    supportBackendUploadState: ProductionSupportPublicationWorkflowStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportPublicationWorkflowDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportPublicationWorkflowDataClassSchema),
    publicationReference: PublicationWorkflowReferenceSchema,
    manualRequirement: PublicationWorkflowRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        (row.publicPublicationState !== 'implemented' && row.publicPublicationState !== 'executed') ||
        'Expected publication workflow rows to avoid real public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.legalExecutionState !== 'executed' ||
        'Expected publication workflow rows to keep legal disclosure execution manual-required'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected publication workflow rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicationWorkflowDataClasses.includes(dataClass as never)
        ) ||
        'Expected publication workflow rows to exclude child activity, provider, account, billing, and raw support data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicationWorkflowDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected publication workflow rows to enumerate forbidden publication workflow data classes'
    )
  )
);

export const ProductionSupportPublicationWorkflowProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationWorkflowSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportPublicationWorkflowRowSchema),
    nonClaims: Schema.Array(ProductionSupportPublicationWorkflowNonClaimSchema),
    publicRuntimeClaim: ProductionSupportPublicationWorkflowStateSchema,
    legalExecutionClaim: ProductionSupportPublicationWorkflowStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportPublicationWorkflowStateSchema,
    accountLookupExecutionClaim: ProductionSupportPublicationWorkflowStateSchema,
    billingProviderContactClaim: ProductionSupportPublicationWorkflowStateSchema,
    productionSlaClaim: ProductionSupportPublicationWorkflowStateSchema,
    childActivityCustodyClaim: ProductionSupportPublicationWorkflowStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportPublicationWorkflowProofIsHonest(proof) ||
        'Expected publication workflow proof to cover required rows while preserving explicit non-claims'
    )
  )
);

export type ProductionSupportPublicationWorkflowRow = Infer<typeof ProductionSupportPublicationWorkflowRowSchema>;
export type ProductionSupportPublicationWorkflowProof = Infer<typeof ProductionSupportPublicationWorkflowProofSchema>;
export type ProductionSupportPublicationWorkflowItem = Infer<typeof ProductionSupportPublicationWorkflowItemSchema>;

export const decodeProductionSupportPublicationWorkflowProof = Schema.decodeUnknownSync(
  ProductionSupportPublicationWorkflowProofSchema
);

export function summarizeProductionSupportPublicationWorkflowRows(
  rows: ReadonlyArray<ProductionSupportPublicationWorkflowRow>
): Record<ProductionSupportPublicationWorkflowItem, number> {
  return RequiredPublicationWorkflowItems.reduce(
    (summary, item) => ({
      ...summary,
      [item]: rows.filter((row) => row.item === item).length,
    }),
    {} as Record<ProductionSupportPublicationWorkflowItem, number>
  );
}

function productionSupportPublicationWorkflowProofIsHonest(proof: PublicationWorkflowProofCandidate): boolean {
  return (
    RequiredPublicationWorkflowItems.every((item) => proof.rows.some((row) => row.item === item)) &&
    RequiredPublicationWorkflowNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeClaim === 'not-implemented' &&
    proof.legalExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
