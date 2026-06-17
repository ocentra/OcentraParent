import {
  BillingEntitlementContractProof as DomainBillingEntitlementContractProof,
  BillingEntitlementContractProofReadModel as DomainBillingEntitlementContractProofReadModel,
  BillingEntitlementKnownGaps as DomainBillingEntitlementKnownGaps,
  summarizeBillingFailureStates as summarizeDomainBillingFailureStates,
} from '@ocentra-parent/billing-domain/billing-entitlement-proof';
import {
  BillingHostedReturnRoute,
  BillingPortalSessionResponseSchema,
  type BillingPortalSessionResponse,
} from '@ocentra-parent/billing-domain/billing-checkout-portal-boundary';
import {
  BillingInvoiceTaxRefundDisputeProofReadModel as DomainBillingInvoiceTaxRefundDisputeProofReadModel,
  type BillingInvoiceTaxRefundDisputeRow as DomainBillingInvoiceTaxRefundDisputeRow,
  type BillingInvoiceTaxRefundDisputeProof as DomainBillingInvoiceTaxRefundDisputeProof,
} from '@ocentra-parent/billing-domain/billing-invoice-tax-refund-dispute';
import {
  BillingEntitlementRuntimeProofReadModel,
  summarizeBillingEntitlementRuntimeConsumptionStates,
  summarizeBillingEntitlementRuntimeSnapshotStates,
  type BillingEntitlementRuntimeProof,
} from './billing-entitlement-runtime-proof';
import {
  BillingParentVisibleInvoiceRecoveryStateCountsSchema,
  BillingParentVisibleInvoiceVisibilityCountsSchema,
  type BillingParentVisibleManualInvoiceState,
  type BillingEntitlementContractProof as ParentBillingEntitlementContractProof,
  BillingParentVisibleSummarySchema,
  type BillingParentVisibleInvoiceRecoveryStateCounts,
  type BillingParentVisibleInvoiceVisibilityCounts,
  type BillingParentVisibleSummary,
} from './billing-entitlement';

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

export const BillingEntitlementContractProofReadModel =
  DomainBillingEntitlementContractProofReadModel;
export const BillingEntitlementContractProof = DomainBillingEntitlementContractProof;
export const BillingEntitlementKnownGaps = DomainBillingEntitlementKnownGaps;
export const summarizeBillingFailureStates = summarizeDomainBillingFailureStates;

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
    DomainBillingInvoiceTaxRefundDisputeProofReadModel,
    BillingParentVisiblePortalSessionProofReadModel
  );

export function buildParentBillingVisibleSummary(
  contractProof: ParentBillingEntitlementContractProof,
  runtimeProof: BillingEntitlementRuntimeProof,
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof = DomainBillingInvoiceTaxRefundDisputeProofReadModel,
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
    snapshotStates: summarizeBillingEntitlementRuntimeSnapshotStates(
      runtimeProof.snapshotConsumptions
    ),
    deviceConsumptionStates:
      summarizeBillingEntitlementRuntimeConsumptionStates(
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
  contractProof: ParentBillingEntitlementContractProof,
  runtimeProof: BillingEntitlementRuntimeProof,
  expected: {
    readonly parentAccountId: ParentBillingEntitlementContractProof['entitlementSnapshot']['parentAccount']['parentAccountId'];
    readonly familyId: ParentBillingEntitlementContractProof['entitlementSnapshot']['family']['familyId'];
  },
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof = DomainBillingInvoiceTaxRefundDisputeProofReadModel,
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
  contractProof: ParentBillingEntitlementContractProof
): number {
  return contractProof.deviceLimitDecisions.filter(
    (entry) => entry.decision === 'allowed' || entry.decision === 'grace'
  ).length;
}

function summarizeParentVisibleSeatComposition(
  contractProof: ParentBillingEntitlementContractProof
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

function summarizeParentVisibleReferralCreditSummary(
  contractProof: ParentBillingEntitlementContractProof
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
  contractProof: ParentBillingEntitlementContractProof
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
  contractProof: ParentBillingEntitlementContractProof,
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
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
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleInvoiceVisibilityCounts {
  const counts: Record<DomainBillingInvoiceTaxRefundDisputeRow['invoiceVisibility'], number> = {
    'customer-portal-hosted': 0,
    'download-link-issued': 0,
    'manual-support-required': 0,
  };

  for (const row of invoiceProof.rows) {
    counts[row.invoiceVisibility] += 1;
  }

  return BillingParentVisibleInvoiceVisibilityCountsSchema.parse(counts);
}

function summarizeInvoiceRecoveryStates(
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleInvoiceRecoveryStateCounts {
  const counts: Record<DomainBillingInvoiceTaxRefundDisputeRow['recoveryState'], number> = {
    active: 0,
    trialing: 0,
    'past-due': 0,
    grace: 0,
    cancelled: 0,
    unpaid: 0,
    'support-required': 0,
  };

  for (const row of invoiceProof.rows) {
    counts[row.recoveryState] += 1;
  }

  return BillingParentVisibleInvoiceRecoveryStateCountsSchema.parse(counts);
}

function summarizeParentVisibleManualInvoiceState(
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
): BillingParentVisibleManualInvoiceState {
  const manualSupportRequiredCount = invoiceProof.rows.filter(
    (row) => row.invoiceVisibility === 'manual-support-required'
  ).length;
  const manualReviewStateCount = invoiceProof.rows.filter(
    (row) => row.parentVisibleState === 'manual-review'
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
  contractProof: ParentBillingEntitlementContractProof
) {
  return {
    selfServiceVisible: true as const,
    managedBy: 'billing-portal-session-create' as const,
    currentPlanId: contractProof.plan.planId,
    returnPath: BillingHostedReturnRoute.PortalReturn.relativePath,
  };
}

function summarizeParentVisibleCancellationAction(
  contractProof: ParentBillingEntitlementContractProof,
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
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
  contractProof: ParentBillingEntitlementContractProof,
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof
): DomainBillingInvoiceTaxRefundDisputeRow {
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
  invoiceProof: DomainBillingInvoiceTaxRefundDisputeProof,
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-invoice-grace'
    | 'billing-invoice-unpaid'
    | 'billing-tax-manual-support'
    | 'billing-invoice-cancel-immediate'
    | 'billing-invoice-cancel-period-end'
): DomainBillingInvoiceTaxRefundDisputeRow {
  const row = invoiceProof.rows.find((entry) => entry.boundaryId === boundaryId);
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
