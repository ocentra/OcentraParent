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
  authState?: string;
  blocker?: string;
  missingHeader?: string;
  maxBytes?: number;
}

const WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE = {
  status: 'manual-required',
  authState: 'provider-webhook-signature-required',
  blocker: 'billing-rate-limit-transaction-owner-unavailable',
} as const;

const PAYPAL_VERIFIER_UNAVAILABLE = {
  error: 'manual-required',
  authState: 'provider-webhook-signature-required',
  blocker: 'paypal-provider-verifier-unavailable',
} as const;

async function assertWebhookRateLimitOwnerUnavailable(response: Response): Promise<void> {
  assert.equal(response.status, 503);
  assert.deepEqual(await readJson<unknown>(response), WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
}

describe('provider webhook fuzz smoke', () => {
  it('fails a spread of verified Stripe payload shapes closed at serialized rate-limit admission', async () => {
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
      const signature = await createStripeSignature(
        payload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        Math.floor(Date.now() / 1000)
      );

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

      await assertWebhookRateLimitOwnerUnavailable(response);
    }

    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('does not parse signed malformed payloads before serialized rate-limit admission succeeds', async () => {
    const harness = createTestHarness();
    const payloads = ['{"id":"evt_truncated"', '\u0000\u0001binary-junk'];

    for (const payload of payloads) {
      const signature = await createStripeSignature(
        payload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        Math.floor(Date.now() / 1000)
      );

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

      await assertWebhookRateLimitOwnerUnavailable(response);
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
    const signature = await createStripeSignature(
      payload,
      harness.env.STRIPE_WEBHOOK_SECRET ?? '',
      Math.floor(Date.now() / 1000)
    );

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

  it('fails closed on missing Stripe credentials and an unavailable PayPal verifier', async () => {
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
    assert.equal(randomHeaderOnly.response.status, 503);
    assert.deepEqual(await readJson<unknown>(randomHeaderOnly.response), PAYPAL_VERIFIER_UNAVAILABLE);
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('does not claim a provider-route mismatch before the PayPal verifier is bound', async () => {
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

    assert.equal(response.status, 503);
    assert.deepEqual(await readJson<unknown>(response), PAYPAL_VERIFIER_UNAVAILABLE);
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });
});
