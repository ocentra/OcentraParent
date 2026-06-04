import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeProofReadModel } from './app-install-purchase-runtime-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const PlatformArtifactProofText = Schema.String.pipe(Schema.minLength(1));
const PlatformArtifactSchemaVersion = 'app-install-purchase-platform-artifact-proof';
const SourceRuntimeProofVersion = 'app-install-purchase-runtime-proof';
const PlatformArtifactTimestamp = '2026-06-04T07:05:00.000Z';
const PlatformArtifactClaimBoundary =
  'platform artifact proof only; no store integration no provider API no platform adapter no child-device delivery no runtime report delivery no real install or purchase interception not generic app blocking';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredReportSurfaces = [
  'request-audit-history',
  'parent-decision-audit-history',
  'child-facing-state-report',
  'platform-limitation-report',
] as const;
const PlatformArtifactNonClaims = [
  'no-store-integration',
  'no-provider-api',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'not-generic-app-blocking',
] as const;

export const AppInstallPurchasePlatformArtifactProofSchemaVersionSchema = withParser(
  Schema.Literal(PlatformArtifactSchemaVersion)
);
const AppInstallPurchasePlatformArtifactStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchasePlatformArtifactKindSchema = withParser(
  Schema.Literal('platform-store-metadata-artifact', 'platform-limitation-report-artifact')
);
const AppInstallPurchasePlatformArtifactSourceStateSchema = withParser(
  Schema.Literal('parent-owned-artifact-attached')
);
const AppInstallPurchasePlatformArtifactStoreMetadataStateSchema = withParser(
  Schema.Literal('requires-platform-artifact', 'platform-unavailable')
);
const AppInstallPurchasePlatformArtifactPackageSourceStateSchema = withParser(
  Schema.Literal('requires-package-source-artifact', 'requires-device-proof-artifact', 'platform-unavailable')
);
const AppInstallPurchasePlatformArtifactClaimStateSchema = withParser(Schema.Literal('boundary-only'));
const AppInstallPurchasePlatformArtifactStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformArtifactProviderApiClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformArtifactAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchasePlatformArtifactDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePlatformArtifactRuntimeReportClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePlatformArtifactAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformArtifactReportSurfaceSchema = withParser(Schema.Literal(...RequiredReportSurfaces));
const AppInstallPurchasePlatformArtifactNonClaimSchema = withParser(Schema.Literal(...PlatformArtifactNonClaims));

const PlatformArtifactRefSchema = PlatformArtifactProofText.pipe(Schema.brand('AppInstallPurchasePlatformArtifactRef'));
const PlatformArtifactRowIdSchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactRowId')
);
const PlatformArtifactSourceRowIdSchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactSourceRowId')
);
const PlatformArtifactPackageSourceRowIdSchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactPackageSourceRowId')
);
const PlatformArtifactProofRefSchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactProofRef')
);
const PlatformArtifactReportRefSchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactReportRef')
);
const PlatformArtifactClaimBoundarySchema = PlatformArtifactProofText.pipe(
  Schema.brand('AppInstallPurchasePlatformArtifactClaimBoundary')
);

const PlatformStoreArtifactRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformArtifactProofSchemaVersionSchema,
  artifactRowId: PlatformArtifactRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchasePlatformArtifactStoreSurfaceSchema,
  platformSourceRowId: PlatformArtifactSourceRowIdSchema,
  packageSourceArtifactRowId: PlatformArtifactPackageSourceRowIdSchema,
  artifactRef: PlatformArtifactRefSchema,
  artifactKind: AppInstallPurchasePlatformArtifactKindSchema,
  artifactSourceState: AppInstallPurchasePlatformArtifactSourceStateSchema,
  sourceStoreMetadataArtifactState: AppInstallPurchasePlatformArtifactStoreMetadataStateSchema,
  sourcePackageArtifactState: AppInstallPurchasePlatformArtifactPackageSourceStateSchema,
  runtimeClaimState: AppInstallPurchasePlatformArtifactClaimStateSchema,
  storeIntegrationClaim: AppInstallPurchasePlatformArtifactStoreIntegrationClaimSchema,
  providerApiClaim: AppInstallPurchasePlatformArtifactProviderApiClaimSchema,
  platformAdapterClaim: AppInstallPurchasePlatformArtifactAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchasePlatformArtifactDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchasePlatformArtifactRuntimeReportClaimSchema,
  appBlockingClaim: AppInstallPurchasePlatformArtifactAppBlockingClaimSchema,
  requiredProofRefs: Schema.Array(PlatformArtifactProofRefSchema),
  reportRefs: Schema.Array(PlatformArtifactReportRefSchema),
  claimBoundary: PlatformArtifactClaimBoundarySchema,
  attachedAt: ParentTimestampSchema,
});

const ReportRuntimeEvidenceRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformArtifactProofSchemaVersionSchema,
  reportSurface: AppInstallPurchasePlatformArtifactReportSurfaceSchema,
  artifactRef: PlatformArtifactRefSchema,
  artifactSourceState: AppInstallPurchasePlatformArtifactSourceStateSchema,
  runtimeReportDeliveryClaim: AppInstallPurchasePlatformArtifactRuntimeReportClaimSchema,
  providerApiClaim: AppInstallPurchasePlatformArtifactProviderApiClaimSchema,
  platformAdapterClaim: AppInstallPurchasePlatformArtifactAdapterClaimSchema,
  auditEventRefs: Schema.Array(PlatformArtifactProofRefSchema),
  reportRefs: Schema.Array(PlatformArtifactReportRefSchema),
  claimBoundary: PlatformArtifactClaimBoundarySchema,
  attachedAt: ParentTimestampSchema,
});

type PlatformStoreArtifactRowCandidate = Infer<typeof PlatformStoreArtifactRowBaseSchema>;
type ReportRuntimeEvidenceRowCandidate = Infer<typeof ReportRuntimeEvidenceRowBaseSchema>;

export const AppInstallPurchasePlatformStoreArtifactRowSchema = withParser(
  PlatformStoreArtifactRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformStoreArtifactRowIsHonest(row) ||
        'Expected platform/store artifact rows to attach proof refs without store, provider, adapter, delivery, report, interception, or blocking claims'
    )
  )
);
export const AppInstallPurchaseReportRuntimeEvidenceRowSchema = withParser(
  ReportRuntimeEvidenceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportRuntimeEvidenceRowIsHonest(row) ||
        'Expected report runtime evidence rows to attach parent-owned artifacts without report delivery or adapter claims'
    )
  )
);

const PlatformArtifactProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformArtifactProofSchemaVersionSchema,
  sourceRuntimeProofVersion: Schema.Literal(SourceRuntimeProofVersion),
  platformStoreArtifacts: Schema.Array(AppInstallPurchasePlatformStoreArtifactRowSchema),
  reportRuntimeEvidence: Schema.Array(AppInstallPurchaseReportRuntimeEvidenceRowSchema),
  nonClaims: Schema.Array(AppInstallPurchasePlatformArtifactNonClaimSchema),
  knownGaps: Schema.Array(PlatformArtifactProofRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformArtifactProof = Infer<typeof PlatformArtifactProofBaseSchema>;

export const AppInstallPurchasePlatformArtifactProofSchema = withParser(
  PlatformArtifactProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        platformArtifactProofIsHonest(proof) ||
        'Expected app install/purchase platform artifact proof to attach artifact refs while preserving runtime non-claims'
    )
  )
);

export const AppInstallPurchasePlatformArtifactKnownGaps = [
  'Attached artifacts are parent-owned proof references only; no Google Play, Apple App Store, Microsoft Store, Mac App Store, or package-manager API integration is implemented.',
  'Package-source rows remain linked as requirements; this proof does not capture real child-device package artifacts.',
  'Report runtime evidence is a proof artifact boundary only; no portal report delivery or runtime report writer is implemented.',
] as const;

export const AppInstallPurchasePlatformArtifactProofReadModel = AppInstallPurchasePlatformArtifactProofSchema.parse({
  schemaVersion: PlatformArtifactSchemaVersion,
  sourceRuntimeProofVersion: SourceRuntimeProofVersion,
  platformStoreArtifacts:
    AppInstallPurchaseRuntimeProofReadModel.platformRuntimeArtifacts.map(platformStoreArtifactRow),
  reportRuntimeEvidence: AppInstallPurchaseRuntimeProofReadModel.reportIntegrationBoundaries.map(reportRuntimeRow),
  nonClaims: PlatformArtifactNonClaims,
  knownGaps: AppInstallPurchasePlatformArtifactKnownGaps,
  updatedAt: PlatformArtifactTimestamp,
});

export function summarizeAppInstallPurchasePlatformArtifactProof(proof: AppInstallPurchasePlatformArtifactProof) {
  return {
    platformArtifactRows: proof.platformStoreArtifacts.length,
    reportRuntimeEvidenceRows: proof.reportRuntimeEvidence.length,
    attachedPlatformArtifacts: proof.platformStoreArtifacts.filter(
      (row) => row.artifactSourceState === 'parent-owned-artifact-attached'
    ).length,
    unavailableStoreMetadataRows: proof.platformStoreArtifacts.filter(
      (row) => row.sourceStoreMetadataArtifactState === 'platform-unavailable'
    ).length,
  } as const;
}

function platformStoreArtifactRow(
  row: (typeof AppInstallPurchaseRuntimeProofReadModel.platformRuntimeArtifacts)[number]
) {
  const artifactKind =
    row.storeMetadataArtifactState === 'platform-unavailable'
      ? 'platform-limitation-report-artifact'
      : 'platform-store-metadata-artifact';
  return {
    schemaVersion: PlatformArtifactSchemaVersion,
    artifactRowId: `platform-artifact-${row.platform}-${row.storeSurface}`,
    platform: row.platform,
    storeSurface: row.storeSurface,
    platformSourceRowId: row.platformSourceRowId,
    packageSourceArtifactRowId: row.packageSourceArtifactRowId,
    artifactRef: `parent-owned-${row.platform}-${row.storeSurface}-artifact-ref`,
    artifactKind,
    artifactSourceState: 'parent-owned-artifact-attached',
    sourceStoreMetadataArtifactState: row.storeMetadataArtifactState,
    sourcePackageArtifactState: row.packageSourceArtifactState,
    runtimeClaimState: row.runtimeClaimState,
    storeIntegrationClaim: 'not-claimed',
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    requiredProofRefs: row.requiredProofRefs,
    reportRefs: row.reportRefs,
    claimBoundary: PlatformArtifactClaimBoundary,
    attachedAt: PlatformArtifactTimestamp,
  } as const;
}

function reportRuntimeRow(row: (typeof AppInstallPurchaseRuntimeProofReadModel.reportIntegrationBoundaries)[number]) {
  return {
    schemaVersion: PlatformArtifactSchemaVersion,
    reportSurface: row.surface,
    artifactRef: `parent-owned-${row.surface}-runtime-evidence-ref`,
    artifactSourceState: 'parent-owned-artifact-attached',
    runtimeReportDeliveryClaim: row.runtimeReportClaim,
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    auditEventRefs: row.auditEventRefs,
    reportRefs: row.reportRefs,
    claimBoundary: PlatformArtifactClaimBoundary,
    attachedAt: PlatformArtifactTimestamp,
  } as const;
}

function platformStoreArtifactRowIsHonest(row: PlatformStoreArtifactRowCandidate): boolean {
  return (
    row.artifactSourceState === 'parent-owned-artifact-attached' &&
    row.requiredProofRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    claimsStayUnimplemented(row) &&
    platformArtifactKindMatchesSourceState(row) &&
    artifactBoundaryIsExplicit(row.claimBoundary)
  );
}

function reportRuntimeEvidenceRowIsHonest(row: ReportRuntimeEvidenceRowCandidate): boolean {
  return (
    row.artifactSourceState === 'parent-owned-artifact-attached' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    artifactBoundaryIsExplicit(row.claimBoundary)
  );
}

function platformArtifactProofIsHonest(proof: AppInstallPurchasePlatformArtifactProof): boolean {
  return (
    proof.sourceRuntimeProofVersion === SourceRuntimeProofVersion &&
    platformArtifactRowsAreComplete(proof.platformStoreArtifacts) &&
    reportRuntimeRowsAreComplete(proof.reportRuntimeEvidence) &&
    nonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function claimsStayUnimplemented(row: PlatformStoreArtifactRowCandidate): boolean {
  return (
    row.runtimeClaimState === 'boundary-only' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed'
  );
}

function platformArtifactKindMatchesSourceState(row: PlatformStoreArtifactRowCandidate): boolean {
  if (row.sourceStoreMetadataArtifactState === 'platform-unavailable') {
    return row.artifactKind === 'platform-limitation-report-artifact';
  }
  return row.artifactKind === 'platform-store-metadata-artifact';
}

function platformArtifactRowsAreComplete(rows: readonly PlatformStoreArtifactRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    rows.every((row) => platformStoreArtifactRowIsHonest(row))
  );
}

function reportRuntimeRowsAreComplete(rows: readonly ReportRuntimeEvidenceRowCandidate[]): boolean {
  const surfaces = new Set(rows.map((row) => row.reportSurface));
  return (
    rows.length === RequiredReportSurfaces.length &&
    RequiredReportSurfaces.every((surface) => surfaces.has(surface)) &&
    rows.every((row) => reportRuntimeEvidenceRowIsHonest(row))
  );
}

function nonClaimsAreComplete(nonClaims: readonly (typeof PlatformArtifactNonClaims)[number][]): boolean {
  const claimSet = new Set(nonClaims);
  return PlatformArtifactNonClaims.every((claim) => claimSet.has(claim));
}

function artifactBoundaryIsExplicit(boundary: typeof PlatformArtifactClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no store integration') &&
    boundary.includes('no provider API') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('not generic app blocking')
  );
}
