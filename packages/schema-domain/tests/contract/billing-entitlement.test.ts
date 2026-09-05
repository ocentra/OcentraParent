import { describe, expect, it } from 'vitest';

import {
  BillingDeviceLimitDecisionSchema,
  BillingEntitlementSeatCompositionSchema,
  BillingFeatureDecisionSchema,
  BillingPlanSchema,
  BillingSubscriptionStatusProofRowSchema,
} from '../../src/billing-entitlement';

const starterDisplayLabel = 'billing.plan.starter';

const starterPlan = {
  schemaVersion: 'billing-entitlement-contract-proof',
  planId: 'starter-monthly',
  displayTextToken: starterDisplayLabel,
  activeState: 'trial-only',
  parentPortalAccess: {
    includedParentPortalCount: 1,
    extraParentSlotState: 'separate-paid-addon',
  },
  deviceLimit: 1,
  featureEntitlements: [
    {
      featureCode: 'local-safety',
      included: true,
      gateable: false,
      safetyCritical: true,
      localSafetyBehavior: 'local-only',
      childActivityCustody: 'not-included',
    },
  ],
  retentionExportAllowance: {
    advancedReportDays: 30,
    exportAllowed: true,
    parentOwnedStorageRequired: true,
  },
  priceReference: 'catalog.starter.monthly.usd',
  updatedAt: '2026-09-02T20:00:00Z',
} as const;

const requestedDevice = {
  deviceId: 'device-pricing-wp01',
  childProfileId: null,
  label: 'Child device',
  platform: 'windows',
} as const;

const overLimitDenial = {
  schemaVersion: 'billing-entitlement-contract-proof',
  decisionId: 'decision-pricing-wp01-denied',
  requestedDevice,
  entitlementSnapshotId: 'snapshot-pricing-wp01',
  activeDeviceCount: 2,
  planDeviceLimit: 1,
  requestedDeviceAlreadyTrusted: false,
  decision: 'denied',
  reasonCode: 'limit-exceeded',
  deviceActivationBehavior: 'deny-new-device',
  auditReference: 'audit-pricing-wp01-denied',
  existingLocalSafetyBehavior: 'local-only',
} as const;

const existingDeviceGrace = {
  ...overLimitDenial,
  decisionId: 'decision-pricing-wp01-grace',
  requestedDeviceAlreadyTrusted: true,
  decision: 'grace',
  deviceActivationBehavior: 'grace-existing-devices',
  auditReference: 'audit-pricing-wp01-grace',
  existingLocalSafetyBehavior: 'grace-with-local-safety',
} as const;

const graceFailureState = {
  failureKind: 'payment-required',
  parentVisibleState: 'grace',
  localSafetyBehavior: 'grace-with-local-safety',
  retainEvidenceExportAccess: true,
  existingLocalSafetyContinues: true,
  parentResolution: 'payment-update',
  retryAllowed: true,
  retryAfter: null,
} as const;

describe('Rust-generated billing entitlement plan edge', () => {
  it('accepts one parent portal and one starter child seat with separate parent expansion', () => {
    expect(BillingPlanSchema.parse(starterPlan)).toEqual(starterPlan);
  });

  it('rejects open-ended parent grants and paid gating of safety-critical behavior', () => {
    expect(() =>
      BillingPlanSchema.parse({
        ...starterPlan,
        parentPortalAccess: {
          ...starterPlan.parentPortalAccess,
          includedParentPortalCount: 2,
        },
      })
    ).toThrowError(/Expected 1/);

    expect(() =>
      BillingPlanSchema.parse({
        ...starterPlan,
        featureEntitlements: [
          {
            ...starterPlan.featureEntitlements[0],
            gateable: true,
          },
        ],
      })
    ).toThrowError(/safety-critical local behavior to stay outside paid entitlement gates/);
  });

  it('rejects game-economy display and price references', () => {
    const rejectedDisplayLabel = 'billing.plan.loot-crate';
    const rejectedPlans = [
      {
        ...starterPlan,
        displayTextToken: rejectedDisplayLabel,
      },
      {
        ...starterPlan,
        priceReference: 'catalog.battle-pass.monthly',
      },
    ] as const;

    for (const plan of rejectedPlans) {
      expect(() => BillingPlanSchema.parse(plan)).toThrowError(
        /billing plans to reject game-economy pricing tokens or marketplace semantics/
      );
    }
  });
});

describe('Rust-generated billing entitlement seat composition edge', () => {
  it('accepts checked seat composition and rejects negative, mismatched, and overflow totals', () => {
    const composition = {
      baseChildDeviceLimit: 1,
      activeReferralCredits: 2,
      paidExtraChildDeviceSeats: 3,
      effectiveChildDeviceLimit: 6,
    } as const;
    expect(BillingEntitlementSeatCompositionSchema.parse(composition)).toEqual(composition);

    expect(() =>
      BillingEntitlementSeatCompositionSchema.parse({
        ...composition,
        activeReferralCredits: -1,
      })
    ).toThrowError(/billing counts to be non-negative u32 integers/);
    expect(() =>
      BillingEntitlementSeatCompositionSchema.parse({
        ...composition,
        effectiveChildDeviceLimit: 5,
      })
    ).toThrowError(/effective child-device limit to equal base seats/);
    expect(() =>
      BillingEntitlementSeatCompositionSchema.parse({
        baseChildDeviceLimit: 4_294_967_295,
        activeReferralCredits: 1,
        paidExtraChildDeviceSeats: 0,
        effectiveChildDeviceLimit: 4_294_967_295,
      })
    ).toThrowError(/within u32 range/);
  });
});

describe('Rust-generated billing entitlement device limit edge', () => {
  it('blocks new devices above the limit while preserving explicit grace for trusted devices', () => {
    expect(BillingDeviceLimitDecisionSchema.parse(overLimitDenial)).toEqual(overLimitDenial);
    expect(BillingDeviceLimitDecisionSchema.parse(existingDeviceGrace)).toEqual(existingDeviceGrace);

    expect(() =>
      BillingDeviceLimitDecisionSchema.parse({
        ...overLimitDenial,
        deviceActivationBehavior: 'allow-new-device',
      })
    ).toThrowError(/untrusted device above the plan limit to be denied new-device activation/);
    expect(() =>
      BillingDeviceLimitDecisionSchema.parse({
        ...overLimitDenial,
        activeDeviceCount: 0,
      })
    ).toThrowError(/limit-exceeded decisions to reflect an active count at or above the plan limit/);
    expect(() =>
      BillingDeviceLimitDecisionSchema.parse({
        ...existingDeviceGrace,
        existingLocalSafetyBehavior: 'local-only',
      })
    ).toThrowError(/trusted existing device above the plan limit to enter safety-preserving grace/);
    expect(() =>
      BillingDeviceLimitDecisionSchema.parse({
        ...existingDeviceGrace,
        decision: 'allowed',
        deviceActivationBehavior: 'allow-new-device',
        existingLocalSafetyBehavior: 'unchanged',
      })
    ).toThrowError(/trusted existing device above the plan limit to enter safety-preserving grace/);
  });
});

describe('Rust-generated billing entitlement grace edge', () => {
  it('requires grace status to remain parent-visible and safety-preserving', () => {
    const graceStatus = {
      schemaVersion: 'billing-entitlement-contract-proof',
      subscriptionStatus: 'grace',
      source: 'signed-local-snapshot',
      parentVisibleState: 'grace',
      localSafetyBehavior: 'grace-with-local-safety',
      evidenceExportAccess: 'retained',
      childActivityCustody: 'not-included',
      deviceActivationBehavior: 'grace-existing-devices',
      failureState: graceFailureState,
    } as const;
    expect(BillingSubscriptionStatusProofRowSchema.parse(graceStatus)).toEqual(graceStatus);

    expect(() =>
      BillingSubscriptionStatusProofRowSchema.parse({
        ...graceStatus,
        parentVisibleState: 'available',
      })
    ).toThrowError(/grace subscription status to remain parent-visible/);
  });

  it('keeps safety-critical grace and manual review outside paid shutdown behavior', () => {
    const safetyGrace = {
      featureCode: 'local-safety',
      decision: 'grace',
      reasonCode: 'payment-required',
      safetyCritical: true,
      localSafetyBehavior: 'grace-with-local-safety',
      evidenceExportAccess: 'retained',
      childActivityCustody: 'not-included',
    } as const;
    expect(BillingFeatureDecisionSchema.parse(safetyGrace)).toEqual(safetyGrace);

    expect(() =>
      BillingFeatureDecisionSchema.parse({
        ...safetyGrace,
        localSafetyBehavior: 'unchanged',
      })
    ).toThrowError(/safety-critical grace decisions to preserve local safety behavior explicitly/);
    expect(() =>
      BillingFeatureDecisionSchema.parse({
        ...safetyGrace,
        decision: 'manual-required',
        localSafetyBehavior: 'local-only',
      })
    ).toThrowError(/safety-critical manual review to preserve local safety behavior explicitly/);
  });
});
