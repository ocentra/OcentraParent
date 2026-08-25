import { getMissingBindings, resolveAuthAdapterMode, type Env } from './env.js';
import {
  BillingAccountRuntimeStatusRowSchema,
  BillingReferralInviteResultSchema,
  BillingReferralSummarySchema,
  type BillingReferralInviteResult,
  type BillingReferralInviteState,
  type BillingReferralSummary,
  BillingSupportAdminAccountSummarySchema,
  BillingSupportAdminAuditEventSummarySchema,
  BillingSupportAdminDisputeSummarySchema,
  BillingSupportAdminInvoiceSummarySchema,
  BillingSupportAdminReferralSummarySchema,
  type BillingSupportAdminAccountSummary as DomainBillingSupportAdminAccountSummary,
  type BillingSupportAdminAuditEventSummary as DomainBillingSupportAdminAuditEventSummary,
  type BillingSupportAdminDisputeSummary as DomainBillingSupportAdminDisputeSummary,
  type BillingSupportAdminInvoiceSummary as DomainBillingSupportAdminInvoiceSummary,
  type BillingSupportAdminReferralSummary as DomainBillingSupportAdminReferralSummary,
  BillingSupportAdminReconciliationSummarySchema,
  type BillingSupportAdminReconciliationSummary,
} from './generated/billing-contracts.js';

type BillingAccountRuntimeStatusRow = ReturnType<typeof BillingAccountRuntimeStatusRowSchema.parse>;
type ReferralInviteResult = BillingReferralInviteResult;
type BillingStatusAccountStatus = 'trialing' | 'active' | 'grace' | 'manual-review' | 'unavailable';
type BillingStatusSubscriptionStatus = BillingAccountRuntimeStatusRow['subscriptionStatus'];
type BillingStatusParentVisibleState = 'available' | 'grace' | 'stale' | 'unavailable' | 'manual-review';
type BillingStatusLocalSafetyBehavior = BillingAccountRuntimeStatusRow['localSafetyBehavior'];

export interface PricingFeatureSummary {
  code: string;
  label: string;
  included: boolean;
  safetyCritical: boolean;
}

export interface PricingPlanSummary {
  planId: string;
  displayName: string;
  interval: 'monthly' | 'yearly';
  priceCents: number;
  currency: 'USD';
  deviceLimit: number;
  activeState: 'active' | 'trial-only' | 'retired' | 'manual-required';
  featureSummary: ReadonlyArray<PricingFeatureSummary>;
}

export interface ReferralFixture {
  ownerSubject: string;
  referralCode: string;
  invitedFamilies: number;
  creditedFamilies: number;
  status: 'pending' | 'credited';
  updatedAt: string;
}

export interface BillingFailureStateSummary {
  failureKind: string;
  parentResolution: string;
  retryAllowed: boolean;
  retryAfter: string | null;
}

export interface BillingSeatCompositionSummary {
  baseIncludedSeats: number;
  activeReferralCredits: number;
  paidExtraSeats: number;
  effectiveLimit: number;
  availableDeviceSlots: number;
}

export interface BillingReferralStatusSummary {
  referralCode: string | null;
  availableCredits: number;
  activeReferredParents: number;
  pendingInvites: number;
  inviteLinkVisible: boolean;
}

export interface BillingManualInvoiceStateSummary {
  visible: boolean;
  invoiceState: 'manual-support-required' | null;
}

export type BillingPortalVisibleState = 'ready' | 'degraded' | 'stale' | 'offline' | 'manual-required';

export interface BillingStatusSummary {
  status: 'ok';
  environment: string;
  authAdapterMode: string;
  parentAccountRef: string;
  familyRef: string;
  subject: string;
  accountStatus: BillingStatusAccountStatus;
  subscriptionStatus: BillingStatusSubscriptionStatus;
  portalVisibleState: BillingPortalVisibleState;
  parentVisibleState: BillingStatusParentVisibleState;
  localSafetyBehavior: BillingStatusLocalSafetyBehavior;
  childActivityCustody: 'not-included';
  evidenceExportAccess: 'retained';
  providerSecretCustody: 'not-present';
  providerMode: 'stripe-hosted' | 'manual-invoice';
  nextRenewalAt: string | null;
  plan: PricingPlanSummary;
  deviceUsage: {
    activeDevices: number;
    trustedDevices: number;
    limit: number;
  };
  seatComposition: BillingSeatCompositionSummary;
  referralSummary: BillingReferralStatusSummary;
  manualInvoiceState: BillingManualInvoiceStateSummary;
  source: 'signed-local-snapshot' | 'manual-admin-review';
  failureState: BillingFailureStateSummary | null;
  warnings: ReadonlyArray<string>;
  auditReference: string;
  updatedAt: string;
}

export type AdminBillingAccountSummary = DomainBillingSupportAdminAccountSummary;

export interface BillingInvoiceSummary {
  invoiceId: string;
  invoiceNumber: string;
  parentAccountRef: string;
  familyRef: string;
  planId: string;
  currency: 'USD';
  subtotalCents: number;
  taxCents: number;
  totalCents: number;
  invoiceVisibility: 'customer-portal-hosted' | 'manual-support-required';
  paymentState: 'paid' | 'grace' | 'unpaid' | 'refunded';
  provider: 'stripe' | 'manual-invoice';
  hostedUrl: string | null;
  periodStart: string;
  periodEnd: string;
  updatedAt: string;
  auditReference: string;
}

export interface BillingPlanChangeSummary {
  requestId: string;
  status: 'accepted' | 'rejected';
  currentPlanId: string;
  targetPlanId: string | null;
  changeKind: 'upgrade' | 'downgrade' | 'invalid';
  hostedUrl: string | null;
  pendingEntitlementConfirmation: boolean;
  rejectionReason: 'invalid-plan' | null;
  effectiveAt: string | null;
  auditReference: string;
}

export interface BillingCancellationSummary {
  requestId: string;
  status: 'accepted';
  cancellationState: 'scheduled-period-end' | 'already-in-grace' | 'manual-review-required';
  currentPlanId: string;
  retainsPaidAccessUntil: string;
  parentVisibleState: 'available' | 'grace' | 'manual-review' | 'stale' | 'unavailable';
  auditReference: string;
}

export interface BillingEntitlementSnapshotSummary {
  snapshotId: string;
  subject: string;
  parentAccountRef: string;
  familyRef: string;
  planId: string;
  subscriptionStatus: 'active' | 'grace' | 'past-due';
  source: 'signed-local-snapshot' | 'manual-admin-review';
  signatureState: 'signed' | 'manual-required';
  signedAt: string;
  deviceLimit: number;
  activeDevices: number;
  trustedDevices: number;
  availableDeviceSlots: number;
  parentVisibleState: 'available' | 'grace' | 'manual-review';
  localSafetyBehavior: 'unchanged' | 'grace-with-local-safety' | 'manual-review-with-local-safety';
  failureState: BillingFailureStateSummary | null;
  auditReference: string;
}

export interface BillingLicenseDecisionSummary {
  requestId: string;
  subject: string;
  deviceId: string;
  decision: 'allowed' | 'denied' | 'grace' | 'manual-review';
  reasonCode: 'within-plan' | 'limit-exceeded' | 'payment-required' | 'manual-review';
  deviceActivationBehavior:
    | 'allow-new-device'
    | 'deny-new-device'
    | 'grace-existing-devices'
    | 'manual-review-required';
  requestedDeviceAlreadyTrusted: boolean;
  planId: string;
  currentActiveDevices: number;
  limit: number;
  auditReference: string;
}

export interface ManualInvoiceResult {
  requestId: string;
  status: 'accepted';
  invoiceState: 'manual-support-required';
  region: string;
  provider: 'manual-invoice';
  auditReference: string;
}

export type AdminBillingInvoiceSummary = DomainBillingSupportAdminInvoiceSummary;
export type AdminBillingDisputeSummary = DomainBillingSupportAdminDisputeSummary;
export type AdminBillingReferralSummary = DomainBillingSupportAdminReferralSummary;

export type BillingAuditEventSummary = DomainBillingSupportAdminAuditEventSummary;

interface BillingAccountFixture {
  subject: string;
  parentAccountRef: string;
  familyRef: string;
  accountStatus: 'active' | 'grace' | 'manual-review';
  subscriptionStatus: 'active' | 'grace' | 'past-due';
  parentVisibleState: 'available' | 'grace' | 'manual-review';
  localSafetyBehavior: 'unchanged' | 'grace-with-local-safety' | 'manual-review-with-local-safety';
  childActivityCustody: 'not-included';
  evidenceExportAccess: 'retained';
  providerSecretCustody: 'not-present';
  planId: string;
  activeDevices: number;
  trustedDevices: number;
  source: 'signed-local-snapshot' | 'manual-admin-review';
  failureState: BillingFailureStateSummary | null;
  auditReference: string;
  updatedAt: string;
}

const FIXTURE_TIMESTAMP = '2026-06-14T00:00:00.000Z';
const PERIOD_END_TIMESTAMP = '2026-07-14T00:00:00.000Z';

function fixtureBoundaryId(subject: string): string {
  return `billing-account-runtime-${subject.replace(/[^A-Za-z0-9_-]/g, '-')}`;
}

function portalVisibleStateFor(parentVisibleState: BillingStatusParentVisibleState): BillingPortalVisibleState {
  switch (parentVisibleState) {
    case 'available':
      return 'ready';
    case 'grace':
      return 'degraded';
    case 'stale':
      return 'stale';
    case 'unavailable':
      return 'offline';
    case 'manual-review':
      return 'manual-required';
  }
}

export const LOCAL_PRICING_PLANS: ReadonlyArray<PricingPlanSummary> = [
  {
    planId: 'family-free',
    displayName: 'Family Free',
    interval: 'monthly',
    priceCents: 0,
    currency: 'USD',
    deviceLimit: 1,
    activeState: 'active',
    featureSummary: [
      {
        code: 'evidence-export',
        label: 'Evidence export access',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'activity-review',
        label: 'Activity review',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'live-webhooks',
        label: 'Live provider sync',
        included: false,
        safetyCritical: false,
      },
    ],
  },
  {
    planId: 'family-core',
    displayName: 'Family Core',
    interval: 'monthly',
    priceCents: 1299,
    currency: 'USD',
    deviceLimit: 5,
    activeState: 'active',
    featureSummary: [
      {
        code: 'evidence-export',
        label: 'Evidence export access',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'activity-review',
        label: 'Activity review',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'billing-portal',
        label: 'Hosted billing portal',
        included: true,
        safetyCritical: false,
      },
    ],
  },
  {
    planId: 'family-max',
    displayName: 'Family Max',
    interval: 'monthly',
    priceCents: 2499,
    currency: 'USD',
    deviceLimit: 10,
    activeState: 'active',
    featureSummary: [
      {
        code: 'evidence-export',
        label: 'Evidence export access',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'activity-review',
        label: 'Activity review',
        included: true,
        safetyCritical: true,
      },
      {
        code: 'priority-support',
        label: 'Priority support queue',
        included: true,
        safetyCritical: false,
      },
    ],
  },
] as const;

export const LOCAL_REFERRALS: ReadonlyArray<ReferralFixture> = [
  {
    ownerSubject: 'parent:demo-active',
    referralCode: 'REF-FAMILY-CORE',
    invitedFamilies: 4,
    creditedFamilies: 2,
    status: 'credited',
    updatedAt: FIXTURE_TIMESTAMP,
  },
  {
    ownerSubject: 'parent:demo-grace',
    referralCode: 'REF-FAMILY-MAX',
    invitedFamilies: 1,
    creditedFamilies: 0,
    status: 'pending',
    updatedAt: FIXTURE_TIMESTAMP,
  },
] as const;

const LOCAL_BILLING_ACCOUNTS: ReadonlyArray<BillingAccountFixture> = [
  {
    subject: 'parent:demo-active',
    parentAccountRef: 'parent-account:demo-active',
    familyRef: 'family:demo-active',
    accountStatus: 'active',
    subscriptionStatus: 'active',
    parentVisibleState: 'available',
    localSafetyBehavior: 'unchanged',
    childActivityCustody: 'not-included',
    evidenceExportAccess: 'retained',
    providerSecretCustody: 'not-present',
    planId: 'family-core',
    activeDevices: 3,
    trustedDevices: 3,
    source: 'signed-local-snapshot',
    failureState: null,
    auditReference: 'audit:billing:demo-active',
    updatedAt: FIXTURE_TIMESTAMP,
  },
  {
    subject: 'parent:demo-maxed',
    parentAccountRef: 'parent-account:demo-maxed',
    familyRef: 'family:demo-maxed',
    accountStatus: 'active',
    subscriptionStatus: 'active',
    parentVisibleState: 'available',
    localSafetyBehavior: 'unchanged',
    childActivityCustody: 'not-included',
    evidenceExportAccess: 'retained',
    providerSecretCustody: 'not-present',
    planId: 'family-core',
    activeDevices: 5,
    trustedDevices: 5,
    source: 'signed-local-snapshot',
    failureState: null,
    auditReference: 'audit:billing:demo-maxed',
    updatedAt: FIXTURE_TIMESTAMP,
  },
  {
    subject: 'parent:demo-grace',
    parentAccountRef: 'parent-account:demo-grace',
    familyRef: 'family:demo-grace',
    accountStatus: 'grace',
    subscriptionStatus: 'grace',
    parentVisibleState: 'grace',
    localSafetyBehavior: 'grace-with-local-safety',
    childActivityCustody: 'not-included',
    evidenceExportAccess: 'retained',
    providerSecretCustody: 'not-present',
    planId: 'family-max',
    activeDevices: 9,
    trustedDevices: 9,
    source: 'signed-local-snapshot',
    failureState: {
      failureKind: 'payment-required',
      parentResolution: 'payment-update',
      retryAllowed: true,
      retryAfter: '2026-06-15T00:00:00.000Z',
    },
    auditReference: 'audit:billing:demo-grace',
    updatedAt: FIXTURE_TIMESTAMP,
  },
  {
    subject: 'parent:demo-review',
    parentAccountRef: 'parent-account:demo-review',
    familyRef: 'family:demo-review',
    accountStatus: 'manual-review',
    subscriptionStatus: 'past-due',
    parentVisibleState: 'manual-review',
    localSafetyBehavior: 'manual-review-with-local-safety',
    childActivityCustody: 'not-included',
    evidenceExportAccess: 'retained',
    providerSecretCustody: 'not-present',
    planId: 'family-free',
    activeDevices: 1,
    trustedDevices: 1,
    source: 'manual-admin-review',
    failureState: {
      failureKind: 'provider-unavailable',
      parentResolution: 'manual-support-review',
      retryAllowed: false,
      retryAfter: null,
    },
    auditReference: 'audit:billing:demo-review',
    updatedAt: FIXTURE_TIMESTAMP,
  },
] as const;

function planById(planId: string): PricingPlanSummary {
  const plan = LOCAL_PRICING_PLANS.find((entry) => entry.planId === planId);
  return plan ?? LOCAL_PRICING_PLANS[0];
}

function availableDeviceSlotsFor(account: BillingAccountFixture, plan: PricingPlanSummary): number {
  return Math.max(plan.deviceLimit - account.activeDevices, 0);
}

function subjectIndex(subject: string): number {
  let hash = 0;
  for (const char of subject) {
    hash = (hash * 33 + char.charCodeAt(0)) % LOCAL_BILLING_ACCOUNTS.length;
  }
  return hash;
}

function accountFixtureForSubject(subject: string): BillingAccountFixture {
  const exactMatch = LOCAL_BILLING_ACCOUNTS.find((fixture) => fixture.subject === subject);
  if (exactMatch) {
    return exactMatch;
  }
  return LOCAL_BILLING_ACCOUNTS[subjectIndex(subject)];
}

function directReferralFixtureForSubject(subject: string): ReferralFixture | null {
  return LOCAL_REFERRALS.find((entry) => entry.ownerSubject === subject) ?? null;
}

function syntheticInvoiceId(subject: string, suffix: string): string {
  return `${subject.replace(/[^A-Za-z0-9_-]/g, '-')}-${suffix}`;
}

function invoiceRowsForAccount(account: BillingAccountFixture): ReadonlyArray<BillingInvoiceSummary> {
  const plan = planById(account.planId);
  const baseInvoice = {
    parentAccountRef: account.parentAccountRef,
    familyRef: account.familyRef,
    planId: account.planId,
    currency: 'USD' as const,
    periodStart: '2026-06-01T00:00:00.000Z',
    periodEnd: PERIOD_END_TIMESTAMP,
    updatedAt: account.updatedAt,
  };

  if (account.subscriptionStatus === 'active') {
    return [
      {
        ...baseInvoice,
        invoiceId: syntheticInvoiceId(account.subject, 'invoice-current'),
        invoiceNumber: 'INV-1001',
        subtotalCents: plan.priceCents,
        taxCents: Math.round(plan.priceCents * 0.08),
        totalCents: plan.priceCents + Math.round(plan.priceCents * 0.08),
        invoiceVisibility: 'customer-portal-hosted',
        paymentState: 'paid',
        provider: 'stripe',
        hostedUrl: `https://billing.stripe.com/p/session/${syntheticInvoiceId(account.subject, 'invoice')}`,
        auditReference: `${account.auditReference}:invoice-current`,
      },
    ];
  }

  if (account.subscriptionStatus === 'grace') {
    return [
      {
        ...baseInvoice,
        invoiceId: syntheticInvoiceId(account.subject, 'invoice-grace'),
        invoiceNumber: 'INV-2001',
        subtotalCents: plan.priceCents,
        taxCents: Math.round(plan.priceCents * 0.08),
        totalCents: plan.priceCents + Math.round(plan.priceCents * 0.08),
        invoiceVisibility: 'customer-portal-hosted',
        paymentState: 'grace',
        provider: 'stripe',
        hostedUrl: `https://billing.stripe.com/p/session/${syntheticInvoiceId(account.subject, 'grace')}`,
        auditReference: `${account.auditReference}:invoice-grace`,
      },
      {
        ...baseInvoice,
        invoiceId: syntheticInvoiceId(account.subject, 'invoice-previous'),
        invoiceNumber: 'INV-1999',
        subtotalCents: plan.priceCents,
        taxCents: Math.round(plan.priceCents * 0.08),
        totalCents: plan.priceCents + Math.round(plan.priceCents * 0.08),
        invoiceVisibility: 'customer-portal-hosted',
        paymentState: 'paid',
        provider: 'stripe',
        hostedUrl: `https://billing.stripe.com/p/session/${syntheticInvoiceId(account.subject, 'previous')}`,
        auditReference: `${account.auditReference}:invoice-previous`,
      },
    ];
  }

  return [
    {
      ...baseInvoice,
      invoiceId: syntheticInvoiceId(account.subject, 'invoice-review'),
      invoiceNumber: 'INV-3001',
      subtotalCents: 4999,
      taxCents: 0,
      totalCents: 4999,
      invoiceVisibility: 'manual-support-required',
      paymentState: 'unpaid',
      provider: 'manual-invoice',
      hostedUrl: null,
      auditReference: `${account.auditReference}:invoice-review`,
    },
  ];
}

function seatCompositionForAccount(
  account: BillingAccountFixture,
  plan: PricingPlanSummary
): BillingSeatCompositionSummary {
  const referral = directReferralFixtureForSubject(account.subject);
  const baseIncludedSeats = 1;
  const activeReferralCredits = referral?.creditedFamilies ?? 0;
  const paidExtraSeats = Math.max(plan.deviceLimit - baseIncludedSeats - activeReferralCredits, 0);

  return {
    baseIncludedSeats,
    activeReferralCredits,
    paidExtraSeats,
    effectiveLimit: baseIncludedSeats + activeReferralCredits + paidExtraSeats,
    availableDeviceSlots: availableDeviceSlotsFor(account, plan),
  };
}

function providerModeForInvoices(invoices: ReadonlyArray<BillingInvoiceSummary>): 'stripe-hosted' | 'manual-invoice' {
  return invoices.some((invoice) => invoice.provider === 'manual-invoice') ? 'manual-invoice' : 'stripe-hosted';
}

function nextRenewalAtForInvoices(invoices: ReadonlyArray<BillingInvoiceSummary>): string | null {
  const hostedInvoice = invoices.find(
    (invoice) => invoice.provider === 'stripe' && invoice.paymentState !== 'refunded'
  );
  return hostedInvoice?.periodEnd ?? null;
}

function manualInvoiceStateForInvoices(
  invoices: ReadonlyArray<BillingInvoiceSummary>
): BillingManualInvoiceStateSummary {
  const manualInvoiceVisible = invoices.some((invoice) => invoice.invoiceVisibility === 'manual-support-required');

  return {
    visible: manualInvoiceVisible,
    invoiceState: manualInvoiceVisible ? 'manual-support-required' : null,
  };
}

function runtimeAccountStatusForFixture(fixture: BillingAccountFixture): 'active' | 'past-due' | 'manual-review' {
  switch (fixture.accountStatus) {
    case 'active':
      return 'active';
    case 'grace':
      return 'past-due';
    case 'manual-review':
      return 'manual-review';
  }
}

function runtimeParentVisibleStateForFixture(
  fixture: BillingAccountFixture
): 'available' | 'past-due' | 'manual-review' {
  switch (fixture.parentVisibleState) {
    case 'available':
      return 'available';
    case 'grace':
      return 'past-due';
    case 'manual-review':
      return 'manual-review';
  }
}

function runtimeSourceForFixture(fixture: BillingAccountFixture): 'signed-local-snapshot' | 'manual-admin-review' {
  switch (fixture.accountStatus) {
    case 'active':
      return 'signed-local-snapshot';
    case 'grace':
      return 'signed-local-snapshot';
    case 'manual-review':
      return 'manual-admin-review';
  }
}

function runtimeBackendStateForFixture(fixture: BillingAccountFixture): 'ready' | 'degraded' | 'unavailable' {
  switch (fixture.accountStatus) {
    case 'active':
      return 'ready';
    case 'grace':
      return 'degraded';
    case 'manual-review':
      return 'unavailable';
  }
}

function runtimeFailureStateForFixture(fixture: BillingAccountFixture) {
  if (fixture.failureState === null) {
    return null;
  }

  return {
    failureKind: fixture.failureState.failureKind,
    parentVisibleState: runtimeParentVisibleStateForFixture(fixture),
    localSafetyBehavior: fixture.localSafetyBehavior,
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution: fixture.failureState.parentResolution,
    retryAllowed: fixture.failureState.retryAllowed,
    retryAfter: fixture.failureState.retryAfter,
  } as const;
}

function billingStatusAccountStatusForRuntime(
  accountStatus: BillingAccountRuntimeStatusRow['accountStatus']
): BillingStatusAccountStatus {
  switch (accountStatus) {
    case 'trialing':
      return 'trialing';
    case 'active':
      return 'active';
    case 'past-due':
      return 'grace';
    case 'backend-unavailable':
    case 'provider-unavailable':
      return 'unavailable';
    case 'manual-review':
      return 'manual-review';
  }
  throw new Error('unsupported runtime account status');
}

function billingStatusParentVisibleStateForRuntime(
  parentVisibleState: BillingAccountRuntimeStatusRow['parentVisibleState']
): BillingStatusParentVisibleState {
  switch (parentVisibleState) {
    case 'available':
      return 'available';
    case 'past-due':
      return 'grace';
    case 'stale':
      return 'stale';
    case 'unavailable':
      return 'unavailable';
    case 'manual-review':
      return 'manual-review';
  }
  throw new Error('unsupported runtime parent visible state');
}

function billingAccountRuntimeStatusRowForFixture(
  fixture: BillingAccountFixture,
  invoices: ReadonlyArray<BillingInvoiceSummary>
) {
  return BillingAccountRuntimeStatusRowSchema.parse({
    schemaVersion: 'billing-account-runtime-boundary-proof',
    boundaryId: fixtureBoundaryId(fixture.subject),
    parentAccount: {
      parentAccountId: fixture.parentAccountRef,
    },
    family: {
      familyId: fixture.familyRef,
    },
    accountStatus: runtimeAccountStatusForFixture(fixture),
    subscriptionStatus: fixture.subscriptionStatus,
    source: runtimeSourceForFixture(fixture),
    backendRuntimeState: runtimeBackendStateForFixture(fixture),
    parentVisibleState: runtimeParentVisibleStateForFixture(fixture),
    localSafetyBehavior: fixture.localSafetyBehavior,
    evidenceExportAccess: fixture.evidenceExportAccess,
    childActivityCustody: fixture.childActivityCustody,
    providerSecretCustody: fixture.providerSecretCustody,
    providerMode: providerModeForInvoices(invoices),
    nextRenewalAt: nextRenewalAtForInvoices(invoices),
    manualInvoiceState: manualInvoiceStateForInvoices(invoices),
    failureState: runtimeFailureStateForFixture(fixture),
    auditReference: fixture.auditReference,
  });
}

function referralStatusForSubject(subject: string): BillingReferralStatusSummary {
  const referral = directReferralFixtureForSubject(subject);
  if (!referral) {
    return {
      referralCode: null,
      availableCredits: 0,
      activeReferredParents: 0,
      pendingInvites: 0,
      inviteLinkVisible: false,
    };
  }

  return {
    referralCode: referral.referralCode,
    availableCredits: referral.creditedFamilies,
    activeReferredParents: referral.creditedFamilies,
    pendingInvites: Math.max(referral.invitedFamilies - referral.creditedFamilies, 0),
    inviteLinkVisible: true,
  };
}

function disputeRowsForAccount(account: BillingAccountFixture): ReadonlyArray<AdminBillingDisputeSummary> {
  if (account.subject === 'parent:demo-grace') {
    return [
      {
        disputeId: 'dispute-demo-grace-opened',
        parentAccountRef: account.parentAccountRef,
        familyRef: account.familyRef,
        invoiceId: syntheticInvoiceId(account.subject, 'invoice-grace'),
        disputeState: 'dispute-opened',
        entitlementEffect: 'manual-review-required',
        manualRequired: true,
        auditReference: `${account.auditReference}:dispute-opened`,
        updatedAt: account.updatedAt,
      } as AdminBillingDisputeSummary,
    ];
  }

  if (account.subject === 'parent:demo-review') {
    return [
      {
        disputeId: 'dispute-demo-review-lost',
        parentAccountRef: account.parentAccountRef,
        familyRef: account.familyRef,
        invoiceId: syntheticInvoiceId(account.subject, 'invoice-review'),
        disputeState: 'dispute-lost',
        entitlementEffect: 'revoke-paid-access',
        manualRequired: true,
        auditReference: `${account.auditReference}:dispute-lost`,
        updatedAt: account.updatedAt,
      } as AdminBillingDisputeSummary,
    ];
  }

  return [];
}

function auditEventRowsForAccount(account: BillingAccountFixture): ReadonlyArray<BillingAuditEventSummary> {
  return [
    {
      eventId: `${account.auditReference}:status-read`,
      eventType: 'billing.status.read',
      actorRole: 'parent',
      parentAccountRef: account.parentAccountRef,
      familyRef: account.familyRef,
      auditReference: account.auditReference,
      createdAt: account.updatedAt,
    } as BillingAuditEventSummary,
    {
      eventId: `${account.auditReference}:invoice-visible`,
      eventType: 'billing.invoice.visible',
      actorRole: account.accountStatus === 'manual-review' ? 'support' : 'parent',
      parentAccountRef: account.parentAccountRef,
      familyRef: account.familyRef,
      auditReference: `${account.auditReference}:invoice`,
      createdAt: account.updatedAt,
    } as BillingAuditEventSummary,
  ];
}

function inviteStateForReferral(referral: ReferralFixture): BillingReferralInviteState {
  if (referral.status === 'credited') {
    return 'qualified-credit-granted';
  }
  return 'invite-created';
}

export function buildBillingStatusSummary(subject: string, env: Env): BillingStatusSummary {
  const fixture = accountFixtureForSubject(subject);
  const missingBindings = getMissingBindings(env);
  const warnings: string[] = [];

  if (resolveAuthAdapterMode(env).startsWith('account-auth-adapter')) {
    warnings.push('account-backend-not-wired');
  }
  if (missingBindings.length > 0) {
    warnings.push(`missing-bindings:${missingBindings.join(',')}`);
  }

  const plan = planById(fixture.planId);
  const invoices = invoiceRowsForAccount(fixture);
  const runtimeStatusRow = billingAccountRuntimeStatusRowForFixture(fixture, invoices);
  const parentVisibleState = billingStatusParentVisibleStateForRuntime(runtimeStatusRow.parentVisibleState);

  return {
    status: 'ok',
    environment: env.ENVIRONMENT,
    authAdapterMode: resolveAuthAdapterMode(env),
    parentAccountRef: fixture.parentAccountRef,
    familyRef: fixture.familyRef,
    subject,
    accountStatus: billingStatusAccountStatusForRuntime(runtimeStatusRow.accountStatus),
    subscriptionStatus: runtimeStatusRow.subscriptionStatus,
    portalVisibleState: portalVisibleStateFor(parentVisibleState),
    parentVisibleState,
    localSafetyBehavior: runtimeStatusRow.localSafetyBehavior,
    childActivityCustody: runtimeStatusRow.childActivityCustody,
    evidenceExportAccess: runtimeStatusRow.evidenceExportAccess,
    providerSecretCustody: runtimeStatusRow.providerSecretCustody,
    providerMode: runtimeStatusRow.providerMode,
    nextRenewalAt: runtimeStatusRow.nextRenewalAt,
    plan,
    deviceUsage: {
      activeDevices: fixture.activeDevices,
      trustedDevices: fixture.trustedDevices,
      limit: plan.deviceLimit,
    },
    seatComposition: seatCompositionForAccount(fixture, plan),
    referralSummary: referralStatusForSubject(subject),
    manualInvoiceState: runtimeStatusRow.manualInvoiceState,
    source: fixture.source,
    failureState:
      runtimeStatusRow.failureState === null
        ? null
        : {
            failureKind: runtimeStatusRow.failureState.failureKind,
            parentResolution: runtimeStatusRow.failureState.parentResolution,
            retryAllowed: runtimeStatusRow.failureState.retryAllowed,
            retryAfter: runtimeStatusRow.failureState.retryAfter,
          },
    warnings,
    auditReference: fixture.auditReference,
    updatedAt: fixture.updatedAt,
  };
}

export function buildBillingInvoices(subject: string): ReadonlyArray<BillingInvoiceSummary> {
  return invoiceRowsForAccount(accountFixtureForSubject(subject));
}

function planChangeSummaryFromCurrentPlan(
  currentPlan: PricingPlanSummary,
  auditReference: string,
  requestId: string,
  targetPlanId: string | null,
  pricingPlans: ReadonlyArray<PricingPlanSummary>
): BillingPlanChangeSummary {
  const targetPlan = targetPlanId ? (pricingPlans.find((plan) => plan.planId === targetPlanId) ?? null) : null;

  if (!targetPlan || targetPlan.planId === currentPlan.planId || targetPlan.priceCents === 0) {
    return {
      requestId,
      status: 'rejected',
      currentPlanId: currentPlan.planId,
      targetPlanId,
      changeKind: 'invalid',
      hostedUrl: null,
      pendingEntitlementConfirmation: false,
      rejectionReason: 'invalid-plan',
      effectiveAt: null,
      auditReference: `${auditReference}:change-plan-rejected`,
    };
  }

  const changeKind = targetPlan.priceCents > currentPlan.priceCents ? 'upgrade' : 'downgrade';
  return {
    requestId,
    status: 'accepted',
    currentPlanId: currentPlan.planId,
    targetPlanId: targetPlan.planId,
    changeKind,
    hostedUrl:
      changeKind === 'upgrade'
        ? `https://checkout.stripe.com/c/pay/change-${targetPlan.planId}`
        : `https://billing.stripe.com/p/session/change-${targetPlan.planId}`,
    pendingEntitlementConfirmation: true,
    rejectionReason: null,
    effectiveAt: PERIOD_END_TIMESTAMP,
    auditReference: `${auditReference}:change-plan-${changeKind}`,
  };
}

function cancellationStateForStatus(
  accountStatus: BillingStatusSummary['accountStatus'],
  subscriptionStatus: BillingStatusSummary['subscriptionStatus']
): BillingCancellationSummary['cancellationState'] {
  if (accountStatus === 'manual-review') {
    return 'manual-review-required';
  }
  if (subscriptionStatus === 'grace') {
    return 'already-in-grace';
  }
  return 'scheduled-period-end';
}

export function buildBillingPlanChangeSummaryFromStatus(
  status: BillingStatusSummary,
  requestId: string,
  targetPlanId: string | null,
  pricingPlans: ReadonlyArray<PricingPlanSummary>
): BillingPlanChangeSummary {
  return planChangeSummaryFromCurrentPlan(status.plan, status.auditReference, requestId, targetPlanId, pricingPlans);
}

export function buildBillingPlanChangeSummary(
  subject: string,
  requestId: string,
  targetPlanId: string | null
): BillingPlanChangeSummary {
  const account = accountFixtureForSubject(subject);
  return planChangeSummaryFromCurrentPlan(
    planById(account.planId),
    account.auditReference,
    requestId,
    targetPlanId,
    LOCAL_PRICING_PLANS
  );
}

export function buildBillingCancellationSummaryFromStatus(
  status: BillingStatusSummary,
  requestId: string
): BillingCancellationSummary {
  return {
    requestId,
    status: 'accepted',
    cancellationState: cancellationStateForStatus(status.accountStatus, status.subscriptionStatus),
    currentPlanId: status.plan.planId,
    retainsPaidAccessUntil: status.nextRenewalAt ?? PERIOD_END_TIMESTAMP,
    parentVisibleState: status.parentVisibleState,
    auditReference: `${status.auditReference}:cancel`,
  };
}

export function buildBillingCancellationSummary(subject: string, requestId: string): BillingCancellationSummary {
  return buildBillingCancellationSummaryFromStatus(
    buildBillingStatusSummary(subject, {
      ENVIRONMENT: 'local',
      AUTH_ADAPTER_MODE: 'local-safe-fixture',
    } as Env),
    requestId
  );
}

export function buildBillingReferralSummary(subject: string): BillingReferralSummary {
  const referral =
    LOCAL_REFERRALS.find((entry) => entry.ownerSubject === subject) ??
    LOCAL_REFERRALS[subjectIndex(subject) % LOCAL_REFERRALS.length];
  const account = accountFixtureForSubject(subject);

  return BillingReferralSummarySchema.parse({
    subject,
    referralCode: referral.referralCode,
    availableCredits: referral.creditedFamilies,
    activeReferredParents: referral.creditedFamilies,
    pendingInvites: Math.max(referral.invitedFamilies - referral.creditedFamilies, 0),
    invites: [
      {
        inviteId: `${referral.referralCode}-invite-1`,
        inviteState: inviteStateForReferral(referral),
        referralCode: referral.referralCode,
        invitedIdentifier: `invite+${referral.referralCode.toLowerCase()}@example.com`,
        auditReference: `${account.auditReference}:referral-invite`,
        updatedAt: referral.updatedAt,
      },
    ],
    auditReference: `${account.auditReference}:referrals`,
  });
}

export function buildReferralInviteResult(
  subject: string,
  requestId: string,
  invitedIdentifier: string | null
): ReferralInviteResult {
  const referral = buildBillingReferralSummary(subject);
  const normalizedInvite = invitedIdentifier?.trim().toLowerCase() ?? '';

  if (normalizedInvite.length === 0) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'manual-review',
      inviteState: 'fraud-review',
      referralCode: referral.referralCode,
      rejectionReason: 'fraud-review',
      auditReference: `${referral.auditReference}:invite-fraud-review`,
    });
  }

  if (normalizedInvite.includes(subject.replace(/[^A-Za-z0-9]/g, '').toLowerCase())) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'rejected',
      inviteState: null,
      referralCode: referral.referralCode,
      rejectionReason: 'self-referral-rejected',
      auditReference: `${referral.auditReference}:invite-self-rejected`,
    });
  }

  if (normalizedInvite.includes('same-household')) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'rejected',
      inviteState: null,
      referralCode: referral.referralCode,
      rejectionReason: 'same-household-rejected',
      auditReference: `${referral.auditReference}:invite-household-rejected`,
    });
  }

  if (normalizedInvite.includes('device-farm')) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'rejected',
      inviteState: 'fraud-review',
      referralCode: referral.referralCode,
      rejectionReason: 'same-device-farm-rejected',
      auditReference: `${referral.auditReference}:invite-device-farm-review`,
    });
  }

  if (normalizedInvite.includes('same-payment-method') || normalizedInvite.includes('shared-card')) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'manual-review',
      inviteState: 'fraud-review',
      referralCode: referral.referralCode,
      rejectionReason: 'same-payment-method-manual-review',
      auditReference: `${referral.auditReference}:invite-payment-method-review`,
    });
  }

  if (normalizedInvite.includes('fraud-review')) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'manual-review',
      inviteState: 'fraud-review',
      referralCode: referral.referralCode,
      rejectionReason: 'fraud-review',
      auditReference: `${referral.auditReference}:invite-fraud-review`,
    });
  }

  return BillingReferralInviteResultSchema.parse({
    requestId,
    status: 'accepted',
    inviteState: 'invite-created',
    referralCode: referral.referralCode,
    rejectionReason: null,
    auditReference: `${referral.auditReference}:invite-created`,
  });
}

export function buildEntitlementSnapshot(subject: string): BillingEntitlementSnapshotSummary {
  const account = accountFixtureForSubject(subject);
  const plan = planById(account.planId);

  return {
    snapshotId: `snapshot-${subject.replace(/[^A-Za-z0-9_-]/g, '-')}`,
    subject,
    parentAccountRef: account.parentAccountRef,
    familyRef: account.familyRef,
    planId: plan.planId,
    subscriptionStatus: account.subscriptionStatus,
    source: account.source,
    signatureState: account.source === 'manual-admin-review' ? 'manual-required' : 'signed',
    signedAt: account.updatedAt,
    deviceLimit: plan.deviceLimit,
    activeDevices: account.activeDevices,
    trustedDevices: account.trustedDevices,
    availableDeviceSlots: availableDeviceSlotsFor(account, plan),
    parentVisibleState: account.parentVisibleState,
    localSafetyBehavior: account.localSafetyBehavior,
    failureState: account.failureState,
    auditReference: `${account.auditReference}:snapshot`,
  };
}

export function buildLicenseDecision(
  subject: string,
  requestId: string,
  deviceId: string,
  requestedNewDevice: boolean
): BillingLicenseDecisionSummary {
  const account = accountFixtureForSubject(subject);
  const plan = planById(account.planId);
  const requestedDeviceAlreadyTrusted = !requestedNewDevice;
  const atDeviceLimit = account.activeDevices >= plan.deviceLimit;

  if (account.accountStatus === 'manual-review') {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'manual-review',
      reasonCode: 'manual-review',
      deviceActivationBehavior: 'manual-review-required',
      requestedDeviceAlreadyTrusted,
      planId: plan.planId,
      currentActiveDevices: account.activeDevices,
      limit: plan.deviceLimit,
      auditReference: `${account.auditReference}:license-check-review`,
    };
  }

  if (requestedNewDevice && atDeviceLimit) {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'denied',
      reasonCode: 'limit-exceeded',
      deviceActivationBehavior: 'deny-new-device',
      requestedDeviceAlreadyTrusted,
      planId: plan.planId,
      currentActiveDevices: account.activeDevices,
      limit: plan.deviceLimit,
      auditReference: `${account.auditReference}:license-check-denied`,
    };
  }

  if (account.subscriptionStatus === 'grace' && requestedNewDevice) {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'grace',
      reasonCode: 'payment-required',
      deviceActivationBehavior: 'grace-existing-devices',
      requestedDeviceAlreadyTrusted,
      planId: plan.planId,
      currentActiveDevices: account.activeDevices,
      limit: plan.deviceLimit,
      auditReference: `${account.auditReference}:license-check-grace`,
    };
  }

  return {
    requestId,
    subject,
    deviceId,
    decision: 'allowed',
    reasonCode: 'within-plan',
    deviceActivationBehavior: 'allow-new-device',
    requestedDeviceAlreadyTrusted,
    planId: plan.planId,
    currentActiveDevices: account.activeDevices,
    limit: plan.deviceLimit,
    auditReference: `${account.auditReference}:license-check-allowed`,
  };
}

export function buildManualInvoiceResult(
  subject: string,
  requestId: string,
  region: string | null
): ManualInvoiceResult {
  const account = accountFixtureForSubject(subject);
  return {
    requestId,
    status: 'accepted',
    invoiceState: 'manual-support-required',
    region: region?.trim() || 'manual-enterprise',
    provider: 'manual-invoice',
    auditReference: `${account.auditReference}:manual-invoice`,
  };
}

export function listAdminBillingAccounts(query: string | null): ReadonlyArray<AdminBillingAccountSummary> {
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const rows: ReadonlyArray<AdminBillingAccountSummary> = LOCAL_BILLING_ACCOUNTS.map((fixture) =>
    BillingSupportAdminAccountSummarySchema.parse({
      parentAccountRef: fixture.parentAccountRef,
      familyRef: fixture.familyRef,
      parentVisibleState: fixture.parentVisibleState,
      subscriptionStatus: fixture.subscriptionStatus,
      planId: fixture.planId,
      evidenceExportAccess: fixture.evidenceExportAccess,
      childActivityCustody: fixture.childActivityCustody,
      providerSecretCustody: fixture.providerSecretCustody,
      manualRequired: fixture.accountStatus === 'manual-review',
      failureKind: fixture.failureState?.failureKind ?? null,
      auditReference: fixture.auditReference,
      updatedAt: fixture.updatedAt,
    })
  );

  if (!loweredQuery) {
    return rows;
  }

  return rows.filter((row) =>
    [row.parentAccountRef, row.familyRef, row.planId, row.parentVisibleState].some((value) =>
      value.toLowerCase().includes(loweredQuery)
    )
  );
}

export function listAdminBillingInvoices(query: string | null): ReadonlyArray<AdminBillingInvoiceSummary> {
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const rows = LOCAL_BILLING_ACCOUNTS.flatMap((account) =>
    invoiceRowsForAccount(account).map((invoice) =>
      BillingSupportAdminInvoiceSummarySchema.parse({
        ...invoice,
        manualRequired: invoice.invoiceVisibility === 'manual-support-required',
      })
    )
  );

  if (!loweredQuery) {
    return rows;
  }

  return rows.filter((row) =>
    [row.invoiceId, row.invoiceNumber, row.parentAccountRef, row.familyRef, row.planId].some((value) =>
      value.toLowerCase().includes(loweredQuery)
    )
  );
}

export function listAdminBillingDisputes(query: string | null): ReadonlyArray<AdminBillingDisputeSummary> {
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const rows = LOCAL_BILLING_ACCOUNTS.flatMap((account) =>
    disputeRowsForAccount(account).map((row) => BillingSupportAdminDisputeSummarySchema.parse(row))
  );

  if (!loweredQuery) {
    return rows;
  }

  return rows.filter((row) =>
    [row.disputeId, row.parentAccountRef, row.familyRef, row.invoiceId, row.disputeState].some((value) =>
      value.toLowerCase().includes(loweredQuery)
    )
  );
}

export function listAdminBillingReferrals(query: string | null): ReadonlyArray<AdminBillingReferralSummary> {
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const rows: ReadonlyArray<AdminBillingReferralSummary> = LOCAL_REFERRALS.map((referral) =>
    BillingSupportAdminReferralSummarySchema.parse({
      referralCode: referral.referralCode,
      ownerSubject: referral.ownerSubject,
      creditedFamilies: referral.creditedFamilies,
      invitedFamilies: referral.invitedFamilies,
      abuseReviewState: referral.status === 'pending' ? 'review-required' : 'clear',
      auditReference: `audit:referral:${referral.referralCode.toLowerCase()}`,
      updatedAt: referral.updatedAt,
    })
  );

  if (!loweredQuery) {
    return rows;
  }

  return rows.filter((row) =>
    [row.referralCode, row.ownerSubject, row.abuseReviewState].some((value) =>
      value.toLowerCase().includes(loweredQuery)
    )
  );
}

export function buildReconciliationSummary(
  requestId: string,
  queued: boolean
): BillingSupportAdminReconciliationSummary {
  return BillingSupportAdminReconciliationSummarySchema.parse({
    requestId,
    status: 'accepted',
    queued,
    driftFamiliesVisible: 2,
    retryBacklogVisible: 1,
    deadLetterVisible: 0,
    auditReference: 'audit:billing:reconciliation',
  });
}

export function listBillingAuditEvents(query: string | null): ReadonlyArray<BillingAuditEventSummary> {
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const rows = LOCAL_BILLING_ACCOUNTS.flatMap((account) =>
    auditEventRowsForAccount(account).map((event) => BillingSupportAdminAuditEventSummarySchema.parse(event))
  );

  if (!loweredQuery) {
    return rows;
  }

  return rows.filter((row) =>
    [row.eventId, row.eventType, row.parentAccountRef, row.familyRef, row.actorRole].some((value) =>
      value.toLowerCase().includes(loweredQuery)
    )
  );
}

export function buildLocalSeedSummary(env: Env): {
  generatedAt: string;
  environment: string;
  authAdapterMode: string;
  pricingPlanCount: number;
  adminAccountCount: number;
  referralFixtureCount: number;
  manualReviewAccountCount: number;
} {
  const adminAccounts = listAdminBillingAccounts(null);
  return {
    generatedAt: FIXTURE_TIMESTAMP,
    environment: env.ENVIRONMENT,
    authAdapterMode: resolveAuthAdapterMode(env),
    pricingPlanCount: LOCAL_PRICING_PLANS.length,
    adminAccountCount: adminAccounts.length,
    referralFixtureCount: LOCAL_REFERRALS.length,
    manualReviewAccountCount: adminAccounts.filter((account) => account.manualRequired).length,
  };
}
