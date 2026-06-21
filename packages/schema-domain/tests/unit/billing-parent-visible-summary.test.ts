import { describe, expect, it } from 'vitest';
import { BillingInvoiceTaxRefundDisputeProofReadModel } from '../../src/billing-invoice-tax-refund-dispute';
import { BillingEntitlementContractProofSchema } from '../../src/billing-entitlement';
import {
  BillingEntitlementContractProofReadModel,
  buildParentBillingVisibleSummary,
  buildParentBillingVisibleSummaryForExpectedHousehold,
  isBillingSafeParentSummary,
} from '../../src/billing-entitlement-proof';
import {
  BillingEntitlementRuntimeProofReadModel,
  BillingEntitlementRuntimeProofSchema,
} from '../../src/billing-entitlement-runtime-proof';

const expectedParentVisibleSummary = {
  parentAccountId: 'parent-account-billing-entitlement-proof-1',
  familyId: 'family-billing-entitlement-proof-1',
  currentPlanId: 'family-plus-monthly',
  currentSubscriptionStatus: 'active',
  childDeviceUsage: {
    limit: 5,
    activeCount: 2,
  },
  visibleFailureCounts: {
    'provider-unavailable': 1,
    'network-unavailable': 1,
    'stale-snapshot': 1,
    'payment-required': 1,
    'account-mismatch': 1,
    'validation-failed': 1,
  },
  snapshotStates: {
    'snapshot-active': 1,
    'snapshot-stale': 1,
    'payment-required': 1,
    'provider-unavailable': 1,
    'manual-review': 1,
  },
  deviceConsumptionStates: {
    'accepted-local': 1,
    'accepted-grace': 1,
    'blocked-new-device': 1,
    'manual-required': 1,
    'unavailable-local-safety': 0,
  },
  seatComposition: {
    baseChildDeviceLimit: 1,
    activeReferralCredits: 2,
    paidExtraChildDeviceSeats: 2,
    effectiveChildDeviceLimit: 5,
  },
  referralCreditSummary: {
    activeQualifiedReferralParents: 2,
    activeReferralCredits: 2,
    pendingReferralInvites: 1,
    revokedReferralCredits: 1,
  },
  licenseSnapshot: {
    source: 'signed-local-snapshot',
    signatureState: 'schema-valid-local',
    subscriptionStatus: 'active',
    parentVisibleState: 'available',
    localSafetyBehavior: 'unchanged',
    generatedAt: '2026-06-03T09:57:32.000Z',
    expiresAt: '2026-06-10T09:57:32.000Z',
    failureKind: null,
  },
  invoiceSummary: {
    visibilityStates: {
      'customer-portal-hosted': 11,
      'download-link-issued': 1,
      'manual-support-required': 3,
    },
    recoveryStates: {
      active: 6,
      trialing: 0,
      'past-due': 0,
      grace: 2,
      cancelled: 3,
      unpaid: 1,
      'support-required': 3,
    },
    hostedInvoiceSurface: 'customer-portal-hosted-only',
    providerMode: 'stripe-hosted',
    nextRenewalAt: '2026-07-14T00:00:00.000Z',
    manualInvoiceState: {
      visible: true,
      manualSupportRequiredCount: 3,
      manualReviewStateCount: 3,
    },
  },
  portalHandoff: {
    sessionKind: 'billing-portal-session-create',
    returnPath: '/family/billing/manage',
    hostedUrlVisible: true,
  },
  changePlanAction: {
    selfServiceVisible: true,
    managedBy: 'billing-portal-session-create',
    currentPlanId: 'family-plus-monthly',
    returnPath: '/family/billing/manage',
  },
  cancellationAction: {
    selfServiceVisible: true,
    currentSubscriptionStatus: 'active',
    immediate: {
      recoveryState: 'cancelled',
      parentVisibleState: 'locked',
    },
    periodEnd: {
      recoveryState: 'grace',
      parentVisibleState: 'grace',
    },
  },
  safetyNonClaims: {
    noChildActivityCustody: true,
    noPortalUi: true,
    noProductionBillingClaim: true,
  },
} as const;

describe('billing parent-visible summary contracts centralized in schema-domain', () => {
  it(
    'derives a parent-visible billing summary without child-private or raw provider fields',
    provesParentVisibleSummary
  );

  it(
    'rejects parent summaries that smuggle raw child-device or provider payload details',
    rejectsUnsafeParentSummary
  );

  it(
    'denies parent billing summaries when the requested household context does not match the entitlement snapshot owner',
    deniesWrongHouseholdSummary
  );
});

function provesParentVisibleSummary() {
  const parentVisibleSummary = buildParentBillingVisibleSummary(
    readContractProof(),
    readRuntimeProof()
  );

  expect(parentVisibleSummary).toEqual(expectedParentVisibleSummary);

  const serializedSummary = JSON.stringify(parentVisibleSummary);
  expect(serializedSummary).not.toContain('childProfileId');
  expect(serializedSummary).not.toContain('deviceId');
  expect(serializedSummary).not.toContain('providerReference');
  expect(serializedSummary).not.toContain('actorId');
  expect(serializedSummary).not.toContain('auditReference');
  expect(serializedSummary).not.toContain('supportAuditState');
  expect(serializedSummary).not.toContain('boundaryId');
}

function rejectsUnsafeParentSummary() {
  const safeSummary = buildParentBillingVisibleSummary(
    readContractProof(),
    readRuntimeProof()
  );
  const unsafeSummary = {
    ...safeSummary,
    auditReference:
      BillingInvoiceTaxRefundDisputeProofReadModel.rows[0].auditReference,
  };

  expect(isBillingSafeParentSummary(safeSummary)).toBe(true);
  expect(isBillingSafeParentSummary(unsafeSummary)).toBe(false);
}

function deniesWrongHouseholdSummary() {
  const allowedSummary = buildParentBillingVisibleSummaryForExpectedHousehold(
    readContractProof(),
    readRuntimeProof(),
    {
      parentAccountId: 'parent-account-billing-entitlement-proof-1',
      familyId: 'family-billing-entitlement-proof-1',
    }
  );

  expect(allowedSummary.parentAccountId).toBe(
    'parent-account-billing-entitlement-proof-1'
  );
  expect(allowedSummary.familyId).toBe('family-billing-entitlement-proof-1');

  expect(() =>
    buildParentBillingVisibleSummaryForExpectedHousehold(
      readContractProof(),
      readRuntimeProof(),
      {
        parentAccountId: 'parent-account-billing-entitlement-proof-1',
        familyId: 'family-billing-entitlement-proof-2',
      }
    )
  ).toThrow('wrong-household-denied');
}

function readContractProof() {
  return BillingEntitlementContractProofSchema.parse(BillingEntitlementContractProofReadModel);
}

function readRuntimeProof() {
  return BillingEntitlementRuntimeProofSchema.parse(BillingEntitlementRuntimeProofReadModel);
}
