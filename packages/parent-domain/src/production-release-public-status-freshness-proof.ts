import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionReleasePublicStatusFreshnessNonClaimSchema,
  ProductionReleasePublicStatusFreshnessSchemaVersionSchema,
  ProductionReleasePublicStatusFreshnessSignalSchema,
  ProductionReleasePublicStatusFreshnessStateSchema,
  ProductionReleasePublicStatusFreshnessSurfaceSchema,
  PublicStatusFreshnessReferenceSchema,
  PublicStatusFreshnessRequirementSchema,
  RequiredPublicStatusFreshnessNonClaims,
  RequiredPublicStatusFreshnessSurfaces,
} from './production-release-public-status-freshness-values';

export * from './production-release-public-status-freshness-values';

type PublicStatusFreshnessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeExecutionClaim: string;
  readonly accountBackendRuntimeClaim: string;
  readonly billingProviderRuntimeClaim: string;
  readonly productionPublishingState: string;
  readonly signingStoreProofState: string;
  readonly updaterExecutionState: string;
  readonly supportBackendUploadState: string;
  readonly productionSlaClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionReleasePublicStatusFreshnessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicStatusFreshnessSchemaVersionSchema,
    surface: ProductionReleasePublicStatusFreshnessSurfaceSchema,
    freshnessSignal: ProductionReleasePublicStatusFreshnessSignalSchema,
    sourceContractState: ProductionReleasePublicStatusFreshnessStateSchema,
    freshnessPolicyState: ProductionReleasePublicStatusFreshnessStateSchema,
    publicRuntimeState: ProductionReleasePublicStatusFreshnessStateSchema,
    backendState: ProductionReleasePublicStatusFreshnessStateSchema,
    manualRequirement: PublicStatusFreshnessRequirementSchema,
    evidenceReference: PublicStatusFreshnessReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.publicRuntimeState !== 'source-contract-ready' ||
        'Expected public status freshness rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.backendState !== 'source-contract-ready' ||
        'Expected public status freshness rows to avoid backend runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.freshnessPolicyState === 'freshness-policy-ready' ||
        'Expected every public status surface to carry a freshness policy boundary'
    )
  )
);

export const ProductionReleasePublicStatusFreshnessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicStatusFreshnessSchemaVersionSchema,
    rows: Schema.Array(ProductionReleasePublicStatusFreshnessRowSchema),
    nonClaims: Schema.Array(ProductionReleasePublicStatusFreshnessNonClaimSchema),
    publicRuntimeExecutionClaim: ProductionReleasePublicStatusFreshnessStateSchema,
    accountBackendRuntimeClaim: ProductionReleasePublicStatusFreshnessStateSchema,
    billingProviderRuntimeClaim: ProductionReleasePublicStatusFreshnessStateSchema,
    productionPublishingState: ProductionReleasePublicStatusFreshnessStateSchema,
    signingStoreProofState: ProductionReleasePublicStatusFreshnessStateSchema,
    updaterExecutionState: ProductionReleasePublicStatusFreshnessStateSchema,
    supportBackendUploadState: ProductionReleasePublicStatusFreshnessStateSchema,
    productionSlaClaim: ProductionReleasePublicStatusFreshnessStateSchema,
    childActivityCustodyClaim: ProductionReleasePublicStatusFreshnessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionReleasePublicStatusFreshnessProofIsHonest(proof) ||
        'Expected public status freshness proof to cover all surfaces while preserving public runtime non-claims'
    )
  )
);

export type ProductionReleasePublicStatusFreshnessRow = Infer<typeof ProductionReleasePublicStatusFreshnessRowSchema>;
export type ProductionReleasePublicStatusFreshnessProof = Infer<
  typeof ProductionReleasePublicStatusFreshnessProofSchema
>;
export type ProductionReleasePublicStatusFreshnessSurface = Infer<
  typeof ProductionReleasePublicStatusFreshnessSurfaceSchema
>;

export const ProductionReleasePublicStatusFreshnessReadModel = ProductionReleasePublicStatusFreshnessProofSchema.parse({
  schemaVersion: 'production-release-public-status-freshness-proof',
  rows: [
    freshnessRow('public-download', 'download-manifest', 'backend-required'),
    freshnessRow('release-status', 'release-channel', 'publication-required'),
    freshnessRow('update-status', 'update-channel', 'manual-required'),
    freshnessRow('account-status', 'account-snapshot', 'backend-required'),
    freshnessRow('subscription-status', 'subscription-snapshot', 'backend-required'),
    freshnessRow('support-status', 'support-incident-status', 'manual-required'),
  ],
  nonClaims: RequiredPublicStatusFreshnessNonClaims,
  publicRuntimeExecutionClaim: 'not-implemented',
  accountBackendRuntimeClaim: 'backend-required',
  billingProviderRuntimeClaim: 'not-implemented',
  productionPublishingState: 'publication-required',
  signingStoreProofState: 'manual-required',
  updaterExecutionState: 'manual-required',
  supportBackendUploadState: 'manual-required',
  productionSlaClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T21:36:00.000Z'),
});

export const ProductionReleasePublicStatusFreshnessKnownGaps = [
  'Public status freshness rows are source-contract proof only; family.ocentra.ca runtime remains unimplemented.',
  'Account backend, billing provider runtime, production publishing, signing/store proof, updater execution, support backend upload, and production SLA remain manual-required or not implemented.',
  'No child activity custody, provider secrets, support backend payloads, remote support transcripts, or account lookup results are included.',
] as const;

export const decodeProductionReleasePublicStatusFreshnessProof = Schema.decodeUnknownSync(
  ProductionReleasePublicStatusFreshnessProofSchema
);

export function summarizeProductionReleasePublicStatusFreshnessRows(
  rows: ReadonlyArray<ProductionReleasePublicStatusFreshnessRow>
): Record<ProductionReleasePublicStatusFreshnessSurface, number> {
  return RequiredPublicStatusFreshnessSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionReleasePublicStatusFreshnessSurface, number>
  );
}

function freshnessRow(
  surface: ProductionReleasePublicStatusFreshnessSurface,
  freshnessSignal: ProductionReleasePublicStatusFreshnessRow['freshnessSignal'],
  backendState: ProductionReleasePublicStatusFreshnessRow['backendState']
) {
  return {
    schemaVersion: 'production-release-public-status-freshness-proof',
    surface,
    freshnessSignal,
    sourceContractState: 'source-contract-ready',
    freshnessPolicyState: 'freshness-policy-ready',
    publicRuntimeState: 'not-implemented',
    backendState,
    manualRequirement: `${surface}-requires-public-runtime-and-freshness-smoke-before-product-claim`,
    evidenceReference: `production-release-public-status-freshness-${surface}`,
  } as const;
}

function productionReleasePublicStatusFreshnessProofIsHonest(proof: PublicStatusFreshnessProofCandidate): boolean {
  return (
    RequiredPublicStatusFreshnessSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredPublicStatusFreshnessNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.accountBackendRuntimeClaim === 'backend-required' &&
    proof.billingProviderRuntimeClaim === 'not-implemented' &&
    proof.productionPublishingState === 'publication-required' &&
    proof.signingStoreProofState === 'manual-required' &&
    proof.updaterExecutionState === 'manual-required' &&
    proof.supportBackendUploadState === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
