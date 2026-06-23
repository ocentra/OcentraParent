import { type Infer, Schema, withParser } from './effect';
import { ParentAccountReferenceSchema, FamilyReferenceSchema, ParentActorReferenceSchema } from './family-references';
import { ParentDeviceReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';
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
  BillingFeatureCodeSchema,
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
import { buildBillingFailureStateSchema } from './billing-support-admin-common-values';

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
    deviceLimit: PositiveBillingLimitSchema,
    featureEntitlements: Schema.Array(BillingFeatureEntitlementSchema),
    retentionExportAllowance: Schema.Struct({
      advancedReportDays: PositiveBillingLimitSchema,
      exportAllowed: Schema.Boolean,
      parentOwnedStorageRequired: Schema.Boolean,
    }),
    priceReference: BillingPriceReferenceSchema,
    updatedAt: ParentTimestampSchema,
  })
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
        composition.effectiveChildDeviceLimit ===
          composition.baseChildDeviceLimit +
            composition.activeReferralCredits +
            composition.paidExtraChildDeviceSeats ||
        'Expected effective child-device limit to equal base seats plus active referral credits plus paid extra child-device seats'
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

export const decodeBillingEntitlementContractProof = Schema.decodeUnknownSync(BillingEntitlementContractProofSchema);

function billingEntitlementProofIsHonest(proof: {
  readonly entitlementSnapshot: {
    readonly activeReferralCredits: number;
  };
  readonly referralCreditSummary: {
    readonly activeReferralCredits: number;
  };
  readonly billingSyncEvents: ReadonlyArray<{ readonly failureState: BillingFailureState | null }>;
  readonly failureStates: ReadonlyArray<BillingFailureState>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly subscriptionStatusProofRows: ReadonlyArray<{ readonly subscriptionStatus: string }>;
  readonly deviceLimitDecisions: ReadonlyArray<{
    readonly decision: string;
    readonly reasonCode: string;
    readonly activeDeviceCount: number;
    readonly planDeviceLimit: number;
  }>;
}): boolean {
  const requiredNonClaims = [
    'no-stripe-sdk',
    'no-billing-provider-backend',
    'no-provider-token-custody',
    'no-child-activity-custody',
    'no-safety-shutdown',
    'no-portal-ui',
  ];
  const requiredSubscriptionStatuses = ['trialing', 'active', 'past-due', 'expired', 'grace', 'unavailable'];
  return (
    proof.referralCreditSummary.activeReferralCredits === proof.entitlementSnapshot.activeReferralCredits &&
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    requiredSubscriptionStatuses.every((status) =>
      proof.subscriptionStatusProofRows.some((row) => row.subscriptionStatus === status)
    ) &&
    proof.failureStates.length >= 3 &&
    proof.deviceLimitDecisions.some(
      (decision) =>
        decision.decision === 'denied' &&
        decision.reasonCode === 'limit-exceeded' &&
        decision.activeDeviceCount >= decision.planDeviceLimit
    ) &&
    proof.billingSyncEvents.every(
      (event) => event.failureState === null || event.failureState.retainEvidenceExportAccess
    )
  );
}
