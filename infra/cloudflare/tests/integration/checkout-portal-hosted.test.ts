import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  BillingCheckoutSessionResponseSchema,
  BillingPortalSessionResponseSchema,
  BillingSupportAdminAuditEventsResponseSchema,
} from '../../src/generated/billing-contracts.js';
import { createTestHarness, executeRequest, readJson } from '../../src/testing.js';

const interactiveHeaders = {
  origin: 'http://localhost:3000',
  authorization: 'Bearer parent:demo-active',
  'x-ocentra-csrf': 'interactive-parent-session',
};

describe('hosted checkout and portal routes', () => {
  it('creates hosted checkout sessions for paid plans without exposing secrets and audits them once', async () => {
    const harness = createTestHarness();
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'checkout-request-1',
        planId: 'family-core',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'passed-turnstile',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingCheckoutSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.status, 'accepted');
    assert.equal(contract.kind, 'checkout-session-create');
    assert.ok(contract.hostedUrl !== null);
    assert.match(contract.hostedUrl, /^https:\/\/checkout\.stripe\.com\//);
    assert.equal(contract.hostedUrl.includes('client_secret='), false);
    assert.equal((body as { pendingEntitlementConfirmation?: unknown }).pendingEntitlementConfirmation, true);
    assert.equal(harness.queueMessages.length, 0);

    const replay = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'checkout-request-1',
        planId: 'family-core',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'passed-turnstile',
      },
    });
    const replayBody = BillingCheckoutSessionResponseSchema.parse(await readJson<unknown>(replay.response));
    assert.equal(replay.response.status, 200);
    assert.deepEqual(replayBody, contract);

    const audit = await executeRequest({
      path: '/admin/billing/audit?q=checkout-request-1',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = BillingSupportAdminAuditEventsResponseSchema.parse(await readJson<unknown>(audit.response));
    const checkoutEvents = auditBody.results.filter(
      (event: (typeof auditBody.results)[number]) => event.eventId === 'billing-checkout-session:checkout-request-1'
    );
    assert.equal(audit.response.status, 200);
    assert.equal(checkoutEvents.length, 1);
    assert.equal(checkoutEvents[0]?.eventType, 'billing.checkout-session.created');
  });

  it('creates hosted portal sessions on the allowlisted return path and records the audit trail', async () => {
    const harness = createTestHarness();
    const { response } = await executeRequest({
      path: '/auth/billing/portal',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'portal-request-1',
        returnPath: '/family/billing/manage',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingPortalSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.status, 'accepted');
    assert.equal(contract.kind, 'billing-portal-session-create');
    assert.ok(contract.hostedUrl !== null);
    assert.match(contract.hostedUrl, /^https:\/\/billing\.stripe\.com\//);
    assert.equal(harness.queueMessages.length, 0);

    const audit = await executeRequest({
      path: '/admin/billing/audit?q=portal-request-1',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = BillingSupportAdminAuditEventsResponseSchema.parse(await readJson<unknown>(audit.response));
    const portalEvents = auditBody.results.filter(
      (event: (typeof auditBody.results)[number]) => event.eventId === 'billing-portal-session:portal-request-1'
    );
    assert.equal(audit.response.status, 200);
    assert.equal(portalEvents.length, 1);
    assert.equal(portalEvents[0]?.eventType, 'billing.portal-session.created');
  });

  it('rejects checkout requests from non-parent household actors', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer child:demo-child',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: {
        requestId: 'checkout-child-rejected',
        planId: 'family-core',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'trusted-authenticated-session',
      },
    });

    const body = await readJson<unknown>(response);
    const contract = BillingCheckoutSessionResponseSchema.parse(body);
    assert.equal(response.status, 200);
    assert.equal(contract.status, 'rejected');
    assert.equal(contract.rejectionReason, 'unauthorized-role');
  });
});
