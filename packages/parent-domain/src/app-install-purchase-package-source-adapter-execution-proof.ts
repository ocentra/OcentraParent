import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePackageSourceCaptureStatusProofReadModel } from './app-install-purchase-package-source-capture-status-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const PackageSourceAdapterExecutionText = Schema.String.pipe(Schema.minLength(1));
const PackageSourceAdapterExecutionProofVersion = 'app-install-purchase-package-source-adapter-execution-proof';
const SourcePackageSourceCaptureStatusProofVersion = 'app-install-purchase-package-source-capture-status-proof';
const PackageSourceAdapterExecutionTimestamp = '2026-06-05T15:15:00.000Z';
const PackageSourceAdapterExecutionClaimBoundary =
  'package-source adapter execution proof only; no provider API execution no store integration no portal approval UI no production platform adapter no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredAdapterExecutionStates = [
  'local-adapter-executed',
  'manual-host-proof-required',
  'device-management-required',
  'apple-entitlement-required',
  'platform-unavailable',
] as const;
const RequiredAdapterKinds = [
  'windows-local-package-source-reader',
  'macos-manual-host-proof',
  'linux-package-manager-unavailable',
  'android-device-owner-required',
  'ios-family-controls-entitlement-required',
] as const;
const PackageSourceAdapterExecutionNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-portal-approval-ui',
  'no-production-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchasePackageSourceAdapterExecutionProofSchemaVersionSchema = withParser(
  Schema.Literal(PackageSourceAdapterExecutionProofVersion)
);
const AppInstallPurchasePackageSourceAdapterExecutionSourceSchemaVersionSchema = withParser(
  Schema.Literal(SourcePackageSourceCaptureStatusProofVersion)
);
const AppInstallPurchasePackageSourceAdapterExecutionStateSchema = withParser(
  Schema.Literal(...RequiredAdapterExecutionStates)
);
const AppInstallPurchasePackageSourceAdapterKindSchema = withParser(Schema.Literal(...RequiredAdapterKinds));
const AppInstallPurchasePackageSourceAdapterExecutionCaptureStatusSchema = withParser(
  Schema.Literal('captured', 'blocked', 'manual-required', 'unavailable')
);
const AppInstallPurchasePackageSourceAdapterExecutionStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchasePackageSourceAdapterExecutionClaimSchema = withParser(Schema.Literal('proof-executed'));
const AppInstallPurchasePackageSourceAdapterNoExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchasePackageSourceAdapterNoDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePackageSourceAdapterNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePackageSourceAdapterNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchasePackageSourceAdapterCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchasePackageSourceAdapterNonClaimSchema = withParser(
  Schema.Literal(...PackageSourceAdapterExecutionNonClaims)
);

const PackageSourceAdapterExecutionRowIdSchema = PackageSourceAdapterExecutionText.pipe(
  Schema.brand('AppInstallPurchasePackageSourceAdapterExecutionRowId')
);
const PackageSourceAdapterExecutionRefSchema = PackageSourceAdapterExecutionText.pipe(
  Schema.brand('AppInstallPurchasePackageSourceAdapterExecutionRef')
);
const PackageSourceAdapterExecutionAuditRefSchema = PackageSourceAdapterExecutionText.pipe(
  Schema.brand('AppInstallPurchasePackageSourceAdapterExecutionAuditRef')
);
const PackageSourceAdapterExecutionReportRefSchema = PackageSourceAdapterExecutionText.pipe(
  Schema.brand('AppInstallPurchasePackageSourceAdapterExecutionReportRef')
);
const PackageSourceAdapterExecutionClaimBoundarySchema = PackageSourceAdapterExecutionText.pipe(
  Schema.brand('AppInstallPurchasePackageSourceAdapterExecutionClaimBoundary')
);

const PackageSourceAdapterExecutionRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePackageSourceAdapterExecutionProofSchemaVersionSchema,
  packageSourceAdapterExecutionRowId: PackageSourceAdapterExecutionRowIdSchema,
  sourcePackageSourceCaptureStatusProofVersion:
    AppInstallPurchasePackageSourceAdapterExecutionSourceSchemaVersionSchema,
  sourcePackageSourceCaptureRowId: PackageSourceAdapterExecutionRefSchema,
  sourcePackageSourceCaptureStatus: AppInstallPurchasePackageSourceAdapterExecutionCaptureStatusSchema,
  sourcePackageSourceCaptureArtifactRefs: Schema.Array(PackageSourceAdapterExecutionRefSchema),
  sourcePackageSourceAuditRefs: Schema.Array(PackageSourceAdapterExecutionAuditRefSchema),
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchasePackageSourceAdapterExecutionStoreSurfaceSchema,
  adapterKind: AppInstallPurchasePackageSourceAdapterKindSchema,
  adapterExecutionState: AppInstallPurchasePackageSourceAdapterExecutionStateSchema,
  adapterExecutionAttemptRefs: Schema.Array(PackageSourceAdapterExecutionRefSchema),
  adapterExecutionArtifactRefs: Schema.Array(PackageSourceAdapterExecutionRefSchema),
  auditEventRefs: Schema.Array(PackageSourceAdapterExecutionAuditRefSchema),
  reportRefs: Schema.Array(PackageSourceAdapterExecutionReportRefSchema),
  requiredProofRefs: Schema.Array(PackageSourceAdapterExecutionRefSchema),
  packageSourceAdapterExecutionClaim: AppInstallPurchasePackageSourceAdapterExecutionClaimSchema,
  providerApiExecutionClaim: AppInstallPurchasePackageSourceAdapterNoExecutionClaimSchema,
  storeIntegrationClaim: AppInstallPurchasePackageSourceAdapterNotClaimedSchema,
  portalApprovalUiClaim: AppInstallPurchasePackageSourceAdapterNotImplementedSchema,
  productionPlatformAdapterClaim: AppInstallPurchasePackageSourceAdapterNotImplementedSchema,
  childDeliveryClaim: AppInstallPurchasePackageSourceAdapterNoDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchasePackageSourceAdapterNoDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchasePackageSourceAdapterNotClaimedSchema,
  appBlockingClaim: AppInstallPurchasePackageSourceAdapterNotClaimedSchema,
  childDataCustody: AppInstallPurchasePackageSourceAdapterCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchasePackageSourceAdapterNotClaimedSchema,
  claimBoundary: PackageSourceAdapterExecutionClaimBoundarySchema,
  executedAt: ParentTimestampSchema,
});

type PackageSourceAdapterExecutionRowCandidate = Infer<typeof PackageSourceAdapterExecutionRowBaseSchema>;

export const AppInstallPurchasePackageSourceAdapterExecutionRowSchema = withParser(
  PackageSourceAdapterExecutionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        packageSourceAdapterExecutionRowIsHonest(row) ||
        'Expected app install/purchase package-source adapter execution rows to link capture status refs without provider, store, portal, production adapter, delivery, report, custody, interception, or blocking claims'
    )
  )
);

const PackageSourceAdapterExecutionProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePackageSourceAdapterExecutionProofSchemaVersionSchema,
  sourcePackageSourceCaptureStatusProofVersion:
    AppInstallPurchasePackageSourceAdapterExecutionSourceSchemaVersionSchema,
  packageSourceAdapterExecutionRows: Schema.Array(AppInstallPurchasePackageSourceAdapterExecutionRowSchema),
  nonClaims: Schema.Array(AppInstallPurchasePackageSourceAdapterNonClaimSchema),
  knownGaps: Schema.Array(PackageSourceAdapterExecutionRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePackageSourceAdapterExecutionProof = Infer<
  typeof PackageSourceAdapterExecutionProofBaseSchema
>;

export const AppInstallPurchasePackageSourceAdapterExecutionProofSchema = withParser(
  PackageSourceAdapterExecutionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        packageSourceAdapterExecutionProofIsHonest(proof) ||
        'Expected app install/purchase package-source adapter execution proof to cover adapter execution states and preserve non-claims'
    )
  )
);

export const AppInstallPurchasePackageSourceAdapterExecutionKnownGaps = [
  'Package-source adapter execution rows are parent-domain proof rows; production host adapters still need physical platform evidence.',
  'Android and iOS adapter execution remains blocked until managed-profile/device-owner or Apple entitlement proof exists.',
  'Provider/store APIs, store integration, portal approval UI, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchasePackageSourceAdapterExecutionProofReadModel =
  AppInstallPurchasePackageSourceAdapterExecutionProofSchema.parse({
    schemaVersion: PackageSourceAdapterExecutionProofVersion,
    sourcePackageSourceCaptureStatusProofVersion: SourcePackageSourceCaptureStatusProofVersion,
    packageSourceAdapterExecutionRows:
      AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.map(
        packageSourceAdapterExecutionRow
      ),
    nonClaims: PackageSourceAdapterExecutionNonClaims,
    knownGaps: AppInstallPurchasePackageSourceAdapterExecutionKnownGaps,
    updatedAt: PackageSourceAdapterExecutionTimestamp,
  });

export function summarizeAppInstallPurchasePackageSourceAdapterExecutionProof(
  proof: AppInstallPurchasePackageSourceAdapterExecutionProof
) {
  return {
    packageSourceAdapterExecutionRows: proof.packageSourceAdapterExecutionRows.length,
    localAdapterExecutedRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) => row.adapterExecutionState === 'local-adapter-executed'
    ).length,
    manualHostProofRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) => row.adapterExecutionState === 'manual-host-proof-required'
    ).length,
    blockedRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) =>
        row.adapterExecutionState === 'device-management-required' ||
        row.adapterExecutionState === 'apple-entitlement-required'
    ).length,
    unavailableRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) => row.adapterExecutionState === 'platform-unavailable'
    ).length,
    artifactLinkedRows: proof.packageSourceAdapterExecutionRows.filter(adapterExecutionArtifactCoverageIsComplete)
      .length,
    providerExecutedRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.packageSourceAdapterExecutionRows.filter(
      (row) => row.childDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function packageSourceAdapterExecutionRow(
  row: (typeof AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows)[number]
) {
  return {
    schemaVersion: PackageSourceAdapterExecutionProofVersion,
    packageSourceAdapterExecutionRowId: `package-source-adapter-execution-${row.platform}-${row.storeSurface}`,
    sourcePackageSourceCaptureStatusProofVersion: SourcePackageSourceCaptureStatusProofVersion,
    sourcePackageSourceCaptureRowId: row.packageSourceCaptureRowId,
    sourcePackageSourceCaptureStatus: row.packageSourceCaptureStatus,
    sourcePackageSourceCaptureArtifactRefs: row.packageSourceCaptureArtifactRefs,
    sourcePackageSourceAuditRefs: row.auditEventRefs,
    platform: row.platform,
    storeSurface: row.storeSurface,
    adapterKind: adapterKind(row.platform),
    adapterExecutionState: adapterExecutionState(row.platform),
    adapterExecutionAttemptRefs: [`package-source-adapter-execution-${row.platform}-${row.storeSurface}-attempt-ref`],
    adapterExecutionArtifactRefs: adapterExecutionArtifacts(row),
    auditEventRefs: [`package-source-adapter-execution-${row.platform}-${row.storeSurface}-audit-ref`],
    reportRefs: row.reportRefs,
    requiredProofRefs: [...row.requiredProofRefs, row.packageSourceCaptureRowId],
    packageSourceAdapterExecutionClaim: 'proof-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    portalApprovalUiClaim: 'not-implemented',
    productionPlatformAdapterClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: PackageSourceAdapterExecutionClaimBoundary,
    executedAt: PackageSourceAdapterExecutionTimestamp,
  } as const;
}

function adapterKind(
  platform: (typeof AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows)[number]['platform']
) {
  if (platform === 'windows') {
    return 'windows-local-package-source-reader';
  }
  if (platform === 'macos') {
    return 'macos-manual-host-proof';
  }
  if (platform === 'linux') {
    return 'linux-package-manager-unavailable';
  }
  if (platform === 'android') {
    return 'android-device-owner-required';
  }
  return 'ios-family-controls-entitlement-required';
}

function adapterExecutionState(
  platform: (typeof AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows)[number]['platform']
) {
  if (platform === 'windows') {
    return 'local-adapter-executed';
  }
  if (platform === 'macos') {
    return 'manual-host-proof-required';
  }
  if (platform === 'linux') {
    return 'platform-unavailable';
  }
  if (platform === 'android') {
    return 'device-management-required';
  }
  return 'apple-entitlement-required';
}

function adapterExecutionArtifacts(
  row: (typeof AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows)[number]
) {
  if (row.platform === 'windows') {
    return [
      ...row.packageSourceCaptureArtifactRefs,
      `package-source-adapter-execution-${row.platform}-${row.storeSurface}-artifact-ref`,
    ] as const;
  }
  return row.packageSourceCaptureArtifactRefs;
}

function packageSourceAdapterExecutionRowIsHonest(row: PackageSourceAdapterExecutionRowCandidate): boolean {
  return (
    adapterExecutionStateMatchesPlatform(row) &&
    adapterExecutionArtifactCoverageIsComplete(row) &&
    row.sourcePackageSourceAuditRefs.length > 0 &&
    row.adapterExecutionAttemptRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    packageSourceAdapterExecutionClaimsStayBounded(row) &&
    packageSourceAdapterExecutionBoundaryIsExplicit(row.claimBoundary)
  );
}

function adapterExecutionStateMatchesPlatform(row: PackageSourceAdapterExecutionRowCandidate): boolean {
  const expectedKind = adapterKind(row.platform);
  const expectedState = adapterExecutionState(row.platform);
  return row.adapterKind === expectedKind && row.adapterExecutionState === expectedState;
}

function adapterExecutionArtifactCoverageIsComplete(row: PackageSourceAdapterExecutionRowCandidate): boolean {
  return (
    row.sourcePackageSourceCaptureStatusProofVersion === SourcePackageSourceCaptureStatusProofVersion &&
    row.sourcePackageSourceCaptureRowId.length > 0 &&
    row.sourcePackageSourceCaptureArtifactRefs.length > 0 &&
    row.adapterExecutionArtifactRefs.length >= row.sourcePackageSourceCaptureArtifactRefs.length
  );
}

function packageSourceAdapterExecutionClaimsStayBounded(row: PackageSourceAdapterExecutionRowCandidate): boolean {
  return (
    row.packageSourceAdapterExecutionClaim === 'proof-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.productionPlatformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function packageSourceAdapterExecutionProofIsHonest(
  proof: AppInstallPurchasePackageSourceAdapterExecutionProof
): boolean {
  return (
    proof.sourcePackageSourceCaptureStatusProofVersion === SourcePackageSourceCaptureStatusProofVersion &&
    packageSourceAdapterExecutionRowsAreComplete(proof.packageSourceAdapterExecutionRows) &&
    packageSourceAdapterExecutionNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function packageSourceAdapterExecutionRowsAreComplete(
  rows: readonly PackageSourceAdapterExecutionRowCandidate[]
): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(rows.map((row) => row.adapterExecutionState));
  return (
    rows.length === AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.length &&
    keys.size === rows.length &&
    RequiredAdapterExecutionStates.every((state) => states.has(state)) &&
    rows.every((row) => packageSourceAdapterExecutionRowIsHonest(row))
  );
}

function packageSourceAdapterExecutionNonClaimsAreComplete(
  nonClaims: readonly (typeof PackageSourceAdapterExecutionNonClaims)[number][]
): boolean {
  const claimSet = new Set(nonClaims);
  return PackageSourceAdapterExecutionNonClaims.every((claim) => claimSet.has(claim));
}

function packageSourceAdapterExecutionBoundaryIsExplicit(
  boundary: typeof PackageSourceAdapterExecutionClaimBoundarySchema.Type
): boolean {
  return (
    boundary.includes('no provider API execution') &&
    boundary.includes('no store integration') &&
    boundary.includes('no portal approval UI') &&
    boundary.includes('no production platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('no app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}
