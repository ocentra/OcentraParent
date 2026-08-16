import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

describe('kill switch', () => {
  it('halts state-changing authenticated billing, admin, and webhook routes before handler execution', async () => {
    const sharedEnv = {
      BILLING_ROUTE_KILL_SWITCH: 'true',
    } as const;
    const cases = [
      executeRequest({
        path: '/auth/billing/change-plan',
        method: 'POST',
        body: { requestId: 'change-plan-kill-switch', planId: 'family-max' },
        headers: {
          authorization: 'Bearer parent:demo-active',
          origin: 'http://localhost:3000',
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        envOverrides: sharedEnv,
      }),
      executeRequest({
        path: '/admin/billing/refunds',
        method: 'POST',
        body: { requestId: 'admin-refund-kill-switch', invoiceId: 'inv-1', amountCents: 1000 },
        headers: {
          authorization: 'Bearer parent:admin-agent',
          'x-ocentra-role': 'admin',
        },
        envOverrides: sharedEnv,
      }),
      executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        headers: {
          'stripe-signature': 't=1710000000,v1=abcdef',
          'content-length': '0',
        },
        envOverrides: sharedEnv,
      }),
    ];

    for (const pending of cases) {
      const { response } = await pending;
      const body = await readJson<any>(response);
      assert.equal(response.status, 503);
      assert.equal(body.error, 'billing-route-kill-switch-enabled');
      assert.equal(body.status, 'manual-required');
    }
  });

  it('keeps read-only health and pricing routes available while the kill switch is enabled', async () => {
    const health = await executeRequest({
      path: '/health',
      envOverrides: {
        BILLING_ROUTE_KILL_SWITCH: 'true',
      },
    });
    const pricing = await executeRequest({
      path: '/public/pricing',
      envOverrides: {
        BILLING_ROUTE_KILL_SWITCH: 'true',
      },
    });

    assert.equal(health.response.status, 200);
    assert.equal(pricing.response.status, 200);
  });
});
