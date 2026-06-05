import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenPublicDocsStatusDataClasses,
  ProductionReleasePublicDocsStatusAudienceSchema,
  ProductionReleasePublicDocsStatusDataClassSchema,
  ProductionReleasePublicDocsStatusDocumentSchema,
  ProductionReleasePublicDocsStatusExecutionClaimSchema,
  ProductionReleasePublicDocsStatusNonClaimSchema,
  ProductionReleasePublicDocsStatusSchemaVersionSchema,
  ProductionReleasePublicDocsStatusSourceSchema,
  ProductionReleasePublicDocsStatusStateSchema,
  PublicDocsStatusReferenceSchema,
  PublicDocsStatusRequirementSchema,
  RequiredPublicDocsStatusDocuments,
  RequiredPublicDocsStatusNonClaims,
} from './production-release-public-docs-status-values';

export * from './production-release-public-docs-status-values';

type PublicDocsStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly document: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicWebsitePublicationClaim: string;
  readonly supportBackendUploadClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionReleasePublicDocsStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicDocsStatusSchemaVersionSchema,
    document: ProductionReleasePublicDocsStatusDocumentSchema,
    sourceDocumentState: ProductionReleasePublicDocsStatusStateSchema,
    publicPublicationState: ProductionReleasePublicDocsStatusStateSchema,
    publicRouteState: ProductionReleasePublicDocsStatusStateSchema,
    sourceProof: ProductionReleasePublicDocsStatusSourceSchema,
    disclosureAudience: ProductionReleasePublicDocsStatusAudienceSchema,
    supportSafeDataClasses: Schema.Array(ProductionReleasePublicDocsStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionReleasePublicDocsStatusDataClassSchema),
    publicationReference: PublicDocsStatusReferenceSchema,
    proofRequirement: PublicDocsStatusRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        (row.sourceDocumentState !== 'published' && row.sourceDocumentState !== 'implemented') ||
        'Expected public docs source rows to avoid live publication claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicPublicationState !== 'published' && row.publicPublicationState !== 'implemented') ||
        'Expected public docs status rows to require manual public publication proof'
    ),
    Schema.filter(
      (row) =>
        (row.publicRouteState !== 'implemented' && row.publicRouteState !== 'published') ||
        'Expected public docs status rows to avoid public website route implementation claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicDocsStatusDataClasses.includes(dataClass as never)
        ) || 'Expected public docs status rows to exclude child activity, support bundle, and provider contact data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicDocsStatusDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected public docs status rows to enumerate forbidden custody and support-operation data classes'
    )
  )
);

export const ProductionReleasePublicDocsStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicDocsStatusSchemaVersionSchema,
    rows: Schema.Array(ProductionReleasePublicDocsStatusRowSchema),
    nonClaims: Schema.Array(ProductionReleasePublicDocsStatusNonClaimSchema),
    publicWebsitePublicationClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    supportBackendUploadClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    accountLookupExecutionClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    billingProviderContactClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    remoteSupportSessionClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    productionSlaClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    childActivityCustodyClaim: ProductionReleasePublicDocsStatusExecutionClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionReleasePublicDocsStatusProofIsHonest(proof) ||
        'Expected public docs status proof to cover all required docs while preserving explicit non-claims'
    )
  )
);

export type ProductionReleasePublicDocsStatusRow = Infer<typeof ProductionReleasePublicDocsStatusRowSchema>;
export type ProductionReleasePublicDocsStatusProof = Infer<typeof ProductionReleasePublicDocsStatusProofSchema>;
export type ProductionReleasePublicDocsStatusDocument = Infer<typeof ProductionReleasePublicDocsStatusDocumentSchema>;

export const decodeProductionReleasePublicDocsStatusProof = Schema.decodeUnknownSync(
  ProductionReleasePublicDocsStatusProofSchema
);

export function summarizeProductionReleasePublicDocsStatusRows(
  rows: ReadonlyArray<ProductionReleasePublicDocsStatusRow>
): Record<ProductionReleasePublicDocsStatusDocument, number> {
  return RequiredPublicDocsStatusDocuments.reduce(
    (summary, documentName) => ({
      ...summary,
      [documentName]: rows.filter((row) => row.document === documentName).length,
    }),
    {} as Record<ProductionReleasePublicDocsStatusDocument, number>
  );
}

function productionReleasePublicDocsStatusProofIsHonest(proof: PublicDocsStatusProofCandidate): boolean {
  return (
    RequiredPublicDocsStatusDocuments.every((documentName) =>
      proof.rows.some((row) => row.document === documentName)
    ) &&
    RequiredPublicDocsStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicWebsitePublicationClaim === 'manual-required' &&
    proof.supportBackendUploadClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
