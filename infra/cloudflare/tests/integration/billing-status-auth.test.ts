import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

interface BillingStatusResponse {
  status: 'ok';
  parentAccountRef: string;
  portalVisibleState: 'ready' | 'degraded' | 'stale' | 'offline' | 'manual-required';
  childActivityCustody: 'not-included';
  providerSecretCustody: 'not-present';
  providerMode: 'stripe-hosted' | 'manual-invoice';
  nextRenewalAt: string | null;
  seatComposition: {
    baseIncludedSeats: number;
    activeReferralCredits: number;
    paidExtraSeats: number;
    effectiveLimit: number;
    availableDeviceSlots: number;
  };
  referralSummary: {
    referralCode: string | null;
    availableCredits: number;
    activeReferredParents: number;
    pendingInvites: number;
    inviteLinkVisible: boolean;
  };
  manualInvoiceState: {
    visible: boolean;
    invoiceState: 'manual-support-required' | null;
  };
}

describe('GET /auth/billing/status', () => {
  it('rejects missing authorization', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 401);
    assert.equal(body.error, 'authentication-required');
  });

  it('returns a support-safe billing status summary for authenticated parents', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });

    const body = await readJson<BillingStatusResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.equal(body.parentAccountRef, 'parent-account:demo-grace');
    assert.equal(body.portalVisibleState, 'degraded');
    assert.equal(body.childActivityCustody, 'not-included');
    assert.equal(body.providerSecretCustody, 'not-present');
    assert.equal(body.providerMode, 'stripe-hosted');
    assert.equal(body.nextRenewalAt, '2026-07-14T00:00:00.000Z');
    assert.deepEqual(body.seatComposition, {
      baseIncludedSeats: 1,
      activeReferralCredits: 0,
      paidExtraSeats: 9,
      effectiveLimit: 10,
      availableDeviceSlots: 1,
    });
    assert.deepEqual(body.referralSummary, {
      referralCode: 'REF-FAMILY-MAX',
      availableCredits: 0,
      activeReferredParents: 0,
      pendingInvites: 1,
      inviteLinkVisible: true,
    });
    assert.deepEqual(body.manualInvoiceState, {
      visible: false,
      invoiceState: null,
    });
  });

  it('fails trusted-device billing routes with an explicit trusted-device-required outcome', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'forbidden');
    assert.equal(body.authState, 'trusted-parent-device-required');
    assert.equal(body.reason, 'trusted-parent-device-required');
  });

  it('keeps degraded and manual-review status responses redacted and client-safe', async () => {
    const stripeSecretKey = ['sk', 'live', 'status', 'fixture'].join('_');
    const stripeWebhookSecret = ['whsec', 'status', 'fixture'].join('_');
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
      envOverrides: {
        STRIPE_SECRET_KEY: stripeSecretKey,
        STRIPE_WEBHOOK_SECRET: stripeWebhookSecret,
      },
    });

    const text = await response.text();
    const body = JSON.parse(text) as BillingStatusResponse & { failureState?: unknown };
    assert.equal(response.status, 200);
    assert.equal(body.portalVisibleState, 'manual-required');
    assert.equal(text.includes(stripeSecretKey), false);
    assert.equal(text.includes(stripeWebhookSecret), false);
    assert.equal(text.includes('evidence://'), false);
    assert.equal(text.includes('support-bundle-secret'), false);
    assert.ok(body.failureState !== undefined);
  });
});
