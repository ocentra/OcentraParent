import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

describe('GET /public/pricing', () => {
  it('is reachable without private billing auth and returns local-safe pricing fixtures', async () => {
    const { response } = await executeRequest({
      path: '/public/pricing',
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.equal(body.plans.length, 3);
    assert.ok(body.plans.every((plan: any) => typeof plan.planId === 'string'));
    assert.ok(body.plans.some((plan: any) => plan.featureSummary.some((feature: any) => feature.safetyCritical)));
  });

  it('does not disclose provider secrets or admin-only data', async () => {
    const stripeSecretKey = ['sk', 'live', 'price', 'fixture'].join('_');
    const paypalClientSecret = ['paypal', 'price', 'fixture'].join('-');
    const { response } = await executeRequest({
      path: '/public/pricing',
      envOverrides: {
        STRIPE_SECRET_KEY: stripeSecretKey,
        PAYPAL_CLIENT_SECRET: paypalClientSecret,
      },
    });

    const text = await response.text();
    assert.equal(text.includes(stripeSecretKey), false);
    assert.equal(text.includes(paypalClientSecret), false);
    assert.equal(text.includes('actorRole'), false);
    assert.equal(text.includes('parentAccountRef'), false);
    assert.equal(text.includes('manualActionsPending'), false);
  });

  it('remains reachable even when provider-backed billing secrets are absent', async () => {
    const { response } = await executeRequest({
      path: '/public/pricing',
      envOverrides: {
        STRIPE_SECRET_KEY: undefined,
        PAYPAL_CLIENT_SECRET: undefined,
      },
    });

    assert.equal(response.status, 200);
  });
});
