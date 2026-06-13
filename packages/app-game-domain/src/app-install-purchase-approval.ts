import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
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
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import { AppInstallPurchaseApprovalPackageSourceArtifactRowSchema } from './app-install-purchase-approval-package-sources';
import { AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema } from './app-install-purchase-approval-platform-sources';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const AppInstallPurchaseApprovalSchemaVersionSchema = withParser(
  Schema.Literal('app-install-purchase-approval-contract-proof')
);
export const AppInstallPurchaseApprovalRequestKindSchema = withParser(
  Schema.Literal('install', 'purchase', 'subscription')
);
export const AppInstallPurchaseApprovalStoreSurfaceSchema = withParser(
  Schema.Literal(
    'google-play',
    'apple-app-store',
    'mac-app-store',
    'microsoft-store',
    'linux-package-manager',
    'parent-manual-entry',
    'unknown-store'
  )
);
export const AppInstallPurchaseApprovalStoreMetadataFreshnessSchema = withParser(
  Schema.Literal('fresh', 'stale', 'unknown', 'manual-required', 'unavailable')
);
export const AppInstallPurchaseApprovalMetadataSourceStateSchema = withParser(
  Schema.Literal('supported', 'manual-required', 'unavailable')
);
export const AppInstallPurchaseApprovalDecisionActionSchema = withParser(
  Schema.Literal('approve', 'deny', 'time-box', 'review-needed')
);
export const AppInstallPurchaseApprovalStateSchema = withParser(
  Schema.Literal('pending-parent-review', 'approved', 'denied', 'time-box-active', 'expired', 'review-needed')
);
export const AppInstallPurchaseApprovalExpiryStateSchema = withParser(
  Schema.Literal('not-expiring', 'time-box-active', 'expired', 'review-needed')
);
export const AppInstallPurchaseApprovalPlatformSupportStateSchema = withParser(
  Schema.Literal('supported', 'manual-required', 'unavailable')
);
export const AppInstallPurchaseApprovalPurchaseKindSchema = withParser(
  Schema.Literal('one-time-purchase', 'in-app-purchase', 'subscription')
);
export const AppInstallPurchaseApprovalSubscriptionPeriodSchema = withParser(
  Schema.Literal('weekly', 'monthly', 'annual', 'unknown')
);
const AppInstallPurchaseApprovalChildFacingStatusSchema = withParser(
  Schema.Literal(
    'pending-parent-review-visible',
    'approved-visible',
    'denied-visible',
    'time-box-visible',
    'review-needed-visible'
  )
);
const AppInstallPurchaseApprovalAuditReportSurfaceSchema = withParser(
  Schema.Literal(
    'request-audit-history',
    'parent-decision-audit-history',
    'child-facing-state-report',
    'platform-limitation-report'
  )
);
const AppInstallPurchaseApprovalProofIntegrationStateSchema = withParser(
  Schema.Literal('contract-only', 'manual-required', 'unavailable')
);
export const AppInstallPurchaseApprovalAuditEventKindSchema = withParser(
  Schema.Literal(
    'request-recorded',
    'metadata-source-evaluated',
    'parent-decision-recorded',
    'approval-expired',
    'platform-limitation-recorded'
  )
);
export const AppInstallPurchaseApprovalNonClaimSchema = withParser(
  Schema.Literal(
    'no-store-integration',
    'no-billing-entitlement-logic',
    'no-portal-ui',
    'no-platform-adapter',
    'no-store-policy-bypass',
    'no-real-install-or-purchase-interception',
    'not-generic-app-blocking'
  )
);
export const AppInstallPurchaseApprovalStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
export const AppInstallPurchaseApprovalBillingEntitlementClaimSchema = withParser(Schema.Literal('not-claimed'));
export const AppInstallPurchaseApprovalPortalUiClaimSchema = withParser(Schema.Literal('not-implemented'));
export const AppInstallPurchaseApprovalPlatformAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
export const AppInstallPurchaseApprovalInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
export const AppInstallPurchaseApprovalRuntimeBlockingSeparationSchema = withParser(
  Schema.Literal('separate-from-generic-app-blocking')
);

const AppInstallPurchaseApprovalRequestIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalRequestId');
const AppInstallPurchaseApprovalDecisionIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalDecisionId');
const AppInstallPurchaseApprovalAuditEventIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalAuditEventId');
const AppInstallPurchaseApprovalStoreListingIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalStoreListingId');
const AppInstallPurchaseApprovalAppTitleSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalAppTitle');
const AppInstallPurchaseApprovalPublisherNameSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalPublisherName');
const AppInstallPurchaseApprovalCategorySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalCategory');
const AppInstallPurchaseApprovalAgeRatingSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalAgeRating');
const AppInstallPurchaseApprovalReviewReasonSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalReviewReason');
const AppInstallPurchaseApprovalProofRequirementSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalProofRequirement');
const AppInstallPurchaseApprovalUnavailableReasonSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalUnavailableReason');
const AppInstallPurchaseApprovalManualRequirementSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalManualRequirement');
const AppInstallPurchaseApprovalClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalClaimBoundary');
const AppInstallPurchaseApprovalPriceDisplaySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalPriceDisplay');
const AppInstallPurchaseApprovalChildStateIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalChildStateId');
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

export type AppInstallPurchaseApprovalStoreMetadata = Infer<typeof AppInstallPurchaseApprovalStoreMetadataBaseSchema>;
export type AppInstallPurchaseApprovalStateSnapshot = Infer<typeof AppInstallPurchaseApprovalStateSnapshotBaseSchema>;
export type PurchaseRequest = Infer<typeof PurchaseRequestBaseSchema>;
export type AppInstallPurchaseApprovalDecision = Infer<typeof AppInstallPurchaseApprovalDecisionBaseSchema>;
export type AppInstallPurchaseApprovalPlatformSupportRow = Infer<
  typeof AppInstallPurchaseApprovalPlatformSupportRowBaseSchema
>;
export type AppInstallPurchaseApprovalContractProofCandidate = Infer<
  typeof AppInstallPurchaseApprovalContractProofBaseSchema
>;

export type AppInstallPurchaseApprovalRequestKind = Infer<typeof AppInstallPurchaseApprovalRequestKindSchema>;
export type AppInstallPurchaseApprovalStoreSurface = Infer<typeof AppInstallPurchaseApprovalStoreSurfaceSchema>;
export type AppInstallPurchaseApprovalStoreMetadataFreshness = Infer<
  typeof AppInstallPurchaseApprovalStoreMetadataFreshnessSchema
>;
export type AppInstallPurchaseApprovalDecisionAction = Infer<typeof AppInstallPurchaseApprovalDecisionActionSchema>;
export type AppInstallPurchaseApprovalState = Infer<typeof AppInstallPurchaseApprovalStateSchema>;
export type AppInstallPurchaseApprovalPlatformSupportState = Infer<
  typeof AppInstallPurchaseApprovalPlatformSupportStateSchema
>;
export type AppInstallPurchaseApprovalNonClaim = Infer<typeof AppInstallPurchaseApprovalNonClaimSchema>;
export type AppInstallRequest = Infer<typeof AppInstallRequestSchema>;
export type AppInstallPurchaseApprovalContractProof = Infer<typeof AppInstallPurchaseApprovalContractProofBaseSchema>;

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

