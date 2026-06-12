import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenPublicationStatusFreshnessDataClasses,
  ProductionSupportPublicationStatusFreshnessDataClassSchema,
  ProductionSupportPublicationStatusFreshnessNonClaimSchema,
  ProductionSupportPublicationStatusFreshnessSchemaVersionSchema,
  ProductionSupportPublicationStatusFreshnessSourceProofSchema,
  ProductionSupportPublicationStatusFreshnessStateSchema,
  ProductionSupportPublicationStatusFreshnessSurfaceSchema,
  PublicationStatusFreshnessReferenceSchema,
  PublicationStatusFreshnessRequirementSchema,
  RequiredPublicationStatusFreshnessNonClaims,
  RequiredPublicationStatusFreshnessSurfaces,
} from './production-support-publication-status-freshness-values';

export * from './production-support-publication-status-freshness-values';

type PublicationStatusFreshnessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeClaim: string;
  readonly supportPublicationExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportPublicationStatusFreshnessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationStatusFreshnessSchemaVersionSchema,
    surface: ProductionSupportPublicationStatusFreshnessSurfaceSchema,
    sourceProof: ProductionSupportPublicationStatusFreshnessSourceProofSchema,
    sourceContractState: ProductionSupportPublicationStatusFreshnessStateSchema,
    freshnessPolicyState: ProductionSupportPublicationStatusFreshnessStateSchema,
    publicPublicationState: ProductionSupportPublicationStatusFreshnessStateSchema,
    publicRuntimeState: ProductionSupportPublicationStatusFreshnessStateSchema,
    supportBackendUploadState: ProductionSupportPublicationStatusFreshnessStateSchema,
    legalExecutionState: ProductionSupportPublicationStatusFreshnessStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportPublicationStatusFreshnessDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportPublicationStatusFreshnessDataClassSchema),
    freshnessReference: PublicationStatusFreshnessReferenceSchema,
    manualRequirement: PublicationStatusFreshnessRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.freshnessPolicyState === 'freshness-policy-ready' ||
        'Expected every support publication status row to carry freshness policy readiness'
    ),
    Schema.filter(
      (row) =>
        row.publicPublicationState !== 'implemented' ||
        'Expected support publication status rows to avoid public publication execution claims'
    ),
    Schema.filter(
      (row) =>
        row.publicRuntimeState !== 'implemented' ||
        'Expected support publication status rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportBackendUploadState !== 'executed' ||
        'Expected support publication status rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.legalExecutionState !== 'executed' ||
        'Expected support publication status rows to avoid legal disclosure execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicationStatusFreshnessDataClasses.includes(dataClass as never)
        ) ||
        'Expected support publication status rows to exclude sensitive support, provider, account, billing, and custody data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicationStatusFreshnessDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected support publication status rows to enumerate forbidden data classes'
    )
  )
);

export const ProductionSupportPublicationStatusFreshnessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportPublicationStatusFreshnessSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportPublicationStatusFreshnessRowSchema),
    nonClaims: Schema.Array(ProductionSupportPublicationStatusFreshnessNonClaimSchema),
    publicRuntimeClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    supportPublicationExecutionClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    accountLookupExecutionClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    billingProviderContactClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    productionSlaClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    childActivityCustodyClaim: ProductionSupportPublicationStatusFreshnessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportPublicationStatusFreshnessProofIsHonest(proof) ||
        'Expected support publication status freshness proof to cover all rows while preserving explicit non-claims'
    )
  )
);

export type ProductionSupportPublicationStatusFreshnessRow = Infer<
  typeof ProductionSupportPublicationStatusFreshnessRowSchema
>;
export type ProductionSupportPublicationStatusFreshnessProof = Infer<
  typeof ProductionSupportPublicationStatusFreshnessProofSchema
>;
export type ProductionSupportPublicationStatusFreshnessSurface = Infer<
  typeof ProductionSupportPublicationStatusFreshnessSurfaceSchema
>;

export const decodeProductionSupportPublicationStatusFreshnessProof = Schema.decodeUnknownSync(
  ProductionSupportPublicationStatusFreshnessProofSchema
);

export function summarizeProductionSupportPublicationStatusFreshnessRows(
  rows: ReadonlyArray<ProductionSupportPublicationStatusFreshnessRow>
): Record<ProductionSupportPublicationStatusFreshnessSurface, number> {
  return RequiredPublicationStatusFreshnessSurfaces.reduce(
    (summary, surface) => ({
      ...summary,
      [surface]: rows.filter((row) => row.surface === surface).length,
    }),
    {} as Record<ProductionSupportPublicationStatusFreshnessSurface, number>
  );
}

function productionSupportPublicationStatusFreshnessProofIsHonest(
  proof: PublicationStatusFreshnessProofCandidate
): boolean {
  return (
    RequiredPublicationStatusFreshnessSurfaces.every((surface) => proof.rows.some((row) => row.surface === surface)) &&
    RequiredPublicationStatusFreshnessNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeClaim === 'not-implemented' &&
    proof.supportPublicationExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
