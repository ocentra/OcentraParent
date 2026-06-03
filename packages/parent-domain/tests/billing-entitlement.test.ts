import { describe, expect, it } from 'vitest';
import {
  BillingDeviceLimitDecisionSchema,
  BillingEntitlementContractProofSchema,
  BillingEntitlementSnapshotSchema,
  BillingFailureStateSchema,
  BillingFeatureDecisionSchema,
  BillingFeatureEntitlementSchema,
  BillingSubscriptionStatusProofRowSchema,
  BillingSyncEventSchema,
} from '../src/billing-entitlement';
import {
  BillingEntitlementContractProofReadModel,
  summarizeBillingFailureStates,
} from '../src/billing-entitlement-proof';

describe('billing entitlement contracts', () => {
  acceptsBillingEntitlementProofWithoutProviderClaims();
  rejectsPaidGatesForSafetyCriticalBehavior();
  rejectsLockedSafetyCriticalDecisionAndDroppedExportAccess();
  rejectsUnavailableSnapshotsWithoutFailureState();
  rejectsDegradedSubscriptionRowsWithoutFailureState();
  rejectsProviderReferencesOutsideBackendBoundary();
  rejectsDeniedDeviceLimitDecisionWithAllowedReason();
  rejectsAllowedDeviceActivationAtPlanLimit();
  rejectsBillingFailuresThatDropLocalSafetyContinuation();
  rejectsProofOverclaims();
});

function acceptsBillingEntitlementProofWithoutProviderClaims(): void {
  it('accepts plan entitlement subscription device-limit and failure contracts without provider or custody claims', () => {
    const proof = BillingEntitlementContractProofSchema.parse(BillingEntitlementContractProofReadModel);

    expect(proof.plan.planId).toBe('family-plus-monthly');
    expect(proof.plan.deviceLimit).toBe(5);
    expect(proof.entitlementSnapshot.subscriptionStatus).toBe('active');
    expect(subscriptionStatusCounts(proof)).toEqual({
      trialing: 1,
      active: 1,
      'past-due': 1,
      expired: 1,
      grace: 1,
      unavailable: 1,
    });
    expect(featureDecisionCounts(proof)).toEqual({
      available: 2,
      grace: 1,
      'local-only': 2,
    });
    expect(deviceLimitDecisionCounts(proof)).toEqual({
      allowed: 1,
      denied: 1,
      grace: 1,
      'manual-review': 1,
    });
    expect(summarizeBillingFailureStates(proof.failureStates)).toEqual({
      'provider-unavailable': 1,
      'network-unavailable': 1,
      'stale-snapshot': 1,
      'payment-required': 1,
      'account-mismatch': 1,
      'validation-failed': 1,
    });
    expect(proof.nonClaims).toEqual([
      'no-stripe-sdk',
      'no-billing-provider-backend',
      'no-provider-token-custody',
      'no-child-activity-custody',
      'no-safety-shutdown',
      'no-portal-ui',
    ]);
  });
}

function rejectsPaidGatesForSafetyCriticalBehavior(): void {
  it('rejects paid gates that can disable safety-critical local behavior', () => {
    const safetyCriticalFeature = BillingEntitlementContractProofReadModel.plan.featureEntitlements.find(
      (entry) => entry.featureCode === 'local-evidence-capture'
    );
    if (safetyCriticalFeature === undefined) {
      throw new Error('missing safety-critical local evidence feature');
    }

    expect(
      BillingFeatureEntitlementSchema.safeParse({
        ...safetyCriticalFeature,
        gateable: true,
      }).success
    ).toBe(false);
  });
}

function rejectsLockedSafetyCriticalDecisionAndDroppedExportAccess(): void {
  it('rejects entitlement decisions that lock safety-critical access or drop export access', () => {
    const exportDecision = BillingEntitlementContractProofReadModel.entitlementSnapshot.featureDecisions.find(
      (entry) => entry.featureCode === 'evidence-export-access'
    );
    if (exportDecision === undefined) {
      throw new Error('missing evidence export entitlement decision');
    }

    expect(
      BillingFeatureDecisionSchema.safeParse({
        ...exportDecision,
        decision: 'locked',
      }).success
    ).toBe(false);
    expect(
      BillingFeatureDecisionSchema.safeParse({
        ...exportDecision,
        evidenceExportAccess: 'removed',
      }).success
    ).toBe(false);
  });
}

function rejectsUnavailableSnapshotsWithoutFailureState(): void {
  it('rejects unavailable snapshots without visible failure state', () => {
    const snapshot = BillingEntitlementContractProofReadModel.entitlementSnapshot;

    expect(
      BillingEntitlementSnapshotSchema.safeParse({
        ...snapshot,
        source: 'unavailable',
        subscriptionStatus: 'unavailable',
        signatureState: 'unavailable',
        failureState: null,
      }).success
    ).toBe(false);
    expect(
      BillingEntitlementSnapshotSchema.safeParse({
        ...snapshot,
        source: 'unavailable',
        subscriptionStatus: 'unavailable',
        signatureState: 'unavailable',
        failureState: BillingEntitlementContractProofReadModel.failureStates[0],
      }).success
    ).toBe(true);
  });
}

function rejectsDegradedSubscriptionRowsWithoutFailureState(): void {
  it('rejects degraded subscription status proof rows without failure state', () => {
    const pastDueRow = BillingEntitlementContractProofReadModel.subscriptionStatusProofRows.find(
      (entry) => entry.subscriptionStatus === 'past-due'
    );
    if (pastDueRow === undefined) {
      throw new Error('missing past-due subscription proof row');
    }

    expect(
      BillingSubscriptionStatusProofRowSchema.safeParse({
        ...pastDueRow,
        failureState: null,
      }).success
    ).toBe(false);
  });
}

function rejectsProviderReferencesOutsideBackendBoundary(): void {
  it('rejects provider references outside the backend boundary', () => {
    const syncEvent = BillingEntitlementContractProofReadModel.billingSyncEvents[0];

    expect(
      BillingSyncEventSchema.safeParse({
        ...syncEvent,
        providerBoundary: 'none',
      }).success
    ).toBe(false);
  });
}

function rejectsDeniedDeviceLimitDecisionWithAllowedReason(): void {
  it('rejects denied device-limit decisions that use an allowed reason', () => {
    const deniedDecision = BillingEntitlementContractProofReadModel.deviceLimitDecisions.find(
      (entry) => entry.decision === 'denied'
    );
    if (deniedDecision === undefined) {
      throw new Error('missing denied device-limit decision');
    }

    expect(
      BillingDeviceLimitDecisionSchema.safeParse({
        ...deniedDecision,
        reasonCode: 'within-plan',
      }).success
    ).toBe(false);
  });
}

function rejectsAllowedDeviceActivationAtPlanLimit(): void {
  it('rejects new-device activation when the plan device limit is already reached', () => {
    const allowedDecision = BillingEntitlementContractProofReadModel.deviceLimitDecisions.find(
      (entry) => entry.decision === 'allowed'
    );
    if (allowedDecision === undefined) {
      throw new Error('missing allowed device-limit decision');
    }

    expect(
      BillingDeviceLimitDecisionSchema.safeParse({
        ...allowedDecision,
        activeDeviceCount: 5,
      }).success
    ).toBe(false);
    expect(
      BillingDeviceLimitDecisionSchema.safeParse({
        ...allowedDecision,
        activeDeviceCount: 5,
        requestedDeviceAlreadyTrusted: true,
      }).success
    ).toBe(true);
  });
}

function rejectsBillingFailuresThatDropLocalSafetyContinuation(): void {
  it('rejects billing failure states that drop existing local safety continuation', () => {
    const paymentRequiredFailure = BillingEntitlementContractProofReadModel.failureStates.find(
      (entry) => entry.failureKind === 'payment-required'
    );
    if (paymentRequiredFailure === undefined) {
      throw new Error('missing payment-required billing failure state');
    }

    expect(
      BillingFailureStateSchema.safeParse({
        ...paymentRequiredFailure,
        existingLocalSafetyContinues: false,
      }).success
    ).toBe(false);
  });
}

function rejectsProofOverclaims(): void {
  it('rejects proof overclaims for SDK backend custody safety shutdown or portal UI', () => {
    const proof = BillingEntitlementContractProofReadModel;

    for (const invalidProof of [
      { ...proof, stripeSdkClaim: 'included' },
      { ...proof, billingProviderBackendClaim: 'implemented' },
      { ...proof, portalUiClaim: 'implemented' },
      { ...proof, childActivityCustodyClaim: 'supported' },
      { ...proof, nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-safety-shutdown') },
    ]) {
      expect(BillingEntitlementContractProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function featureDecisionCounts(proof: typeof BillingEntitlementContractProofReadModel) {
  return countBy(proof.entitlementSnapshot.featureDecisions.map((entry) => entry.decision));
}

function subscriptionStatusCounts(proof: typeof BillingEntitlementContractProofReadModel) {
  return countBy(proof.subscriptionStatusProofRows.map((entry) => entry.subscriptionStatus));
}

function deviceLimitDecisionCounts(proof: typeof BillingEntitlementContractProofReadModel) {
  return countBy(proof.deviceLimitDecisions.map((entry) => entry.decision));
}

function countBy<T extends string>(values: ReadonlyArray<T>): Partial<Record<T, number>> {
  const counts: Partial<Record<T, number>> = {};
  for (const value of values) {
    counts[value] = (counts[value] ?? 0) + 1;
  }
  return counts;
}
