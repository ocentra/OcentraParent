import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildArtifactDeliveryProofReadModel } from './app-install-purchase-child-artifact-delivery-proof';
import { AppInstallPurchaseStoreStatusHandoffProofReadModel } from './app-install-purchase-store-status-handoff-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const PackageSourceCaptureProofVersion = 'app-install-purchase-package-source-capture-status-proof';
const SourceChildArtifactDeliveryProofVersion = 'app-install-purchase-child-artifact-delivery-proof';
const SourceStoreStatusHandoffProofVersion = 'app-install-purchase-store-status-handoff-proof';
const PackageSourceCaptureTimestamp = '2026-06-05T08:15:00.000Z';
const PackageSourceCaptureClaimBoundary =
  'package-source capture status proof only; no provider API execution no store integration no portal approval UI no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredCaptureStatuses = ['captured', 'blocked', 'manual-required', 'unavailable'] as const;
const PackageSourceCapturePlatformExpectations = {
  windows: {
    packageSourceArtifactState: 'requires-package-source-artifact',
    childArtifactSourceState: 'child-package-artifact-ref-attached',
    captureRequestState: 'accepted-for-local-package-source-proof',
    packageSourceCaptureStatus: 'captured',
    platformLimitationState: 'local-package-source-readable',
  },
  macos: {
    packageSourceArtifactState: 'requires-package-source-artifact',
    childArtifactSourceState: 'child-package-artifact-ref-attached',
    captureRequestState: 'manual-host-proof-required',
    packageSourceCaptureStatus: 'manual-required',
    platformLimitationState: 'requires-manual-host-proof',
  },
  linux: {
    packageSourceArtifactState: 'platform-unavailable',
    childArtifactSourceState: 'platform-unavailable',
    captureRequestState: 'platform-unavailable',
    packageSourceCaptureStatus: 'unavailable',
    platformLimitationState: 'platform-unavailable',
  },
  android: {
    packageSourceArtifactState: 'requires-device-proof-artifact',
    childArtifactSourceState: 'child-package-artifact-ref-attached',
    captureRequestState: 'blocked-by-device-management-policy',
    packageSourceCaptureStatus: 'blocked',
    platformLimitationState: 'requires-device-owner-or-managed-profile',
  },
  ios: {
    packageSourceArtifactState: 'requires-device-proof-artifact',
    childArtifactSourceState: 'child-package-artifact-ref-attached',
    captureRequestState: 'blocked-by-apple-entitlement',
    packageSourceCaptureStatus: 'blocked',
    platformLimitationState: 'requires-apple-entitlement',
  },
} as const;
const PackageSourceCaptureNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-portal-approval-ui',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchasePackageSourceCaptureStatusProofSchemaVersionSchema = withParser(
  Schema.Literal(PackageSourceCaptureProofVersion)
);
const AppInstallPurchasePackageSourceCaptureStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchasePackageSourceCaptureStatusSchema = withParser(Schema.Literal(...RequiredCaptureStatuses));
const AppInstallPurchasePackageSourceCaptureRequestStateSchema = withParser(
  Schema.Literal(
    'accepted-for-local-package-source-proof',
    'manual-host-proof-required',
    'blocked-by-device-management-policy',
    'blocked-by-apple-entitlement',
    'platform-unavailable'
  )
);
const AppInstallPurchasePackageSourceCapturePlatformLimitationStateSchema = withParser(
  Schema.Literal(
    'local-package-source-readable',
    'requires-manual-host-proof',
    'requires-device-owner-or-managed-profile',
    'requires-apple-entitlement',
    'platform-unavailable'
  )
);
const AppInstallPurchasePackageSourceCapturePackageArtifactStateSchema = withParser(
  Schema.Literal('requires-package-source-artifact', 'requires-device-proof-artifact', 'platform-unavailable')
);
const AppInstallPurchasePackageSourceCaptureChildArtifactSourceStateSchema = withParser(
  Schema.Literal('child-package-artifact-ref-attached', 'platform-unavailable')
);
const AppInstallPurchasePackageSourceCaptureStoreStatusHandoffStateSchema = withParser(
  Schema.Literal(
    'approved-api-status-proof-required',
    'store-entitlement-status-proof-required',
    'manual-platform-status-review-required',
    'platform-store-status-unavailable'
  )
);
const AppInstallPurchasePackageSourceCaptureClaimSchema = withParser(Schema.Literal('capture-status-proof-only'));
const AppInstallPurchasePackageSourceCaptureExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchasePackageSourceCaptureDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchasePackageSourceCaptureProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchasePackageSourceCaptureIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePackageSourceCaptureUiClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchasePackageSourceCaptureAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchasePackageSourceCaptureInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePackageSourceCaptureBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePackageSourceCaptureCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchasePackageSourceCaptureHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchasePackageSourceCaptureNonClaimSchema = withParser(
  Schema.Literal(...PackageSourceCaptureNonClaims)
);

const PackageSourceCaptureRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureRowId');
const PackageSourceCaptureSourceRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureSourceRowId');
const PackageSourceCaptureRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureRef');
const PackageSourceCaptureAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureAuditRef');
const PackageSourceCaptureReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureReportRef');
const PackageSourceCaptureClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchasePackageSourceCaptureClaimBoundary');

const PackageSourceCaptureRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePackageSourceCaptureStatusProofSchemaVersionSchema,
  packageSourceCaptureRowId: PackageSourceCaptureRowIdSchema,
  sourceChildArtifactDeliveryProofVersion: Schema.Literal(SourceChildArtifactDeliveryProofVersion),
  sourceChildPackageArtifactRowId: PackageSourceCaptureSourceRowIdSchema,
  sourceChildPackageArtifactRef: PackageSourceCaptureRefSchema,
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  sourceStoreStatusHandoffRowId: PackageSourceCaptureSourceRowIdSchema,
  sourceStoreStatusHandoffState: AppInstallPurchasePackageSourceCaptureStoreStatusHandoffStateSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchasePackageSourceCaptureStoreSurfaceSchema,
  packageSourceArtifactState: AppInstallPurchasePackageSourceCapturePackageArtifactStateSchema,
  childArtifactSourceState: AppInstallPurchasePackageSourceCaptureChildArtifactSourceStateSchema,
  captureRequestState: AppInstallPurchasePackageSourceCaptureRequestStateSchema,
  packageSourceCaptureStatus: AppInstallPurchasePackageSourceCaptureStatusSchema,
  platformLimitationState: AppInstallPurchasePackageSourceCapturePlatformLimitationStateSchema,
  packageSourceCaptureArtifactRefs: Schema.Array(PackageSourceCaptureRefSchema),
  sourceStoreStatusEvidenceRefs: Schema.Array(PackageSourceCaptureRefSchema),
  auditEventRefs: Schema.Array(PackageSourceCaptureAuditRefSchema),
  reportRefs: Schema.Array(PackageSourceCaptureReportRefSchema),
  requiredProofRefs: Schema.Array(PackageSourceCaptureRefSchema),
  packageSourceCaptureClaim: AppInstallPurchasePackageSourceCaptureClaimSchema,
  packageSourceCaptureExecutionClaim: AppInstallPurchasePackageSourceCaptureExecutionClaimSchema,
  providerApiExecutionClaim: AppInstallPurchasePackageSourceCaptureProviderClaimSchema,
  storeIntegrationClaim: AppInstallPurchasePackageSourceCaptureIntegrationClaimSchema,
  portalApprovalUiClaim: AppInstallPurchasePackageSourceCaptureUiClaimSchema,
  platformAdapterClaim: AppInstallPurchasePackageSourceCaptureAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchasePackageSourceCaptureDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchasePackageSourceCaptureDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchasePackageSourceCaptureInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchasePackageSourceCaptureBlockingClaimSchema,
  childDataCustody: AppInstallPurchasePackageSourceCaptureCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchasePackageSourceCaptureHostedCustodyClaimSchema,
  claimBoundary: PackageSourceCaptureClaimBoundarySchema,
  capturedAt: ParentTimestampSchema,
});

type PackageSourceCaptureRowCandidate = Infer<typeof PackageSourceCaptureRowBaseSchema>;

export const AppInstallPurchasePackageSourceCaptureStatusRowSchema = withParser(
  PackageSourceCaptureRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        packageSourceCaptureRowIsHonest(row) ||
        'Expected app install/purchase package-source capture status rows to link child artifact and store status evidence without provider, store, portal, adapter, delivery, custody, interception, or blocking claims'
    )
  )
);

const PackageSourceCaptureProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePackageSourceCaptureStatusProofSchemaVersionSchema,
  sourceChildArtifactDeliveryProofVersion: Schema.Literal(SourceChildArtifactDeliveryProofVersion),
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  packageSourceCaptureRows: Schema.Array(AppInstallPurchasePackageSourceCaptureStatusRowSchema),
  nonClaims: Schema.Array(AppInstallPurchasePackageSourceCaptureNonClaimSchema),
  knownGaps: Schema.Array(PackageSourceCaptureRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePackageSourceCaptureStatusProof = Infer<typeof PackageSourceCaptureProofBaseSchema>;

export const AppInstallPurchasePackageSourceCaptureStatusProofSchema = withParser(
  PackageSourceCaptureProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        packageSourceCaptureProofIsHonest(proof) ||
        'Expected app install/purchase package-source capture status proof to cover platform sources and preserve non-claims'
    )
  )
);

export const AppInstallPurchasePackageSourceCaptureStatusKnownGaps = [
  'Captured package-source status rows are proof/read-model rows only; real child-device platform adapter execution remains unimplemented.',
  'Android and iOS package-source capture remains blocked until device-owner/managed-profile or Apple entitlement proof exists.',
  'Provider/store APIs, store integration, portal approval UI, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchasePackageSourceCaptureStatusProofReadModel =
  AppInstallPurchasePackageSourceCaptureStatusProofSchema.parse({
    schemaVersion: PackageSourceCaptureProofVersion,
    sourceChildArtifactDeliveryProofVersion: SourceChildArtifactDeliveryProofVersion,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    packageSourceCaptureRows:
      AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts.map(packageSourceCaptureRow),
    nonClaims: PackageSourceCaptureNonClaims,
    knownGaps: AppInstallPurchasePackageSourceCaptureStatusKnownGaps,
    updatedAt: PackageSourceCaptureTimestamp,
  });

export function summarizeAppInstallPurchasePackageSourceCaptureStatusProof(
  proof: AppInstallPurchasePackageSourceCaptureStatusProof
) {
  return {
    packageSourceCaptureRows: proof.packageSourceCaptureRows.length,
    capturedRows: proof.packageSourceCaptureRows.filter((row) => row.packageSourceCaptureStatus === 'captured').length,
    blockedRows: proof.packageSourceCaptureRows.filter((row) => row.packageSourceCaptureStatus === 'blocked').length,
    manualRequiredRows: proof.packageSourceCaptureRows.filter(
      (row) => row.packageSourceCaptureStatus === 'manual-required'
    ).length,
    unavailableRows: proof.packageSourceCaptureRows.filter((row) => row.packageSourceCaptureStatus === 'unavailable')
      .length,
    artifactLinkedRows: proof.packageSourceCaptureRows.filter(packageSourceCaptureArtifactsAreComplete).length,
    auditLinkedRows: proof.packageSourceCaptureRows.filter((row) => row.auditEventRefs.length > 0).length,
    reportLinkedRows: proof.packageSourceCaptureRows.filter((row) => row.reportRefs.length > 0).length,
    deliveredRows: proof.packageSourceCaptureRows.filter((row) => row.childDeliveryClaim !== 'not-delivered').length,
  } as const;
}

function packageSourceCaptureRow(
  row: (typeof AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts)[number]
) {
  const storeStatusRow = storeStatusHandoffRowFor(row.platform, row.storeSurface);

  return {
    schemaVersion: PackageSourceCaptureProofVersion,
    packageSourceCaptureRowId: `package-source-capture-status-${row.platform}-${row.storeSurface}`,
    sourceChildArtifactDeliveryProofVersion: SourceChildArtifactDeliveryProofVersion,
    sourceChildPackageArtifactRowId: row.childArtifactRowId,
    sourceChildPackageArtifactRef: row.childPackageArtifactRef,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    sourceStoreStatusHandoffRowId: storeStatusRow.storeStatusHandoffRowId,
    sourceStoreStatusHandoffState: storeStatusRow.storeStatusHandoffState,
    platform: row.platform,
    storeSurface: row.storeSurface,
    packageSourceArtifactState: row.packageSourceArtifactState,
    childArtifactSourceState: row.childArtifactSourceState,
    captureRequestState: captureRequestState(row.platform),
    packageSourceCaptureStatus: packageSourceCaptureStatus(row.platform),
    platformLimitationState: platformLimitationState(row.platform),
    packageSourceCaptureArtifactRefs: [
      row.childPackageArtifactRef,
      `package-source-capture-${row.platform}-${row.storeSurface}-artifact-ref`,
    ],
    sourceStoreStatusEvidenceRefs: storeStatusRow.storeStatusHandoffEvidenceRefs,
    auditEventRefs: [`package-source-capture-${row.platform}-${row.storeSurface}-audit-ref`],
    reportRefs: [...row.reportRefs, ...storeStatusRow.sourceReportRuntimeRefs],
    requiredProofRefs: [...row.requiredProofRefs, storeStatusRow.sourcePlatformAdapterBoundaryRowId],
    packageSourceCaptureClaim: 'capture-status-proof-only',
    packageSourceCaptureExecutionClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: row.storeIntegrationClaim,
    portalApprovalUiClaim: 'not-implemented',
    platformAdapterClaim: row.platformAdapterClaim,
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: 'not-delivered',
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: 'not-claimed',
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: PackageSourceCaptureClaimBoundary,
    capturedAt: PackageSourceCaptureTimestamp,
  } as const;
}

function storeStatusHandoffRowFor(
  platform: (typeof RequiredPlatformSources)[number][0],
  storeSurface: (typeof RequiredPlatformSources)[number][1]
) {
  const row = AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.find(
    (storeStatusRow) => storeStatusRow.platform === platform && storeStatusRow.storeSurface === storeSurface
  );
  if (!row) {
    throw new Error(`Missing store status handoff row for ${platform}:${storeSurface}`);
  }
  return row;
}

function captureRequestState(platform: (typeof RequiredPlatformSources)[number][0]) {
  if (platform === 'windows') {
    return 'accepted-for-local-package-source-proof';
  }
  if (platform === 'macos') {
    return 'manual-host-proof-required';
  }
  if (platform === 'android') {
    return 'blocked-by-device-management-policy';
  }
  if (platform === 'ios') {
    return 'blocked-by-apple-entitlement';
  }
  return 'platform-unavailable';
}

function packageSourceCaptureStatus(platform: (typeof RequiredPlatformSources)[number][0]) {
  if (platform === 'windows') {
    return 'captured';
  }
  if (platform === 'macos') {
    return 'manual-required';
  }
  if (platform === 'linux') {
    return 'unavailable';
  }
  return 'blocked';
}

function platformLimitationState(platform: (typeof RequiredPlatformSources)[number][0]) {
  if (platform === 'windows') {
    return 'local-package-source-readable';
  }
  if (platform === 'macos') {
    return 'requires-manual-host-proof';
  }
  if (platform === 'android') {
    return 'requires-device-owner-or-managed-profile';
  }
  if (platform === 'ios') {
    return 'requires-apple-entitlement';
  }
  return 'platform-unavailable';
}

function packageSourceCaptureRowIsHonest(row: PackageSourceCaptureRowCandidate): boolean {
  return (
    packageSourceCaptureStatusMatchesPlatform(row) &&
    packageSourceCaptureArtifactsAreComplete(row) &&
    row.sourceStoreStatusEvidenceRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    packageSourceCaptureClaimsStayUnimplemented(row) &&
    packageSourceCaptureBoundaryIsExplicit(row.claimBoundary)
  );
}

function packageSourceCaptureStatusMatchesPlatform(row: PackageSourceCaptureRowCandidate): boolean {
  const expected = PackageSourceCapturePlatformExpectations[row.platform];
  return (
    row.packageSourceArtifactState === expected.packageSourceArtifactState &&
    row.childArtifactSourceState === expected.childArtifactSourceState &&
    row.captureRequestState === expected.captureRequestState &&
    row.packageSourceCaptureStatus === expected.packageSourceCaptureStatus &&
    row.platformLimitationState === expected.platformLimitationState
  );
}

function packageSourceCaptureArtifactsAreComplete(row: PackageSourceCaptureRowCandidate): boolean {
  return (
    row.sourceChildArtifactDeliveryProofVersion === SourceChildArtifactDeliveryProofVersion &&
    row.sourceChildPackageArtifactRowId.length > 0 &&
    row.sourceChildPackageArtifactRef.length > 0 &&
    row.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    row.sourceStoreStatusHandoffRowId.length > 0 &&
    row.packageSourceCaptureArtifactRefs.length > 0
  );
}

function packageSourceCaptureClaimsStayUnimplemented(row: PackageSourceCaptureRowCandidate): boolean {
  return (
    row.packageSourceCaptureClaim === 'capture-status-proof-only' &&
    row.packageSourceCaptureExecutionClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function packageSourceCaptureProofIsHonest(proof: AppInstallPurchasePackageSourceCaptureStatusProof): boolean {
  return (
    proof.sourceChildArtifactDeliveryProofVersion === SourceChildArtifactDeliveryProofVersion &&
    proof.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    packageSourceCaptureRowsAreComplete(proof.packageSourceCaptureRows) &&
    packageSourceCaptureNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function packageSourceCaptureRowsAreComplete(rows: readonly PackageSourceCaptureRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  const statuses = new Set(rows.map((row) => row.packageSourceCaptureStatus));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    RequiredCaptureStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => packageSourceCaptureRowIsHonest(row))
  );
}

function packageSourceCaptureNonClaimsAreComplete(
  nonClaims: readonly (typeof PackageSourceCaptureNonClaims)[number][]
): boolean {
  const claimSet = new Set(nonClaims);
  return PackageSourceCaptureNonClaims.every((claim) => claimSet.has(claim));
}

function packageSourceCaptureBoundaryIsExplicit(
  boundary: typeof PackageSourceCaptureClaimBoundarySchema.Type
): boolean {
  return (
    boundary.includes('no provider API execution') &&
    boundary.includes('no store integration') &&
    boundary.includes('no portal approval UI') &&
    boundary.includes('no platform adapter implementation') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('no app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}

