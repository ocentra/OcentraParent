import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  BillingCheckoutSessionResponseSchema,
  BillingPortalSessionResponseSchema,
  BillingReferralInviteResultSchema,
  BillingSupportAdminAccountsResponseSchema,
  BillingSupportAdminAuditEventsResponseSchema,
  BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminReferralsResponseSchema,
  BillingSupportAdminReconciliationSummarySchema,
  BillingSupportAdminRefundResultSchema,
} from '../../src/generated/billing-contracts.js';
import { createStripeSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

interface PricingContractResponse {
  plans: Array<{
    planId: string;
    displayName: string;
    priceCents: number;
    currency: string;
  }>;
}

interface BillingChangePlanContractResponse {
  status: string;
  currentPlanId: string;
  targetPlanId: string | null;
}

interface BillingCancelContractResponse {
  status: string;
  cancellationState: string;
  retainsPaidAccessUntil: string;
}

interface BillingWebhookContractResponse {
  status: string;
  provider: string;
  queued: boolean;
  proofIdFamily: string;
  eventId: string;
  eventType: string;
}

interface BillingStatusContractResponse {
  status: string;
  providerMode: string;
  nextRenewalAt: string | null;
  seatComposition: {
    baseIncludedSeats: number;
    activeReferralCredits: number;
    paidExtraSeats: number;
    effectiveLimit: number;
    availableDeviceSlots: number;
  };
  referralSummary: {
    referralCode: string;
    availableCredits: number;
    activeReferredParents: number;
    pendingInvites: number;
    inviteLinkVisible: boolean;
  };
  manualInvoiceState: {
    visible: boolean;
    invoiceState: string | null;
  };
}

interface BillingInvoicesContractResponse {
  invoices: Array<{
    provider: string;
  }>;
}

interface BillingSnapshotContractResponse {
  status?: string;
  snapshot: {
    signatureState: string;
    subscriptionStatus: string;
    source: string;
    deviceLimit?: number;
    availableDeviceSlots?: number;
  };
}

interface BillingLicenseContractResponse {
  decision: string;
  reasonCode: string;
  deviceActivationBehavior?: string;
  requestedDeviceAlreadyTrusted: boolean;
  currentActiveDevices: number;
  limit: number;
}

interface BillingRefundContractResponse {
  requestId: string;
  status: string;
  invoiceId: string | null;
  refundState: string;
  amountCents: number | null;
  auditReference: string;
  rejectionReason: string | null;
}

interface BillingReconciliationContractResponse {
  requestId: string;
  status: string;
  queued: boolean;
  driftFamiliesVisible: number;
  retryBacklogVisible: number;
  deadLetterVisible: number;
  auditReference: string;
}

const interactiveHeaders = {
  origin: 'http://localhost:3000',
  authorization: 'Bearer parent:demo-active',
  'x-ocentra-csrf': 'interactive-parent-session',
};

describe('billing api contract', () => {
  it('returns pricing rows with the public contract fields expected by payment', async () => {
    const { response } = await executeRequest({
      path: '/public/pricing',
    });

    const body = await readJson<PricingContractResponse>(response);
    assert.equal(response.status, 200);
    assert.ok(
      body.plans.every(
        (plan) =>
          typeof plan.planId === 'string' &&
          typeof plan.displayName === 'string' &&
          typeof plan.priceCents === 'number' &&
          typeof plan.currency === 'string'
      )
    );
  });

  it('returns accepted hosted checkout session contracts for paid plans', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'checkout-contract-request',
        planId: 'family-max',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingCheckoutSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.schemaVersion, 'billing-checkout-portal-boundary');
    assert.equal(contract.kind, 'checkout-session-create');
    assert.equal(contract.status, 'accepted');
    assert.equal(contract.rejectionReason, null);
    assert.ok(contract.hostedUrl !== null);
    assert.match(contract.hostedUrl, /^https:\/\/checkout\.stripe\.com\//);
  });

  it('returns explicit rejected checkout contracts for invalid plans', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'checkout-invalid-plan',
        planId: 'family-free',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingCheckoutSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.kind, 'checkout-session-create');
    assert.equal(contract.status, 'rejected');
    assert.equal(contract.rejectionReason, 'invalid-plan');
  });

  it('returns hosted portal contracts on the shared portal schema', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/portal',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'portal-contract-request',
        returnPath: '/family/billing/manage',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingPortalSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.kind, 'billing-portal-session-create');
    assert.equal(contract.status, 'accepted');
    assert.equal(contract.rejectionReason, null);
    assert.ok(contract.hostedUrl !== null);
    assert.match(contract.hostedUrl, /^https:\/\/billing\.stripe\.com\//);
  });

  it('returns implemented change-plan and cancel contracts instead of manual-required placeholders', async () => {
    const changePlanResponse = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'change-plan-contract',
        planId: 'family-max',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const changePlanBody = await readJson<BillingChangePlanContractResponse>(changePlanResponse.response);
    assert.equal(changePlanResponse.response.status, 200);
    assert.equal(changePlanBody.status, 'accepted');
    assert.equal(changePlanBody.currentPlanId, 'family-core');
    assert.equal(changePlanBody.targetPlanId, 'family-max');

    const cancelResponse = await executeRequest({
      path: '/auth/billing/cancel',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'cancel-contract',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const cancelBody = await readJson<BillingCancelContractResponse>(cancelResponse.response);
    assert.equal(cancelResponse.response.status, 200);
    assert.equal(cancelBody.status, 'accepted');
    assert.equal(typeof cancelBody.cancellationState, 'string');
    assert.equal(typeof cancelBody.retainsPaidAccessUntil, 'string');
  });

  it('returns explicit rejected referral invite contracts for household-safe negatives', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'referral-contract-same-household',
        invitee: 'same-household@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingReferralInviteResultSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.status, 'rejected');
    assert.equal(contract.inviteState, null);
    assert.equal(contract.rejectionReason, 'same-household-rejected');
    assert.equal(typeof contract.referralCode, 'string');
  });

  it('returns distinct referral abuse contracts for device-farm rejection and payment-method manual review', async () => {
    const deviceFarm = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'referral-contract-device-farm',
        invitee: 'device-farm@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const deviceFarmContract = BillingReferralInviteResultSchema.parse(await readJson<unknown>(deviceFarm.response));
    assert.equal(deviceFarm.response.status, 200);
    assert.equal(deviceFarmContract.status, 'rejected');
    assert.equal(deviceFarmContract.inviteState, 'fraud-review');
    assert.equal(deviceFarmContract.rejectionReason, 'same-device-farm-rejected');

    const paymentMethod = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      headers: interactiveHeaders,
      body: {
        requestId: 'referral-contract-payment-method',
        invitee: 'same-payment-method@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const paymentMethodContract = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(paymentMethod.response)
    );
    assert.equal(paymentMethod.response.status, 200);
    assert.equal(paymentMethodContract.status, 'manual-review');
    assert.equal(paymentMethodContract.inviteState, 'fraud-review');
    assert.equal(paymentMethodContract.rejectionReason, 'same-payment-method-manual-review');
  });

  it('returns partial refund, invoice-not-found rejection, and reconciliation visibility contracts', async () => {
    const partialRefund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-contract-partial',
        invoiceId: 'parent-demo-active-invoice-current',
        amountCents: 500,
      },
    });
    const partialRefundContract = BillingSupportAdminRefundResultSchema.parse(
      await readJson<BillingRefundContractResponse>(partialRefund.response)
    );
    assert.equal(partialRefund.response.status, 200);
    assert.equal(partialRefundContract.status, 'accepted');
    assert.equal(partialRefundContract.refundState, 'refund-requested');
    assert.equal(partialRefundContract.amountCents, 500);

    const missingInvoiceRefund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-contract-missing',
        invoiceId: 'missing-invoice',
      },
    });
    const missingInvoiceRefundContract = BillingSupportAdminRefundResultSchema.parse(
      await readJson<BillingRefundContractResponse>(missingInvoiceRefund.response)
    );
    assert.equal(missingInvoiceRefund.response.status, 200);
    assert.equal(missingInvoiceRefundContract.status, 'rejected');
    assert.equal(missingInvoiceRefundContract.refundState, 'manual-review-required');
    assert.equal(missingInvoiceRefundContract.rejectionReason, 'invoice-not-found');

    const reconciliation = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-contract-visibility',
      },
    });
    const reconciliationContract = BillingSupportAdminReconciliationSummarySchema.parse(
      await readJson<BillingReconciliationContractResponse>(reconciliation.response)
    );
    assert.equal(reconciliation.response.status, 202);
    assert.equal(reconciliationContract.status, 'accepted');
    assert.equal(reconciliationContract.queued, true);
    assert.equal(reconciliationContract.driftFamiliesVisible, 2);
    assert.equal(reconciliationContract.retryBacklogVisible, 1);
    assert.equal(reconciliationContract.deadLetterVisible, 0);
  });

  it('returns shared support/admin read contracts for account search, invoice search, disputes, referrals, and audit timelines', async () => {
    const accounts = await executeRequest({
      path: '/admin/billing/accounts?q=review',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });
    const accountsContract = BillingSupportAdminAccountsResponseSchema.parse(
      await readJson<unknown>(accounts.response)
    );
    assert.equal(accounts.response.status, 200);
    assert.equal(accountsContract.actorRole, 'support');
    assert.equal(
      accountsContract.manualActionsPending,
      accountsContract.results.filter((row: (typeof accountsContract.results)[number]) => row.manualRequired).length
    );
    assert.equal(accountsContract.nonClaims.includes('no-child-activity-custody'), true);

    const invoices = await executeRequest({
      path: '/admin/billing/invoices?q=INV-',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });
    const invoicesContract = BillingSupportAdminInvoicesResponseSchema.parse(
      await readJson<unknown>(invoices.response)
    );
    assert.equal(invoices.response.status, 200);
    assert.equal(invoicesContract.actorRole, 'support');
    assert.equal(invoicesContract.resultCount >= 1, true);

    const disputes = await executeRequest({
      path: '/admin/billing/disputes',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const disputesContract = BillingSupportAdminDisputesResponseSchema.parse(
      await readJson<unknown>(disputes.response)
    );
    assert.equal(disputes.response.status, 200);
    assert.equal(disputesContract.actorRole, 'admin');

    const referrals = await executeRequest({
      path: '/admin/billing/referrals',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const referralsContract = BillingSupportAdminReferralsResponseSchema.parse(
      await readJson<unknown>(referrals.response)
    );
    assert.equal(referrals.response.status, 200);
    assert.equal(referralsContract.actorRole, 'admin');

    const audit = await executeRequest({
      path: '/admin/billing/audit',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditContract = BillingSupportAdminAuditEventsResponseSchema.parse(await readJson<unknown>(audit.response));
    assert.equal(audit.response.status, 200);
    assert.equal(auditContract.actorRole, 'admin');
    assert.equal(auditContract.results.length >= 1, true);
  });

  it('returns Stripe webhook ack contracts with event identity and proof family', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'evt_contract_valid',
      type: 'invoice.payment_succeeded',
    });
    const signature = await createStripeSignature(payload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: payload,
      headers: {
        'content-type': 'application/json',
        'stripe-signature': signature,
      },
    });

    const body = await readJson<BillingWebhookContractResponse>(response);
    assert.equal(response.status, 202);
    assert.equal(body.status, 'accepted');
    assert.equal(body.provider, 'stripe');
    assert.equal(body.queued, true);
    assert.equal(body.proofIdFamily, 'payment-route.webhook-stripe');
    assert.equal(body.eventId, 'evt_contract_valid');
    assert.equal(body.eventType, 'invoice.payment_succeeded');
    assert.equal(harness.queueMessages.length, 1);
  });

  it('keeps invoices and entitlement snapshot responses billing-safe', async () => {
    const status = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const statusBody = await readJson<BillingStatusContractResponse>(status.response);
    assert.equal(status.response.status, 200);
    assert.equal(statusBody.status, 'ok');
    assert.equal(statusBody.providerMode, 'stripe-hosted');
    assert.equal(statusBody.nextRenewalAt, '2026-07-14T00:00:00.000Z');
    assert.deepEqual(statusBody.seatComposition, {
      baseIncludedSeats: 1,
      activeReferralCredits: 2,
      paidExtraSeats: 2,
      effectiveLimit: 5,
      availableDeviceSlots: 2,
    });
    assert.deepEqual(statusBody.referralSummary, {
      referralCode: 'REF-FAMILY-CORE',
      availableCredits: 2,
      activeReferredParents: 2,
      pendingInvites: 2,
      inviteLinkVisible: true,
    });
    assert.deepEqual(statusBody.manualInvoiceState, {
      visible: false,
      invoiceState: null,
    });

    const invoices = await executeRequest({
      path: '/auth/billing/invoices',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const invoicesBody = await readJson<BillingInvoicesContractResponse>(invoices.response);
    assert.equal(invoices.response.status, 200);
    assert.ok(invoicesBody.invoices.every((invoice) => invoice.provider !== 'raw-provider-payload'));

    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingSnapshotContractResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.status, 'ok');
    assert.equal(snapshotBody.snapshot.signatureState, 'signed');
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'active');
    assert.equal(snapshotBody.snapshot.source, 'signed-local-snapshot');
    assert.equal(snapshotBody.snapshot.deviceLimit, 5);
    assert.equal(snapshotBody.snapshot.availableDeviceSlots, 2);
  });

  it('returns explicit limit-exceeded contracts for over-limit trusted-device activation', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'contract-license-maxed',
        deviceId: 'device-6',
        requestedNewDevice: true,
      },
    });

    const body = await readJson<BillingLicenseContractResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.decision, 'denied');
    assert.equal(body.reasonCode, 'limit-exceeded');
    assert.equal(body.deviceActivationBehavior, 'deny-new-device');
    assert.equal(body.requestedDeviceAlreadyTrusted, false);
    assert.equal(body.currentActiveDevices, 5);
    assert.equal(body.limit, 5);
  });

  it('keeps already-trusted device contracts allowed at the plan limit', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'contract-license-maxed-existing',
        deviceId: 'device-5',
        requestedNewDevice: false,
      },
    });

    const body = await readJson<BillingLicenseContractResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.decision, 'allowed');
    assert.equal(body.reasonCode, 'within-plan');
    assert.equal(body.requestedDeviceAlreadyTrusted, true);
    assert.equal(body.currentActiveDevices, 5);
    assert.equal(body.limit, 5);
  });

  it('preserves manual-required snapshot contracts for manual-admin review accounts', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-review',
        'x-ocentra-trusted-device': 'true',
      },
    });

    const body = await readJson<BillingSnapshotContractResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.snapshot.signatureState, 'manual-required');
    assert.equal(body.snapshot.subscriptionStatus, 'past-due');
    assert.equal(body.snapshot.source, 'manual-admin-review');
  });
});
