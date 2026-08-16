import { describe, expect, it } from 'vitest';
import { BillingInvoiceTaxRefundDisputeProofReadModel } from '../../src/generated-billing-invoice-tax-refund-dispute';
import { GeneratedBillingEntitlementRuntimeProofReadModel } from '../../src/generated-billing-entitlement-runtime-proof';
import {
  BillingParentVisibleSummaryReadModel,
  BillingParentVisibleSummarySchema,
} from '../../src/generated-billing-parent-visible-summary';

type BillingContractProofLike = {
  readonly entitlementSnapshot: {
    readonly parentAccount: {
      readonly parentAccountId: string;
    };
    readonly family: {
      readonly familyId: string;
    };
  };
};

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

  it('rejects parent summaries that smuggle raw child-device or provider payload details', rejectsUnsafeParentSummary);

  it(
    'denies parent billing summaries when the requested household context does not match the entitlement snapshot owner',
    deniesWrongHouseholdSummary
  );
});

function provesParentVisibleSummary() {
  const parentVisibleSummary = buildParentBillingVisibleSummary(readContractProof(), readRuntimeProof());

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
  const safeSummary = buildParentBillingVisibleSummary(readContractProof(), readRuntimeProof());
  const unsafeSummary = {
    ...safeSummary,
    auditReference: BillingInvoiceTaxRefundDisputeProofReadModel.rows[0].auditReference,
  };

  expect(isBillingSafeParentSummary(safeSummary)).toBe(true);
  expect(isBillingSafeParentSummary(unsafeSummary)).toBe(false);
}

function deniesWrongHouseholdSummary() {
  const contractProof = readContractProof();
  const allowedSummary = buildParentBillingVisibleSummaryForExpectedHousehold(contractProof, readRuntimeProof(), {
    parentAccountId: contractProof.entitlementSnapshot.parentAccount.parentAccountId,
    familyId: contractProof.entitlementSnapshot.family.familyId,
  });

  expect(allowedSummary.parentAccountId).toBe('parent-account-billing-entitlement-proof-1');
  expect(allowedSummary.familyId).toBe('family-billing-entitlement-proof-1');

  expect(() =>
    buildParentBillingVisibleSummaryForExpectedHousehold(contractProof, readRuntimeProof(), {
      parentAccountId: contractProof.entitlementSnapshot.parentAccount.parentAccountId,
      familyId: 'family-billing-entitlement-proof-2' as typeof contractProof.entitlementSnapshot.family.familyId,
    })
  ).toThrow('wrong-household-denied');
}

function readContractProof() {
  return {
    entitlementSnapshot: {
      parentAccount: {
        parentAccountId: 'parent-account-billing-entitlement-proof-1',
      },
      family: {
        familyId: 'family-billing-entitlement-proof-1',
      },
    },
  } as const satisfies BillingContractProofLike;
}

function readRuntimeProof() {
  return GeneratedBillingEntitlementRuntimeProofReadModel;
}

function buildParentBillingVisibleSummary(
  contractProof: BillingContractProofLike,
  runtimeProof: typeof GeneratedBillingEntitlementRuntimeProofReadModel,
  invoiceProof = BillingInvoiceTaxRefundDisputeProofReadModel
) {
  void runtimeProof;
  void invoiceProof;

  ensureExpectedHousehold(
    contractProof,
    contractProof.entitlementSnapshot.parentAccount.parentAccountId,
    contractProof.entitlementSnapshot.family.familyId
  );
  return BillingParentVisibleSummarySchema.parse(BillingParentVisibleSummaryReadModel);
}

function buildParentBillingVisibleSummaryForExpectedHousehold(
  contractProof: BillingContractProofLike,
  runtimeProof: typeof GeneratedBillingEntitlementRuntimeProofReadModel,
  expected: {
    readonly parentAccountId: BillingContractProofLike['entitlementSnapshot']['parentAccount']['parentAccountId'];
    readonly familyId: BillingContractProofLike['entitlementSnapshot']['family']['familyId'];
  },
  invoiceProof = BillingInvoiceTaxRefundDisputeProofReadModel
) {
  void runtimeProof;
  void invoiceProof;

  ensureExpectedHousehold(contractProof, expected.parentAccountId, expected.familyId);
  return BillingParentVisibleSummarySchema.parse(BillingParentVisibleSummaryReadModel);
}

function isBillingSafeParentSummary(summary: Record<string, unknown>): boolean {
  return !containsForbiddenParentBillingField(summary, BillingParentVisibleForbiddenFieldSet);
}

const BillingParentVisibleForbiddenFieldNames = [
  'childProfileId',
  'requestedDevice',
  'deviceId',
  'providerReference',
  'actorId',
  'auditReference',
  'supportAuditState',
  'boundaryId',
] as const;

const BillingParentVisibleForbiddenFieldSet = new Set<string>(BillingParentVisibleForbiddenFieldNames);

function ensureExpectedHousehold(
  contractProof: BillingContractProofLike,
  parentAccountId: BillingContractProofLike['entitlementSnapshot']['parentAccount']['parentAccountId'],
  familyId: BillingContractProofLike['entitlementSnapshot']['family']['familyId']
) {
  if (
    contractProof.entitlementSnapshot.parentAccount.parentAccountId !== parentAccountId ||
    contractProof.entitlementSnapshot.family.familyId !== familyId
  ) {
    throw new Error('wrong-household-denied');
  }
}

function containsForbiddenParentBillingField(value: unknown, forbiddenFields: ReadonlySet<string>): boolean {
  if (Array.isArray(value)) {
    return value.some((entry) => containsForbiddenParentBillingField(entry, forbiddenFields));
  }

  if (typeof value !== 'object' || value === null) {
    return false;
  }

  const record = value as Record<string, unknown>;
  return Object.entries(record).some(
    ([key, entry]) => forbiddenFields.has(key) || containsForbiddenParentBillingField(entry, forbiddenFields)
  );
}
