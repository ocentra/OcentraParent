/* thin adapter over Rust-generated app-install purchase approval contracts */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  appInstallPurchaseApprovalContractProofIsHonest,
  auditReportIntegrationIsHonest,
  approvalDecisionIsConsistent,
  approvalStateSnapshotIsConsistent,
  childFacingStateIsConsistent,
  platformSupportRowIsHonest,
  purchaseRequestKindIsConsistent,
  storeMetadataFreshnessIsConsistent,
} from './app-install-purchase-approval-rules';
import {
  AppInstallPurchaseApprovalContractRuntime,
  GeneratedAppInstallPurchaseApprovalAuditEventKinds,
  GeneratedAppInstallPurchaseApprovalAuditReportSurfaces,
  GeneratedAppInstallPurchaseApprovalBillingEntitlementClaims,
  GeneratedAppInstallPurchaseApprovalChildFacingStatuses,
  GeneratedAppInstallPurchaseApprovalDecisionActions,
  GeneratedAppInstallPurchaseApprovalExpiryStates,
  GeneratedAppInstallPurchaseApprovalInterceptionClaims,
  GeneratedAppInstallPurchaseApprovalNonClaims,
  GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims,
  GeneratedAppInstallPurchaseApprovalPortalUiClaims,
  GeneratedAppInstallPurchaseApprovalProofIntegrationStates,
  GeneratedAppInstallPurchaseApprovalPurchaseKinds,
  GeneratedAppInstallPurchaseApprovalRequestKinds,
  GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparations,
  GeneratedAppInstallPurchaseApprovalStates,
  GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims,
  GeneratedAppInstallPurchaseApprovalStoreMetadataFreshnessStates,
  GeneratedAppInstallPurchaseApprovalStoreSurfaces,
  GeneratedAppInstallPurchaseApprovalSubscriptionPeriods,
  GeneratedAppInstallPurchaseApprovalSupportStates,
  type GeneratedAppInstallPurchaseApprovalContractProof,
  type GeneratedAppInstallPurchaseApprovalDecision,
  type GeneratedAppInstallPurchaseApprovalDecisionAction,
  type GeneratedAppInstallPurchaseApprovalNonClaim,
  type GeneratedAppInstallPurchaseApprovalPlatformSupportRow,
  type GeneratedAppInstallPurchaseApprovalRequestKind,
  type GeneratedAppInstallPurchaseApprovalState,
  type GeneratedAppInstallPurchaseApprovalStoreMetadata,
  type GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness,
  type GeneratedAppInstallPurchaseApprovalStoreSurface,
  type GeneratedAppInstallPurchaseApprovalSupportState,
  type GeneratedAppInstallRequest,
  type GeneratedPurchaseRequest,
  type GeneratedAppInstallPurchaseApprovalStateSnapshot,
} from './generated/app-install-purchase-approval-contracts';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import { AppInstallPurchaseApprovalPackageSourceArtifactRowSchema } from './app-install-purchase-approval-package-sources';
import { AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema } from './app-install-purchase-approval-platform-sources';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppInstallPurchaseApprovalSchemaVersionSchema = withParser(
  Schema.Literal(AppInstallPurchaseApprovalContractRuntime.SchemaVersion)
);
export const AppInstallPurchaseApprovalRequestKindSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalRequestKinds)
);
export const AppInstallPurchaseApprovalStoreSurfaceSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalStoreSurfaces)
);
export const AppInstallPurchaseApprovalStoreMetadataFreshnessSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalStoreMetadataFreshnessStates)
);
export const AppInstallPurchaseApprovalMetadataSourceStateSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalSupportStates)
);
export const AppInstallPurchaseApprovalDecisionActionSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalDecisionActions)
);
export const AppInstallPurchaseApprovalStateSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalStates)
);
export const AppInstallPurchaseApprovalExpiryStateSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalExpiryStates)
);
export const AppInstallPurchaseApprovalPlatformSupportStateSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalSupportStates)
);
export const AppInstallPurchaseApprovalPurchaseKindSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalPurchaseKinds)
);
export const AppInstallPurchaseApprovalSubscriptionPeriodSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalSubscriptionPeriods)
);
const AppInstallPurchaseApprovalChildFacingStatusSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalChildFacingStatuses)
);
const AppInstallPurchaseApprovalAuditReportSurfaceSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalAuditReportSurfaces)
);
const AppInstallPurchaseApprovalProofIntegrationStateSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalProofIntegrationStates)
);
export const AppInstallPurchaseApprovalAuditEventKindSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalAuditEventKinds)
);
export const AppInstallPurchaseApprovalNonClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalNonClaims)
);
export const AppInstallPurchaseApprovalStoreIntegrationClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalStoreIntegrationClaims)
);
export const AppInstallPurchaseApprovalBillingEntitlementClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalBillingEntitlementClaims)
);
export const AppInstallPurchaseApprovalPortalUiClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalPortalUiClaims)
);
export const AppInstallPurchaseApprovalPlatformAdapterClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalPlatformAdapterClaims)
);
export const AppInstallPurchaseApprovalInterceptionClaimSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalInterceptionClaims)
);
export const AppInstallPurchaseApprovalRuntimeBlockingSeparationSchema = withParser(
  Schema.Literal(...GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparations)
);

const AppInstallPurchaseApprovalRequestIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalRequestId');
const AppInstallPurchaseApprovalDecisionIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalDecisionId');
const AppInstallPurchaseApprovalAuditEventIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalAuditEventId'
);
const AppInstallPurchaseApprovalStoreListingIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalStoreListingId'
);
const AppInstallPurchaseApprovalAppTitleSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalAppTitle');
const AppInstallPurchaseApprovalPublisherNameSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPublisherName'
);
const AppInstallPurchaseApprovalCategorySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalCategory');
const AppInstallPurchaseApprovalAgeRatingSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalAgeRating');
const AppInstallPurchaseApprovalReviewReasonSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalReviewReason'
);
const AppInstallPurchaseApprovalProofRequirementSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalProofRequirement'
);
const AppInstallPurchaseApprovalUnavailableReasonSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalUnavailableReason'
);
const AppInstallPurchaseApprovalManualRequirementSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalManualRequirement'
);
const AppInstallPurchaseApprovalClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalClaimBoundary'
);
const AppInstallPurchaseApprovalPriceDisplaySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalPriceDisplay'
);
const AppInstallPurchaseApprovalChildStateIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseApprovalChildStateId'
);
const AppInstallPurchaseApprovalReportRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalReportRef');

export const AppInstallPurchaseApprovalAuditEventRefSchema = withParser(
  Schema.Struct({
    auditEventId: AppInstallPurchaseApprovalAuditEventIdSchema,
    eventKind: AppInstallPurchaseApprovalAuditEventKindSchema,
    recordedAt: ParentTimestampSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  })
);

const AppInstallPurchaseApprovalStoreMetadataBaseSchema = Schema.Struct({
  storeSurface: AppInstallPurchaseApprovalStoreSurfaceSchema,
  sourceState: AppInstallPurchaseApprovalMetadataSourceStateSchema,
  freshness: AppInstallPurchaseApprovalStoreMetadataFreshnessSchema,
  listingId: Schema.Union(AppInstallPurchaseApprovalStoreListingIdSchema, Schema.Null),
  appTitle: Schema.Union(AppInstallPurchaseApprovalAppTitleSchema, Schema.Null),
  publisherName: Schema.Union(AppInstallPurchaseApprovalPublisherNameSchema, Schema.Null),
  category: Schema.Union(AppInstallPurchaseApprovalCategorySchema, Schema.Null),
  ageRating: Schema.Union(AppInstallPurchaseApprovalAgeRatingSchema, Schema.Null),
  refreshedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  staleAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  proofRequirement: AppInstallPurchaseApprovalProofRequirementSchema,
});

export const AppInstallPurchaseApprovalStoreMetadataSchema = withParser(
  AppInstallPurchaseApprovalStoreMetadataBaseSchema.pipe(
    Schema.filter(
      (metadata) =>
        storeMetadataFreshnessIsConsistent(metadata) ||
        'Expected store metadata freshness to match source support, refresh timestamp, and stale timestamp'
    )
  )
);

const AppInstallPurchaseApprovalStateSnapshotBaseSchema = Schema.Struct({
  state: AppInstallPurchaseApprovalStateSchema,
  expiryState: AppInstallPurchaseApprovalExpiryStateSchema,
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  reviewReason: Schema.Union(AppInstallPurchaseApprovalReviewReasonSchema, Schema.Null),
});

export const AppInstallPurchaseApprovalStateSnapshotSchema = withParser(
  AppInstallPurchaseApprovalStateSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        approvalStateSnapshotIsConsistent(snapshot) ||
        'Expected approval state snapshot to model expiry, time-box, and review-needed states explicitly'
    )
  )
);

const AppInstallPurchaseApprovalRequestFields = {
  schemaVersion: AppInstallPurchaseApprovalSchemaVersionSchema,
  requestId: AppInstallPurchaseApprovalRequestIdSchema,
  requestKind: AppInstallPurchaseApprovalRequestKindSchema,
  family: FamilyReferenceSchema,
  child: ChildProfileReferenceSchema,
  device: ParentDeviceReferenceSchema,
  platform: ParentPlatformSchema,
  storeMetadata: AppInstallPurchaseApprovalStoreMetadataSchema,
  approvalState: AppInstallPurchaseApprovalStateSnapshotSchema,
  requestedAt: ParentTimestampSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  auditEventRefs: Schema.Array(AppInstallPurchaseApprovalAuditEventRefSchema),
} as const;

const AppInstallPurchaseApprovalRequestBaseSchema = Schema.Struct(AppInstallPurchaseApprovalRequestFields);

export const AppInstallRequestSchema = withParser(
  AppInstallPurchaseApprovalRequestBaseSchema.pipe(
    Schema.filter((request) => request.requestKind === 'install' || 'Expected install requests to use install kind')
  )
    .pipe(
      Schema.filter(
        (request) =>
          request.evidenceReferences.length > 0 ||
          'Expected install requests to cite the source evidence or platform limitation proof'
      )
    )
    .pipe(
      Schema.filter(
        (request) => request.auditEventRefs.length > 0 || 'Expected install requests to carry audit event refs'
      )
    )
);

const PurchaseRequestBaseSchema = Schema.Struct({
  ...AppInstallPurchaseApprovalRequestFields,
  purchaseKind: AppInstallPurchaseApprovalPurchaseKindSchema,
  subscriptionPeriod: Schema.Union(AppInstallPurchaseApprovalSubscriptionPeriodSchema, Schema.Null),
  priceDisplay: Schema.Union(AppInstallPurchaseApprovalPriceDisplaySchema, Schema.Null),
  billingEntitlementClaim: AppInstallPurchaseApprovalBillingEntitlementClaimSchema,
});

export const PurchaseRequestSchema = withParser(
  PurchaseRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        request.requestKind === 'purchase' ||
        request.requestKind === 'subscription' ||
        'Expected purchase requests to use purchase or subscription kind'
    )
  )
    .pipe(
      Schema.filter(
        (request) =>
          purchaseRequestKindIsConsistent(request) ||
          'Expected subscription requests to carry subscription period and non-subscription purchases to omit it'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.billingEntitlementClaim === 'not-claimed' ||
          'Expected purchase approval contracts to avoid billing entitlement logic'
      )
    )
);

const AppInstallPurchaseApprovalDecisionBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalSchemaVersionSchema,
  decisionId: AppInstallPurchaseApprovalDecisionIdSchema,
  requestId: AppInstallPurchaseApprovalRequestIdSchema,
  requestKind: AppInstallPurchaseApprovalRequestKindSchema,
  decisionAction: AppInstallPurchaseApprovalDecisionActionSchema,
  resultingState: AppInstallPurchaseApprovalStateSnapshotSchema,
  parentAction: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  decidedAt: ParentTimestampSchema,
  auditEventRefs: Schema.Array(AppInstallPurchaseApprovalAuditEventRefSchema),
});

export const AppInstallPurchaseApprovalDecisionSchema = withParser(
  AppInstallPurchaseApprovalDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) =>
        approvalDecisionIsConsistent(decision) ||
        'Expected approval decisions to map approve, deny, time-box, and review-needed actions to honest states'
    )
  )
);

const AppInstallPurchaseApprovalChildFacingStateBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalSchemaVersionSchema,
  childStateId: AppInstallPurchaseApprovalChildStateIdSchema,
  requestId: AppInstallPurchaseApprovalRequestIdSchema,
  requestKind: AppInstallPurchaseApprovalRequestKindSchema,
  platform: ParentPlatformSchema,
  childVisibleStatus: AppInstallPurchaseApprovalChildFacingStatusSchema,
  sourceApprovalState: AppInstallPurchaseApprovalStateSnapshotSchema,
  deliveryState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  deliveryRequirement: AppInstallPurchaseApprovalProofRequirementSchema,
  auditEventRefs: Schema.Array(AppInstallPurchaseApprovalAuditEventRefSchema),
  reportRefs: Schema.Array(AppInstallPurchaseApprovalReportRefSchema),
  claimBoundary: AppInstallPurchaseApprovalClaimBoundarySchema,
});

const AppInstallPurchaseApprovalChildFacingStateSchema = withParser(
  AppInstallPurchaseApprovalChildFacingStateBaseSchema.pipe(
    Schema.filter(
      (state) =>
        childFacingStateIsConsistent(state) ||
        'Expected child-facing install/purchase states to mirror approval state, cite audit/report refs, and avoid delivery claims'
    )
  )
);

const AppInstallPurchaseApprovalAuditReportIntegrationBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalSchemaVersionSchema,
  surface: AppInstallPurchaseApprovalAuditReportSurfaceSchema,
  integrationState: AppInstallPurchaseApprovalProofIntegrationStateSchema,
  auditEventRefs: Schema.Array(AppInstallPurchaseApprovalAuditEventRefSchema),
  reportRefs: Schema.Array(AppInstallPurchaseApprovalReportRefSchema),
  proofRequirement: AppInstallPurchaseApprovalProofRequirementSchema,
  claimBoundary: AppInstallPurchaseApprovalClaimBoundarySchema,
});

const AppInstallPurchaseApprovalAuditReportIntegrationSchema = withParser(
  AppInstallPurchaseApprovalAuditReportIntegrationBaseSchema.pipe(
    Schema.filter(
      (integration) =>
        auditReportIntegrationIsHonest(integration) ||
        'Expected app install/purchase audit/report status rows to stay contract-only without portal runtime claims'
    )
  )
);

const AppInstallPurchaseApprovalPlatformSupportRowBaseSchema = Schema.Struct({
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseApprovalStoreSurfaceSchema,
  contractRequestState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  storeMetadataState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  installInterceptionState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  purchaseInterceptionState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  subscriptionInterceptionState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  childPendingState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  approvalDeliveryState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  manualRequirement: Schema.Union(AppInstallPurchaseApprovalManualRequirementSchema, Schema.Null),
  unavailableReason: Schema.Union(AppInstallPurchaseApprovalUnavailableReasonSchema, Schema.Null),
  proofRequirement: AppInstallPurchaseApprovalProofRequirementSchema,
  claimBoundary: AppInstallPurchaseApprovalClaimBoundarySchema,
});

export const AppInstallPurchaseApprovalPlatformSupportRowSchema = withParser(
  AppInstallPurchaseApprovalPlatformSupportRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformSupportRowIsHonest(row) ||
        'Expected platform support rows to support contract proof only and avoid store/interception overclaims'
    )
  )
);

const AppInstallPurchaseApprovalContractProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalSchemaVersionSchema,
  installRequest: AppInstallRequestSchema,
  purchaseRequest: PurchaseRequestSchema,
  subscriptionRequest: PurchaseRequestSchema,
  approvalDecisions: Schema.Array(AppInstallPurchaseApprovalDecisionSchema),
  platformSupportMatrix: Schema.Array(AppInstallPurchaseApprovalPlatformSupportRowSchema),
  platformSourceMetadata: Schema.Array(AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema),
  packageSourceArtifacts: Schema.Array(AppInstallPurchaseApprovalPackageSourceArtifactRowSchema),
  childFacingStates: Schema.Array(AppInstallPurchaseApprovalChildFacingStateSchema),
  auditReportIntegration: Schema.Array(AppInstallPurchaseApprovalAuditReportIntegrationSchema),
  nonClaims: Schema.Array(AppInstallPurchaseApprovalNonClaimSchema),
  storeIntegrationClaim: AppInstallPurchaseApprovalStoreIntegrationClaimSchema,
  billingEntitlementClaim: AppInstallPurchaseApprovalBillingEntitlementClaimSchema,
  portalUiClaim: AppInstallPurchaseApprovalPortalUiClaimSchema,
  platformAdapterClaim: AppInstallPurchaseApprovalPlatformAdapterClaimSchema,
  interceptionClaim: AppInstallPurchaseApprovalInterceptionClaimSchema,
  runtimeBlockingSeparation: AppInstallPurchaseApprovalRuntimeBlockingSeparationSchema,
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseApprovalStoreMetadata = Infer<typeof AppInstallPurchaseApprovalStoreMetadataBaseSchema> &
  GeneratedAppInstallPurchaseApprovalStoreMetadata;
export type AppInstallPurchaseApprovalStateSnapshot = Infer<typeof AppInstallPurchaseApprovalStateSnapshotBaseSchema> &
  GeneratedAppInstallPurchaseApprovalStateSnapshot;
export type PurchaseRequest = Infer<typeof PurchaseRequestBaseSchema> & GeneratedPurchaseRequest;
export type AppInstallPurchaseApprovalDecision = Infer<typeof AppInstallPurchaseApprovalDecisionBaseSchema> &
  GeneratedAppInstallPurchaseApprovalDecision;
export type AppInstallPurchaseApprovalPlatformSupportRow = Infer<
  typeof AppInstallPurchaseApprovalPlatformSupportRowBaseSchema
> &
  GeneratedAppInstallPurchaseApprovalPlatformSupportRow;
export type AppInstallPurchaseApprovalContractProofCandidate = Infer<
  typeof AppInstallPurchaseApprovalContractProofBaseSchema
>;

export type AppInstallPurchaseApprovalRequestKind = Infer<typeof AppInstallPurchaseApprovalRequestKindSchema> &
  GeneratedAppInstallPurchaseApprovalRequestKind;
export type AppInstallPurchaseApprovalStoreSurface = Infer<typeof AppInstallPurchaseApprovalStoreSurfaceSchema> &
  GeneratedAppInstallPurchaseApprovalStoreSurface;
export type AppInstallPurchaseApprovalStoreMetadataFreshness = Infer<
  typeof AppInstallPurchaseApprovalStoreMetadataFreshnessSchema
> &
  GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness;
export type AppInstallPurchaseApprovalDecisionAction = Infer<typeof AppInstallPurchaseApprovalDecisionActionSchema> &
  GeneratedAppInstallPurchaseApprovalDecisionAction;
export type AppInstallPurchaseApprovalState = Infer<typeof AppInstallPurchaseApprovalStateSchema> &
  GeneratedAppInstallPurchaseApprovalState;
export type AppInstallPurchaseApprovalPlatformSupportState = Infer<
  typeof AppInstallPurchaseApprovalPlatformSupportStateSchema
> &
  GeneratedAppInstallPurchaseApprovalSupportState;
export type AppInstallPurchaseApprovalNonClaim = Infer<typeof AppInstallPurchaseApprovalNonClaimSchema> &
  GeneratedAppInstallPurchaseApprovalNonClaim;
export type AppInstallRequest = Infer<typeof AppInstallRequestSchema> & GeneratedAppInstallRequest;
export type AppInstallPurchaseApprovalContractProof = Infer<typeof AppInstallPurchaseApprovalContractProofBaseSchema> &
  GeneratedAppInstallPurchaseApprovalContractProof;

export const AppInstallPurchaseApprovalContractProofSchema = withParser(
  AppInstallPurchaseApprovalContractProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        appInstallPurchaseApprovalContractProofIsHonest(proof) ||
        'Expected app install and purchase approval proof to stay contract-only with honest platform limitations'
    )
  )
);

export const AppInstallPurchaseApprovalSupportState = {
  Supported: AppInstallPurchaseApprovalPlatformSupportStateSchema.parse('supported'),
  ManualRequired: AppInstallPurchaseApprovalPlatformSupportStateSchema.parse('manual-required'),
  Unavailable: AppInstallPurchaseApprovalPlatformSupportStateSchema.parse('unavailable'),
} as const;

export const AppInstallPurchaseApprovalNonClaim = {
  NoStoreIntegration: AppInstallPurchaseApprovalNonClaimSchema.parse('no-store-integration'),
  NoBillingEntitlementLogic: AppInstallPurchaseApprovalNonClaimSchema.parse('no-billing-entitlement-logic'),
  NoPortalUi: AppInstallPurchaseApprovalNonClaimSchema.parse('no-portal-ui'),
  NoPlatformAdapter: AppInstallPurchaseApprovalNonClaimSchema.parse('no-platform-adapter'),
  NoStorePolicyBypass: AppInstallPurchaseApprovalNonClaimSchema.parse('no-store-policy-bypass'),
  NoRealInstallOrPurchaseInterception: AppInstallPurchaseApprovalNonClaimSchema.parse(
    'no-real-install-or-purchase-interception'
  ),
  NotGenericAppBlocking: AppInstallPurchaseApprovalNonClaimSchema.parse('not-generic-app-blocking'),
} as const;
