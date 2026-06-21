import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ForbiddenPublicationRuntimeReadinessDataClasses,
  ProductionSupportPublicationRuntimeReadinessDataClassSchema,
  ProductionSupportPublicationRuntimeReadinessItemSchema,
  ProductionSupportPublicationRuntimeReadinessManualRequirementSchema,
  ProductionSupportPublicationRuntimeReadinessNonClaimSchema,
  ProductionSupportPublicationRuntimeReadinessRuntimeRefSchema,
  ProductionSupportPublicationRuntimeReadinessSchemaVersionSchema,
  ProductionSupportPublicationRuntimeReadinessSourceProofSchema,
  ProductionSupportPublicationRuntimeReadinessStateSchema,
  RequiredPublicationRuntimeReadinessItems,
  RequiredPublicationRuntimeReadinessNonClaims,
} from './production-support-publication-runtime-readiness-values';

type PublicationRuntimeReadinessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly item: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeExecutionClaim: string;
  readonly publicationRunnerExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportPublicationRuntimeReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationRuntimeReadinessSchemaVersionSchema,
    item: ProductionSupportPublicationRuntimeReadinessItemSchema,
    sourceProof: ProductionSupportPublicationRuntimeReadinessSourceProofSchema,
    sourceContractState: ProductionSupportPublicationRuntimeReadinessStateSchema,
    runtimeAdapterState: ProductionSupportPublicationRuntimeReadinessStateSchema,
    publicationRunnerState: ProductionSupportPublicationRuntimeReadinessStateSchema,
    supportBackendUploadState: ProductionSupportPublicationRuntimeReadinessStateSchema,
    publicRuntimeState: ProductionSupportPublicationRuntimeReadinessStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportPublicationRuntimeReadinessDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportPublicationRuntimeReadinessDataClassSchema),
    runtimeRef: ProductionSupportPublicationRuntimeReadinessRuntimeRefSchema,
    manualRequirement: ProductionSupportPublicationRuntimeReadinessManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        (row.publicRuntimeState !== 'implemented' && row.publicRuntimeState !== 'executed') ||
        'Expected publication runtime readiness rows to avoid real public runtime claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicationRunnerState !== 'implemented' && row.publicationRunnerState !== 'executed') ||
        'Expected publication runtime readiness rows to keep publication runner execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected publication runtime readiness rows to keep support backend upload execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicationRuntimeReadinessDataClasses.includes(dataClass as never)
        ) || 'Expected publication runtime readiness rows to exclude sensitive support/runtime data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicationRuntimeReadinessDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected publication runtime readiness rows to enumerate forbidden runtime data classes'
    )
  )
);

export const ProductionSupportPublicationRuntimeReadinessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationRuntimeReadinessSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportPublicationRuntimeReadinessRowSchema),
    nonClaims: Schema.Array(ProductionSupportPublicationRuntimeReadinessNonClaimSchema),
    publicRuntimeExecutionClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    publicationRunnerExecutionClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    accountLookupExecutionClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    billingProviderContactClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    productionSlaClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    childActivityCustodyClaim: ProductionSupportPublicationRuntimeReadinessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportPublicationRuntimeReadinessProofIsHonest(proof) ||
        'Expected publication runtime readiness proof to cover rows and preserve explicit non-claims'
    )
  )
);

export type ProductionSupportPublicationRuntimeReadinessRow = Infer<
  typeof ProductionSupportPublicationRuntimeReadinessRowSchema
>;
export type ProductionSupportPublicationRuntimeReadinessProof = Infer<
  typeof ProductionSupportPublicationRuntimeReadinessProofSchema
>;
export type ProductionSupportPublicationRuntimeReadinessItem = Infer<
  typeof ProductionSupportPublicationRuntimeReadinessItemSchema
>;

export const decodeProductionSupportPublicationRuntimeReadinessProof = Schema.decodeUnknownSync(
  ProductionSupportPublicationRuntimeReadinessProofSchema
);

export function summarizeProductionSupportPublicationRuntimeReadinessRows(
  rows: ReadonlyArray<ProductionSupportPublicationRuntimeReadinessRow>
): Record<ProductionSupportPublicationRuntimeReadinessItem, number> {
  return RequiredPublicationRuntimeReadinessItems.reduce(
    (summary, item) => ({
      ...summary,
      [item]: rows.filter((row) => row.item === item).length,
    }),
    {} as Record<ProductionSupportPublicationRuntimeReadinessItem, number>
  );
}

function productionSupportPublicationRuntimeReadinessProofIsHonest(
  proof: PublicationRuntimeReadinessProofCandidate
): boolean {
  return (
    RequiredPublicationRuntimeReadinessItems.every((item) => proof.rows.some((row) => row.item === item)) &&
    RequiredPublicationRuntimeReadinessNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.publicationRunnerExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
