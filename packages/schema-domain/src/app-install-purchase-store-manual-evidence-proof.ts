import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePlatformProofReadinessProofReadModel } from './app-install-purchase-platform-proof-readiness';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseStoreManualEvidenceRowGenerated,
  storeManualEvidenceProofIsHonestGenerated,
  storeManualEvidenceRowIsHonestGenerated,
  summarizeAppInstallPurchaseStoreManualEvidenceGenerated,
} from './generated/app-install-purchase-platform-provider-helpers';

const ProofVersion = 'app-install-purchase-store-manual-evidence-proof';
const SourceProofVersion = 'app-install-purchase-platform-proof-readiness';
const CheckedAt = '2026-06-06T08:24:00.000Z';
const Platforms = ['windows', 'macos', 'linux', 'android', 'ios'] as const;
const StoreSurfaces = [
  'microsoft-store',
  'mac-app-store',
  'linux-package-manager',
  'google-play',
  'apple-app-store',
] as const;
const StoreManualEvidenceStates = [
  'manual-evidence-required',
  'store-policy-review-required',
  'store-unavailable',
] as const;
const NonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-writer-delivery',
  'no-runtime-report-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const Boundary =
  'store manual evidence proof only; translates platform proof readiness into manual evidence requirements before app install product claims no Google Play execution no Apple App Store execution no Microsoft Store execution no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime writer delivery no runtime report delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const BoundaryFragments = [
  'manual evidence requirements before app install product claims',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime writer delivery',
  'no runtime report delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseStoreManualEvidenceProofSchemaVersionSchema = withParser(Schema.Literal(ProofVersion));
const PlatformSchema = withParser(Schema.Literal(...Platforms));
const StoreSurfaceSchema = withParser(Schema.Literal(...StoreSurfaces));
const StoreManualEvidenceStateSchema = withParser(Schema.Literal(...StoreManualEvidenceStates));
const NonClaimSchema = withParser(Schema.Literal(...NonClaims));
const RefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseStoreManualEvidenceRef');
const BoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseStoreManualEvidenceBoundary');
const NotExecutedSchema = withParser(Schema.Literal('not-executed'));
const NotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const NotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const NotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const CustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreManualEvidenceProofSchemaVersionSchema,
  platform: PlatformSchema,
  storeSurface: StoreSurfaceSchema,
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourceProofVersion),
  sourcePlatformProofReadinessState: Schema.Literal('manual-proof-required', 'policy-blocked', 'unavailable'),
  sourceManualEvidenceRefs: Schema.Array(RefSchema),
  storeManualEvidenceState: StoreManualEvidenceStateSchema,
  providerApiExecutionClaim: NotExecutedSchema,
  storeIntegrationClaim: NotClaimedSchema,
  platformAdapterClaim: NotImplementedSchema,
  childDeviceDeliveryClaim: NotDeliveredSchema,
  runtimeWriterDeliveryClaim: NotDeliveredSchema,
  runtimeReportDeliveryClaim: NotDeliveredSchema,
  appBlockingClaim: NotClaimedSchema,
  childDataCustody: CustodySchema,
  ocentraHostedFamilyDataCustodyClaim: NotClaimedSchema,
  claimBoundary: BoundarySchema,
  checkedAt: ParentTimestampSchema,
});

type RowCandidate = Infer<typeof RowBaseSchema>;

export const AppInstallPurchaseStoreManualEvidenceRowSchema = withParser(
  RowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        rowIsHonest(row) ||
        'Expected app install/purchase store manual evidence rows to keep store execution, adapter, delivery, custody, and blocking claims out of manual evidence requirements'
    )
  )
);

const ProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreManualEvidenceProofSchemaVersionSchema,
  sourcePlatformProofReadinessProofVersion: Schema.Literal(SourceProofVersion),
  storeManualEvidenceRows: Schema.Array(AppInstallPurchaseStoreManualEvidenceRowSchema),
  nonClaims: Schema.Array(NonClaimSchema),
  knownGaps: Schema.Array(RefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseStoreManualEvidenceProof = Infer<typeof ProofBaseSchema>;

export const AppInstallPurchaseStoreManualEvidenceProofSchema = withParser(
  ProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        proofIsHonest(proof) ||
        'Expected app install/purchase store manual evidence proof to cover every target store and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseStoreManualEvidenceKnownGaps = [
  'Store manual evidence rows name evidence requirements only; no Google Play Apple App Store Microsoft Store or billing provider execution is implemented.',
  'Manual evidence refs remain parent-owned proof requirements until real store credentials entitlements platform adapters and review evidence exist.',
  'Runtime writer delivery child-device delivery app blocking child activity data and hosted custody remain unimplemented.',
] as const;

export const AppInstallPurchaseStoreManualEvidenceProofReadModel =
  AppInstallPurchaseStoreManualEvidenceProofSchema.parse({
    schemaVersion: ProofVersion,
    sourcePlatformProofReadinessProofVersion: SourceProofVersion,
    storeManualEvidenceRows: AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows.map(
      (row) => storeManualEvidenceRow(row)
    ),
    nonClaims: NonClaims,
    knownGaps: AppInstallPurchaseStoreManualEvidenceKnownGaps,
    updatedAt: CheckedAt,
  });

export function summarizeAppInstallPurchaseStoreManualEvidence(proof: AppInstallPurchaseStoreManualEvidenceProof) {
  return summarizeAppInstallPurchaseStoreManualEvidenceGenerated(proof);
}

function storeManualEvidenceRow(
  sourceRow: (typeof AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows)[number]
) {
  return buildAppInstallPurchaseStoreManualEvidenceRowGenerated(
    sourceRow,
    SourceProofVersion,
    Boundary,
    CheckedAt
  );
}

function rowIsHonest(row: RowCandidate): boolean {
  return storeManualEvidenceRowIsHonestGenerated(row, SourceProofVersion, BoundaryFragments);
}

function proofIsHonest(proof: AppInstallPurchaseStoreManualEvidenceProof): boolean {
  return (
    storeManualEvidenceProofIsHonestGenerated(
      proof,
      SourceProofVersion,
      Platforms,
      StoreSurfaces,
      NonClaims
    ) &&
    proof.storeManualEvidenceRows.every(rowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
