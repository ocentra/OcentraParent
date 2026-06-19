import {
  BillingDeviceLimitDecisionSchema as DomainBillingDeviceLimitDecisionSchema,
  BillingEntitlementContractProofSchema as DomainBillingEntitlementContractProofSchema,
  BillingEntitlementSnapshotSchema as DomainBillingEntitlementSnapshotSchema,
  BillingFailureStateSchema as DomainBillingFailureStateSchema,
  BillingFeatureDecisionSchema as DomainBillingFeatureDecisionSchema,
  BillingFeatureEntitlementSchema as DomainBillingFeatureEntitlementSchema,
  BillingPlanSchema as DomainBillingPlanSchema,
  BillingSubscriptionStatusProofRowSchema as DomainBillingSubscriptionStatusProofRowSchema,
  BillingSyncEventSchema as DomainBillingSyncEventSchema,
  decodeBillingEntitlementContractProof as decodeDomainBillingEntitlementContractProof,
  type BillingDeviceLimitDecision as DomainBillingDeviceLimitDecision,
  type BillingEntitlementContractProof as DomainBillingEntitlementContractProof,
  type BillingEntitlementSnapshot as DomainBillingEntitlementSnapshot,
  type BillingFailureState as DomainBillingFailureState,
  type BillingPlan as DomainBillingPlan,
  type BillingSubscriptionStatusProofRow as DomainBillingSubscriptionStatusProofRow,
  type BillingSyncEvent as DomainBillingSyncEvent,
} from '@ocentra-parent/billing-domain/billing-entitlement';
import {
  BillingEntitlementSourceSchema,
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentVisibleStateSchema,
  BillingPlanIdSchema,
  BillingSignatureStateSchema,
  BillingSubscriptionStatusSchema,
  NonNegativeBillingCountSchema,
  PositiveBillingLimitSchema,
} from '@ocentra-parent/billing-domain/billing-entitlement-values';
import { BillingHostedReturnPathSchema } from '@ocentra-parent/billing-domain/billing-checkout-portal-boundary-values';
import {
  BillingCollectionRecoveryStateSchema,
  BillingInvoiceHostedSurfaceClaimSchema,
  BillingInvoiceProviderModeSchema,
} from '@ocentra-parent/billing-domain/billing-invoice-tax-refund-dispute-values';
import {
  FamilyIdSchema,
  ParentAccountIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const BillingFeatureEntitlementSchema = DomainBillingFeatureEntitlementSchema;
export const BillingPlanSchema = DomainBillingPlanSchema;
export const BillingFailureStateSchema = DomainBillingFailureStateSchema;
export const BillingSubscriptionStatusProofRowSchema =
  DomainBillingSubscriptionStatusProofRowSchema;
export const BillingFeatureDecisionSchema = DomainBillingFeatureDecisionSchema;
export const BillingEntitlementSnapshotSchema = DomainBillingEntitlementSnapshotSchema;
export const BillingSyncEventSchema = DomainBillingSyncEventSchema;
export const BillingDeviceLimitDecisionSchema = DomainBillingDeviceLimitDecisionSchema;
export const BillingEntitlementContractProofSchema =
  DomainBillingEntitlementContractProofSchema;

export type BillingFeatureEntitlement = Infer<
  typeof BillingFeatureEntitlementSchema
>;
export type BillingPlan = DomainBillingPlan;
export type BillingFailureState = DomainBillingFailureState;
export type BillingSubscriptionStatusProofRow =
  DomainBillingSubscriptionStatusProofRow;
export type BillingFeatureDecision = Infer<typeof BillingFeatureDecisionSchema>;
export type BillingEntitlementSnapshot = DomainBillingEntitlementSnapshot;
export type BillingSyncEvent = DomainBillingSyncEvent;
export type BillingDeviceLimitDecision = DomainBillingDeviceLimitDecision;
export type BillingEntitlementContractProof = DomainBillingEntitlementContractProof;

export const decodeBillingEntitlementContractProof =
  decodeDomainBillingEntitlementContractProof;

export const BillingParentVisibleChildDeviceUsageSchema = withParser(
  Schema.Struct({
    limit: PositiveBillingLimitSchema,
    activeCount: NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleFailureCountsSchema = withParser(
  Schema.Struct({
    'provider-unavailable': NonNegativeBillingCountSchema,
    'network-unavailable': NonNegativeBillingCountSchema,
    'stale-snapshot': NonNegativeBillingCountSchema,
    'payment-required': NonNegativeBillingCountSchema,
    'account-mismatch': NonNegativeBillingCountSchema,
    'validation-failed': NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleSnapshotStateCountsSchema = withParser(
  Schema.Struct({
    'snapshot-active': NonNegativeBillingCountSchema,
    'snapshot-stale': NonNegativeBillingCountSchema,
    'payment-required': NonNegativeBillingCountSchema,
    'provider-unavailable': NonNegativeBillingCountSchema,
    'manual-review': NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleDeviceConsumptionCountsSchema = withParser(
  Schema.Struct({
    'accepted-local': NonNegativeBillingCountSchema,
    'accepted-grace': NonNegativeBillingCountSchema,
    'blocked-new-device': NonNegativeBillingCountSchema,
    'manual-required': NonNegativeBillingCountSchema,
    'unavailable-local-safety': NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleSeatCompositionSchema = withParser(
  Schema.Struct({
    baseChildDeviceLimit: PositiveBillingLimitSchema,
    activeReferralCredits: NonNegativeBillingCountSchema,
    paidExtraChildDeviceSeats: NonNegativeBillingCountSchema,
    effectiveChildDeviceLimit: PositiveBillingLimitSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.effectiveChildDeviceLimit ===
          summary.baseChildDeviceLimit +
            summary.activeReferralCredits +
            summary.paidExtraChildDeviceSeats ||
        'Expected parent-visible seat composition to keep effective limit math aligned'
    )
  )
);

export const BillingParentVisibleReferralCreditSummarySchema = withParser(
  Schema.Struct({
    activeQualifiedReferralParents: NonNegativeBillingCountSchema,
    activeReferralCredits: NonNegativeBillingCountSchema,
    pendingReferralInvites: NonNegativeBillingCountSchema,
    revokedReferralCredits: NonNegativeBillingCountSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.activeReferralCredits ===
          summary.activeQualifiedReferralParents ||
        'Expected parent-visible referral credits to match active qualified referral parents'
    )
  )
);

export const BillingParentVisibleLicenseSnapshotSchema = withParser(
  Schema.Struct({
    source: BillingEntitlementSourceSchema,
    signatureState: BillingSignatureStateSchema,
    subscriptionStatus: BillingSubscriptionStatusSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    generatedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
    failureKind: Schema.Union(BillingFailureKindSchema, Schema.Null),
  })
);

export const BillingParentVisibleInvoiceVisibilityCountsSchema = withParser(
  Schema.Struct({
    'customer-portal-hosted': NonNegativeBillingCountSchema,
    'download-link-issued': NonNegativeBillingCountSchema,
    'manual-support-required': NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleInvoiceRecoveryStateCountsSchema = withParser(
  Schema.Struct({
    active: NonNegativeBillingCountSchema,
    trialing: NonNegativeBillingCountSchema,
    'past-due': NonNegativeBillingCountSchema,
    grace: NonNegativeBillingCountSchema,
    cancelled: NonNegativeBillingCountSchema,
    unpaid: NonNegativeBillingCountSchema,
    'support-required': NonNegativeBillingCountSchema,
  })
);

export const BillingParentVisibleManualInvoiceStateSchema = withParser(
  Schema.Struct({
    visible: Schema.Boolean,
    manualSupportRequiredCount: NonNegativeBillingCountSchema,
    manualReviewStateCount: NonNegativeBillingCountSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.visible ===
          (summary.manualSupportRequiredCount > 0 ||
            summary.manualReviewStateCount > 0) ||
        'Expected parent-visible manual invoice visibility to match manual invoice state counts'
    )
  )
);

export const BillingParentVisibleInvoiceSummarySchema = withParser(
  Schema.Struct({
    visibilityStates: BillingParentVisibleInvoiceVisibilityCountsSchema,
    recoveryStates: BillingParentVisibleInvoiceRecoveryStateCountsSchema,
    hostedInvoiceSurface: BillingInvoiceHostedSurfaceClaimSchema,
    providerMode: BillingInvoiceProviderModeSchema,
    nextRenewalAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    manualInvoiceState: BillingParentVisibleManualInvoiceStateSchema,
  })
);

export const BillingParentVisiblePortalHandoffSchema = withParser(
  Schema.Struct({
    sessionKind: Schema.Literal('billing-portal-session-create'),
    returnPath: BillingHostedReturnPathSchema,
    hostedUrlVisible: Schema.Literal(true),
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.returnPath === '/family/billing/manage' ||
        'Expected parent billing portal handoff to reuse the allowlisted family billing management route'
    )
  )
);

export const BillingParentVisibleChangePlanActionSchema = withParser(
  Schema.Struct({
    selfServiceVisible: Schema.Literal(true),
    managedBy: Schema.Literal('billing-portal-session-create'),
    currentPlanId: BillingPlanIdSchema,
    returnPath: BillingHostedReturnPathSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.returnPath === '/family/billing/manage' ||
        'Expected parent change-plan actions to return through the allowlisted family billing management route'
    )
  )
);

export const BillingParentVisibleCancellationModeSchema = withParser(
  Schema.Struct({
    recoveryState: BillingCollectionRecoveryStateSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
  })
);

export const BillingParentVisibleCancellationActionSchema = withParser(
  Schema.Struct({
    selfServiceVisible: Schema.Literal(true),
    currentSubscriptionStatus: BillingSubscriptionStatusSchema,
    immediate: BillingParentVisibleCancellationModeSchema,
    periodEnd: BillingParentVisibleCancellationModeSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.immediate.recoveryState === 'cancelled' &&
          summary.periodEnd.recoveryState === 'grace' ||
        'Expected parent cancellation visibility to keep immediate cancel distinct from period-end grace'
    )
  )
);

export const BillingParentVisibleSafetyNonClaimsSchema = withParser(
  Schema.Struct({
    noChildActivityCustody: Schema.Literal(true),
    noPortalUi: Schema.Literal(true),
    noProductionBillingClaim: Schema.Literal(true),
  })
);

export const BillingParentVisibleSummarySchema = withParser(
  Schema.Struct({
    parentAccountId: ParentAccountIdSchema,
    familyId: FamilyIdSchema,
    currentPlanId: BillingPlanIdSchema,
    currentSubscriptionStatus: BillingSubscriptionStatusSchema,
    childDeviceUsage: BillingParentVisibleChildDeviceUsageSchema,
    visibleFailureCounts: BillingParentVisibleFailureCountsSchema,
    snapshotStates: BillingParentVisibleSnapshotStateCountsSchema,
    deviceConsumptionStates:
      BillingParentVisibleDeviceConsumptionCountsSchema,
    seatComposition: BillingParentVisibleSeatCompositionSchema,
    referralCreditSummary: BillingParentVisibleReferralCreditSummarySchema,
    licenseSnapshot: BillingParentVisibleLicenseSnapshotSchema,
    invoiceSummary: BillingParentVisibleInvoiceSummarySchema,
    portalHandoff: BillingParentVisiblePortalHandoffSchema,
    changePlanAction: BillingParentVisibleChangePlanActionSchema,
    cancellationAction: BillingParentVisibleCancellationActionSchema,
    safetyNonClaims: BillingParentVisibleSafetyNonClaimsSchema,
  })
);

export type BillingParentVisibleChildDeviceUsage = Infer<
  typeof BillingParentVisibleChildDeviceUsageSchema
>;
export type BillingParentVisibleFailureCounts = Infer<
  typeof BillingParentVisibleFailureCountsSchema
>;
export type BillingParentVisibleSnapshotStateCounts = Infer<
  typeof BillingParentVisibleSnapshotStateCountsSchema
>;
export type BillingParentVisibleDeviceConsumptionCounts = Infer<
  typeof BillingParentVisibleDeviceConsumptionCountsSchema
>;
export type BillingParentVisibleSeatComposition = Infer<
  typeof BillingParentVisibleSeatCompositionSchema
>;
export type BillingParentVisibleReferralCreditSummary = Infer<
  typeof BillingParentVisibleReferralCreditSummarySchema
>;
export type BillingParentVisibleLicenseSnapshot = Infer<
  typeof BillingParentVisibleLicenseSnapshotSchema
>;
export type BillingParentVisibleInvoiceVisibilityCounts = Infer<
  typeof BillingParentVisibleInvoiceVisibilityCountsSchema
>;
export type BillingParentVisibleInvoiceRecoveryStateCounts = Infer<
  typeof BillingParentVisibleInvoiceRecoveryStateCountsSchema
>;
export type BillingParentVisibleManualInvoiceState = Infer<
  typeof BillingParentVisibleManualInvoiceStateSchema
>;
export type BillingParentVisibleInvoiceSummary = Infer<
  typeof BillingParentVisibleInvoiceSummarySchema
>;
export type BillingParentVisiblePortalHandoff = Infer<
  typeof BillingParentVisiblePortalHandoffSchema
>;
export type BillingParentVisibleChangePlanAction = Infer<
  typeof BillingParentVisibleChangePlanActionSchema
>;
export type BillingParentVisibleCancellationMode = Infer<
  typeof BillingParentVisibleCancellationModeSchema
>;
export type BillingParentVisibleCancellationAction = Infer<
  typeof BillingParentVisibleCancellationActionSchema
>;
export type BillingParentVisibleSafetyNonClaims = Infer<
  typeof BillingParentVisibleSafetyNonClaimsSchema
>;
export type BillingParentVisibleSummary = Infer<
  typeof BillingParentVisibleSummarySchema
>;

export const decodeBillingParentVisibleSummary = Schema.decodeUnknownSync(
  BillingParentVisibleSummarySchema
);
