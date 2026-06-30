import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentReviewActionProofReadModel } from './app-install-purchase-parent-review-action-proof';
import { AppInstallPurchasePlatformAdapterBoundaryProofReadModel } from './app-install-purchase-platform-adapter-boundary-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseParentActionRuntimeHandoffRowGenerated,
  parentActionRuntimeHandoffProofIsHonestGenerated,
  parentActionRuntimeHandoffRowIsHonestGenerated,
  summarizeAppInstallPurchaseParentActionRuntimeHandoffProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const ParentActionRuntimeHandoffVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourceParentReviewActionProofVersion = AppInstallPurchaseParentReviewActionProofReadModel.schemaVersion;
const SourcePlatformAdapterBoundaryProofVersion = AppInstallPurchasePlatformAdapterBoundaryProofReadModel.schemaVersion;
const ParentActionRuntimeHandoffTimestamp = '2026-06-05T05:25:00.000Z';
const ParentActionRuntimeHandoffClaimBoundary =
  'parent action runtime handoff proof only; no portal approval UI no runtime action writer implementation no parent action runtime delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data not generic app blocking no Ocentra-hosted family data custody';
const RequiredHandoffActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredRuntimeHandoffStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredParentActionRuntimeHandoffNonClaims = [
  'no-portal-approval-ui',
  'no-runtime-action-writer-implementation',
  'no-parent-action-runtime-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'not-generic-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const RequiredParentActionRuntimeHandoffBoundaryFragments = [
  'no portal approval UI',
  'no runtime action writer implementation',
  'no parent action runtime delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'not generic app blocking',
  'no Ocentra-hosted family data custody',
] as const;
export const AppInstallPurchaseParentActionRuntimeHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(ParentActionRuntimeHandoffVersion)
);
const AppInstallPurchaseParentActionRuntimeHandoffStatusSchema = withParser(
  Schema.Literal(...RequiredRuntimeHandoffStatuses)
);
const AppInstallPurchaseParentActionRuntimeWriterClaimSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required')
);
const AppInstallPurchaseParentActionRuntimeDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseParentActionRuntimePortalClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentActionRuntimeProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseParentActionRuntimeStoreClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionRuntimeAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentActionRuntimeInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionRuntimeBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionRuntimeDataCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchaseParentActionRuntimeHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionRuntimeHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredParentActionRuntimeHandoffNonClaims)
);

const ParentActionRuntimeHandoffRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionRuntimeHandoffRowId'
);
const ParentActionRuntimeHandoffRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionRuntimeHandoffRef'
);
const ParentActionRuntimeHandoffAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionRuntimeHandoffAuditRef'
);
const ParentActionRuntimeHandoffClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionRuntimeHandoffClaimBoundary'
);

const ParentActionRuntimeHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionRuntimeHandoffProofSchemaVersionSchema,
  runtimeHandoffRowId: ParentActionRuntimeHandoffRowIdSchema,
  sourceParentReviewActionProofVersion: Schema.Literal(SourceParentReviewActionProofVersion),
  sourceParentReviewActionRowId: ParentActionRuntimeHandoffRefSchema,
  sourceDecisionAction: Schema.Literal(...RequiredHandoffActions),
  sourceRequestKind: ParentActionRuntimeHandoffRefSchema,
  resultingApprovalState: ParentActionRuntimeHandoffRefSchema,
  parentActionReferenceId: Schema.Union(ParentActionRuntimeHandoffRefSchema, Schema.Null),
  runtimeHandoffStatus: AppInstallPurchaseParentActionRuntimeHandoffStatusSchema,
  runtimeActionWriterClaim: AppInstallPurchaseParentActionRuntimeWriterClaimSchema,
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseParentActionRuntimeDeliveryClaimSchema,
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  platformAdapterBoundaryRefs: Schema.Array(ParentActionRuntimeHandoffRefSchema),
  auditEventRefs: Schema.Array(ParentActionRuntimeHandoffAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ParentActionRuntimeHandoffRefSchema),
  portalApprovalUiClaim: AppInstallPurchaseParentActionRuntimePortalClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseParentActionRuntimeProviderClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseParentActionRuntimeStoreClaimSchema,
  platformAdapterClaim: AppInstallPurchaseParentActionRuntimeAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchaseParentActionRuntimeDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseParentActionRuntimeDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseParentActionRuntimeInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseParentActionRuntimeBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseParentActionRuntimeDataCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseParentActionRuntimeHostedCustodyClaimSchema,
  claimBoundary: ParentActionRuntimeHandoffClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ParentActionRuntimeHandoffRowCandidate = Infer<typeof ParentActionRuntimeHandoffRowBaseSchema>;
export const AppInstallPurchaseParentActionRuntimeHandoffRowSchema = withParser(
  ParentActionRuntimeHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentActionRuntimeHandoffRowIsHonest(row) ||
        'Expected parent action runtime handoff rows to link review actions to runtime handoff status without delivery, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const ParentActionRuntimeHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionRuntimeHandoffProofSchemaVersionSchema,
  sourceParentReviewActionProofVersion: Schema.Literal(SourceParentReviewActionProofVersion),
  sourcePlatformAdapterBoundaryProofVersion: Schema.Literal(SourcePlatformAdapterBoundaryProofVersion),
  runtimeHandoffRows: Schema.Array(AppInstallPurchaseParentActionRuntimeHandoffRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseParentActionRuntimeHandoffNonClaimSchema),
  knownGaps: Schema.Array(ParentActionRuntimeHandoffRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseParentActionRuntimeHandoffProof = Infer<typeof ParentActionRuntimeHandoffProofBaseSchema>;
export const AppInstallPurchaseParentActionRuntimeHandoffProofSchema = withParser(
  ParentActionRuntimeHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        parentActionRuntimeHandoffProofIsHonest(proof) ||
        'Expected app install/purchase parent action runtime handoff proof to cover review actions and preserve runtime delivery non-claims'
    )
  )
);

export const AppInstallPurchaseParentActionRuntimeHandoffKnownGaps = [
  'Parent action runtime handoff rows are contract/readiness rows only; no runtime action writer or delivery worker is implemented.',
  'Portal approval UI, provider/store execution, platform adapters, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until a parent approval UI and runtime action delivery path exist.',
] as const;

export const AppInstallPurchaseParentActionRuntimeHandoffProofReadModel =
  AppInstallPurchaseParentActionRuntimeHandoffProofSchema.parse({
    schemaVersion: ParentActionRuntimeHandoffVersion,
    sourceParentReviewActionProofVersion: SourceParentReviewActionProofVersion,
    sourcePlatformAdapterBoundaryProofVersion: SourcePlatformAdapterBoundaryProofVersion,
    runtimeHandoffRows:
      AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows.map(parentActionRuntimeHandoffRow),
    nonClaims: RequiredParentActionRuntimeHandoffNonClaims,
    knownGaps: AppInstallPurchaseParentActionRuntimeHandoffKnownGaps,
    updatedAt: ParentActionRuntimeHandoffTimestamp,
  });

export function summarizeAppInstallPurchaseParentActionRuntimeHandoffProof(
  proof: AppInstallPurchaseParentActionRuntimeHandoffProof
) {
  return summarizeAppInstallPurchaseParentActionRuntimeHandoffProofGenerated(proof);
}

function parentActionRuntimeHandoffRow(
  row: (typeof AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows)[number]
) {
  return buildAppInstallPurchaseParentActionRuntimeHandoffRowGenerated(
    row,
    SourceParentReviewActionProofVersion,
    SourcePlatformAdapterBoundaryProofVersion,
    AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows.map(
      (adapterRow) => adapterRow.adapterBoundaryRowId
    ),
    ParentActionRuntimeHandoffClaimBoundary,
    ParentActionRuntimeHandoffTimestamp
  );
}

function parentActionRuntimeHandoffRowIsHonest(row: ParentActionRuntimeHandoffRowCandidate): boolean {
  return parentActionRuntimeHandoffRowIsHonestGenerated(
    row,
    SourceParentReviewActionProofVersion,
    SourcePlatformAdapterBoundaryProofVersion,
    AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows.length,
    RequiredParentActionRuntimeHandoffBoundaryFragments
  );
}

function parentActionRuntimeHandoffProofIsHonest(proof: AppInstallPurchaseParentActionRuntimeHandoffProof): boolean {
  return (
    parentActionRuntimeHandoffProofIsHonestGenerated(
      proof,
      SourceParentReviewActionProofVersion,
      SourcePlatformAdapterBoundaryProofVersion,
      RequiredHandoffActions,
      RequiredRuntimeHandoffStatuses,
      RequiredParentActionRuntimeHandoffNonClaims
    ) &&
    proof.runtimeHandoffRows.every((row) => parentActionRuntimeHandoffRowIsHonest(row)) &&
    proof.knownGaps.length > 0
  );
}
