import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeProofReadModel } from './app-install-purchase-runtime-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchasePlatformStoreArtifactRowGenerated,
  buildAppInstallPurchaseReportRuntimeEvidenceRowGenerated,
  platformArtifactProofIsHonestGenerated,
  platformStoreArtifactRowIsHonestGenerated,
  reportRuntimeEvidenceRowIsHonestGenerated,
  summarizeAppInstallPurchasePlatformArtifactProofGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';
const PlatformArtifactSchemaVersion = 'app-install-purchase-platform-artifact-proof';
const SourceRuntimeProofVersion = AppInstallPurchaseRuntimeProofReadModel.schemaVersion;
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
const PlatformArtifactBoundaryFragments = [
  'no store integration',
  'no provider API',
  'no platform adapter',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'not generic app blocking',
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

const PlatformArtifactRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformArtifactRef');
const PlatformArtifactRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformArtifactRowId');
const PlatformArtifactSourceRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformArtifactSourceRowId');
const PlatformArtifactPackageSourceRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformArtifactPackageSourceRowId'
);
const PlatformArtifactProofRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformArtifactProofRef');
const PlatformArtifactReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformArtifactReportRef');
const PlatformArtifactClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchasePlatformArtifactClaimBoundary'
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
  return summarizeAppInstallPurchasePlatformArtifactProofGenerated(proof);
}

function platformStoreArtifactRow(
  row: (typeof AppInstallPurchaseRuntimeProofReadModel.platformRuntimeArtifacts)[number]
) {
  return buildAppInstallPurchasePlatformStoreArtifactRowGenerated(
    row,
    PlatformArtifactClaimBoundary,
    PlatformArtifactTimestamp
  );
}

function reportRuntimeRow(row: (typeof AppInstallPurchaseRuntimeProofReadModel.reportIntegrationBoundaries)[number]) {
  return buildAppInstallPurchaseReportRuntimeEvidenceRowGenerated(
    {
      reportSurface: row.surface,
      runtimeReportClaim: row.runtimeReportClaim,
      auditEventRefs: row.auditEventRefs,
      reportRefs: row.reportRefs,
    },
    PlatformArtifactClaimBoundary,
    PlatformArtifactTimestamp
  );
}

function platformStoreArtifactRowIsHonest(row: PlatformStoreArtifactRowCandidate): boolean {
  return platformStoreArtifactRowIsHonestGenerated(row, PlatformArtifactBoundaryFragments);
}

function reportRuntimeEvidenceRowIsHonest(row: ReportRuntimeEvidenceRowCandidate): boolean {
  return reportRuntimeEvidenceRowIsHonestGenerated(row, PlatformArtifactBoundaryFragments);
}

function platformArtifactProofIsHonest(proof: AppInstallPurchasePlatformArtifactProof): boolean {
  return (
    platformArtifactProofIsHonestGenerated(
      proof,
      SourceRuntimeProofVersion,
      RequiredPlatformSources,
      RequiredReportSurfaces,
      PlatformArtifactNonClaims
    ) &&
    proof.platformStoreArtifacts.every(platformStoreArtifactRowIsHonest) &&
    proof.reportRuntimeEvidence.every(reportRuntimeEvidenceRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
