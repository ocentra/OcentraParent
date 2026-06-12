import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovedApiEntitlementProofReadModel } from './app-install-purchase-approved-api-entitlement-proof';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const PlatformAdapterBoundaryText = Schema.String.pipe(Schema.minLength(1));
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
const AppInstallPurchasePlatformAdapterRuntimeReportDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePlatformAdapterInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterChildDataCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchasePlatformAdapterHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePlatformAdapterBoundaryNonClaimSchema = withParser(
  Schema.Literal(...PlatformAdapterBoundaryNonClaims)
);

const PlatformAdapterBoundaryRowIdSchema = PlatformAdapterBoundaryText.pipe(
  Schema.brand('AppInstallPurchasePlatformAdapterBoundaryRowId')
);
const PlatformAdapterBoundarySourceRowIdSchema = PlatformAdapterBoundaryText.pipe(
  Schema.brand('AppInstallPurchasePlatformAdapterBoundarySourceRowId')
);
const PlatformAdapterBoundaryRefSchema = PlatformAdapterBoundaryText.pipe(
  Schema.brand('AppInstallPurchasePlatformAdapterBoundaryRef')
);
const PlatformAdapterBoundaryReportRefSchema = PlatformAdapterBoundaryText.pipe(
  Schema.brand('AppInstallPurchasePlatformAdapterBoundaryReportRef')
);
const PlatformAdapterBoundaryClaimBoundarySchema = PlatformAdapterBoundaryText.pipe(
  Schema.brand('AppInstallPurchasePlatformAdapterBoundaryClaimBoundary')
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
  return {
    adapterBoundaryRows: proof.adapterBoundaryRows.length,
    notImplementedRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'not-implemented').length,
    manualRequiredRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'manual-required').length,
    unavailableRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'unavailable').length,
    reportRuntimeLinkedRows: proof.adapterBoundaryRows.filter((row) => row.reportRuntimeRefs.length > 0).length,
  } as const;
}

function platformAdapterBoundaryRow(
  row: (typeof AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows)[number]
) {
  return {
    schemaVersion: PlatformAdapterBoundaryVersion,
    adapterBoundaryRowId: `platform-adapter-boundary-${row.platform}-${row.storeSurface}`,
    sourceApprovedApiEntitlementRowId: row.evidenceRowId,
    platform: row.platform,
    storeSurface: row.storeSurface,
    adapterEvidenceState: adapterEvidenceState(row.evidenceStatus),
    adapterRuntimeState: adapterRuntimeState(row.evidenceStatus),
    approvedApiEvidenceRef: row.approvedApiEvidenceRef,
    entitlementEvidenceRef: row.entitlementEvidenceRef,
    limitationReportRef: row.limitationReportRef,
    reportRuntimeRefs: AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.map(
      (reportRow) => reportRow.outputReportRef
    ),
    adapterReadinessEvidenceRefs: row.requiredProofRefs.map((proofRef) => `${proofRef}-adapter-readiness`),
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: PlatformAdapterBoundaryClaimBoundary,
    evaluatedAt: PlatformAdapterBoundaryTimestamp,
  } as const;
}

function adapterEvidenceState(
  status: (typeof AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows)[number]['evidenceStatus']
) {
  if (status === 'approved-api-evidence-required') {
    return 'approved-api-adapter-evidence-required';
  }
  if (status === 'store-entitlement-evidence-required') {
    return 'entitlement-adapter-evidence-required';
  }
  if (status === 'manual-platform-review-required') {
    return 'manual-platform-review-required';
  }
  return 'platform-unavailable';
}

function adapterRuntimeState(
  status: (typeof AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows)[number]['evidenceStatus']
) {
  if (status === 'platform-unavailable') {
    return 'unavailable';
  }
  if (status === 'manual-platform-review-required') {
    return 'manual-required';
  }
  return 'not-implemented';
}

function platformAdapterBoundaryRowIsHonest(row: PlatformAdapterBoundaryRowCandidate): boolean {
  return (
    adapterEvidenceMatchesRuntime(row) &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    row.reportRuntimeRefs.length > 0 &&
    row.adapterReadinessEvidenceRefs.length > 0 &&
    platformAdapterBoundaryIsExplicit(row.claimBoundary)
  );
}

function adapterEvidenceMatchesRuntime(row: PlatformAdapterBoundaryRowCandidate): boolean {
  if (row.adapterEvidenceState === 'platform-unavailable') {
    return row.adapterRuntimeState === 'unavailable';
  }
  if (row.adapterEvidenceState === 'manual-platform-review-required') {
    return row.adapterRuntimeState === 'manual-required';
  }
  return row.adapterRuntimeState === 'not-implemented';
}

function platformAdapterBoundaryProofIsHonest(proof: AppInstallPurchasePlatformAdapterBoundaryProof): boolean {
  return (
    proof.sourceApprovedApiEntitlementProofVersion === SourceApprovedApiEntitlementProofVersion &&
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    platformAdapterRowsAreComplete(proof.adapterBoundaryRows) &&
    platformAdapterNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function platformAdapterRowsAreComplete(rows: readonly PlatformAdapterBoundaryRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  const evidenceStates = new Set(rows.map((row) => row.adapterEvidenceState));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    RequiredAdapterEvidenceStates.every((state) => evidenceStates.has(state)) &&
    rows.every((row) => platformAdapterBoundaryRowIsHonest(row))
  );
}

function platformAdapterNonClaimsAreComplete(
  nonClaims: readonly (typeof PlatformAdapterBoundaryNonClaims)[number][]
): boolean {
  const claimSet = new Set(nonClaims);
  return PlatformAdapterBoundaryNonClaims.every((claim) => claimSet.has(claim));
}

function platformAdapterBoundaryIsExplicit(boundary: typeof PlatformAdapterBoundaryClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no platform adapter implementation') &&
    boundary.includes('no provider API execution') &&
    boundary.includes('no store integration') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('no app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}
