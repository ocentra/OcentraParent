import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { AppInstallPurchaseProviderStoreApiExecutionProofReadModel } from './app-install-purchase-provider-store-api-execution-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchasePlatformAdapterEvidenceGapRowGenerated,
  platformAdapterEvidenceGapProofIsHonestGenerated,
  platformAdapterEvidenceGapRowIsHonestGenerated,
  summarizeAppInstallPurchasePlatformAdapterEvidenceGapProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const ProofVersion = 'app-install-purchase-platform-adapter-evidence-gap-proof';
const SourceProviderStoreApiExecutionProofVersion = 'app-install-purchase-provider-store-api-execution-proof';
const SourcePlatformProofReadinessProofVersion = 'app-install-purchase-platform-proof-readiness';
const UpdatedAt = '2026-06-07T00:36:00.000Z';
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const PlatformAdapterEvidenceGapStates = [
  'adapter-evidence-gap',
  'manual-adapter-evidence-required',
  'platform-unavailable',
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
  'no-real-platform-adapter-evidence-attached',
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
  'platform adapter evidence gap proof boundary only; links provider store API execution rows to per-platform adapter evidence requirements across Windows macOS Linux Android and iOS while keeping real adapter evidence separate from manual-required unavailable and blocked-before-claim states no product claim approval no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no real platform adapter evidence attached no child-device delivery no runtime writer delivery no runtime report delivery no portal approval UI no portal report UI no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'platform adapter evidence gap proof boundary only',
  'provider store API execution rows',
  'per-platform adapter evidence requirements',
  'Windows macOS Linux Android and iOS',
  'real adapter evidence separate',
  'manual-required unavailable and blocked-before-claim states',
  'no product claim approval',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no real platform adapter evidence attached',
  'no child-device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no portal approval UI',
  'no portal report UI',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema = withParser(
  Schema.Literal(ProofVersion)
);
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const PlatformAdapterEvidenceGapStateSchema = withParser(Schema.Literal(...PlatformAdapterEvidenceGapStates));
const SourceProviderStoreApiExecutionStateSchema = withParser(
  Schema.Literal('execution-ready', 'manual-required', 'unavailable', 'blocked-before-claim')
);
const SourcePlatformProofReadinessStateSchema = withParser(
  Schema.Literal('manual-proof-required', 'policy-blocked', 'unavailable')
);
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformAdapterEvidenceGapRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformAdapterEvidenceGapBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const NoRealAdapterEvidenceSchema = withParser(Schema.Literal('no-real-adapter-evidence-attached'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema,
  platformAdapterEvidenceGapRowId: RefSchema,
  sourceProviderStoreApiExecutionProofVersion: Schema.Literal(SourceProviderStoreApiExecutionProofVersion),
  sourceProviderStoreApiExecutionRowId: RefSchema,
  sourceProviderStoreApiExecutionState: SourceProviderStoreApiExecutionStateSchema,
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  sourcePlatformProofReadinessState: SourcePlatformProofReadinessStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: StoreSurfaceSchema,
  platformAdapterEvidenceGapState: PlatformAdapterEvidenceGapStateSchema,
  providerStoreApiExecutionEvidenceRefs: Schema.Array(RefSchema),
  requiredPlatformAdapterEvidenceRefs: Schema.Array(RefSchema),
  requiredManualPlatformEvidenceRefs: Schema.Array(RefSchema),
  requiredProviderCredentialRefs: Schema.Array(RefSchema),
  requiredPortalTestRefs: Schema.Array(RefSchema),
  requiredChildDeliveryRefs: Schema.Array(RefSchema),
  blockerRefs: Schema.Array(RefSchema),
  auditEventRefs: Schema.Array(RefSchema),
  reportRuntimeRefs: Schema.Array(RefSchema),
  realPlatformAdapterEvidenceState: NoRealAdapterEvidenceSchema,
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

type PlatformAdapterEvidenceGapRowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchasePlatformAdapterEvidenceGapRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformAdapterEvidenceGapRowIsHonest(row) ||
        'Expected platform adapter evidence gap rows to attach provider/store API source refs and keep adapter implementation, provider execution, delivery, portal, blocking, and custody unclaimed'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterEvidenceGapProofSchemaVersionSchema,
  sourceProviderStoreApiExecutionProofVersion: Schema.Literal(SourceProviderStoreApiExecutionProofVersion),
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourcePlatformProofReadinessProofVersion),
  platformAdapterEvidenceGapRows: Schema.Array(AppInstallPurchasePlatformAdapterEvidenceGapRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformAdapterEvidenceGapProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformAdapterEvidenceGapProofIsHonest(proof) ||
        'Expected platform adapter evidence gap proof to cover every platform/store row and preserve adapter and product-claim non-claims'
    )
  )
);

export const AppInstallPurchasePlatformAdapterEvidenceGapKnownGaps = [
  'Platform adapter evidence gap rows consume provider/store API execution rows but do not attach real platform adapter evidence.',
  'Windows is an adapter-evidence-gap row because provider/store API execution proof is ready but a real Windows platform adapter proof is still missing.',
  'macOS remains manual-adapter-evidence-required; Linux is platform-unavailable; Android and iOS stay blocked-before-claim until device-owner managed-profile entitlement review platform adapter and child delivery evidence exist.',
] as const;

export const AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel =
  AppInstallPurchasePlatformAdapterEvidenceGapProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceProviderStoreApiExecutionProofVersion: SourceProviderStoreApiExecutionProofVersion,
    sourcePlatformProofReadinessProofVersion: SourcePlatformProofReadinessProofVersion,
    platformAdapterEvidenceGapRows:
      AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows.map(
        platformAdapterEvidenceGapRow
      ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchasePlatformAdapterEvidenceGapKnownGaps,
    updatedAt: UpdatedAt,
  });

export function summarizeAppInstallPurchasePlatformAdapterEvidenceGapProof(
  proof: AppInstallPurchasePlatformAdapterEvidenceGapProof
) {
  return summarizeAppInstallPurchasePlatformAdapterEvidenceGapProofGenerated(proof);
}

function platformAdapterEvidenceGapRow(
  sourceRow: (typeof AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows)[number]
) {
  const platformReadinessRow = matchingPlatformProofReadinessRow(sourceRow.platform);
  return buildAppInstallPurchasePlatformAdapterEvidenceGapRowGenerated(
    sourceRow,
    platformReadinessRow,
    SourceProviderStoreApiExecutionProofVersion,
    SourcePlatformProofReadinessProofVersion,
    Boundary,
    UpdatedAt
  );
}

function matchingPlatformProofReadinessRow(platform: string) {
  const row = AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows.find(
    (candidate) => candidate.platform === platform
  );
  if (!row) {
    throw new Error(`missing platform proof readiness row for ${platform}`);
  }
  return row;
}

function platformAdapterEvidenceGapRowIsHonest(row: PlatformAdapterEvidenceGapRowCandidate): boolean {
  return (
    row.sourceProviderStoreApiExecutionProofVersion === SourceProviderStoreApiExecutionProofVersion &&
    row.sourcePlatformProofReadinessProofVersion === SourcePlatformProofReadinessProofVersion &&
    platformAdapterEvidenceGapRowIsHonestGenerated(row, BoundaryFragments)
  );
}

function platformAdapterEvidenceGapProofIsHonest(proof: AppInstallPurchasePlatformAdapterEvidenceGapProof): boolean {
  return (
    proof.sourceProviderStoreApiExecutionProofVersion === SourceProviderStoreApiExecutionProofVersion &&
    proof.sourcePlatformProofReadinessProofVersion === SourcePlatformProofReadinessProofVersion &&
    platformAdapterEvidenceGapProofIsHonestGenerated(
      proof,
      StoreSurfaces,
      PlatformAdapterEvidenceGapStates,
      NonClaims
    ) &&
    proof.platformAdapterEvidenceGapRows.every(platformAdapterEvidenceGapRowIsHonest)
  );
}
