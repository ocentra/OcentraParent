/* generated from crates/schema/src/billing_entitlement_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import { ParentAccountReferenceSchema, FamilyReferenceSchema, ParentActorReferenceSchema } from './family-references';
import { ParentDeviceReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';
import { billingEntitlementProofIsHonest } from './generated-billing-entitlement-proof';
import {
  BillingAuditReferenceSchema,
  BillingChildActivityCustodyClaimSchema,
  BillingChildActivityCustodySchema,
  BillingDeviceLimitDecisionIdSchema,
  BillingDeviceLimitDecisionStateSchema,
  BillingDeviceLimitReasonSchema,
  BillingDeviceActivationBehaviorSchema,
  BillingDisplayTextTokenSchema,
  BillingEntitlementDecisionStateSchema,
  BillingEntitlementNonClaimSchema,
  BillingEntitlementSchemaVersionSchema,
  BillingEntitlementSnapshotIdSchema,
  BillingEntitlementSourceSchema,
  BillingEvidenceExportAccessSchema,
  BillingExtraParentSlotStateSchema,
  BillingFeatureCodeSchema,
  BillingIncludedParentPortalCountSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentVisibleStateSchema,
  BillingPlanActiveStateSchema,
  BillingPlanIdSchema,
  BillingPortalUiClaimSchema,
  BillingPriceReferenceSchema,
  BillingProviderBackendClaimSchema,
  BillingProviderBoundarySchema,
  BillingProviderReferenceSchema,
  BillingReasonCodeSchema,
  BillingSignatureStateSchema,
  BillingStripeSdkClaimSchema,
  BillingSubscriptionStatusSchema,
  BillingSyncEventIdSchema,
  NonNegativeBillingCountSchema,
  PositiveBillingLimitSchema,
} from './billing-entitlement-values';
import { buildBillingFailureStateSchema } from './generated-billing-support-admin-common-values';

const RejectedBillingGameEconomyPattern = /\b(coin|coins|gem|gems|loot|battle-?pass|skin|crate|marketplace)\b/i;

export const BillingFeatureEntitlementSchema = withParser(
  Schema.Struct({
    featureCode: BillingFeatureCodeSchema,
    included: Schema.Boolean,
    gateable: Schema.Boolean,
    safetyCritical: Schema.Boolean,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
  }).pipe(
    Schema.filter(
      (feature) =>
        !feature.safetyCritical ||
        !feature.gateable ||
        'Expected safety-critical local behavior to stay outside paid entitlement gates'
    )
  )
);

export const BillingPlanSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    planId: BillingPlanIdSchema,
    displayTextToken: BillingDisplayTextTokenSchema,
    activeState: BillingPlanActiveStateSchema,
    parentPortalAccess: Schema.Struct({
      includedParentPortalCount: BillingIncludedParentPortalCountSchema,
      extraParentSlotState: BillingExtraParentSlotStateSchema,
    }),
    deviceLimit: PositiveBillingLimitSchema,
    featureEntitlements: Schema.Array(BillingFeatureEntitlementSchema),
    retentionExportAllowance: Schema.Struct({
      advancedReportDays: PositiveBillingLimitSchema,
      exportAllowed: Schema.Boolean,
      parentOwnedStorageRequired: Schema.Boolean,
    }),
    priceReference: BillingPriceReferenceSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (plan) =>
        plan.displayTextToken.startsWith('billing.plan.') ||
        'Expected billing plan display tokens to stay inside the billing plan namespace'
    ),
    Schema.filter(
      (plan) =>
        (!RejectedBillingGameEconomyPattern.test(plan.displayTextToken) &&
          !RejectedBillingGameEconomyPattern.test(plan.priceReference)) ||
        'Expected billing plans to reject game-economy pricing tokens or marketplace semantics'
    )
  )
);

export const BillingFailureStateSchema = buildBillingFailureStateSchema('billing');

export const BillingSubscriptionStatusProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    subscriptionStatus: BillingSubscriptionStatusSchema,
    source: BillingEntitlementSourceSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    deviceActivationBehavior: BillingDeviceActivationBehaviorSchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (row) =>
        !['past-due', 'expired', 'unknown', 'unavailable'].includes(row.subscriptionStatus) ||
        row.failureState !== null ||
        'Expected degraded subscription status rows to carry a parent-visible failure state'
    )
  )
);

export const BillingFeatureDecisionSchema = withParser(
  Schema.Struct({
    featureCode: BillingFeatureCodeSchema,
    decision: BillingEntitlementDecisionStateSchema,
    reasonCode: BillingReasonCodeSchema,
    safetyCritical: Schema.Boolean,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        !decision.safetyCritical ||
        (decision.decision !== 'locked' && decision.decision !== 'unavailable') ||
        'Expected safety-critical behavior not to be locked or made unavailable by billing state'
    )
  )
);

export const BillingEntitlementSeatCompositionSchema = withParser(
  Schema.Struct({
    baseChildDeviceLimit: PositiveBillingLimitSchema,
    activeReferralCredits: NonNegativeBillingCountSchema,
    paidExtraChildDeviceSeats: NonNegativeBillingCountSchema,
    effectiveChildDeviceLimit: PositiveBillingLimitSchema,
  }).pipe(
    Schema.filter(
      (composition) =>
        (Number.isInteger(
          composition.baseChildDeviceLimit + composition.activeReferralCredits + composition.paidExtraChildDeviceSeats
        ) &&
          composition.baseChildDeviceLimit +
            composition.activeReferralCredits +
            composition.paidExtraChildDeviceSeats <=
            4_294_967_295 &&
          composition.effectiveChildDeviceLimit ===
            composition.baseChildDeviceLimit +
              composition.activeReferralCredits +
              composition.paidExtraChildDeviceSeats) ||
        'Expected effective child-device limit to equal base seats plus active referral credits plus paid extra child-device seats within u32 range'
    )
  )
);

export const BillingEntitlementSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    snapshotId: BillingEntitlementSnapshotIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    planId: BillingPlanIdSchema,
    subscriptionStatus: BillingSubscriptionStatusSchema,
    source: BillingEntitlementSourceSchema,
    signatureState: BillingSignatureStateSchema,
    generatedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
    deviceLimit: PositiveBillingLimitSchema,
    baseChildDeviceLimit: PositiveBillingLimitSchema,
    activeReferralCredits: NonNegativeBillingCountSchema,
    paidExtraChildDeviceSeats: NonNegativeBillingCountSchema,
    effectiveChildDeviceLimit: PositiveBillingLimitSchema,
    featureDecisions: Schema.Array(BillingFeatureDecisionSchema),
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        snapshot.source !== 'unavailable' ||
        snapshot.failureState !== null ||
        'Expected unavailable entitlement snapshots to carry a parent-visible failure state'
    ),
    Schema.filter(
      (snapshot) =>
        BillingEntitlementSeatCompositionSchema.safeParse({
          baseChildDeviceLimit: snapshot.baseChildDeviceLimit,
          activeReferralCredits: snapshot.activeReferralCredits,
          paidExtraChildDeviceSeats: snapshot.paidExtraChildDeviceSeats,
          effectiveChildDeviceLimit: snapshot.effectiveChildDeviceLimit,
        }).success || 'Expected entitlement snapshots to keep seat composition math aligned'
    ),
    Schema.filter(
      (snapshot) =>
        snapshot.deviceLimit === snapshot.effectiveChildDeviceLimit ||
        'Expected deviceLimit to mirror the effective child-device limit surfaced by the snapshot'
    )
  )
);

export const BillingSyncEventSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    syncEventId: BillingSyncEventIdSchema,
    previousStatus: BillingSubscriptionStatusSchema,
    nextStatus: BillingSubscriptionStatusSchema,
    source: BillingEntitlementSourceSchema,
    actor: ParentActorReferenceSchema,
    recordedAt: ParentTimestampSchema,
    providerReference: Schema.Union(BillingProviderReferenceSchema, Schema.Null),
    providerBoundary: BillingProviderBoundarySchema,
    failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (event) =>
        event.providerReference === null ||
        event.providerBoundary === 'backend-reference-only' ||
        'Expected provider references to remain behind the billing backend boundary'
    )
  )
);

export const BillingDeviceLimitDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    decisionId: BillingDeviceLimitDecisionIdSchema,
    requestedDevice: ParentDeviceReferenceSchema,
    entitlementSnapshotId: BillingEntitlementSnapshotIdSchema,
    activeDeviceCount: NonNegativeBillingCountSchema,
    planDeviceLimit: PositiveBillingLimitSchema,
    requestedDeviceAlreadyTrusted: Schema.Boolean,
    decision: BillingDeviceLimitDecisionStateSchema,
    reasonCode: BillingDeviceLimitReasonSchema,
    deviceActivationBehavior: BillingDeviceActivationBehaviorSchema,
    auditReference: BillingAuditReferenceSchema,
    existingLocalSafetyBehavior: BillingLocalSafetyBehaviorSchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        decision.decision !== 'denied' ||
        decision.reasonCode !== 'within-plan' ||
        'Expected denied device-limit decisions to carry a denial reason'
    ),
    Schema.filter(
      (decision) =>
        decision.decision !== 'allowed' ||
        decision.requestedDeviceAlreadyTrusted ||
        decision.activeDeviceCount < decision.planDeviceLimit ||
        'Expected new-device activation to require capacity below the plan limit'
    ),
    Schema.filter(
      (decision) =>
        decision.decision === 'allowed' ||
        decision.existingLocalSafetyBehavior !== 'unchanged' ||
        'Expected non-allowed device-limit decisions to keep existing local safety behavior explicit'
    )
  )
);

export const BillingReferralCreditSummarySchema = withParser(
  Schema.Struct({
    activeQualifiedReferralParents: NonNegativeBillingCountSchema,
    activeReferralCredits: NonNegativeBillingCountSchema,
    pendingReferralInvites: NonNegativeBillingCountSchema,
    revokedReferralCredits: NonNegativeBillingCountSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.activeReferralCredits === summary.activeQualifiedReferralParents ||
        'Expected active referral credits to match the count of active qualified referral parents'
    )
  )
);

export const BillingEntitlementContractProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    plan: BillingPlanSchema,
    entitlementSnapshot: BillingEntitlementSnapshotSchema,
    referralCreditSummary: BillingReferralCreditSummarySchema,
    subscriptionStatusProofRows: Schema.Array(BillingSubscriptionStatusProofRowSchema),
    billingSyncEvents: Schema.Array(BillingSyncEventSchema),
    deviceLimitDecisions: Schema.Array(BillingDeviceLimitDecisionSchema),
    failureStates: Schema.Array(BillingFailureStateSchema),
    nonClaims: Schema.Array(BillingEntitlementNonClaimSchema),
    stripeSdkClaim: BillingStripeSdkClaimSchema,
    billingProviderBackendClaim: BillingProviderBackendClaimSchema,
    portalUiClaim: BillingPortalUiClaimSchema,
    childActivityCustodyClaim: BillingChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingEntitlementProofIsHonest(proof) ||
        'Expected billing entitlement proof to keep provider UI custody and safety-shutdown non-claims explicit'
    )
  )
);

export type BillingPlan = Infer<typeof BillingPlanSchema>;
export type BillingEntitlementSeatComposition = Infer<typeof BillingEntitlementSeatCompositionSchema>;
export type BillingEntitlementSnapshot = Infer<typeof BillingEntitlementSnapshotSchema>;
export type BillingSubscriptionStatusProofRow = Infer<typeof BillingSubscriptionStatusProofRowSchema>;
export type BillingSyncEvent = Infer<typeof BillingSyncEventSchema>;
export type BillingDeviceLimitDecision = Infer<typeof BillingDeviceLimitDecisionSchema>;
export type BillingReferralCreditSummary = Infer<typeof BillingReferralCreditSummarySchema>;
export type BillingFailureState = Infer<typeof BillingFailureStateSchema>;
export type BillingEntitlementContractProof = Infer<typeof BillingEntitlementContractProofSchema>;
