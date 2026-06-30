import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildArtifactDeliveryProofReadModel } from './app-install-purchase-child-artifact-delivery-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  apiEntitlementEvidenceRowIsHonestGenerated,
  apiEntitlementProofIsHonestGenerated,
  buildAppInstallPurchaseApprovedApiEntitlementEvidenceRowGenerated,
  summarizeAppInstallPurchaseApprovedApiEntitlementProofGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';
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
const ApiEntitlementBoundaryFragments = [
  'no store integration',
  'no provider API execution',
  'no platform adapter',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'not generic app blocking',
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

const ApiEntitlementRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementRowId');
const ApiEntitlementChildArtifactRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApiEntitlementChildArtifactRowId'
);
const ApiEntitlementRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementRef');
const ApiEntitlementReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementReportRef');
const ApiEntitlementAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementAuditRef');
const ApiEntitlementProofRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementProofRef');
const ApiEntitlementClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApiEntitlementClaimBoundary');

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
  return summarizeAppInstallPurchaseApprovedApiEntitlementProofGenerated(proof);
}

function apiEntitlementEvidenceRow(
  row: (typeof AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts)[number]
) {
  return buildAppInstallPurchaseApprovedApiEntitlementEvidenceRowGenerated(
    row,
    SourceChildArtifactProofVersion,
    ApiEntitlementClaimBoundary,
    ApiEntitlementTimestamp
  );
}

function apiEntitlementEvidenceRowIsHonest(row: ApiEntitlementEvidenceRowCandidate): boolean {
  return apiEntitlementEvidenceRowIsHonestGenerated(row, ApiEntitlementBoundaryFragments);
}

function apiEntitlementProofIsHonest(proof: AppInstallPurchaseApprovedApiEntitlementProof): boolean {
  return (
    apiEntitlementProofIsHonestGenerated(
      proof,
      SourceChildArtifactProofVersion,
      RequiredPlatformSources,
      RequiredApprovalStatuses,
      ApiEntitlementNonClaims
    ) &&
    proof.evidenceRows.every(apiEntitlementEvidenceRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
