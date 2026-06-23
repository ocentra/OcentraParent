import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

interface BillingStatusErrorResponse {
  error: string;
  missingHeader?: string;
}

interface BillingFailureStateSummary {
  failureKind: string;
  parentResolution: string;
  retryAllowed: boolean;
  retryAfter: string | null;
}

interface BillingStatusResponse {
  status: 'ok';
  parentAccountRef: string;
  portalVisibleState: 'ready' | 'degraded' | 'stale' | 'offline' | 'manual-required';
  parentVisibleState: 'available' | 'grace' | 'manual-review';
  localSafetyBehavior: 'unchanged' | 'grace-with-local-safety' | 'manual-review-with-local-safety';
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
  warnings: string[];
  failureState: BillingFailureStateSummary | null;
}

describe('portal to worker billing status flow', () => {
  it('rejects portal callers without parent auth before returning any billing state', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        origin: 'http://localhost:3000',
      },
    });

    const body = await readJson<BillingStatusErrorResponse>(response);
    assert.equal(response.status, 401);
    assert.equal(body.error, 'authentication-required');
    assert.equal(body.missingHeader, 'authorization');
  });

  it('accepts the local portal origin and returns a support-safe active billing status', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer parent:demo-active',
      },
    });

    const body = await readJson<BillingStatusResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(response.headers.get('access-control-allow-origin'), 'http://localhost:3000');
    assert.equal(body.parentAccountRef, 'parent-account:demo-active');
    assert.equal(body.portalVisibleState, 'ready');
    assert.equal(body.parentVisibleState, 'available');
    assert.equal(body.localSafetyBehavior, 'unchanged');
    assert.equal(body.childActivityCustody, 'not-included');
    assert.equal(body.providerSecretCustody, 'not-present');
    assert.equal(body.providerMode, 'stripe-hosted');
    assert.equal(body.nextRenewalAt, '2026-07-14T00:00:00.000Z');
    assert.deepEqual(body.seatComposition, {
      baseIncludedSeats: 1,
      activeReferralCredits: 2,
      paidExtraSeats: 2,
      effectiveLimit: 5,
      availableDeviceSlots: 2,
    });
    assert.deepEqual(body.referralSummary, {
      referralCode: 'REF-FAMILY-CORE',
      availableCredits: 2,
      activeReferredParents: 2,
      pendingInvites: 2,
      inviteLinkVisible: true,
    });
    assert.deepEqual(body.manualInvoiceState, {
      visible: false,
      invoiceState: null,
    });
    assert.deepEqual(body.warnings, []);
    assert.equal(body.failureState, null);
  });

  it('surfaces grace state through a redacted portal-safe summary instead of a raw worker failure', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        origin: 'http://127.0.0.1:3000',
        authorization: 'Bearer parent:demo-grace',
      },
    });

    const body = await readJson<BillingStatusResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(response.headers.get('access-control-allow-origin'), 'http://127.0.0.1:3000');
    assert.equal(body.portalVisibleState, 'degraded');
    assert.equal(body.parentVisibleState, 'grace');
    assert.equal(body.localSafetyBehavior, 'grace-with-local-safety');
    assert.equal(body.childActivityCustody, 'not-included');
    assert.equal(body.providerSecretCustody, 'not-present');
    assert.equal(body.providerMode, 'stripe-hosted');
    assert.equal(body.nextRenewalAt, '2026-07-14T00:00:00.000Z');
    assert.deepEqual(body.manualInvoiceState, {
      visible: false,
      invoiceState: null,
    });
    assert.deepEqual(body.failureState, {
      failureKind: 'payment-required',
      parentResolution: 'payment-update',
      retryAllowed: true,
      retryAfter: '2026-06-15T00:00:00.000Z',
    });
  });

  it('keeps manual-review billing states redacted for the portal boundary', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer parent:demo-review',
      },
    });

    const body = await readJson<BillingStatusResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.portalVisibleState, 'manual-required');
    assert.equal(body.parentVisibleState, 'manual-review');
    assert.equal(body.localSafetyBehavior, 'manual-review-with-local-safety');
    assert.equal(body.childActivityCustody, 'not-included');
    assert.equal(body.providerSecretCustody, 'not-present');
    assert.equal(body.providerMode, 'manual-invoice');
    assert.equal(body.nextRenewalAt, null);
    assert.deepEqual(body.seatComposition, {
      baseIncludedSeats: 1,
      activeReferralCredits: 0,
      paidExtraSeats: 0,
      effectiveLimit: 1,
      availableDeviceSlots: 0,
    });
    assert.deepEqual(body.referralSummary, {
      referralCode: null,
      availableCredits: 0,
      activeReferredParents: 0,
      pendingInvites: 0,
      inviteLinkVisible: false,
    });
    assert.deepEqual(body.manualInvoiceState, {
      visible: true,
      invoiceState: 'manual-support-required',
    });
    assert.deepEqual(body.failureState, {
      failureKind: 'provider-unavailable',
      parentResolution: 'manual-support-review',
      retryAllowed: false,
      retryAfter: null,
    });
  });
});
