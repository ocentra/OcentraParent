import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeProofReadModel } from './app-install-purchase-runtime-proof';
import { AppInstallPurchasePlatformArtifactProofReadModel } from './app-install-purchase-platform-artifact-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseChildDeliveryBoundaryRowGenerated,
  buildAppInstallPurchaseChildPackageArtifactRowGenerated,
  childArtifactDeliveryProofIsHonestGenerated,
  childDeliveryBoundaryRowIsHonestGenerated,
  childPackageArtifactRowIsHonestGenerated,
  summarizeAppInstallPurchaseChildArtifactDeliveryProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const ChildArtifactSchemaVersion = 'app-install-purchase-child-artifact-delivery-proof';
const SourcePlatformArtifactProofVersion = AppInstallPurchasePlatformArtifactProofReadModel.schemaVersion;
const SourceRuntimeProofVersion = AppInstallPurchaseRuntimeProofReadModel.schemaVersion;
const ChildArtifactTimestamp = '2026-06-04T12:20:00.000Z';
const ChildArtifactClaimBoundary =
  'child artifact delivery boundary proof only; no store integration no provider API no platform adapter no child-device runtime capture no child-device delivery no runtime report delivery no real install or purchase interception no child activity data not generic app blocking';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredChildStatuses = [
  'pending-parent-review-visible',
  'approved-visible',
  'denied-visible',
  'time-box-visible',
  'review-needed-visible',
] as const;
const ChildArtifactNonClaims = [
  'no-store-integration',
  'no-provider-api',
  'no-platform-adapter',
  'no-child-device-runtime-capture',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'not-generic-app-blocking',
] as const;
const ChildArtifactBoundaryFragments = [
  'no store integration',
  'no provider API',
  'no platform adapter',
  'no child-device runtime capture',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'not generic app blocking',
] as const;

export const AppInstallPurchaseChildArtifactDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(ChildArtifactSchemaVersion)
);
const AppInstallPurchaseChildArtifactStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseChildArtifactPackageSourceStateSchema = withParser(
  Schema.Literal('requires-package-source-artifact', 'requires-device-proof-artifact', 'platform-unavailable')
);
const AppInstallPurchaseChildArtifactSourceStateSchema = withParser(
  Schema.Literal('child-package-artifact-ref-attached', 'platform-unavailable')
);
const AppInstallPurchaseChildArtifactCaptureClaimSchema = withParser(Schema.Literal('not-runtime-captured'));
const AppInstallPurchaseChildArtifactDeliveryStateSchema = withParser(Schema.Literal('manual-required', 'unavailable'));
const AppInstallPurchaseChildArtifactDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseChildArtifactProviderApiClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseChildArtifactAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseChildArtifactStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseChildArtifactAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseChildArtifactInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseChildArtifactDataCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseChildVisibleStatusSchema = withParser(Schema.Literal(...RequiredChildStatuses));
const AppInstallPurchaseChildArtifactNonClaimSchema = withParser(Schema.Literal(...ChildArtifactNonClaims));

const ChildArtifactRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactRowId');
const ChildDeliveryRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildDeliveryRowId');
const ChildArtifactPlatformRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactPlatformRowId');
const ChildArtifactPackageRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactPackageRowId');
const ChildArtifactRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactRef');
const ChildArtifactReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactReportRef');
const ChildArtifactAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactAuditRef');
const ChildArtifactProofRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactProofRef');
const ChildArtifactClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactClaimBoundary');
const ChildArtifactRequestIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseChildArtifactRequestId');

const ChildPackageArtifactRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildArtifactDeliveryProofSchemaVersionSchema,
  childArtifactRowId: ChildArtifactRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseChildArtifactStoreSurfaceSchema,
  platformArtifactRowId: ChildArtifactPlatformRowIdSchema,
  packageSourceArtifactRowId: ChildArtifactPackageRowIdSchema,
  platformArtifactRef: ChildArtifactRefSchema,
  childPackageArtifactRef: ChildArtifactRefSchema,
  packageSourceArtifactState: AppInstallPurchaseChildArtifactPackageSourceStateSchema,
  childArtifactSourceState: AppInstallPurchaseChildArtifactSourceStateSchema,
  childArtifactCaptureClaim: AppInstallPurchaseChildArtifactCaptureClaimSchema,
  deliveryState: AppInstallPurchaseChildArtifactDeliveryStateSchema,
  childDeliveryClaim: AppInstallPurchaseChildArtifactDeliveryClaimSchema,
  providerApiClaim: AppInstallPurchaseChildArtifactProviderApiClaimSchema,
  platformAdapterClaim: AppInstallPurchaseChildArtifactAdapterClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseChildArtifactStoreIntegrationClaimSchema,
  interceptionClaim: AppInstallPurchaseChildArtifactInterceptionClaimSchema,
  childDataCustody: AppInstallPurchaseChildArtifactDataCustodyClaimSchema,
  reportRefs: Schema.Array(ChildArtifactReportRefSchema),
  requiredProofRefs: Schema.Array(ChildArtifactProofRefSchema),
  claimBoundary: ChildArtifactClaimBoundarySchema,
  attachedAt: ParentTimestampSchema,
});

const ChildDeliveryBoundaryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildArtifactDeliveryProofSchemaVersionSchema,
  deliveryRowId: ChildDeliveryRowIdSchema,
  sourceChildStateId: ChildDeliveryRowIdSchema,
  requestId: ChildArtifactRequestIdSchema,
  platform: ParentPlatformSchema,
  childVisibleStatus: AppInstallPurchaseChildVisibleStatusSchema,
  deliveryState: AppInstallPurchaseChildArtifactDeliveryStateSchema,
  childArtifactRef: ChildArtifactRefSchema,
  childDeliveryClaim: AppInstallPurchaseChildArtifactDeliveryClaimSchema,
  providerApiClaim: AppInstallPurchaseChildArtifactProviderApiClaimSchema,
  platformAdapterClaim: AppInstallPurchaseChildArtifactAdapterClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseChildArtifactDeliveryClaimSchema,
  appBlockingClaim: AppInstallPurchaseChildArtifactAppBlockingClaimSchema,
  auditEventRefs: Schema.Array(ChildArtifactAuditRefSchema),
  reportRefs: Schema.Array(ChildArtifactReportRefSchema),
  claimBoundary: ChildArtifactClaimBoundarySchema,
  attachedAt: ParentTimestampSchema,
});

type ChildPackageArtifactRowCandidate = Infer<typeof ChildPackageArtifactRowBaseSchema>;
type ChildDeliveryBoundaryRowCandidate = Infer<typeof ChildDeliveryBoundaryRowBaseSchema>;

export const AppInstallPurchaseChildPackageArtifactRowSchema = withParser(
  ChildPackageArtifactRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childPackageArtifactRowIsHonest(row) ||
        'Expected child package artifact rows to attach refs without runtime capture, delivery, store, provider, adapter, interception, or child activity data claims'
    )
  )
);
export const AppInstallPurchaseChildDeliveryBoundaryRowSchema = withParser(
  ChildDeliveryBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childDeliveryBoundaryRowIsHonest(row) ||
        'Expected child delivery boundary rows to cite child artifact refs without delivery, report, adapter, provider, or blocking claims'
    )
  )
);

const ChildArtifactDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildArtifactDeliveryProofSchemaVersionSchema,
  sourcePlatformArtifactProofVersion: Schema.Literal(SourcePlatformArtifactProofVersion),
  sourceRuntimeProofVersion: Schema.Literal(SourceRuntimeProofVersion),
  childPackageArtifacts: Schema.Array(AppInstallPurchaseChildPackageArtifactRowSchema),
  childDeliveryBoundaries: Schema.Array(AppInstallPurchaseChildDeliveryBoundaryRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseChildArtifactNonClaimSchema),
  knownGaps: Schema.Array(ChildArtifactProofRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseChildArtifactDeliveryProof = Infer<typeof ChildArtifactDeliveryProofBaseSchema>;

export const AppInstallPurchaseChildArtifactDeliveryProofSchema = withParser(
  ChildArtifactDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        childArtifactDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase child artifact delivery proof to attach child artifact refs while preserving runtime non-claims'
    )
  )
);

export const AppInstallPurchaseChildArtifactDeliveryKnownGaps = [
  'Child package artifact refs are proof-boundary references only; no production child-device package capture adapter is implemented.',
  'Child pending/result delivery rows cite artifact refs but remain not-delivered until a real child-agent delivery path is proved.',
  'Store/provider APIs, platform adapters, runtime report delivery, portal UX, interception, and app-blocking behavior remain unimplemented.',
] as const;

export const AppInstallPurchaseChildArtifactDeliveryProofReadModel =
  AppInstallPurchaseChildArtifactDeliveryProofSchema.parse({
    schemaVersion: ChildArtifactSchemaVersion,
    sourcePlatformArtifactProofVersion: SourcePlatformArtifactProofVersion,
    sourceRuntimeProofVersion: SourceRuntimeProofVersion,
    childPackageArtifacts:
      AppInstallPurchasePlatformArtifactProofReadModel.platformStoreArtifacts.map(childPackageArtifactRow),
    childDeliveryBoundaries:
      AppInstallPurchaseRuntimeProofReadModel.childDeliveryBoundaries.map(childDeliveryBoundaryRow),
    nonClaims: ChildArtifactNonClaims,
    knownGaps: AppInstallPurchaseChildArtifactDeliveryKnownGaps,
    updatedAt: ChildArtifactTimestamp,
  });

export function summarizeAppInstallPurchaseChildArtifactDeliveryProof(
  proof: AppInstallPurchaseChildArtifactDeliveryProof
) {
  return summarizeAppInstallPurchaseChildArtifactDeliveryProofGenerated(proof);
}

function childPackageArtifactRow(
  row: (typeof AppInstallPurchasePlatformArtifactProofReadModel.platformStoreArtifacts)[number]
) {
  return buildAppInstallPurchaseChildPackageArtifactRowGenerated(
    row,
    ChildArtifactSchemaVersion,
    ChildArtifactClaimBoundary,
    ChildArtifactTimestamp
  );
}

function childDeliveryBoundaryRow(
  row: (typeof AppInstallPurchaseRuntimeProofReadModel.childDeliveryBoundaries)[number]
) {
  return buildAppInstallPurchaseChildDeliveryBoundaryRowGenerated(
    row,
    ChildArtifactSchemaVersion,
    ChildArtifactClaimBoundary,
    ChildArtifactTimestamp
  );
}

function childPackageArtifactRowIsHonest(row: ChildPackageArtifactRowCandidate): boolean {
  return childPackageArtifactRowIsHonestGenerated(row, ChildArtifactBoundaryFragments);
}

function childDeliveryBoundaryRowIsHonest(row: ChildDeliveryBoundaryRowCandidate): boolean {
  return childDeliveryBoundaryRowIsHonestGenerated(row, ChildArtifactBoundaryFragments);
}

function childArtifactDeliveryProofIsHonest(proof: AppInstallPurchaseChildArtifactDeliveryProof): boolean {
  return (
    childArtifactDeliveryProofIsHonestGenerated(
      proof,
      SourcePlatformArtifactProofVersion,
      SourceRuntimeProofVersion,
      RequiredPlatformSources,
      RequiredChildStatuses,
      ChildArtifactNonClaims
    ) &&
    childPackageArtifactRowsAreComplete(proof.childPackageArtifacts) &&
    childDeliveryBoundaryRowsAreComplete(proof.childDeliveryBoundaries) &&
    proof.knownGaps.length > 0
  );
}

function childPackageArtifactRowsAreComplete(rows: readonly ChildPackageArtifactRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    rows.every((row) => childPackageArtifactRowIsHonest(row))
  );
}

function childDeliveryBoundaryRowsAreComplete(rows: readonly ChildDeliveryBoundaryRowCandidate[]): boolean {
  const statuses = new Set(rows.map((row) => row.childVisibleStatus));
  return (
    rows.length === RequiredChildStatuses.length &&
    RequiredChildStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => childDeliveryBoundaryRowIsHonest(row))
  );
}
