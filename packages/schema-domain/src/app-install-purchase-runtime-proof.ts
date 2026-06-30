import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovalStateSnapshotSchema } from './app-install-purchase-approval';
import {
  AppInstallPurchaseApprovalContractProofReadModel,
  AppInstallPurchaseApprovalProofKnownGaps,
} from './app-install-purchase-approval-proof';
import { AppInstallPurchaseApprovalContractRuntime } from './generated/app-install-purchase-approval-contracts';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { appInstallPurchaseRuntimeProofIsHonest } from './app-install-purchase-runtime-proof-rules';
import {
  buildAppInstallPurchaseRuntimeChildDeliveryRowGenerated,
  buildAppInstallPurchaseRuntimePlatformArtifactRowGenerated,
  buildAppInstallPurchaseRuntimeReportIntegrationRowGenerated,
  buildAppInstallPurchaseRuntimeStatusReadinessRowGenerated,
  summarizeAppInstallPurchaseRuntimeProofGenerated,
} from './generated/app-install-purchase-proof-helpers';
const RuntimeSchemaVersion = 'app-install-purchase-runtime-proof';
const SourceApprovalContractProofVersion = AppInstallPurchaseApprovalContractRuntime.SchemaVersion;
const RuntimeBoundary =
  'runtime boundary proof only; no runtime status reader implementation no store integration no platform adapter no child-device delivery no runtime report delivery no real install or purchase interception not generic app blocking';
const RuntimeTimestamp = '2026-06-03T22:35:00.000Z';
const RuntimeNonClaims = [
  'no-store-integration',
  'no-billing-entitlement-logic',
  'no-runtime-status-reader-implementation',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-store-policy-bypass',
  'no-real-install-or-purchase-interception',
  'not-generic-app-blocking',
] as const;

export const AppInstallPurchaseRuntimeProofSchemaVersionSchema = withParser(Schema.Literal(RuntimeSchemaVersion));
const AppInstallPurchaseRuntimeStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseRuntimeRequestKindSchema = withParser(Schema.Literal('install', 'purchase', 'subscription'));
const AppInstallPurchaseRuntimeChildVisibleStatusSchema = withParser(
  Schema.Literal(
    'pending-parent-review-visible',
    'approved-visible',
    'denied-visible',
    'time-box-visible',
    'review-needed-visible'
  )
);
const AppInstallPurchaseRuntimeStoreMetadataArtifactStateSchema = withParser(
  Schema.Literal('requires-platform-artifact', 'platform-unavailable')
);
const AppInstallPurchaseRuntimePackageSourceArtifactStateSchema = withParser(
  Schema.Literal('requires-package-source-artifact', 'requires-device-proof-artifact', 'platform-unavailable')
);
const AppInstallPurchaseRuntimeDeliveryStateSchema = withParser(Schema.Literal('manual-required', 'unavailable'));
const AppInstallPurchaseRuntimeReportIntegrationStateSchema = withParser(
  Schema.Literal('contract-only', 'manual-required')
);
const AppInstallPurchaseRuntimeClaimStateSchema = withParser(Schema.Literal('boundary-only'));
const AppInstallPurchaseRuntimeStatusReadinessClaimSchema = withParser(Schema.Literal('runtime-status-readiness-only'));
const AppInstallPurchaseRuntimeStatusReaderClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseRuntimeDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseRuntimeStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimePlatformAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseRuntimeAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeReportSurfaceSchema = withParser(
  Schema.Literal(
    'request-audit-history',
    'parent-decision-audit-history',
    'child-facing-state-report',
    'platform-limitation-report'
  )
);
const AppInstallPurchaseRuntimeNonClaimSchema = withParser(Schema.Literal(...RuntimeNonClaims));
const RuntimeProofTextRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeProofTextRef');
const RuntimePlatformSourceRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimePlatformSourceRowId');
const RuntimePackageSourceArtifactRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimePackageSourceArtifactRowId'
);
const RuntimeChildStateIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeChildStateId');
const RuntimeRequestIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeRequestId');
const RuntimeAuditEventIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeAuditEventId');
const RuntimeReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeReportRef');
const RuntimeStatusReadinessRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeStatusReadinessRowId');
const RuntimeClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeClaimBoundary');

const AppInstallPurchaseRuntimePlatformArtifactRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeProofSchemaVersionSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseRuntimeStoreSurfaceSchema,
  platformSourceRowId: RuntimePlatformSourceRowIdSchema,
  packageSourceArtifactRowId: RuntimePackageSourceArtifactRowIdSchema,
  storeMetadataArtifactState: AppInstallPurchaseRuntimeStoreMetadataArtifactStateSchema,
  packageSourceArtifactState: AppInstallPurchaseRuntimePackageSourceArtifactStateSchema,
  childPendingDeliveryState: AppInstallPurchaseRuntimeDeliveryStateSchema,
  childResultDeliveryState: AppInstallPurchaseRuntimeDeliveryStateSchema,
  reportIntegrationState: AppInstallPurchaseRuntimeReportIntegrationStateSchema,
  runtimeClaimState: AppInstallPurchaseRuntimeClaimStateSchema,
  requiredProofRefs: Schema.Array(RuntimeProofTextRefSchema),
  reportRefs: Schema.Array(RuntimeReportRefSchema),
  claimBoundary: RuntimeClaimBoundarySchema,
});

const AppInstallPurchaseRuntimeChildDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeProofSchemaVersionSchema,
  childStateId: RuntimeChildStateIdSchema,
  requestId: RuntimeRequestIdSchema,
  requestKind: AppInstallPurchaseRuntimeRequestKindSchema,
  platform: ParentPlatformSchema,
  childVisibleStatus: AppInstallPurchaseRuntimeChildVisibleStatusSchema,
  sourceApprovalState: AppInstallPurchaseApprovalStateSnapshotSchema,
  deliveryState: AppInstallPurchaseRuntimeDeliveryStateSchema,
  runtimeDeliveryClaim: AppInstallPurchaseRuntimeDeliveryClaimSchema,
  auditEventRefs: Schema.Array(RuntimeAuditEventIdSchema),
  reportRefs: Schema.Array(RuntimeReportRefSchema),
  claimBoundary: RuntimeClaimBoundarySchema,
});

const AppInstallPurchaseRuntimeReportIntegrationRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeProofSchemaVersionSchema,
  surface: AppInstallPurchaseRuntimeReportSurfaceSchema,
  integrationState: AppInstallPurchaseRuntimeReportIntegrationStateSchema,
  runtimeReportClaim: AppInstallPurchaseRuntimeDeliveryClaimSchema,
  auditEventRefs: Schema.Array(RuntimeAuditEventIdSchema),
  reportRefs: Schema.Array(RuntimeReportRefSchema),
  claimBoundary: RuntimeClaimBoundarySchema,
});

const AppInstallPurchaseRuntimeStatusReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeProofSchemaVersionSchema,
  statusReadinessRowId: RuntimeStatusReadinessRowIdSchema,
  sourceChildStateId: RuntimeChildStateIdSchema,
  sourceRequestId: RuntimeRequestIdSchema,
  requestKind: AppInstallPurchaseRuntimeRequestKindSchema,
  platform: ParentPlatformSchema,
  childVisibleStatus: AppInstallPurchaseRuntimeChildVisibleStatusSchema,
  sourceApprovalState: AppInstallPurchaseApprovalStateSnapshotSchema,
  sourceDeliveryState: AppInstallPurchaseRuntimeDeliveryStateSchema,
  sourceRuntimeDeliveryClaim: AppInstallPurchaseRuntimeDeliveryClaimSchema,
  statusReadinessClaim: AppInstallPurchaseRuntimeStatusReadinessClaimSchema,
  runtimeStatusReaderClaim: AppInstallPurchaseRuntimeStatusReaderClaimSchema,
  childDeliveryClaim: AppInstallPurchaseRuntimeDeliveryClaimSchema,
  reportRuntimeDeliveryClaim: AppInstallPurchaseRuntimeDeliveryClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseRuntimeStoreIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseRuntimePlatformAdapterClaimSchema,
  appBlockingClaim: AppInstallPurchaseRuntimeAppBlockingClaimSchema,
  auditEventRefs: Schema.Array(RuntimeAuditEventIdSchema),
  reportRefs: Schema.Array(RuntimeReportRefSchema),
  claimBoundary: RuntimeClaimBoundarySchema,
});

const AppInstallPurchaseRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeProofSchemaVersionSchema,
  sourceContractSchemaVersion: Schema.Literal(SourceApprovalContractProofVersion),
  platformRuntimeArtifacts: Schema.Array(AppInstallPurchaseRuntimePlatformArtifactRowBaseSchema),
  childDeliveryBoundaries: Schema.Array(AppInstallPurchaseRuntimeChildDeliveryRowBaseSchema),
  reportIntegrationBoundaries: Schema.Array(AppInstallPurchaseRuntimeReportIntegrationRowBaseSchema),
  statusReadinessBoundaries: Schema.Array(AppInstallPurchaseRuntimeStatusReadinessRowBaseSchema),
  nonClaims: Schema.Array(AppInstallPurchaseRuntimeNonClaimSchema),
  knownGaps: Schema.Array(RuntimeProofTextRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeProof = Infer<typeof AppInstallPurchaseRuntimeProofBaseSchema>;

export const AppInstallPurchaseRuntimeProofSchema = withParser(
  AppInstallPurchaseRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        appInstallPurchaseRuntimeProofIsHonest(proof) ||
        'Expected app install/purchase runtime proof to stay boundary-only with artifact, child-delivery, and report non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeProofKnownGaps = [
  ...AppInstallPurchaseApprovalProofKnownGaps,
  'Runtime proof links source metadata, package-source artifacts, child delivery, and report integration as boundary rows only.',
  'Runtime status readiness rows expose child-facing status coverage only; no runtime status reader, delivery worker, platform adapter, or app blocking behavior is implemented.',
  'No runtime child-device delivery, report UI delivery, provider store API, platform adapter, or app blocking behavior is implemented by this proof.',
] as const;

export const AppInstallPurchaseRuntimeProofReadModel = AppInstallPurchaseRuntimeProofSchema.parse({
  schemaVersion: RuntimeSchemaVersion,
  sourceContractSchemaVersion: SourceApprovalContractProofVersion,
  platformRuntimeArtifacts:
    AppInstallPurchaseApprovalContractProofReadModel.platformSourceMetadata.map(runtimePlatformArtifactRow),
  childDeliveryBoundaries: AppInstallPurchaseApprovalContractProofReadModel.childFacingStates.map(childDeliveryRow),
  reportIntegrationBoundaries:
    AppInstallPurchaseApprovalContractProofReadModel.auditReportIntegration.map(reportIntegrationRow),
  statusReadinessBoundaries: AppInstallPurchaseApprovalContractProofReadModel.childFacingStates.map(statusReadinessRow),
  nonClaims: RuntimeNonClaims,
  knownGaps: AppInstallPurchaseRuntimeProofKnownGaps,
  updatedAt: RuntimeTimestamp,
});

export function summarizeAppInstallPurchaseRuntimeProof(proof: AppInstallPurchaseRuntimeProof) {
  return summarizeAppInstallPurchaseRuntimeProofGenerated(proof);
}

function runtimePlatformArtifactRow(
  sourceRow: (typeof AppInstallPurchaseApprovalContractProofReadModel.platformSourceMetadata)[number]
) {
  return buildAppInstallPurchaseRuntimePlatformArtifactRowGenerated(
    sourceRow,
    AppInstallPurchaseApprovalContractProofReadModel.packageSourceArtifacts,
    RuntimeSchemaVersion,
    RuntimeBoundary
  );
}

function childDeliveryRow(state: (typeof AppInstallPurchaseApprovalContractProofReadModel.childFacingStates)[number]) {
  return buildAppInstallPurchaseRuntimeChildDeliveryRowGenerated(state, RuntimeSchemaVersion, RuntimeBoundary);
}

function reportIntegrationRow(
  row: (typeof AppInstallPurchaseApprovalContractProofReadModel.auditReportIntegration)[number]
) {
  return buildAppInstallPurchaseRuntimeReportIntegrationRowGenerated(row, RuntimeSchemaVersion, RuntimeBoundary);
}

function statusReadinessRow(
  state: (typeof AppInstallPurchaseApprovalContractProofReadModel.childFacingStates)[number]
) {
  return buildAppInstallPurchaseRuntimeStatusReadinessRowGenerated(state, RuntimeSchemaVersion, RuntimeBoundary);
}
