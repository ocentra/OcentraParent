import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  createHmacSignature,
  createStripeSignature,
  createTestHarness,
  executeRequest,
  readJson,
} from '../../src/testing.js';

interface ErrorResponse {
  error: string;
  provider?: string;
  missingHeader?: string;
  maxBytes?: number;
}

describe('stripe webhook fuzz smoke', () => {
  it('accepts a spread of signed JSON payload shapes without surfacing 5xx responses', async () => {
    const harness = createTestHarness();
    const eventTypes = [
      'invoice.paid',
      'checkout.session.completed',
      'payment_failed',
      'dispute_open',
      'dispute_won',
    ] as const;

    for (let index = 0; index < 10; index += 1) {
      const payload = JSON.stringify({
        id: `evt_fuzz_${index}`,
        type: eventTypes[index % eventTypes.length],
        subject: 'parent:demo-active',
        invoiceId: 'parent-demo-active-invoice-current',
        disputeId: `dp_fuzz_${index}`,
        data: {
          object: {
            amount_total: 1000 + index,
            metadata: {
              familyReference: `family:fuzz-${index}`,
            },
          },
        },
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

      assert.equal(response.status, 202);
    }
  });

  it('fails closed on truncated JSON and binary junk without surfacing worker errors', async () => {
    const harness = createTestHarness();
    const payloads = ['{"id":"evt_truncated"', '\u0000\u0001binary-junk'];

    for (const payload of payloads) {
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

      const body = await readJson<ErrorResponse>(response);
      assert.equal(response.status, 400);
      assert.equal(body.error, 'invalid-webhook-payload');
      assert.equal(body.provider, 'stripe');
      assert.equal('message' in body, false);
      assert.equal('requestHeaders' in body, false);
    }

    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('fails oversized signed payloads before webhook processing', async () => {
    const harness = createTestHarness({
      REQUEST_MAX_BYTES: '128',
    });
    const payload = JSON.stringify({
      id: 'evt_oversized',
      type: 'invoice.paid',
      subject: 'parent:demo-active',
      metadata: {
        oversized: 'x'.repeat(512),
      },
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

    const body = await readJson<ErrorResponse>(response);
    assert.equal(response.status, 413);
    assert.equal(body.error, 'payload-too-large');
    assert.equal(body.maxBytes, 128);
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('fails closed on missing or random provider headers', async () => {
    const harness = createTestHarness();

    const missingStripeSignature = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: JSON.stringify({
        id: 'evt_missing_header',
        type: 'invoice.paid',
      }),
      headers: {
        'content-type': 'application/json',
      },
    });
    const missingStripeBody = await readJson<ErrorResponse>(missingStripeSignature.response);
    assert.equal(missingStripeSignature.response.status, 401);
    assert.equal(missingStripeBody.error, 'authentication-required');
    assert.equal(missingStripeBody.missingHeader, 'stripe-signature');

    const randomHeaderOnly = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      harness,
      body: JSON.stringify({
        id: 'evt_random_header',
        type: 'invoice.paid',
      }),
      headers: {
        'content-type': 'application/json',
        'stripe-signature': 't=1710000000,v1=not-used-here',
      },
    });
    const randomHeaderBody = await readJson<ErrorResponse>(randomHeaderOnly.response);
    assert.equal(randomHeaderOnly.response.status, 401);
    assert.equal(randomHeaderBody.error, 'authentication-required');
    assert.equal(randomHeaderBody.missingHeader, 'paypal-transmission-id');
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('rejects explicit wrong-provider payload hints on other provider routes', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'evt_wrong_provider',
      type: 'checkout.session.completed',
      provider: 'stripe',
      subject: 'parent:demo-active',
    });
    const transmissionId = 'paypal-transmission-wrong-provider';
    const transmissionSig = await createHmacSignature(
      `${transmissionId}.${payload}`,
      harness.env.PAYPAL_CLIENT_SECRET ?? ''
    );

    const { response } = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      harness,
      body: payload,
      headers: {
        'content-type': 'application/json',
        'paypal-transmission-id': transmissionId,
        'paypal-transmission-sig': transmissionSig,
      },
    });

    const body = await readJson<ErrorResponse>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'provider-route-mismatch');
    assert.equal(body.provider, 'paypal');
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });
});
