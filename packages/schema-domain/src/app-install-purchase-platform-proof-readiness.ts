import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseLimitationSummaryProofReadModel } from './app-install-purchase-limitation-summary-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchasePlatformProofReadinessRowGenerated,
  platformProofReadinessProofIsHonestGenerated,
  platformProofReadinessRowIsHonestGenerated,
  summarizeAppInstallPurchasePlatformProofReadinessGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';

const ProofVersion = 'app-install-purchase-platform-proof-readiness';
const SourceProofVersion = 'app-install-purchase-limitation-summary-proof';
const CheckedAt = '2026-06-06T04:32:00.000Z';
const Platforms = ['windows', 'macos', 'linux', 'android', 'ios'] as const;
const ReadinessStates = ['manual-proof-required', 'policy-blocked', 'unavailable'] as const;
const NonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'platform proof readiness only; names required manual evidence before app install product claims no Google Play execution no Apple App Store execution no Microsoft Store execution no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'manual evidence before app install product claims',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchasePlatformProofReadinessSchemaVersionSchema = withParser(Schema.Literal(ProofVersion));
const PlatformSchema = withParser(Schema.Literal(...Platforms));
const ReadinessStateSchema = withParser(Schema.Literal(...ReadinessStates));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformProofReadinessRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchasePlatformProofReadinessBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformProofReadinessSchemaVersionSchema,
  platform: PlatformSchema,
  platformProofReadinessState: ReadinessStateSchema,
  sourceLimitationSummaryProofVersion: Schema.Literal(SourceProofVersion),
  sourceLimitationSummaryRowIds: Schema.Array(RefSchema),
  requiredManualEvidenceRefs: Schema.Array(RefSchema),
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  checkedAt: ParentTimestampSchema,
});

type RowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchasePlatformProofReadinessRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        rowIsHonest(row) ||
        'Expected app install/purchase platform proof readiness rows to require real platform evidence without provider, store, adapter, delivery, custody, or blocking claims'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchasePlatformProofReadinessSchemaVersionSchema,
  sourceLimitationSummaryProofVersion: Schema.Literal(SourceProofVersion),
  platformProofReadinessRows: Schema.Array(AppInstallPurchasePlatformProofReadinessRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchasePlatformProofReadinessProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchasePlatformProofReadinessProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        proofIsHonest(proof) ||
        'Expected app install/purchase platform proof readiness proof to cover every target platform and keep real execution unclaimed'
    )
  )
);

export const AppInstallPurchasePlatformProofReadinessKnownGaps = [
  'Platform proof readiness rows name evidence required before product claims; no store/provider execution is implemented.',
  'Windows package-source capture can be proof-ready only after host/package evidence and guarded adapter proof are attached.',
  'macOS, Android, and iOS remain manual or policy-blocked until signing, entitlement, managed-profile, or review evidence exists.',
  'Linux remains unavailable for app-install package-source support until a tested package-manager source path is proved.',
] as const;

export const AppInstallPurchasePlatformProofReadinessProofReadModel =
  AppInstallPurchasePlatformProofReadinessProofSchema.parse({
    schemaVersion: ProofVersion,
    sourceLimitationSummaryProofVersion: SourceProofVersion,
    platformProofReadinessRows: Platforms.map(platformReadinessRow),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchasePlatformProofReadinessKnownGaps,
    updatedAt: CheckedAt,
  });

export function summarizeAppInstallPurchasePlatformProofReadiness(
  proof: AppInstallPurchasePlatformProofReadinessProof
) {
  return summarizeAppInstallPurchasePlatformProofReadinessGenerated(proof);
}

function platformReadinessRow(platform: (typeof Platforms)[number]) {
  return buildAppInstallPurchasePlatformProofReadinessRowGenerated(
    platform,
    SourceProofVersion,
    AppInstallPurchaseLimitationSummaryProofReadModel.limitationSummaryRows.map((row) => row.limitationSummaryRowId),
    Boundary,
    CheckedAt
  );
}

function rowIsHonest(row: RowCandidate): boolean {
  return (
    row.sourceLimitationSummaryProofVersion === SourceProofVersion &&
    platformProofReadinessRowIsHonestGenerated(
      row,
      AppInstallPurchaseLimitationSummaryProofReadModel.limitationSummaryRows.length,
      BoundaryFragments
    )
  );
}

function proofIsHonest(proof: AppInstallPurchasePlatformProofReadinessProof): boolean {
  return (
    platformProofReadinessProofIsHonestGenerated(proof, SourceProofVersion, Platforms, NonClaims) &&
    proof.platformProofReadinessRows.every(rowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
