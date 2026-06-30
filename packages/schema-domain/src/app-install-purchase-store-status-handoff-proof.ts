import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentActionRuntimeHandoffProofReadModel } from './app-install-purchase-parent-action-runtime-handoff-proof';
import { AppInstallPurchasePlatformAdapterBoundaryProofReadModel } from './app-install-purchase-platform-adapter-boundary-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseStoreStatusHandoffRowGenerated,
  storeStatusHandoffProofIsHonestGenerated,
  storeStatusHandoffRowIsHonestGenerated,
  summarizeAppInstallPurchaseStoreStatusHandoffProofGenerated,
} from './generated/app-install-purchase-report-status-helpers';
const StoreStatusHandoffProofVersion = 'app-install-purchase-store-status-handoff-proof';
const SourceParentActionRuntimeHandoffProofVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourcePlatformAdapterBoundaryProofVersion = 'app-install-purchase-platform-adapter-boundary-proof';
const StoreStatusHandoffTimestamp = '2026-06-05T05:30:00.000Z';
const StoreStatusHandoffClaimBoundary =
  'store status handoff proof only; no provider API execution no store integration no platform adapter implementation no parent action runtime delivery no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const RequiredParentActionRuntimeStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredStoreStatusHandoffStates = [
  'approved-api-status-proof-required',
  'store-entitlement-status-proof-required',
  'manual-platform-status-review-required',
  'platform-store-status-unavailable',
] as const;
const StoreStatusHandoffNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-parent-action-runtime-delivery',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(StoreStatusHandoffProofVersion)
);
const AppInstallPurchaseStoreStatusHandoffStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseStoreStatusHandoffStateSchema = withParser(Schema.Literal(...RequiredStoreStatusHandoffStates));
const AppInstallPurchaseStoreStatusRuntimeStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'unavailable')
);
const AppInstallPurchaseStoreStatusAdapterEvidenceStateSchema = withParser(
  Schema.Literal(
    'approved-api-adapter-evidence-required',
    'entitlement-adapter-evidence-required',
    'manual-platform-review-required',
    'platform-unavailable'
  )
);
const AppInstallPurchaseStoreStatusParentActionRuntimeStatusSchema = withParser(
  Schema.Literal(...RequiredParentActionRuntimeStatuses)
);
const AppInstallPurchaseStoreStatusClaimSchema = withParser(Schema.Literal('status-handoff-proof-only'));
const AppInstallPurchaseStoreStatusDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseStoreStatusProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseStoreStatusIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseStoreStatusInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseStoreStatusHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseStoreStatusNonClaimSchema = withParser(Schema.Literal(...StoreStatusHandoffNonClaims));

const StoreStatusHandoffRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseStoreStatusHandoffRowId');
const StoreStatusHandoffSourceRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseStoreStatusHandoffSourceRowId'
);
const StoreStatusHandoffRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseStoreStatusHandoffRef');
const StoreStatusHandoffReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseStoreStatusHandoffReportRef');
const StoreStatusHandoffClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseStoreStatusHandoffClaimBoundary'
);

const StoreStatusHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema,
  storeStatusHandoffRowId: StoreStatusHandoffRowIdSchema,
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  sourcePlatformAdapterBoundaryRowId: StoreStatusHandoffSourceRowIdSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceParentActionRuntimeHandoffRefs: Schema.Array(StoreStatusHandoffRefSchema),
  sourceParentActionRuntimeStatuses: Schema.Array(AppInstallPurchaseStoreStatusParentActionRuntimeStatusSchema),
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseStoreStatusHandoffStoreSurfaceSchema,
  sourceAdapterEvidenceState: AppInstallPurchaseStoreStatusAdapterEvidenceStateSchema,
  sourceAdapterRuntimeState: AppInstallPurchaseStoreStatusRuntimeStateSchema,
  storeStatusHandoffState: AppInstallPurchaseStoreStatusHandoffStateSchema,
  storeStatusRuntimeState: AppInstallPurchaseStoreStatusRuntimeStateSchema,
  storeStatusHandoffEvidenceRefs: Schema.Array(StoreStatusHandoffRefSchema),
  sourceReportRuntimeRefs: Schema.Array(StoreStatusHandoffReportRefSchema),
  storeStatusHandoffClaim: AppInstallPurchaseStoreStatusClaimSchema,
  statusHandoffDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseStoreStatusProviderClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseStoreStatusIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseStoreStatusAdapterClaimSchema,
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  childDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseStoreStatusDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseStoreStatusInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseStoreStatusBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseStoreStatusCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseStoreStatusHostedCustodyClaimSchema,
  claimBoundary: StoreStatusHandoffClaimBoundarySchema,
  handedOffAt: ParentTimestampSchema,
});

type StoreStatusHandoffRowCandidate = Infer<typeof StoreStatusHandoffRowBaseSchema>;

export const AppInstallPurchaseStoreStatusHandoffRowSchema = withParser(
  StoreStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        storeStatusHandoffRowIsHonest(row) ||
        'Expected app install/purchase store status handoff rows to link adapter and parent action runtime evidence without provider, store, adapter, delivery, custody, interception, or blocking claims'
    )
  )
);

const StoreStatusHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseStoreStatusHandoffProofSchemaVersionSchema,
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  storeStatusHandoffRows: Schema.Array(AppInstallPurchaseStoreStatusHandoffRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseStoreStatusNonClaimSchema),
  knownGaps: Schema.Array(StoreStatusHandoffRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseStoreStatusHandoffProof = Infer<typeof StoreStatusHandoffProofBaseSchema>;

export const AppInstallPurchaseStoreStatusHandoffProofSchema = withParser(
  StoreStatusHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        storeStatusHandoffProofIsHonest(proof) ||
        'Expected app install/purchase store status handoff proof to cover platform sources and preserve runtime non-claims'
    )
  )
);

export const AppInstallPurchaseStoreStatusHandoffKnownGaps = [
  'Store status handoff rows are status proof only; no provider/store status API execution is implemented.',
  'Platform adapter rows remain readiness/manual/unavailable boundaries only and do not deliver parent actions to store or child-device runtimes.',
  'Portal approval UX, parent action runtime delivery, child-device delivery, report writer/delivery, real interception, app blocking, and Ocentra-hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchaseStoreStatusHandoffProofReadModel = AppInstallPurchaseStoreStatusHandoffProofSchema.parse(
  {
    schemaVersion: StoreStatusHandoffProofVersion,
    sourcePlatformAdapterBoundaryProofVersion: SourcePlatformAdapterBoundaryProofVersion,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    storeStatusHandoffRows:
      AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows.map(storeStatusHandoffRow),
    nonClaims: StoreStatusHandoffNonClaims,
    knownGaps: AppInstallPurchaseStoreStatusHandoffKnownGaps,
    updatedAt: StoreStatusHandoffTimestamp,
  }
);

export function summarizeAppInstallPurchaseStoreStatusHandoffProof(proof: AppInstallPurchaseStoreStatusHandoffProof) {
  return summarizeAppInstallPurchaseStoreStatusHandoffProofGenerated(proof);
}

function storeStatusHandoffRow(
  row: (typeof AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows)[number]
) {
  return buildAppInstallPurchaseStoreStatusHandoffRowGenerated(
    row,
    AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows,
    SourcePlatformAdapterBoundaryProofVersion,
    SourceParentActionRuntimeHandoffProofVersion,
    StoreStatusHandoffClaimBoundary,
    StoreStatusHandoffTimestamp
  );
}

function storeStatusHandoffRowIsHonest(row: StoreStatusHandoffRowCandidate): boolean {
  return storeStatusHandoffRowIsHonestGenerated(
    row,
    AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.length,
    RequiredParentActionRuntimeStatuses,
    [
      'no provider API execution',
      'no store integration',
      'no platform adapter implementation',
      'no parent action runtime delivery',
      'no child-device delivery',
      'no runtime report delivery',
      'no real install or purchase interception',
      'no child activity data',
      'no app blocking',
      'no Ocentra-hosted family data custody',
    ]
  );
}

function storeStatusHandoffProofIsHonest(proof: AppInstallPurchaseStoreStatusHandoffProof): boolean {
  return (
    storeStatusHandoffProofIsHonestGenerated(
      proof,
      RequiredPlatformSources,
      RequiredStoreStatusHandoffStates,
      StoreStatusHandoffNonClaims
    ) && proof.storeStatusHandoffRows.every(storeStatusHandoffRowIsHonest)
  );
}
