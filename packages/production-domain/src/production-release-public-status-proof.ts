import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenPublicDataClasses,
  ProductionReleasePublicDataClassSchema,
  ProductionReleasePublicNonClaimSchema,
  ProductionReleasePublicSourceSchema,
  ProductionReleasePublicStatusSchemaVersionSchema,
  ProductionReleasePublicSurfaceSchema,
  ProductionReleasePublicSurfaceStateSchema,
  PublicStatusLabelSchema,
  PublicStatusReferenceSchema,
  PublicStatusRequirementSchema,
  RequiredNonClaims,
  RequiredPublicSurfaces,
} from './production-release-public-status-proof-values';

export * from './production-release-public-status-proof-values';

type ProductionReleasePublicStatusProofCandidate = {
  readonly surfaces: ReadonlyArray<{
    readonly surface: string;
  }>;
  readonly manualProofGaps: ReadonlyArray<unknown>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly productionPublishingState: string;
  readonly childActivityCustodyClaim: string;
  readonly publicSupportRuntimeClaim: string;
};

export const ProductionReleasePublicSurfaceStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicStatusSchemaVersionSchema,
    surface: ProductionReleasePublicSurfaceSchema,
    surfaceState: ProductionReleasePublicSurfaceStateSchema,
    routeContractState: ProductionReleasePublicSurfaceStateSchema,
    backendRuntimeState: ProductionReleasePublicSurfaceStateSchema,
    parentVisibleState: ProductionReleasePublicSurfaceStateSchema,
    source: ProductionReleasePublicSourceSchema,
    allowedDataClasses: Schema.Array(ProductionReleasePublicDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionReleasePublicDataClassSchema),
    proofRequirement: PublicStatusRequirementSchema,
    statusReference: PublicStatusReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.backendRuntimeState !== 'implemented' ||
        'Expected public release/account/status rows to avoid backend runtime implementation claims'
    ),
    Schema.filter(
      (row) =>
        row.surfaceState !== 'implemented' ||
        row.routeContractState === 'route-contract-only' ||
        'Expected implemented public surfaces to stay route-contract-only until public runtime proof exists'
    ),
    Schema.filter(
      (row) =>
        row.allowedDataClasses.every((dataClass) => !ForbiddenPublicDataClasses.includes(dataClass as never)) ||
        'Expected public release/account/status rows to exclude child activity or sensitive support data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected public release/account/status rows to enumerate forbidden custody and sensitive data classes'
    )
  )
);

export const ProductionReleaseManualProofGapSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicStatusSchemaVersionSchema,
    gapId: PublicStatusLabelSchema,
    state: ProductionReleasePublicSurfaceStateSchema,
    proofRequirement: PublicStatusRequirementSchema,
    statusReference: PublicStatusReferenceSchema,
  }).pipe(
    Schema.filter(
      (gap) =>
        gap.state !== 'implemented' ||
        'Expected manual proof gaps to stay unimplemented until real signing store support or backend evidence exists'
    )
  )
);

export const ProductionReleasePublicStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicStatusSchemaVersionSchema,
    publicHostState: ProductionReleasePublicSurfaceStateSchema,
    surfaces: Schema.Array(ProductionReleasePublicSurfaceStatusSchema),
    manualProofGaps: Schema.Array(ProductionReleaseManualProofGapSchema),
    nonClaims: Schema.Array(ProductionReleasePublicNonClaimSchema),
    productionPublishingState: ProductionReleasePublicSurfaceStateSchema,
    childActivityCustodyClaim: ProductionReleasePublicSurfaceStateSchema,
    publicSupportRuntimeClaim: ProductionReleasePublicSurfaceStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionReleasePublicStatusProofIsHonest(proof) ||
        'Expected public release status proof to cover all public surfaces while keeping production and custody non-claims explicit'
    )
  )
);

export type ProductionReleasePublicSurfaceStatus = Infer<typeof ProductionReleasePublicSurfaceStatusSchema>;
export type ProductionReleaseManualProofGap = Infer<typeof ProductionReleaseManualProofGapSchema>;
export type ProductionReleasePublicStatusProof = Infer<typeof ProductionReleasePublicStatusProofSchema>;
export type ProductionReleasePublicSurface = Infer<typeof ProductionReleasePublicSurfaceSchema>;

export const decodeProductionReleasePublicStatusProof = Schema.decodeUnknownSync(
  ProductionReleasePublicStatusProofSchema
);

export const ProductionReleasePublicStatusProofReadModel = ProductionReleasePublicStatusProofSchema.parse({
  schemaVersion: 'production-release-public-status-proof',
  publicHostState: 'not-implemented',
  surfaces: [
    surface('public-download', 'route-contract-only', 'backend-required', [
      'release-version',
      'platform',
      'download-status',
    ]),
    surface('release-status', 'manual-required', 'production-promotion-required', ['release-version', 'commit']),
    surface('update-status', 'manual-required', 'manual-required', ['update-status', 'platform']),
    surface('account-status', 'route-contract-only', 'backend-required', ['account-status', 'entitlement-summary']),
    surface('subscription-status', 'route-contract-only', 'backend-required', ['subscription-status']),
    surface('support-status', 'manual-required', 'manual-required', ['support-runbook-status', 'incident-status']),
  ],
  manualProofGaps: [
    gap('windows-signing', 'manual-required'),
    gap('macos-notarization', 'manual-required'),
    gap('android-play-store', 'manual-required'),
    gap('ios-testflight-app-store', 'manual-required'),
    gap('production-publishing-promotion', 'production-promotion-required'),
    gap('support-backend-upload', 'manual-required'),
  ],
  nonClaims: RequiredNonClaims,
  productionPublishingState: 'production-promotion-required',
  childActivityCustodyClaim: 'not-implemented',
  publicSupportRuntimeClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T04:02:09.935Z'),
});

export const ProductionReleasePublicStatusKnownGaps = [
  'family.ocentra.ca public runtime is not implemented.',
  'Account, subscription, download, update, and release status surfaces are contract/manual-readiness only.',
  'Production publishing, signing, notarization, store upload, updater execution, and support backend upload remain manual-required or promotion-required.',
  'No child activity data, support bundle payloads, provider secrets, or parent rules are hosted by this proof.',
] as const;

export function summarizeProductionReleasePublicStatusSurfaces(
  rows: ReadonlyArray<ProductionReleasePublicSurfaceStatus>
): Record<ProductionReleasePublicSurface, number> {
  return RequiredPublicSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionReleasePublicSurface, number>
  );
}

function productionReleasePublicStatusProofIsHonest(proof: ProductionReleasePublicStatusProofCandidate): boolean {
  return (
    RequiredPublicSurfaces.every((surfaceName) => proof.surfaces.some((row) => row.surface === surfaceName)) &&
    RequiredNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.productionPublishingState === 'production-promotion-required' &&
    proof.childActivityCustodyClaim === 'not-implemented' &&
    proof.publicSupportRuntimeClaim === 'not-implemented' &&
    proof.manualProofGaps.length >= 6
  );
}

function surface(
  surfaceName: ProductionReleasePublicSurface,
  routeContractState: ProductionReleasePublicSurfaceStatus['routeContractState'],
  backendRuntimeState: ProductionReleasePublicSurfaceStatus['backendRuntimeState'],
  allowedDataClasses: ProductionReleasePublicSurfaceStatus['allowedDataClasses']
) {
  return {
    schemaVersion: 'production-release-public-status-proof',
    surface: surfaceName,
    surfaceState: routeContractState,
    routeContractState,
    backendRuntimeState,
    parentVisibleState: backendRuntimeState,
    source:
      surfaceName.includes('account') || surfaceName.includes('subscription')
        ? 'billing-account-endpoint-contract'
        : 'release-support-proof',
    allowedDataClasses,
    forbiddenDataClasses: ForbiddenPublicDataClasses,
    proofRequirement: `${surfaceName}-requires-public-runtime-backend-and-manual-platform-proof`,
    statusReference: `public-status-${surfaceName}`,
  } as const;
}

function gap(gapId: string, state: ProductionReleaseManualProofGap['state']) {
  return {
    schemaVersion: 'production-release-public-status-proof',
    gapId,
    state,
    proofRequirement: `${gapId}-real-evidence-required`,
    statusReference: `manual-gap-${gapId}`,
  } as const;
}
