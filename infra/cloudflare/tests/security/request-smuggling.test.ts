import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { createStripeSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

describe('request smuggling guards', () => {
  it('rejects invalid content length values instead of coercing them', async () => {
    const { response } = await executeRequest({
      path: '/health',
      headers: {
        'content-length': '-1',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'invalid-content-length');
  });

  it('rejects ambiguous content length metadata before webhook processing can queue work', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'evt_ambiguous_length',
      type: 'invoice.paid',
      subject: 'parent:demo-active',
    });
    const signature = await createStripeSignature(payload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: payload,
      autoContentLength: false,
      headers: {
        'content-type': 'application/json',
        'content-length': '120, 120',
        'stripe-signature': signature,
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'ambiguous-content-length');
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('rejects state-changing requests without content-length metadata before dispatch', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'evt_missing_length',
      type: 'invoice.paid',
      subject: 'parent:demo-active',
    });
    const signature = await createStripeSignature(payload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: payload,
      autoContentLength: false,
      headers: {
        'content-type': 'application/json',
        'stripe-signature': signature,
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'missing-content-length');
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });

  it('rejects transfer-encoding framing before route dispatch', async () => {
    const harness = createTestHarness();
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      headers: {
        'transfer-encoding': 'chunked',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'unsupported-transfer-encoding');
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 0);
  });
});
