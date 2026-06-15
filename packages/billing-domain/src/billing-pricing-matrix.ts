import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BillingDeviceLimitDecisionSchema,
  BillingEntitlementSnapshotSchema,
  BillingFeatureDecisionSchema,
  BillingFeatureEntitlementSchema,
  BillingPlanSchema,
  BillingSubscriptionStatusProofRowSchema,
  BillingSyncEventSchema,
  type BillingPlan,
} from './billing-entitlement';
import { BillingEntitlementSchemaVersionSchema, BillingEntitlementSourceSchema } from './billing-entitlement-values';

export const BillingPricingTierMatrixRowSchema = withParser(
  Schema.Struct({
    plan: BillingPlanSchema,
    defaultEntitlementSnapshot: BillingEntitlementSnapshotSchema,
    defaultDeviceLimitDecision: BillingDeviceLimitDecisionSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.defaultEntitlementSnapshot.planId === row.plan.planId ||
        'Expected pricing tier entitlement snapshot to use the matching plan id'
    ),
    Schema.filter(
      (row) =>
        row.defaultEntitlementSnapshot.deviceLimit === row.plan.deviceLimit ||
        'Expected pricing tier entitlement snapshot to carry the plan device limit'
    ),
    Schema.filter(
      (row) =>
        row.defaultDeviceLimitDecision.entitlementSnapshotId === row.defaultEntitlementSnapshot.snapshotId ||
        'Expected pricing tier device-limit decision to reference the matching entitlement snapshot'
    ),
    Schema.filter(
      (row) =>
        row.defaultDeviceLimitDecision.planDeviceLimit === row.plan.deviceLimit ||
        'Expected pricing tier device-limit decision to reuse the plan device limit'
    ),
    Schema.filter(
      (row) =>
        row.plan.featureEntitlements.some((feature) => feature.safetyCritical) ||
        'Expected every pricing tier to keep at least one safety-critical feature explicit'
    ),
    Schema.filter(
      (row) =>
        matchingFeatureCodes(row.plan.featureEntitlements, row.defaultEntitlementSnapshot.featureDecisions) ||
        'Expected pricing tier entitlement decisions to track the same feature codes as the plan matrix'
    )
  )
);

export const BillingPricingTrialGraceBoundarySchema = withParser(
  Schema.Struct({
    paidPlan: BillingPlanSchema,
    trialSnapshot: BillingEntitlementSnapshotSchema,
    trialStatusProofRow: BillingSubscriptionStatusProofRowSchema,
    graceSnapshot: BillingEntitlementSnapshotSchema,
    graceStatusProofRow: BillingSubscriptionStatusProofRowSchema,
    graceDeviceLimitDecision: BillingDeviceLimitDecisionSchema,
  }).pipe(
    Schema.filter(
      (boundary) =>
        boundary.trialSnapshot.planId === boundary.paidPlan.planId ||
        'Expected trial entitlement snapshot to reuse the paid plan id'
    ),
    Schema.filter(
      (boundary) =>
        boundary.graceSnapshot.planId === boundary.paidPlan.planId ||
        'Expected grace entitlement snapshot to reuse the paid plan id'
    ),
    Schema.filter(
      (boundary) =>
        boundary.trialSnapshot.subscriptionStatus === 'trialing' &&
        boundary.trialStatusProofRow.subscriptionStatus === 'trialing' &&
        boundary.trialStatusProofRow.failureState === null ||
        'Expected trial boundary rows to remain trialing without degraded failure state'
    ),
    Schema.filter(
      (boundary) =>
        boundary.graceSnapshot.subscriptionStatus === 'grace' &&
        boundary.graceStatusProofRow.subscriptionStatus === 'grace' &&
        boundary.graceSnapshot.failureState !== null &&
        boundary.graceStatusProofRow.failureState !== null &&
        boundary.graceStatusProofRow.failureState.failureKind === boundary.graceSnapshot.failureState.failureKind ||
        'Expected grace boundary rows to carry a matching degraded failure state'
    ),
    Schema.filter(
      (boundary) =>
        boundary.graceDeviceLimitDecision.entitlementSnapshotId === boundary.graceSnapshot.snapshotId ||
        'Expected grace device-limit decisions to reference the grace entitlement snapshot'
    ),
    Schema.filter(
      (boundary) =>
        boundary.graceDeviceLimitDecision.decision === 'grace' &&
        boundary.graceDeviceLimitDecision.reasonCode === 'payment-required' &&
        boundary.graceDeviceLimitDecision.requestedDeviceAlreadyTrusted &&
        boundary.graceDeviceLimitDecision.deviceActivationBehavior === 'grace-existing-devices' ||
        'Expected trial expiration grace to keep existing trusted devices in an explicit grace state'
    )
  )
);

export const BillingPricingSafetyCriticalFreeBoundarySchema = withParser(
  Schema.Struct({
    freePlan: BillingPlanSchema,
    degradedSnapshot: BillingEntitlementSnapshotSchema,
    safetyCriticalPlanFeatures: Schema.Array(BillingFeatureEntitlementSchema),
    safetyCriticalSnapshotDecisions: Schema.Array(BillingFeatureDecisionSchema),
  }).pipe(
    Schema.filter(
      (boundary) =>
        boundary.degradedSnapshot.planId === boundary.freePlan.planId ||
        'Expected free-plan degraded entitlement snapshot to reuse the free plan id'
    ),
    Schema.filter(
      (boundary) =>
        boundary.degradedSnapshot.failureState !== null ||
        'Expected the degraded free-plan boundary to carry explicit failure context'
    ),
    Schema.filter(
      (boundary) =>
        boundary.safetyCriticalPlanFeatures.length > 0 &&
        boundary.safetyCriticalPlanFeatures.every(
          (feature) => feature.included && !feature.gateable && feature.safetyCritical
        ) ||
        'Expected free-plan safety-critical features to remain included and outside paid gates'
    ),
    Schema.filter(
      (boundary) =>
        boundary.safetyCriticalSnapshotDecisions.length > 0 &&
        boundary.safetyCriticalSnapshotDecisions.every(
          (decision) =>
            decision.safetyCritical &&
            decision.decision !== 'locked' &&
            decision.decision !== 'unavailable' &&
            decision.localSafetyBehavior !== 'unchanged'
        ) ||
        'Expected degraded free-plan safety-critical decisions to stay available through local safety behavior'
    ),
    Schema.filter(
      (boundary) =>
        matchingFeatureCodes(boundary.safetyCriticalPlanFeatures, boundary.safetyCriticalSnapshotDecisions) ||
        'Expected degraded safety-critical decisions to match the declared free-plan safety-critical feature set'
    )
  )
);

export const BillingPricingEntitlementSourceOwnerSchema = withParser(
  Schema.Struct({
    authoritativeSourceOwner: BillingEntitlementSourceSchema,
    offlineContinuationSource: BillingEntitlementSourceSchema,
    manualFallbackSource: BillingEntitlementSourceSchema,
    authoritativeSyncEvent: BillingSyncEventSchema,
    offlineEntitlementSnapshot: BillingEntitlementSnapshotSchema,
    offlineDeviceLimitDecision: BillingDeviceLimitDecisionSchema,
    manualReviewEntitlementSnapshot: BillingEntitlementSnapshotSchema,
  }).pipe(
    Schema.filter(
      (owner) =>
        owner.authoritativeSourceOwner === 'billing-backend' ||
        'Expected billing backend to own the authoritative subscription and entitlement source'
    ),
    Schema.filter(
      (owner) =>
        owner.offlineContinuationSource === 'signed-local-snapshot' &&
        owner.manualFallbackSource === 'manual-admin-review' ||
        'Expected offline continuation and manual fallback sources to stay explicit'
    ),
    Schema.filter(
      (owner) =>
        owner.authoritativeSyncEvent.source === owner.authoritativeSourceOwner ||
        'Expected authoritative sync events to originate from the authoritative billing source owner'
    ),
    Schema.filter(
      (owner) =>
        owner.offlineEntitlementSnapshot.source === owner.offlineContinuationSource &&
        owner.offlineEntitlementSnapshot.signatureState !== 'unavailable' ||
        'Expected offline continuation to use a signed or schema-valid entitlement snapshot'
    ),
    Schema.filter(
      (owner) =>
        owner.offlineDeviceLimitDecision.entitlementSnapshotId === owner.offlineEntitlementSnapshot.snapshotId ||
        'Expected offline device-limit decisions to consume the offline entitlement snapshot'
    ),
    Schema.filter(
      (owner) =>
        owner.manualReviewEntitlementSnapshot.source === owner.manualFallbackSource &&
        owner.manualReviewEntitlementSnapshot.failureState !== null ||
        'Expected manual fallback entitlement state to remain explicit and failure-backed'
    )
  )
);

export const BillingPricingMatrixProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingEntitlementSchemaVersionSchema,
    tierMatrix: Schema.Array(BillingPricingTierMatrixRowSchema),
    trialGraceBoundary: BillingPricingTrialGraceBoundarySchema,
    safetyCriticalFreeBoundary: BillingPricingSafetyCriticalFreeBoundarySchema,
    entitlementSourceOwner: BillingPricingEntitlementSourceOwnerSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingPricingMatrixProofIsHonest(proof) ||
        'Expected pricing matrix proof to keep free safety boundaries, trial grace, and source ownership aligned'
    )
  )
);

export type BillingPricingTierMatrixRow = Infer<typeof BillingPricingTierMatrixRowSchema>;
export type BillingPricingTrialGraceBoundary = Infer<typeof BillingPricingTrialGraceBoundarySchema>;
export type BillingPricingSafetyCriticalFreeBoundary = Infer<typeof BillingPricingSafetyCriticalFreeBoundarySchema>;
export type BillingPricingEntitlementSourceOwner = Infer<typeof BillingPricingEntitlementSourceOwnerSchema>;
export type BillingPricingMatrixProof = Infer<typeof BillingPricingMatrixProofSchema>;

type BillingPricingMatrixFeatureEntitlement = Infer<typeof BillingFeatureEntitlementSchema>;
type BillingPricingMatrixFeatureDecision = Infer<typeof BillingFeatureDecisionSchema>;

export const decodeBillingPricingMatrixProof = Schema.decodeUnknownSync(BillingPricingMatrixProofSchema);

export function summarizeBillingPricingTierStates(
  rows: ReadonlyArray<BillingPricingTierMatrixRow>
): Record<BillingPlan['activeState'], number> {
  const counts: Record<BillingPlan['activeState'], number> = {
    active: 0,
    'trial-only': 0,
    retired: 0,
    'manual-required': 0,
  };
  for (const row of rows) {
    counts[row.plan.activeState] += 1;
  }
  return counts;
}

export function listBillingPricingPlanIds(
  rows: ReadonlyArray<BillingPricingTierMatrixRow>
): ReadonlyArray<BillingPlan['planId']> {
  return rows.map((row) => row.plan.planId);
}

function billingPricingMatrixProofIsHonest(proof: {
  readonly tierMatrix: ReadonlyArray<{
    readonly plan: BillingPlan;
  }>;
  readonly trialGraceBoundary: {
    readonly paidPlan: BillingPlan;
  };
  readonly safetyCriticalFreeBoundary: {
    readonly freePlan: BillingPlan;
    readonly degradedSnapshot: {
      readonly failureState: { readonly failureKind: string } | null;
    };
  };
  readonly entitlementSourceOwner: {
    readonly authoritativeSourceOwner: string;
    readonly offlineContinuationSource: string;
    readonly manualFallbackSource: string;
  };
}): boolean {
  if (proof.tierMatrix.length < 3) {
    return false;
  }

  const planIds = proof.tierMatrix.map((row) => row.plan.planId);
  const uniquePlanIds = new Set(planIds);
  const tierDeviceLimits = proof.tierMatrix.map((row) => row.plan.deviceLimit);

  return (
    uniquePlanIds.size === planIds.length &&
    tierDeviceLimits.every((limit, index) => {
      const previousLimit = index === 0 ? undefined : tierDeviceLimits[index - 1];
      return previousLimit === undefined || previousLimit < limit;
    }) &&
    planIds.includes(proof.trialGraceBoundary.paidPlan.planId) &&
    planIds.includes(proof.safetyCriticalFreeBoundary.freePlan.planId) &&
    proof.safetyCriticalFreeBoundary.degradedSnapshot.failureState?.failureKind === 'provider-unavailable' &&
    proof.entitlementSourceOwner.authoritativeSourceOwner === 'billing-backend' &&
    proof.entitlementSourceOwner.offlineContinuationSource === 'signed-local-snapshot' &&
    proof.entitlementSourceOwner.manualFallbackSource === 'manual-admin-review'
  );
}

function matchingFeatureCodes(
  planFeatures: ReadonlyArray<BillingPricingMatrixFeatureEntitlement>,
  snapshotFeatures: ReadonlyArray<BillingPricingMatrixFeatureDecision>
): boolean {
  if (planFeatures.length !== snapshotFeatures.length) {
    return false;
  }

  const snapshotFeatureCodes = new Set(snapshotFeatures.map((feature) => feature.featureCode));
  return planFeatures.every((feature) => snapshotFeatureCodes.has(feature.featureCode));
}
