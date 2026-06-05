import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionReleasePublicDocsFreshnessDocumentSchema,
  ProductionReleasePublicDocsFreshnessNonClaimSchema,
  ProductionReleasePublicDocsFreshnessSchemaVersionSchema,
  ProductionReleasePublicDocsFreshnessSignalSchema,
  ProductionReleasePublicDocsFreshnessStateSchema,
  PublicDocsFreshnessReferenceSchema,
  PublicDocsFreshnessRequirementSchema,
  RequiredPublicDocsFreshnessDocuments,
  RequiredPublicDocsFreshnessNonClaims,
} from './production-release-public-docs-freshness-values';

export * from './production-release-public-docs-freshness-values';

type PublicDocsFreshnessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly document: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicPublicationClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly supportBackendUploadClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionReleasePublicDocsFreshnessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicDocsFreshnessSchemaVersionSchema,
    document: ProductionReleasePublicDocsFreshnessDocumentSchema,
    freshnessSignal: ProductionReleasePublicDocsFreshnessSignalSchema,
    sourceDocumentState: ProductionReleasePublicDocsFreshnessStateSchema,
    freshnessPolicyState: ProductionReleasePublicDocsFreshnessStateSchema,
    publicPublicationState: ProductionReleasePublicDocsFreshnessStateSchema,
    publicRouteState: ProductionReleasePublicDocsFreshnessStateSchema,
    manualRequirement: PublicDocsFreshnessRequirementSchema,
    evidenceReference: PublicDocsFreshnessReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.publicPublicationState !== 'source-contract-ready' ||
        'Expected public docs freshness rows to avoid public publication execution claims'
    ),
    Schema.filter(
      (row) =>
        row.publicRouteState !== 'source-contract-ready' ||
        'Expected public docs freshness rows to avoid public route execution claims'
    ),
    Schema.filter(
      (row) =>
        row.freshnessPolicyState === 'freshness-policy-ready' ||
        'Expected every public docs row to carry a freshness policy boundary'
    )
  )
);

export const ProductionReleasePublicDocsFreshnessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicDocsFreshnessSchemaVersionSchema,
    rows: Schema.Array(ProductionReleasePublicDocsFreshnessRowSchema),
    nonClaims: Schema.Array(ProductionReleasePublicDocsFreshnessNonClaimSchema),
    publicPublicationClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    legalDisclosureExecutionClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    supportBackendUploadClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    accountLookupExecutionClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    billingProviderContactClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    remoteSupportSessionClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    productionSlaClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    childActivityCustodyClaim: ProductionReleasePublicDocsFreshnessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionReleasePublicDocsFreshnessProofIsHonest(proof) ||
        'Expected public docs freshness proof to cover all docs while preserving public publication non-claims'
    )
  )
);

export type ProductionReleasePublicDocsFreshnessRow = Infer<typeof ProductionReleasePublicDocsFreshnessRowSchema>;
export type ProductionReleasePublicDocsFreshnessProof = Infer<typeof ProductionReleasePublicDocsFreshnessProofSchema>;
export type ProductionReleasePublicDocsFreshnessDocument = Infer<
  typeof ProductionReleasePublicDocsFreshnessDocumentSchema
>;

export const ProductionReleasePublicDocsFreshnessReadModel = ProductionReleasePublicDocsFreshnessProofSchema.parse({
  schemaVersion: 'production-release-public-docs-freshness-proof',
  rows: [
    freshnessRow('privacy-policy', 'privacy-review-cadence'),
    freshnessRow('retention-policy', 'retention-review-cadence'),
    freshnessRow('export-delete-process', 'export-delete-review-cadence'),
    freshnessRow('support-runbook', 'support-runbook-review-cadence'),
    freshnessRow('incident-status-disclosure', 'incident-disclosure-review-cadence'),
    freshnessRow('legal-disclosure', 'legal-disclosure-review-cadence'),
  ],
  nonClaims: RequiredPublicDocsFreshnessNonClaims,
  publicPublicationClaim: 'manual-required',
  legalDisclosureExecutionClaim: 'manual-required',
  supportBackendUploadClaim: 'manual-required',
  accountLookupExecutionClaim: 'manual-required',
  billingProviderContactClaim: 'manual-required',
  remoteSupportSessionClaim: 'not-implemented',
  productionSlaClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T23:31:00.000Z'),
});

export const ProductionReleasePublicDocsFreshnessKnownGaps = [
  'Public docs freshness rows are source-contract proof only; family.ocentra.ca publication remains manual-required.',
  'Legal disclosure execution, support backend upload, account lookup, billing provider contact, remote support, and production SLA remain manual-required or not implemented.',
  'No child activity custody, provider secrets, support backend payloads, remote support transcripts, account lookup results, billing provider contact records, or raw support bundle payloads are included.',
] as const;

export const decodeProductionReleasePublicDocsFreshnessProof = Schema.decodeUnknownSync(
  ProductionReleasePublicDocsFreshnessProofSchema
);

export function summarizeProductionReleasePublicDocsFreshnessRows(
  rows: ReadonlyArray<ProductionReleasePublicDocsFreshnessRow>
): Record<ProductionReleasePublicDocsFreshnessDocument, number> {
  return RequiredPublicDocsFreshnessDocuments.reduce(
    (summary, documentName) => ({
      ...summary,
      [documentName]: rows.filter((row) => row.document === documentName).length,
    }),
    {} as Record<ProductionReleasePublicDocsFreshnessDocument, number>
  );
}

function freshnessRow(
  document: ProductionReleasePublicDocsFreshnessDocument,
  freshnessSignal: ProductionReleasePublicDocsFreshnessRow['freshnessSignal']
) {
  return {
    schemaVersion: 'production-release-public-docs-freshness-proof',
    document,
    freshnessSignal,
    sourceDocumentState: 'source-contract-ready',
    freshnessPolicyState: 'freshness-policy-ready',
    publicPublicationState: 'manual-required',
    publicRouteState: 'not-implemented',
    manualRequirement: `${document}-requires-public-publication-and-freshness-smoke-before-product-claim`,
    evidenceReference: `production-release-public-docs-freshness-${document}`,
  } as const;
}

function productionReleasePublicDocsFreshnessProofIsHonest(proof: PublicDocsFreshnessProofCandidate): boolean {
  return (
    RequiredPublicDocsFreshnessDocuments.every((documentName) =>
      proof.rows.some((row) => row.document === documentName)
    ) &&
    RequiredPublicDocsFreshnessNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicPublicationClaim === 'manual-required' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
