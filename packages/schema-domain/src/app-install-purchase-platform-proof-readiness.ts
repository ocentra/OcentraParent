import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseLimitationSummaryProofReadModel } from './app-install-purchase-limitation-summary-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

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
  return {
    platformRows: proof.platformProofReadinessRows.length,
    manualProofRequiredRows: proof.platformProofReadinessRows.filter(
      (row) => row.platformProofReadinessState === 'manual-proof-required'
    ).length,
    policyBlockedRows: proof.platformProofReadinessRows.filter(
      (row) => row.platformProofReadinessState === 'policy-blocked'
    ).length,
    unavailableRows: proof.platformProofReadinessRows.filter((row) => row.platformProofReadinessState === 'unavailable')
      .length,
    providerExecutedRows: proof.platformProofReadinessRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    adapterImplementedRows: proof.platformProofReadinessRows.filter(
      (row) => row.platformAdapterClaim !== 'not-implemented'
    ).length,
  } as const;
}

function platformReadinessRow(platform: (typeof Platforms)[number]) {
  return {
    schemaVersion: ProofVersion,
    platform,
    platformProofReadinessState: platformReadinessState(platform),
    sourceLimitationSummaryProofVersion: SourceProofVersion,
    sourceLimitationSummaryRowIds: AppInstallPurchaseLimitationSummaryProofReadModel.limitationSummaryRows.map(
      (row) => row.limitationSummaryRowId
    ),
    requiredManualEvidenceRefs: requiredEvidenceRefs(platform),
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: Boundary,
    checkedAt: CheckedAt,
  } as const;
}

function platformReadinessState(platform: (typeof Platforms)[number]): (typeof ReadinessStates)[number] {
  if (platform === 'linux') {
    return 'unavailable';
  }
  if (platform === 'android' || platform === 'ios') {
    return 'policy-blocked';
  }
  return 'manual-proof-required';
}

function requiredEvidenceRefs(platform: (typeof Platforms)[number]) {
  const refs = {
    windows: ['windows-host-package-source-proof', 'windows-guarded-adapter-proof'],
    macos: ['macos-signing-receipt-proof', 'macos-store-source-manual-proof'],
    linux: ['linux-package-manager-source-path-proof'],
    android: ['android-device-owner-or-managed-profile-proof', 'google-play-policy-review-proof'],
    ios: ['ios-family-controls-entitlement-proof', 'apple-review-proof'],
  } as const;
  return refs[platform];
}

function rowIsHonest(row: RowCandidate): boolean {
  return (
    row.sourceLimitationSummaryProofVersion === SourceProofVersion &&
    row.sourceLimitationSummaryRowIds.length ===
      AppInstallPurchaseLimitationSummaryProofReadModel.limitationSummaryRows.length &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    BoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function proofIsHonest(proof: AppInstallPurchasePlatformProofReadinessProof): boolean {
  const platforms = new Set(proof.platformProofReadinessRows.map((row) => row.platform));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceLimitationSummaryProofVersion === SourceProofVersion &&
    proof.platformProofReadinessRows.length === Platforms.length &&
    Platforms.every((platform) => platforms.has(platform)) &&
    NonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.platformProofReadinessRows.every(rowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
