import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeProofReadModel } from './app-install-purchase-runtime-proof';
import { AppInstallPurchasePlatformArtifactProofReadModel } from './app-install-purchase-platform-artifact-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ChildArtifactProofText = Schema.String.pipe(Schema.minLength(1));
const ChildArtifactSchemaVersion = 'app-install-purchase-child-artifact-delivery-proof';
const SourcePlatformArtifactProofVersion = 'app-install-purchase-platform-artifact-proof';
const SourceRuntimeProofVersion = 'app-install-purchase-runtime-proof';
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

const ChildArtifactRowIdSchema = ChildArtifactProofText.pipe(Schema.brand('AppInstallPurchaseChildArtifactRowId'));
const ChildDeliveryRowIdSchema = ChildArtifactProofText.pipe(Schema.brand('AppInstallPurchaseChildDeliveryRowId'));
const ChildArtifactPlatformRowIdSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactPlatformRowId')
);
const ChildArtifactPackageRowIdSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactPackageRowId')
);
const ChildArtifactRefSchema = ChildArtifactProofText.pipe(Schema.brand('AppInstallPurchaseChildArtifactRef'));
const ChildArtifactReportRefSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactReportRef')
);
const ChildArtifactAuditRefSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactAuditRef')
);
const ChildArtifactProofRefSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactProofRef')
);
const ChildArtifactClaimBoundarySchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactClaimBoundary')
);
const ChildArtifactRequestIdSchema = ChildArtifactProofText.pipe(
  Schema.brand('AppInstallPurchaseChildArtifactRequestId')
);

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
  return {
    childArtifactRows: proof.childPackageArtifacts.length,
    childDeliveryRows: proof.childDeliveryBoundaries.length,
    attachedChildArtifactRefs: proof.childPackageArtifacts.filter(
      (row) => row.childArtifactSourceState === 'child-package-artifact-ref-attached'
    ).length,
    unavailableChildArtifactRows: proof.childPackageArtifacts.filter(
      (row) => row.childArtifactSourceState === 'platform-unavailable'
    ).length,
    notDeliveredRows: proof.childDeliveryBoundaries.filter((row) => row.childDeliveryClaim === 'not-delivered').length,
  } as const;
}

function childPackageArtifactRow(
  row: (typeof AppInstallPurchasePlatformArtifactProofReadModel.platformStoreArtifacts)[number]
) {
  return {
    schemaVersion: ChildArtifactSchemaVersion,
    childArtifactRowId: `child-package-artifact-${row.platform}-${row.storeSurface}`,
    platform: row.platform,
    storeSurface: row.storeSurface,
    platformArtifactRowId: row.artifactRowId,
    packageSourceArtifactRowId: row.packageSourceArtifactRowId,
    platformArtifactRef: row.artifactRef,
    childPackageArtifactRef: `child-package-source-${row.platform}-${row.storeSurface}-artifact-ref`,
    packageSourceArtifactState: row.sourcePackageArtifactState,
    childArtifactSourceState:
      row.sourcePackageArtifactState === 'platform-unavailable'
        ? 'platform-unavailable'
        : 'child-package-artifact-ref-attached',
    childArtifactCaptureClaim: 'not-runtime-captured',
    deliveryState: row.sourcePackageArtifactState === 'platform-unavailable' ? 'unavailable' : 'manual-required',
    childDeliveryClaim: 'not-delivered',
    providerApiClaim: row.providerApiClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    interceptionClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    reportRefs: row.reportRefs,
    requiredProofRefs: row.requiredProofRefs,
    claimBoundary: ChildArtifactClaimBoundary,
    attachedAt: ChildArtifactTimestamp,
  } as const;
}

function childDeliveryBoundaryRow(
  row: (typeof AppInstallPurchaseRuntimeProofReadModel.childDeliveryBoundaries)[number]
) {
  return {
    schemaVersion: ChildArtifactSchemaVersion,
    deliveryRowId: `child-delivery-boundary-${row.childVisibleStatus}`,
    sourceChildStateId: row.childStateId,
    requestId: row.requestId,
    platform: row.platform,
    childVisibleStatus: row.childVisibleStatus,
    deliveryState: row.deliveryState,
    childArtifactRef: `child-delivery-${row.childVisibleStatus}-artifact-ref`,
    childDeliveryClaim: row.runtimeDeliveryClaim,
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    auditEventRefs: row.auditEventRefs,
    reportRefs: row.reportRefs,
    claimBoundary: ChildArtifactClaimBoundary,
    attachedAt: ChildArtifactTimestamp,
  } as const;
}

function childPackageArtifactRowIsHonest(row: ChildPackageArtifactRowCandidate): boolean {
  return (
    childArtifactSourceStateMatchesPackageState(row) &&
    row.childArtifactCaptureClaim === 'not-runtime-captured' &&
    row.childDeliveryClaim === 'not-delivered' &&
    claimsStayUnimplemented(row) &&
    row.reportRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    childArtifactBoundaryIsExplicit(row.claimBoundary)
  );
}

function childDeliveryBoundaryRowIsHonest(row: ChildDeliveryBoundaryRowCandidate): boolean {
  return (
    row.deliveryState === 'manual-required' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    childArtifactBoundaryIsExplicit(row.claimBoundary)
  );
}

function childArtifactDeliveryProofIsHonest(proof: AppInstallPurchaseChildArtifactDeliveryProof): boolean {
  return (
    proof.sourcePlatformArtifactProofVersion === SourcePlatformArtifactProofVersion &&
    proof.sourceRuntimeProofVersion === SourceRuntimeProofVersion &&
    childPackageArtifactRowsAreComplete(proof.childPackageArtifacts) &&
    childDeliveryBoundaryRowsAreComplete(proof.childDeliveryBoundaries) &&
    nonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function childArtifactSourceStateMatchesPackageState(row: ChildPackageArtifactRowCandidate): boolean {
  if (row.packageSourceArtifactState === 'platform-unavailable') {
    return row.childArtifactSourceState === 'platform-unavailable' && row.deliveryState === 'unavailable';
  }
  return (
    row.childArtifactSourceState === 'child-package-artifact-ref-attached' && row.deliveryState === 'manual-required'
  );
}

function claimsStayUnimplemented(row: ChildPackageArtifactRowCandidate): boolean {
  return (
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.interceptionClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data'
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

function nonClaimsAreComplete(nonClaims: readonly (typeof ChildArtifactNonClaims)[number][]): boolean {
  const claimSet = new Set(nonClaims);
  return ChildArtifactNonClaims.every((claim) => claimSet.has(claim));
}

function childArtifactBoundaryIsExplicit(boundary: typeof ChildArtifactClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no store integration') &&
    boundary.includes('no provider API') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device runtime capture') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('not generic app blocking')
  );
}
