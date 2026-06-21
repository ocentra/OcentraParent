import {
  BillingParentVisibleInvoiceRecoveryStateCountsSchema,
  BillingParentVisibleInvoiceVisibilityCountsSchema,
  BillingParentVisibleSummarySchema,
  type BillingEntitlementContractProof as SharedBillingEntitlementContractProof,
  type BillingParentVisibleInvoiceRecoveryStateCounts,
  type BillingParentVisibleInvoiceVisibilityCounts,
  type BillingParentVisibleManualInvoiceState,
  type BillingParentVisibleSummary,
} from './billing-entitlement';
import {
  BillingHostedReturnRoute,
  BillingPortalSessionResponseSchema,
  type BillingPortalSessionResponse,
} from './billing-checkout-portal-boundary';
import {
  BillingInvoiceTaxRefundDisputeProofReadModel,
  type BillingInvoiceTaxRefundDisputeProof,
  type BillingInvoiceTaxRefundDisputeRow,
} from './billing-invoice-tax-refund-dispute';
import {
  BillingEntitlementRuntimeProofReadModel,
  type BillingEntitlementRuntimeProof,
} from './billing-entitlement-runtime-proof';
import { BillingEntitlementContractProofReadModel } from './billing-entitlement-proof-read-model';

export { BillingEntitlementContractProofReadModel };

export const BillingEntitlementContractProof = BillingEntitlementContractProofReadModel;

export const BillingEntitlementKnownGaps = [
  'Billing provider integration and Stripe customer/subscription storage remain unimplemented.',
  'Account backend, entitlement signing runtime, and subscription sync delivery remain unimplemented.',
  'Portal billing UI and account-management flows remain unimplemented.',
  'Child-device safety modules do not consume these entitlement snapshots yet.',
] as const;

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

const BillingParentVisibleForbiddenFieldSet = new Set<string>(
  BillingParentVisibleForbiddenFieldNames
);

export function summarizeBillingFailureStates(
  failureStates: SharedBillingEntitlementContractProof['failureStates']
): Record<
  | 'provider-unavailable'
  | 'network-unavailable'
  | 'stale-snapshot'
  | 'payment-required'
  | 'account-mismatch'
  | 'validation-failed',
  number
> {
  const counts = {
    'provider-unavailable': 0,
    'network-unavailable': 0,
    'stale-snapshot': 0,
    'payment-required': 0,
    'account-mismatch': 0,
    'validation-failed': 0,
  };
  for (const failureState of failureStates) {
    if (failureState.failureKind in counts) {
      counts[failureState.failureKind] += 1;
    }
  }
  return counts;
}

const BillingParentVisiblePortalSessionProofReadModel =
  BillingPortalSessionResponseSchema.parse({
    schemaVersion: 'billing-checkout-portal-boundary',
    requestId: 'billing-parent-visible-portal-proof',
    kind: 'billing-portal-session-create',
    status: 'accepted',
    hostedSessionId: 'portal-session-parent-visible-proof',
    hostedUrl: 'https://billing.stripe.com/p/session/parent-visible-proof',
    expiresAt: '2026-06-13T09:00:00.000Z',
    rejectionReason: null,
  });

export const BillingParentVisibleSummaryReadModel =
  buildParentBillingVisibleSummary(
    BillingEntitlementContractProofReadModel,
    BillingEntitlementRuntimeProofReadModel,
    BillingInvoiceTaxRefundDisputeProofReadModel,
    BillingParentVisiblePortalSessionProofReadModel
  );

export function buildParentBillingVisibleSummary(
  contractProof: SharedBillingEntitlementContractProof,
  runtimeProof: BillingEntitlementRuntimeProof,
  invoiceProof: BillingInvoiceTaxRefundDisputeProof = BillingInvoiceTaxRefundDisputeProofReadModel,
  portalSession: BillingPortalSessionResponse = BillingParentVisiblePortalSessionProofReadModel
): BillingParentVisibleSummary {
  return BillingParentVisibleSummarySchema.parse({
    parentAccountId: contractProof.entitlementSnapshot.parentAccount.parentAccountId,
    familyId: contractProof.entitlementSnapshot.family.familyId,
    currentPlanId: contractProof.plan.planId,
    currentSubscriptionStatus: contractProof.entitlementSnapshot.subscriptionStatus,
    childDeviceUsage: {
      limit: contractProof.entitlementSnapshot.effectiveChildDeviceLimit,
      activeCount: countVisibleActiveBillingDevices(contractProof),
    },
    visibleFailureCounts: summarizeBillingFailureStates(contractProof.failureStates),
    snapshotStates: summarizeParentVisibleRuntimeSnapshotStates(
      runtimeProof.snapshotConsumptions
    ),
    deviceConsumptionStates:
      summarizeParentVisibleRuntimeConsumptionStates(
        runtimeProof.deviceLimitConsumptions
      ),
    seatComposition: summarizeParentVisibleSeatComposition(contractProof),
    referralCreditSummary:
      summarizeParentVisibleReferralCreditSummary(contractProof),
    licenseSnapshot: summarizeParentVisibleLicenseSnapshot(contractProof),
    invoiceSummary: summarizeParentVisibleInvoiceSummary(
      contractProof,
      invoiceProof
    ),
    portalHandoff: summarizeParentVisiblePortalHandoff(portalSession),
    changePlanAction: summarizeParentVisibleChangePlanAction(contractProof),
    cancellationAction:
      summarizeParentVisibleCancellationAction(contractProof, invoiceProof),
    safetyNonClaims: {
      noChildActivityCustody: contractProof.nonClaims.includes(
        'no-child-activity-custody'
      ),
      noPortalUi: contractProof.nonClaims.includes('no-portal-ui'),
      noProductionBillingClaim: runtimeProof.nonClaims.includes(
        'no-production-billing-claim'
      ),
    },
  });
}

export function buildParentBillingVisibleSummaryForExpectedHousehold(
  contractProof: SharedBillingEntitlementContractProof,
  runtimeProof: BillingEntitlementRuntimeProof,
  expected: {
    readonly parentAccountId: SharedBillingEntitlementContractProof['entitlementSnapshot']['parentAccount']['parentAccountId'];
    readonly familyId: SharedBillingEntitlementContractProof['entitlementSnapshot']['family']['familyId'];
  },
  invoiceProof: BillingInvoiceTaxRefundDisputeProof = BillingInvoiceTaxRefundDisputeProofReadModel,
  portalSession: BillingPortalSessionResponse = BillingParentVisiblePortalSessionProofReadModel
): BillingParentVisibleSummary {
  if (
    contractProof.entitlementSnapshot.parentAccount.parentAccountId !==
      expected.parentAccountId ||
    contractProof.entitlementSnapshot.family.familyId !== expected.familyId
  ) {
    throw new Error('wrong-household-denied');
  }

  return buildParentBillingVisibleSummary(
    contractProof,
    runtimeProof,
    invoiceProof,
    portalSession
  );
}

export function isBillingSafeParentSummary(
  summary: Record<string, unknown>
): boolean {
  return !containsForbiddenParentBillingField(
    summary,
    BillingParentVisibleForbiddenFieldSet
  );
}


function countVisibleActiveBillingDevices(
  contractProof: SharedBillingEntitlementContractProof
): number {
  return contractProof.deviceLimitDecisions.filter(
    (
      entry: SharedBillingEntitlementContractProof['deviceLimitDecisions'][number]
    ) => entry.decision === 'allowed' || entry.decision === 'grace'
  ).length;
}

function summarizeParentVisibleSeatComposition(
  contractProof: SharedBillingEntitlementContractProof
) {
  return {
    baseChildDeviceLimit:
      contractProof.entitlementSnapshot.baseChildDeviceLimit,
    activeReferralCredits:
      contractProof.entitlementSnapshot.activeReferralCredits,
    paidExtraChildDeviceSeats:
      contractProof.entitlementSnapshot.paidExtraChildDeviceSeats,
    effectiveChildDeviceLimit:
      contractProof.entitlementSnapshot.effectiveChildDeviceLimit,
  };
}

function summarizeParentVisibleRuntimeSnapshotStates(
  rows: BillingEntitlementRuntimeProof['snapshotConsumptions']
) {
  const counts = {
    'snapshot-active': 0,
    'snapshot-stale': 0,
    'payment-required': 0,
    'provider-unavailable': 0,
    'manual-review': 0,
  };
  for (const row of rows) {
    counts[row.runtimeState] += 1;
  }
  return counts;
}

function summarizeParentVisibleRuntimeConsumptionStates(
  rows: BillingEntitlementRuntimeProof['deviceLimitConsumptions']
) {
  const counts = {
    'accepted-local': 0,
    'accepted-grace': 0,
    'blocked-new-device': 0,
    'manual-required': 0,
    'unavailable-local-safety': 0,
  };
  for (const row of rows) {
    counts[row.consumptionState] += 1;
  }
  return counts;
}

function summarizeParentVisibleReferralCreditSummary(
  contractProof: SharedBillingEntitlementContractProof
) {
  return {
    activeQualifiedReferralParents:
      contractProof.referralCreditSummary.activeQualifiedReferralParents,
    activeReferralCredits:
      contractProof.referralCreditSummary.activeReferralCredits,
    pendingReferralInvites:
      contractProof.referralCreditSummary.pendingReferralInvites,
    revokedReferralCredits:
      contractProof.referralCreditSummary.revokedReferralCredits,
  };
}

function summarizeParentVisibleLicenseSnapshot(
  contractProof: SharedBillingEntitlementContractProof
) {
  return {
    source: contractProof.entitlementSnapshot.source,
    signatureState: contractProof.entitlementSnapshot.signatureState,
    subscriptionStatus: contractProof.entitlementSnapshot.subscriptionStatus,
    parentVisibleState:
      contractProof.entitlementSnapshot.failureState?.parentVisibleState ??
      'available',
    localSafetyBehavior:
      contractProof.entitlementSnapshot.failureState?.localSafetyBehavior ??
      'unchanged',
    generatedAt: contractProof.entitlementSnapshot.generatedAt,
    expiresAt: contractProof.entitlementSnapshot.expiresAt,
    failureKind:
      contractProof.entitlementSnapshot.failureState?.failureKind ?? null,
  };
}

function summarizeParentVisibleInvoiceSummary(
  contractProof: SharedBillingEntitlementContractProof,
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
) {
  const representativeRow = representativeInvoiceLifecycleRow(
    contractProof,
    invoiceProof
  );
  return {
    visibilityStates: summarizeInvoiceVisibilityStates(invoiceProof),
    recoveryStates: summarizeInvoiceRecoveryStates(invoiceProof),
    hostedInvoiceSurface: invoiceProof.hostedInvoiceClaim,
    providerMode: representativeRow.providerMode,
    nextRenewalAt:
      representativeRow.providerMode === 'stripe-hosted'
        ? representativeRow.periodEnd
        : null,
    manualInvoiceState: summarizeParentVisibleManualInvoiceState(invoiceProof),
  };
}

function summarizeInvoiceVisibilityStates(
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleInvoiceVisibilityCounts {
  const counts: Record<BillingInvoiceTaxRefundDisputeRow['invoiceVisibility'], number> = {
    'customer-portal-hosted': 0,
    'download-link-issued': 0,
    'manual-support-required': 0,
  };

  for (const row of invoiceProof.rows) {
    const visibility = row.invoiceVisibility;
    counts[visibility] = (counts[visibility] ?? 0) + 1;
  }

  return BillingParentVisibleInvoiceVisibilityCountsSchema.parse(counts);
}

function summarizeInvoiceRecoveryStates(
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleInvoiceRecoveryStateCounts {
  const counts: Record<BillingInvoiceTaxRefundDisputeRow['recoveryState'], number> = {
    active: 0,
    trialing: 0,
    'past-due': 0,
    grace: 0,
    cancelled: 0,
    unpaid: 0,
    'support-required': 0,
  };

  for (const row of invoiceProof.rows) {
    const recoveryState = row.recoveryState;
    counts[recoveryState] = (counts[recoveryState] ?? 0) + 1;
  }

  return BillingParentVisibleInvoiceRecoveryStateCountsSchema.parse(counts);
}

function summarizeParentVisibleManualInvoiceState(
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleManualInvoiceState {
  const manualSupportRequiredCount = invoiceProof.rows.filter(
    (row: BillingInvoiceTaxRefundDisputeRow) =>
      row.invoiceVisibility === 'manual-support-required'
  ).length;
  const manualReviewStateCount = invoiceProof.rows.filter(
    (row: BillingInvoiceTaxRefundDisputeRow) =>
      row.parentVisibleState === 'manual-review'
  ).length;

  return {
    visible:
      manualSupportRequiredCount > 0 || manualReviewStateCount > 0,
    manualSupportRequiredCount,
    manualReviewStateCount,
  };
}

function summarizeParentVisiblePortalHandoff(
  portalSession: BillingPortalSessionResponse
) {
  return {
    sessionKind: portalSession.kind,
    returnPath: BillingHostedReturnRoute.PortalReturn.relativePath,
    hostedUrlVisible:
      portalSession.status === 'accepted' && portalSession.hostedUrl !== null,
  };
}

function summarizeParentVisibleChangePlanAction(
  contractProof: SharedBillingEntitlementContractProof
) {
  return {
    selfServiceVisible: true as const,
    managedBy: 'billing-portal-session-create' as const,
    currentPlanId: contractProof.plan.planId,
    returnPath: BillingHostedReturnRoute.PortalReturn.relativePath,
  };
}

function summarizeParentVisibleCancellationAction(
  contractProof: SharedBillingEntitlementContractProof,
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
) {
  const immediate = requiredInvoiceLifecycleRow(
    invoiceProof,
    'billing-invoice-cancel-immediate'
  );
  const periodEnd = requiredInvoiceLifecycleRow(
    invoiceProof,
    'billing-invoice-cancel-period-end'
  );

  return {
    selfServiceVisible: true as const,
    currentSubscriptionStatus: contractProof.entitlementSnapshot.subscriptionStatus,
    immediate: {
      recoveryState: immediate.recoveryState,
      parentVisibleState: immediate.parentVisibleState,
    },
    periodEnd: {
      recoveryState: periodEnd.recoveryState,
      parentVisibleState: periodEnd.parentVisibleState,
    },
  };
}

function representativeInvoiceLifecycleRow(
  contractProof: SharedBillingEntitlementContractProof,
  invoiceProof: BillingInvoiceTaxRefundDisputeProof
): BillingInvoiceTaxRefundDisputeRow {
  if (
    contractProof.entitlementSnapshot.failureState?.parentVisibleState ===
    'manual-review'
  ) {
    return requiredInvoiceLifecycleRow(
      invoiceProof,
      'billing-tax-manual-support'
    );
  }

  switch (contractProof.entitlementSnapshot.subscriptionStatus) {
    case 'grace':
      return requiredInvoiceLifecycleRow(invoiceProof, 'billing-invoice-grace');
    case 'past-due':
      return requiredInvoiceLifecycleRow(invoiceProof, 'billing-invoice-unpaid');
    case 'cancelled':
    case 'expired':
      return requiredInvoiceLifecycleRow(
        invoiceProof,
        'billing-invoice-cancel-immediate'
      );
    default:
      return requiredInvoiceLifecycleRow(invoiceProof, 'billing-invoice-active');
  }
}

function requiredInvoiceLifecycleRow(
  invoiceProof: BillingInvoiceTaxRefundDisputeProof,
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-invoice-grace'
    | 'billing-invoice-unpaid'
    | 'billing-tax-manual-support'
    | 'billing-invoice-cancel-immediate'
    | 'billing-invoice-cancel-period-end'
): BillingInvoiceTaxRefundDisputeRow {
  const row = invoiceProof.rows.find(
    (entry: BillingInvoiceTaxRefundDisputeRow) =>
      entry.boundaryId === boundaryId
  );
  if (row === undefined) {
    throw new Error(`missing invoice lifecycle row: ${boundaryId}`);
  }
  return row;
}

function containsForbiddenParentBillingField(
  value: unknown,
  forbiddenFields: ReadonlySet<string>
): boolean {
  if (Array.isArray(value)) {
    return value.some((entry) =>
      containsForbiddenParentBillingField(entry, forbiddenFields)
    );
  }

  if (typeof value !== 'object' || value === null) {
    return false;
  }

  const record = value as Record<string, unknown>;
  return Object.entries(record).some(
    ([key, entry]) =>
      forbiddenFields.has(key) ||
      containsForbiddenParentBillingField(entry, forbiddenFields)
  );
}
