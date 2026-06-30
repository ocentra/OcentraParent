import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

describe('admin auth rejection boundaries', () => {
  it('rejects admin and support routes when auth is missing', async () => {
    const supportRoute = await executeRequest({
      path: '/admin/billing/accounts',
    });
    const adminRoute = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      body: {
        requestId: 'refund-missing-auth',
        invoiceId: 'invoice-1',
        amountCents: 1000,
      },
    });

    const supportBody = await readJson<any>(supportRoute.response);
    const adminBody = await readJson<any>(adminRoute.response);
    assert.equal(supportRoute.response.status, 401);
    assert.equal(adminRoute.response.status, 401);
    assert.equal(supportBody.error, 'authentication-required');
    assert.equal(adminBody.error, 'authentication-required');
    assert.equal(supportBody.missingHeader, 'authorization');
    assert.equal(adminBody.missingHeader, 'authorization');
  });

  it('rejects callers that only satisfy parent-session auth for admin and support billing routes', async () => {
    const supportRoute = await executeRequest({
      path: '/admin/billing/accounts',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const adminRoute = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      body: {
        requestId: 'refund-parent-only',
        invoiceId: 'invoice-1',
        amountCents: 1000,
      },
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });

    const supportBody = await readJson<any>(supportRoute.response);
    const adminBody = await readJson<any>(adminRoute.response);
    assert.equal(supportRoute.response.status, 403);
    assert.equal(adminRoute.response.status, 403);
    assert.equal(supportBody.reason, 'support-role-required');
    assert.equal(adminBody.reason, 'admin-role-required');
  });

  it('rejects support-owned routes when callers lack support or admin authority', async () => {
    const { response } = await executeRequest({
      path: '/admin/billing/accounts',
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-role': 'parent',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 403);
    assert.equal(body.reason, 'support-role-required');
  });

  it('keeps admin and support rejection payloads redacted and audit-safe', async () => {
    const { response } = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      body: {
        requestId: 'refund-audit-safe',
        invoiceId: 'invoice-1',
        amountCents: 1000,
      },
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
      envOverrides: {
        STRIPE_SECRET_KEY: 'sk_live_admin_secret',
      },
    });

    const text = await response.text();
    const body = JSON.parse(text) as Record<string, unknown>;
    assert.equal(response.status, 403);
    assert.deepEqual(Object.keys(body).sort(), ['authState', 'error', 'reason']);
    assert.equal(text.includes('sk_live_admin_secret'), false);
    assert.equal(text.includes('authorization'), false);
  });

  it('returns manual-required when the account-auth provider decision is still unresolved', async () => {
    const { response } = await executeRequest({
      path: '/admin/billing/accounts',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
      envOverrides: {
        AUTH_ADAPTER_MODE: 'account-auth-adapter-manual-required',
        STRIPE_SECRET_KEY: 'sk_live_manual_required_secret',
      },
    });

    const text = await response.text();
    const body = JSON.parse(text) as Record<string, unknown>;
    assert.equal(response.status, 503);
    assert.equal(body.error, 'manual-required');
    assert.equal(body.authState, 'support-required');
    assert.equal(body.blocker, 'account-auth-adapter-manual-required');
    assert.equal(text.includes('sk_live_manual_required_secret'), false);
    assert.equal(text.includes('authorization'), false);
  });
});
