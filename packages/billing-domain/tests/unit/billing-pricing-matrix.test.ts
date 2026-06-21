import { describe, expect, it } from 'vitest';
import {
  BillingPricingEntitlementSourceOwnerSchema,
  BillingPricingMatrixProofSchema,
  BillingPricingSafetyCriticalFreeBoundarySchema,
  BillingPricingTierMatrixRowSchema,
  BillingPricingTrialGraceBoundarySchema,
} from '@ocentra-parent/schema-domain/billing-pricing-matrix';
import { BillingPricingMatrixProofReadModel } from '@ocentra-parent/schema-domain/billing-pricing-matrix-proof';

describe('billing pricing matrix proof', () => {
  coversBillingPricingTierMatrix();
  coversBillingPricingTrialGraceBoundary();
  coversBillingPricingSafetyCriticalFreeBoundary();
  coversBillingPricingEntitlementSourceOwner();
});

function coversBillingPricingTierMatrix(): void {
  it('billing-pricing.tier-matrix', () => {
    const proof = BillingPricingMatrixProofSchema.parse(BillingPricingMatrixProofReadModel);

    const tierStateCounts = proof.tierMatrix.reduce(
      (counts, row) => {
        counts[row.plan.activeState] += 1;
        return counts;
      },
      {
        active: 0,
        'trial-only': 0,
        retired: 0,
        'manual-required': 0,
      } as Record<(typeof proof.tierMatrix)[number]['plan']['activeState'], number>
    );

    expect(tierStateCounts).toEqual({
      active: 3,
      'trial-only': 0,
      retired: 0,
      'manual-required': 0,
    });
    expect(proof.tierMatrix.map((row) => row.plan.planId)).toEqual([
      'family-safety-free',
      'family-monitor-core',
      'family-monitor-plus',
    ]);
    expect(proof.tierMatrix.map((row) => row.plan.deviceLimit)).toEqual([1, 3, 5]);

    const freeRow = proof.tierMatrix[0];
    if (freeRow === undefined) {
      throw new Error('expected billing pricing proof to include a free tier row');
    }
    expect(freeRow.defaultEntitlementSnapshot.featureDecisions.map((decision) => decision.decision)).toEqual([
      'local-only',
      'local-only',
      'locked',
      'locked',
    ]);
    expect(
      BillingPricingTierMatrixRowSchema.safeParse({
        ...freeRow,
        defaultDeviceLimitDecision: {
          ...freeRow.defaultDeviceLimitDecision,
          planDeviceLimit: 5,
        },
      }).success
    ).toBe(false);
  });
}

function coversBillingPricingTrialGraceBoundary(): void {
  it('billing-pricing.trial-grace-boundary', () => {
    const boundary = BillingPricingTrialGraceBoundarySchema.parse(
      BillingPricingMatrixProofReadModel.trialGraceBoundary
    );

    expect(boundary.paidPlan.planId).toBe('family-monitor-core');
    expect(boundary.trialSnapshot.subscriptionStatus).toBe('trialing');
    expect(boundary.graceSnapshot.subscriptionStatus).toBe('grace');
    expect(boundary.graceStatusProofRow.failureState?.failureKind).toBe('payment-required');
    expect(boundary.graceDeviceLimitDecision.decision).toBe('grace');
    expect(boundary.graceDeviceLimitDecision.requestedDeviceAlreadyTrusted).toBe(true);

    expect(
      BillingPricingTrialGraceBoundarySchema.safeParse({
        ...boundary,
        graceDeviceLimitDecision: {
          ...boundary.graceDeviceLimitDecision,
          decision: 'allowed',
        },
      }).success
    ).toBe(false);
  });
}

function coversBillingPricingSafetyCriticalFreeBoundary(): void {
  it('billing-pricing.safety-critical-free-boundary', () => {
    const boundary = BillingPricingSafetyCriticalFreeBoundarySchema.parse(
      BillingPricingMatrixProofReadModel.safetyCriticalFreeBoundary
    );

    expect(boundary.freePlan.planId).toBe('family-safety-free');
    expect(boundary.degradedSnapshot.failureState?.failureKind).toBe('provider-unavailable');
    expect(boundary.safetyCriticalPlanFeatures.map((feature) => feature.featureCode)).toEqual([
      'local-evidence-capture',
      'evidence-export-access',
    ]);
    expect(boundary.safetyCriticalSnapshotDecisions.map((decision) => decision.decision)).toEqual([
      'local-only',
      'local-only',
    ]);

    expect(
      BillingPricingSafetyCriticalFreeBoundarySchema.safeParse({
        ...boundary,
        safetyCriticalPlanFeatures: [
          {
            ...boundary.safetyCriticalPlanFeatures[0],
            gateable: true,
          },
          boundary.safetyCriticalPlanFeatures[1],
        ],
      }).success
    ).toBe(false);
  });
}

function coversBillingPricingEntitlementSourceOwner(): void {
  it('billing-pricing.entitlement-source-owner', () => {
    const sourceOwner = BillingPricingEntitlementSourceOwnerSchema.parse(
      BillingPricingMatrixProofReadModel.entitlementSourceOwner
    );

    expect(sourceOwner.authoritativeSourceOwner).toBe('billing-backend');
    expect(sourceOwner.offlineContinuationSource).toBe('signed-local-snapshot');
    expect(sourceOwner.manualFallbackSource).toBe('manual-admin-review');
    expect(sourceOwner.authoritativeSyncEvent.source).toBe('billing-backend');
    expect(sourceOwner.offlineEntitlementSnapshot.source).toBe('signed-local-snapshot');
    expect(sourceOwner.manualReviewEntitlementSnapshot.failureState?.failureKind).toBe('validation-failed');

    expect(
      BillingPricingEntitlementSourceOwnerSchema.safeParse({
        ...sourceOwner,
        authoritativeSourceOwner: 'signed-local-snapshot',
      }).success
    ).toBe(false);
  });
}
