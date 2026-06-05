import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildArtifactDeliveryProofReadModel } from './app-install-purchase-child-artifact-delivery-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const ApiEntitlementProofText = Schema.String.pipe(Schema.minLength(1));
const ApiEntitlementProofVersion = 'app-install-purchase-approved-api-entitlement-proof';
const SourceChildArtifactProofVersion = 'app-install-purchase-child-artifact-delivery-proof';
const ApiEntitlementTimestamp = '2026-06-05T00:15:00.000Z';
const ApiEntitlementClaimBoundary =
  'approved API entitlement evidence proof only; no store integration no provider API execution no platform adapter no child-device delivery no runtime report delivery no real install or purchase interception no child activity data not generic app blocking';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredApprovalStatuses = [
  'approved-api-evidence-required',
  'store-entitlement-evidence-required',
  'manual-platform-review-required',
  'platform-unavailable',
] as const;
const ApiEntitlementNonClaims = [
  'no-store-integration',
  'no-provider-api-execution',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'not-generic-app-blocking',
] as const;

export const AppInstallPurchaseApprovedApiEntitlementProofSchemaVersionSchema = withParser(
  Schema.Literal(ApiEntitlementProofVersion)
);
const AppInstallPurchaseApprovedApiEntitlementStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseApprovedApiEntitlementEvidenceStatusSchema = withParser(
  Schema.Literal(...RequiredApprovalStatuses)
);
const AppInstallPurchaseApprovedApiEvidenceSourceSchema = withParser(
  Schema.Literal('approved-store-api', 'store-entitlement', 'manual-platform-review', 'not-available')
);
const AppInstallPurchaseApprovedApiEntitlementClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseApprovedApiIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovedApiAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseApprovedApiDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseApprovedApiInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovedApiBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovedApiDataCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseApprovedApiEntitlementNonClaimSchema = withParser(Schema.Literal(...ApiEntitlementNonClaims));

const ApiEntitlementRowIdSchema = ApiEntitlementProofText.pipe(Schema.brand('AppInstallPurchaseApiEntitlementRowId'));
const ApiEntitlementChildArtifactRowIdSchema = ApiEntitlementProofText.pipe(
  Schema.brand('AppInstallPurchaseApiEntitlementChildArtifactRowId')
);
const ApiEntitlementRefSchema = ApiEntitlementProofText.pipe(Schema.brand('AppInstallPurchaseApiEntitlementRef'));
const ApiEntitlementReportRefSchema = ApiEntitlementProofText.pipe(
  Schema.brand('AppInstallPurchaseApiEntitlementReportRef')
);
const ApiEntitlementAuditRefSchema = ApiEntitlementProofText.pipe(
  Schema.brand('AppInstallPurchaseApiEntitlementAuditRef')
);
const ApiEntitlementProofRefSchema = ApiEntitlementProofText.pipe(
  Schema.brand('AppInstallPurchaseApiEntitlementProofRef')
);
const ApiEntitlementClaimBoundarySchema = ApiEntitlementProofText.pipe(
  Schema.brand('AppInstallPurchaseApiEntitlementClaimBoundary')
);

const ApiEntitlementEvidenceRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovedApiEntitlementProofSchemaVersionSchema,
  evidenceRowId: ApiEntitlementRowIdSchema,
  sourceChildArtifactRowId: ApiEntitlementChildArtifactRowIdSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseApprovedApiEntitlementStoreSurfaceSchema,
  evidenceStatus: AppInstallPurchaseApprovedApiEntitlementEvidenceStatusSchema,
  evidenceSource: AppInstallPurchaseApprovedApiEvidenceSourceSchema,
  approvedApiEvidenceRef: ApiEntitlementRefSchema,
  entitlementEvidenceRef: ApiEntitlementRefSchema,
  limitationReportRef: ApiEntitlementReportRefSchema,
  auditEventRefs: Schema.Array(ApiEntitlementAuditRefSchema),
  requiredProofRefs: Schema.Array(ApiEntitlementProofRefSchema),
  providerApiExecutionClaim: AppInstallPurchaseApprovedApiEntitlementClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseApprovedApiIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseApprovedApiAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchaseApprovedApiDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseApprovedApiDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseApprovedApiInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseApprovedApiBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseApprovedApiDataCustodyClaimSchema,
  claimBoundary: ApiEntitlementClaimBoundarySchema,
  attachedAt: ParentTimestampSchema,
});

type ApiEntitlementEvidenceRowCandidate = Infer<typeof ApiEntitlementEvidenceRowBaseSchema>;

export const AppInstallPurchaseApprovedApiEntitlementEvidenceRowSchema = withParser(
  ApiEntitlementEvidenceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        apiEntitlementEvidenceRowIsHonest(row) ||
        'Expected approved API entitlement evidence rows to cite proof refs without provider, adapter, delivery, report, interception, custody, or blocking claims'
    )
  )
);

const ApprovedApiEntitlementProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovedApiEntitlementProofSchemaVersionSchema,
  sourceChildArtifactProofVersion: Schema.Literal(SourceChildArtifactProofVersion),
  evidenceRows: Schema.Array(AppInstallPurchaseApprovedApiEntitlementEvidenceRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseApprovedApiEntitlementNonClaimSchema),
  knownGaps: Schema.Array(ApiEntitlementProofRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseApprovedApiEntitlementProof = Infer<typeof ApprovedApiEntitlementProofBaseSchema>;

export const AppInstallPurchaseApprovedApiEntitlementProofSchema = withParser(
  ApprovedApiEntitlementProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        apiEntitlementProofIsHonest(proof) ||
        'Expected app install/purchase approved API entitlement proof to cover platform sources while preserving non-claims'
    )
  )
);

export const AppInstallPurchaseApprovedApiEntitlementKnownGaps = [
  'Approved store API refs are evidence requirements only; no Google Play Apple Microsoft or package-manager provider execution is implemented.',
  'Entitlement evidence refs are proof-boundary references only; no production account/store entitlement exchange is implemented.',
  'Platform adapters, child-device delivery, runtime report delivery, portal UX, real interception, and app-blocking behavior remain unimplemented.',
] as const;

export const AppInstallPurchaseApprovedApiEntitlementProofReadModel =
  AppInstallPurchaseApprovedApiEntitlementProofSchema.parse({
    schemaVersion: ApiEntitlementProofVersion,
    sourceChildArtifactProofVersion: SourceChildArtifactProofVersion,
    evidenceRows:
      AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts.map(apiEntitlementEvidenceRow),
    nonClaims: ApiEntitlementNonClaims,
    knownGaps: AppInstallPurchaseApprovedApiEntitlementKnownGaps,
    updatedAt: ApiEntitlementTimestamp,
  });

export function summarizeAppInstallPurchaseApprovedApiEntitlementProof(
  proof: AppInstallPurchaseApprovedApiEntitlementProof
) {
  return {
    evidenceRows: proof.evidenceRows.length,
    approvedApiRequiredRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'approved-api-evidence-required')
      .length,
    entitlementRequiredRows: proof.evidenceRows.filter(
      (row) => row.evidenceStatus === 'store-entitlement-evidence-required'
    ).length,
    manualReviewRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'manual-platform-review-required')
      .length,
    unavailableRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'platform-unavailable').length,
  } as const;
}

function apiEntitlementEvidenceRow(
  row: (typeof AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts)[number]
) {
  const evidenceStatus = apiEntitlementEvidenceStatus(row.platform, row.storeSurface);
  const evidenceSource = apiEntitlementEvidenceSource(evidenceStatus);
  return {
    schemaVersion: ApiEntitlementProofVersion,
    evidenceRowId: `approved-api-entitlement-${row.platform}-${row.storeSurface}`,
    sourceChildArtifactRowId: row.childArtifactRowId,
    platform: row.platform,
    storeSurface: row.storeSurface,
    evidenceStatus,
    evidenceSource,
    approvedApiEvidenceRef: `${row.platform}-${row.storeSurface}-approved-api-evidence-ref`,
    entitlementEvidenceRef: `${row.platform}-${row.storeSurface}-entitlement-evidence-ref`,
    limitationReportRef: `${row.platform}-${row.storeSurface}-api-entitlement-limitation-report-ref`,
    auditEventRefs: [`${row.platform}-${row.storeSurface}-api-entitlement-audit-ref`],
    requiredProofRefs: [...row.requiredProofRefs, `${row.platform}-${row.storeSurface}-approved-api-proof-ref`],
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    claimBoundary: ApiEntitlementClaimBoundary,
    attachedAt: ApiEntitlementTimestamp,
  } as const;
}

function apiEntitlementEvidenceStatus(
  platform: typeof ParentPlatformSchema.Type,
  storeSurface: typeof AppInstallPurchaseApprovedApiEntitlementStoreSurfaceSchema.Type
) {
  if (platform === 'linux' || storeSurface === 'linux-package-manager') {
    return 'platform-unavailable';
  }
  if (platform === 'android' || platform === 'ios') {
    return 'store-entitlement-evidence-required';
  }
  if (platform === 'macos') {
    return 'manual-platform-review-required';
  }
  return 'approved-api-evidence-required';
}

function apiEntitlementEvidenceSource(
  status: typeof AppInstallPurchaseApprovedApiEntitlementEvidenceStatusSchema.Type
) {
  if (status === 'approved-api-evidence-required') {
    return 'approved-store-api';
  }
  if (status === 'store-entitlement-evidence-required') {
    return 'store-entitlement';
  }
  if (status === 'manual-platform-review-required') {
    return 'manual-platform-review';
  }
  return 'not-available';
}

function apiEntitlementEvidenceRowIsHonest(row: ApiEntitlementEvidenceRowCandidate): boolean {
  return (
    evidenceSourceMatchesStatus(row) &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.auditEventRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    apiEntitlementBoundaryIsExplicit(row.claimBoundary)
  );
}

function apiEntitlementProofIsHonest(proof: AppInstallPurchaseApprovedApiEntitlementProof): boolean {
  return (
    proof.sourceChildArtifactProofVersion === SourceChildArtifactProofVersion &&
    apiEntitlementRowsAreComplete(proof.evidenceRows) &&
    apiEntitlementNonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function evidenceSourceMatchesStatus(row: ApiEntitlementEvidenceRowCandidate): boolean {
  if (row.evidenceStatus === 'approved-api-evidence-required') {
    return row.evidenceSource === 'approved-store-api';
  }
  if (row.evidenceStatus === 'store-entitlement-evidence-required') {
    return row.evidenceSource === 'store-entitlement';
  }
  if (row.evidenceStatus === 'manual-platform-review-required') {
    return row.evidenceSource === 'manual-platform-review';
  }
  return row.evidenceSource === 'not-available';
}

function apiEntitlementRowsAreComplete(rows: readonly ApiEntitlementEvidenceRowCandidate[]): boolean {
  const keys = new Set(rows.map((row) => `${row.platform}:${row.storeSurface}`));
  const statuses = new Set(rows.map((row) => row.evidenceStatus));
  return (
    rows.length === RequiredPlatformSources.length &&
    keys.size === rows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    RequiredApprovalStatuses.every((status) => statuses.has(status)) &&
    rows.every((row) => apiEntitlementEvidenceRowIsHonest(row))
  );
}

function apiEntitlementNonClaimsAreComplete(nonClaims: readonly (typeof ApiEntitlementNonClaims)[number][]): boolean {
  const claimSet = new Set(nonClaims);
  return ApiEntitlementNonClaims.every((claim) => claimSet.has(claim));
}

function apiEntitlementBoundaryIsExplicit(boundary: typeof ApiEntitlementClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no store integration') &&
    boundary.includes('no provider API execution') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no real install or purchase interception') &&
    boundary.includes('no child activity data') &&
    boundary.includes('not generic app blocking')
  );
}
