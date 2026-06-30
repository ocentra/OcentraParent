import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel } from './app-install-purchase-product-claim-platform-limitation-fallback-proof';
import { AppInstallPurchaseProductClaimProviderStoreProofReadModel } from './app-install-purchase-product-claim-provider-store-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStoreApiExecutionRowGenerated,
  providerStoreApiExecutionProofIsHonestGenerated,
  providerStoreApiExecutionRowIsHonestGenerated,
  summarizeAppInstallPurchaseProviderStoreApiExecutionProofGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';

const ProofVersion = 'app-install-purchase-provider-store-api-execution-proof';
const SourceProviderStoreProofVersion = 'app-install-purchase-product-claim-provider-store-proof';
const SourcePlatformLimitationFallbackProofVersion =
  'app-install-purchase-product-claim-platform-limitation-fallback-proof';
const UpdatedAt = '2026-06-06T23:35:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const ProviderStoreApiExecutionStates = [
  'execution-ready',
  'manual-required',
  'unavailable',
  'blocked-before-claim',
] as const;
const NonClaims = [
  'no-product-claim-approval',
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'provider store API execution proof boundary only; links provider store product-claim proof rows and platform limitation fallback rows into execution-ready manual-required unavailable and blocked-before-claim evidence states while keeping product claims unapproved no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'provider store API execution proof boundary only',
  'provider store product-claim proof rows',
  'platform limitation fallback rows',
  'execution-ready',
  'manual-required',
  'unavailable',
  'blocked-before-claim',
  'product claims unapproved',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreApiExecutionProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const ProviderStoreApiExecutionStateSchema = withParser(Schema.Literal(...ProviderStoreApiExecutionStates));
const ProviderStoreProductClaimStateSchema = withParser(
  Schema.Literal(
    'provider-store-proof-required',
    'manual-provider-store-proof-required',
    'unsupported-store-proof-blocked'
  )
);
const ProviderStorePreflightStateSchema = withParser(
  Schema.Literal('preflight-ready', 'manual-provider-proof-required', 'provider-unavailable')
);
const PlatformLimitationFallbackStateSchema = withParser(
  Schema.Literal(
    'fallback-parent-workflow-ready',
    'manual-platform-limitation-fallback-required',
    'unsupported-platform-limitation-fallback-blocked'
  )
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreApiExecutionRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreApiExecutionBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreApiExecutionProofSchemaVersionSchema,
  providerStoreApiExecutionRowId: RefSchema,
  sourceProviderStoreProofVersion: Schema.Literal(SourceProviderStoreProofVersion),
  sourceProviderStoreRowId: RefSchema,
  sourceProviderStoreProductClaimState: ProviderStoreProductClaimStateSchema,
  sourceProviderStorePreflightState: ProviderStorePreflightStateSchema,
  sourcePlatformLimitationFallbackProofVersion: Schema.Literal(SourcePlatformLimitationFallbackProofVersion),
  sourcePlatformLimitationFallbackRowId: RefSchema,
  sourcePlatformLimitationFallbackState: PlatformLimitationFallbackStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  providerStoreApiExecutionState: ProviderStoreApiExecutionStateSchema,
  providerApiExecutionEvidenceRefs: Schema.Array(RefSchema),
  providerCredentialRequirementRefs: Schema.Array(RefSchema),
  fallbackParentWorkflowRefs: Schema.Array(RefSchema),
  manualPlatformEvidenceRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  productClaimApprovalClaim: NotClaimedSchema,
  googlePlayExecutionClaim: NotExecutedSchema,
  appleAppStoreExecutionClaim: NotExecutedSchema,
  microsoftStoreExecutionClaim: NotExecutedSchema,
  billingProviderContactClaim: NotExecutedSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformInterceptionClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  portalApprovalUiClaim: NotClaimedSchema,
  portalReportUiClaim: NotClaimedSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreApiExecutionRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseProviderStoreApiExecutionRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreApiExecutionRowIsHonest(row) ||
        'Expected provider/store API execution proof rows to keep source refs attached and preserve provider, store, platform, delivery, portal, blocking, and custody non-claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreApiExecutionProofSchemaVersionSchema,
  sourceProviderStoreProofVersion: Schema.Literal(SourceProviderStoreProofVersion),
  sourcePlatformLimitationFallbackProofVersion: Schema.Literal(SourcePlatformLimitationFallbackProofVersion),
  providerStoreApiExecutionRows: Schema.Array(AppInstallPurchaseProviderStoreApiExecutionRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreApiExecutionProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseProviderStoreApiExecutionProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreApiExecutionProofIsHonest(proof) ||
        'Expected provider/store API execution proof to cover every store surface and preserve all execution non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreApiExecutionKnownGaps = [
  'Provider/store API execution rows are proof-boundary rows only; no Google Play Apple App Store Microsoft Store or billing provider API execution is implemented.',
  'Windows is execution-ready only because prior provider/store and platform limitation proof rows are attached; real credentials and provider API execution evidence remain absent.',
  'macOS remains manual-required, Linux is unavailable, and Android/iOS stay blocked-before-claim until provider/store APIs, platform adapters, child delivery, and portal approval/report UI proof exist.',
] as const;

export const AppInstallPurchaseProviderStoreApiExecutionProofReadModel =
  AppInstallPurchaseProviderStoreApiExecutionProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProviderStoreProofVersion: SourceProviderStoreProofVersion,
    sourcePlatformLimitationFallbackProofVersion: SourcePlatformLimitationFallbackProofVersion,
    providerStoreApiExecutionRows:
      AppInstallPurchaseProductClaimProviderStoreProofReadModel.providerStoreProductClaimRows.map(
        providerStoreApiExecutionRow
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseProviderStoreApiExecutionKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchaseProviderStoreApiExecutionProof(
  proof: AppInstallPurchaseProviderStoreApiExecutionProof
) {
  return summarizeAppInstallPurchaseProviderStoreApiExecutionProofGenerated(proof);
}

function providerStoreApiExecutionRow(
  providerStoreRow: (typeof AppInstallPurchaseProductClaimProviderStoreProofReadModel.providerStoreProductClaimRows)[number]
) {
  const fallbackRow = matchingPlatformLimitationFallbackRow(providerStoreRow.platform, providerStoreRow.storeSurface);
  return buildAppInstallPurchaseProviderStoreApiExecutionRowGenerated(
    providerStoreRow,
    fallbackRow,
    Boundary,
    UpdatedAt
  );
}

function matchingPlatformLimitationFallbackRow(platform: string, storeSurface: string) {
  const row =
    AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel.platformLimitationFallbackRows.find(
      (candidate) => candidate.platform === platform && candidate.storeSurface === storeSurface
    );
  if (!row) {
    throw new Error(`missing platform limitation fallback row for ${platform}:${storeSurface}`);
  }
  return row;
}

function providerStoreApiExecutionRowIsHonest(row: ProviderStoreApiExecutionRowCandidate): boolean {
  return (
    row.sourceProviderStoreProofVersion === SourceProviderStoreProofVersion &&
    row.sourcePlatformLimitationFallbackProofVersion === SourcePlatformLimitationFallbackProofVersion &&
    providerStoreApiExecutionRowIsHonestGenerated(row, BoundaryFragments)
  );
}

function providerStoreApiExecutionProofIsHonest(proof: AppInstallPurchaseProviderStoreApiExecutionProof): boolean {
  return (
    proof.sourceProviderStoreProofVersion === SourceProviderStoreProofVersion &&
    proof.sourcePlatformLimitationFallbackProofVersion === SourcePlatformLimitationFallbackProofVersion &&
    providerStoreApiExecutionProofIsHonestGenerated(
      proof,
      StoreSurfaces,
      ProviderStoreApiExecutionStates,
      NonClaims
    ) &&
    proof.providerStoreApiExecutionRows.every(providerStoreApiExecutionRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
