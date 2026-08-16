import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest } from '../../src/testing.js';

const fixtureValue = (...parts: readonly string[]) => parts.join('_');

describe('provider secret exposure', () => {
  it('never echoes provider secrets in public pricing responses', async () => {
    const stripeSecret = fixtureValue('sk', 'live', 'price', 'secret');
    const paypalSecret = fixtureValue('paypal', 'price', 'secret');
    const { response } = await executeRequest({
      path: '/public/pricing',
      envOverrides: {
        STRIPE_SECRET_KEY: stripeSecret,
        PAYPAL_CLIENT_SECRET: paypalSecret,
      },
    });

    const text = await response.text();
    assert.equal(text.includes(stripeSecret), false);
    assert.equal(text.includes(paypalSecret), false);
  });

  it('never echoes provider secrets in authenticated billing status responses', async () => {
    const stripeSecret = fixtureValue('sk', 'live', 'status', 'secret');
    const webhookSecret = fixtureValue('whsec', 'status', 'secret');
    const { response } = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
      envOverrides: {
        STRIPE_SECRET_KEY: stripeSecret,
        STRIPE_WEBHOOK_SECRET: webhookSecret,
      },
    });

    const text = await response.text();
    assert.equal(text.includes(stripeSecret), false);
    assert.equal(text.includes(webhookSecret), false);
  });

  it('never echoes provider secret refs or raw provider credentials in support-visible admin payloads', async () => {
    const paypalSecret = fixtureValue('paypal', 'admin', 'secret');
    const signingKeyRef = fixtureValue('signing', 'key', 'admin', 'ref');
    const serviceAccountRef = fixtureValue('google', 'play', 'admin', 'ref');
    const { response } = await executeRequest({
      path: '/admin/billing/accounts?q=review',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
      envOverrides: {
        PAYPAL_CLIENT_SECRET: paypalSecret,
        ENTITLEMENT_SIGNING_KEY_REF: signingKeyRef,
        GOOGLE_PLAY_SERVICE_ACCOUNT_REF: serviceAccountRef,
      },
    });

    const text = await response.text();
    assert.equal(text.includes(paypalSecret), false);
    assert.equal(text.includes(signingKeyRef), false);
    assert.equal(text.includes(serviceAccountRef), false);
  });

  it('never exposes child-data markers, evidence refs, or support-bundle markers in client-visible payloads', async () => {
    const statusResponse = await executeRequest({
      path: '/auth/billing/status',
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
    });
    const adminResponse = await executeRequest({
      path: '/admin/billing/accounts?q=review',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });

    const statusText = await statusResponse.response.text();
    const adminText = await adminResponse.response.text();
    for (const text of [statusText, adminText]) {
      assert.equal(text.includes('child-profile-present'), false);
      assert.equal(text.includes('child-device-001'), false);
      assert.equal(text.includes('evidence://'), false);
      assert.equal(text.includes('support-bundle-secret'), false);
      assert.equal(text.includes('recovery-bundle'), false);
    }
  });
});
