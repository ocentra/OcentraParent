import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovedApiEntitlementProofReadModel } from './app-install-purchase-approved-api-entitlement-proof';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchasePlatformAdapterBoundaryRowGenerated,
  platformAdapterBoundaryProofIsHonestGenerated,
  platformAdapterBoundaryRowIsHonestGenerated,
  summarizeAppInstallPurchasePlatformAdapterBoundaryProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const PlatformAdapterBoundaryVersion = 'app-install-purchase-platform-adapter-boundary-proof';
const SourceApprovedApiEntitlementProofVersion = 'app-install-purchase-approved-api-entitlement-proof';
const SourceReportRuntimeProofVersion = 'app-install-purchase-report-runtime-proof';
const PlatformAdapterBoundaryTimestamp = '2026-06-05T02:35:00.000Z';
const PlatformAdapterBoundaryClaimBoundary =
  'platform adapter boundary proof only; no platform adapter implementation no provider API execution no store integration no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredAdapterEvidenceStates = [
  'approved-api-adapter-evidence-required',
  'entitlement-adapter-evidence-required',
  'manual-platform-review-required',
  'platform-unavailable',
] as const;
const PlatformAdapterBoundaryNonClaims = [
  'no-platform-adapter-implementation',
  'no-provider-api-execution',
  'no-store-integration',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const PlatformAdapterBoundaryBoundaryFragments = [
  'no platform adapter implementation',
  'no provider API execution',
  'no store integration',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'no app blocking',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchasePlatformAdapterBoundaryProofSchemaVersionSchema = withParser(
  Schema.Literal(PlatformAdapterBoundaryVersion)
);
const AppInstallPurchasePlatformAdapterBoundaryStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchasePlatformAdapterEvidenceStateSchema = withParser(
  Schema.Literal(...RequiredAdapterEvidenceStates)
);
const AppInstallPurchasePlatformAdapterRuntimeStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'unavailable')
);
const AppInstallPurchasePlatformAdapterProviderApiExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchasePlatformAdapterStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterChildDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePlatformAdapterRuntimeReportDeliveryClaimSchema = withParser(
  Schema.Literal('not-delivered')
);
const AppInstallPurchasePlatformAdapterInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterChildDataCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchasePlatformAdapterHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterBoundaryNonClaimSchema = withParser(
  Schema.Literal(...PlatformAdapterBoundaryNonClaims)
);

const PlatformAdapterBoundaryRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformAdapterBoundaryRowId'
);
const PlatformAdapterBoundarySourceRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformAdapterBoundarySourceRowId'
);
const PlatformAdapterBoundaryRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformAdapterBoundaryRef');
const PlatformAdapterBoundaryReportRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformAdapterBoundaryReportRef'
);
const PlatformAdapterBoundaryClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformAdapterBoundaryClaimBoundary'
);

const PlatformAdapterBoundaryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterBoundaryProofSchemaVersionSchema,
  adapterBoundaryRowId: PlatformAdapterBoundaryRowIdSchema,
  sourceApprovedApiEntitlementRowId: PlatformAdapterBoundarySourceRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchasePlatformAdapterBoundaryStoreSurfaceSchema,
  adapterEvidenceState: AppInstallPurchasePlatformAdapterEvidenceStateSchema,
  adapterRuntimeState: AppInstallPurchasePlatformAdapterRuntimeStateSchema,
  approvedApiEvidenceRef: PlatformAdapterBoundaryRefSchema,
  entitlementEvidenceRef: PlatformAdapterBoundaryRefSchema,
  limitationReportRef: PlatformAdapterBoundaryReportRefSchema,
  reportRuntimeRefs: Schema.Array(PlatformAdapterBoundaryReportRefSchema),
  adapterReadinessEvidenceRefs: Schema.Array(PlatformAdapterBoundaryRefSchema),
  providerApiExecutionClaim: AppInstallPurchasePlatformAdapterProviderApiExecutionClaimSchema,
  storeIntegrationClaim: AppInstallPurchasePlatformAdapterStoreIntegrationClaimSchema,
  childDeliveryClaim: AppInstallPurchasePlatformAdapterChildDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchasePlatformAdapterRuntimeReportDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchasePlatformAdapterInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchasePlatformAdapterAppBlockingClaimSchema,
  childDataCustody: AppInstallPurchasePlatformAdapterChildDataCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchasePlatformAdapterHostedCustodyClaimSchema,
  claimBoundary: PlatformAdapterBoundaryClaimBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type PlatformAdapterBoundaryRowCandidate = Infer<typeof PlatformAdapterBoundaryRowBaseSchema>;

export const AppInstallPurchasePlatformAdapterBoundaryRowSchema = withParser(
  PlatformAdapterBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformAdapterBoundaryRowIsHonest(row) ||
        'Expected app install/purchase platform adapter boundary rows to cite readiness evidence without adapter, provider, store, delivery, report, custody, interception, or blocking claims'
    )
  )
);

const PlatformAdapterBoundaryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformAdapterBoundaryProofSchemaVersionSchema,
  sourceApprovedApiEntitlementProofVersion: Schema.Literal(SourceApprovedApiEntitlementProofVersion),
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  adapterBoundaryRows: Schema.Array(AppInstallPurchasePlatformAdapterBoundaryRowSchema),
  nonClaims: Schema.Array(AppInstallPurchasePlatformAdapterBoundaryNonClaimSchema),
  knownGaps: Schema.Array(PlatformAdapterBoundaryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformAdapterBoundaryProof = Infer<typeof PlatformAdapterBoundaryProofBaseSchema>;

export const AppInstallPurchasePlatformAdapterBoundaryProofSchema = withParser(
  PlatformAdapterBoundaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformAdapterBoundaryProofIsHonest(proof) ||
        'Expected app install/purchase platform adapter boundary proof to cover platform sources while preserving adapter non-claims'
    )
  )
);

export const AppInstallPurchasePlatformAdapterBoundaryKnownGaps = [
  'Platform adapter rows are readiness/evidence boundaries only; no Google Play, Apple App Store, Microsoft Store, Mac App Store, or Linux package-manager adapter is implemented.',
  'Provider API execution and store integration remain proof requirements only and do not run against live providers or store accounts.',
  'Child-device delivery, runtime report writer/delivery, real install or purchase interception, app blocking, and Ocentra-hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchasePlatformAdapterBoundaryProofReadModel =
  AppInstallPurchasePlatformAdapterBoundaryProofSchema.parse({
    schemaVersion: PlatformAdapterBoundaryVersion,
    sourceApprovedApiEntitlementProofVersion: SourceApprovedApiEntitlementProofVersion,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    adapterBoundaryRows:
      AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows.map(platformAdapterBoundaryRow),
    nonClaims: PlatformAdapterBoundaryNonClaims,
    knownGaps: AppInstallPurchasePlatformAdapterBoundaryKnownGaps,
    updatedAt: PlatformAdapterBoundaryTimestamp,
  });

export function summarizeAppInstallPurchasePlatformAdapterBoundaryProof(
  proof: AppInstallPurchasePlatformAdapterBoundaryProof
) {
  return summarizeAppInstallPurchasePlatformAdapterBoundaryProofGenerated(proof);
}

function platformAdapterBoundaryRow(
  row: (typeof AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows)[number]
) {
  return buildAppInstallPurchasePlatformAdapterBoundaryRowGenerated(
    row,
    AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.map((reportRow) => reportRow.outputReportRef),
    PlatformAdapterBoundaryClaimBoundary,
    PlatformAdapterBoundaryTimestamp
  );
}

function platformAdapterBoundaryRowIsHonest(row: PlatformAdapterBoundaryRowCandidate): boolean {
  return platformAdapterBoundaryRowIsHonestGenerated(row, PlatformAdapterBoundaryBoundaryFragments);
}

function platformAdapterBoundaryProofIsHonest(proof: AppInstallPurchasePlatformAdapterBoundaryProof): boolean {
  return (
    proof.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    platformAdapterBoundaryProofIsHonestGenerated(
      proof,
      RequiredPlatformSources,
      RequiredAdapterEvidenceStates,
      PlatformAdapterBoundaryNonClaims
    ) &&
    proof.adapterBoundaryRows.every(platformAdapterBoundaryRowIsHonest)
  );
}
