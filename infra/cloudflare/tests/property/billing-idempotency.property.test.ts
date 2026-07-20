import type { Queue } from '@cloudflare/workers-types';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY } from '../../scripts/local-seed-runtime.js';
import { createStripeSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

const executedLocalQueueReplayFixtures = new Set<string>();
const [acceptedReplayFixture, deadLetterReplayFixture] = LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY;

interface WebhookResponse {
  status: string;
  provider: string;
  queued: boolean;
  eventId: string;
  eventType: string;
  conflictReason?: string;
}

interface ReconciliationResponse {
  status: string;
  queued: boolean;
}

function createThrowingQueue(message: string): Queue {
  return {
    send: async (): Promise<void> => {
      throw new Error(message);
    },
    sendBatch: async (): Promise<void> => {
      throw new Error(message);
    },
  } as unknown as Queue;
}

describe('billing write idempotency', () => {
  it('reuses durable-object outcomes for repeated hosted checkout session writes and keeps one audit row', async () => {
    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const requestId = `checkout-idempotent-${index}`;
      const first = await executeRequest({
        path: '/auth/billing/checkout',
        method: 'POST',
        harness,
        headers: {
          origin: 'http://localhost:3000',
          authorization: 'Bearer parent:demo-active',
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        body: {
          requestId,
          planId: 'family-core',
          successPath: '/family/billing/checkout/success',
          cancelPath: '/family/billing/checkout/cancel',
          abuseGateState: 'passed-turnstile',
        },
      });
      const second = await executeRequest({
        path: '/auth/billing/checkout',
        method: 'POST',
        harness,
        headers: {
          origin: 'http://localhost:3000',
          authorization: 'Bearer parent:demo-active',
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        body: {
          requestId,
          planId: 'family-core',
          successPath: '/family/billing/checkout/success',
          cancelPath: '/family/billing/checkout/cancel',
          abuseGateState: 'passed-turnstile',
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));

      const audit = await executeRequest({
        path: `/admin/billing/audit?q=${requestId}`,
        harness,
        headers: {
          authorization: 'Bearer parent:admin-agent',
          'x-ocentra-role': 'admin',
        },
      });
      const auditBody = await readJson<{
        results: Array<{
          eventId: string;
        }>;
      }>(audit.response);
      const matchingEvents = auditBody.results.filter(
        (event) => event.eventId === `billing-checkout-session:${requestId}`
      );
      assert.equal(matchingEvents.length, 1);
    }
  });

  it('reuses durable-object outcomes for repeated change-plan writes per subject', async () => {
    for (let index = 0; index < 12; index += 1) {
      const parentSubject = 'parent:demo-active';
      const harness = createTestHarness();
      const first = await executeRequest({
        path: '/auth/billing/change-plan',
        method: 'POST',
        harness,
        headers: {
          origin: 'http://localhost:3000',
          authorization: `Bearer ${parentSubject}`,
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        body: {
          requestId: `change-plan-property-${index}`,
          planId: 'family-max',
          abuseGateState: 'passed-turnstile',
        },
      });
      const second = await executeRequest({
        path: '/auth/billing/change-plan',
        method: 'POST',
        harness,
        headers: {
          origin: 'http://localhost:3000',
          authorization: `Bearer ${parentSubject}`,
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        body: {
          requestId: `change-plan-property-${index}`,
          planId: 'family-max',
          abuseGateState: 'passed-turnstile',
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));
      assert.equal(harness.queueMessages.length, 1);
    }
  });

  it('reuses durable-object outcomes for repeated stripe webhook deliveries', async () => {
    const eventTypes = ['invoice.paid', 'checkout.session.completed', 'payment_failed', 'dispute_open'] as const;

    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const payload = JSON.stringify({
        id: `evt_idempotent_${index}`,
        type: eventTypes[index % eventTypes.length],
        subject: 'parent:demo-active',
        invoiceId: 'parent-demo-active-invoice-current',
        disputeId: `dp_idempotent_${index}`,
      });
      const signature = await createStripeSignature(payload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

      const first = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: payload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': signature,
        },
      });
      const second = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: payload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': signature,
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));
      assert.equal(harness.queueMessages.length, 1);
    }
    executedLocalQueueReplayFixtures.add(acceptedReplayFixture);
  });

  it('keeps a dead-lettered reconciliation replay stable for the same request id', async () => {
    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      harness.env.BILLING_RECONCILIATION_QUEUE = createThrowingQueue(`reconciliation-queue-failure-${index}`);

      const first = await executeRequest({
        path: '/admin/billing/reconciliation',
        method: 'POST',
        harness,
        headers: {
          'x-ocentra-internal-call': 'true',
          'x-ocentra-internal-secret': 'internal-test-secret',
        },
        body: {
          requestId: `reconciliation-dead-letter-${index}`,
        },
      });
      const second = await executeRequest({
        path: '/admin/billing/reconciliation',
        method: 'POST',
        harness,
        headers: {
          'x-ocentra-internal-call': 'true',
          'x-ocentra-internal-secret': 'internal-test-secret',
        },
        body: {
          requestId: `reconciliation-dead-letter-${index}`,
        },
      });

      assert.deepEqual(
        await readJson<ReconciliationResponse>(first.response),
        await readJson<ReconciliationResponse>(second.response)
      );
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 1);

      const deadLetter = harness.deadLetterMessages[0] as Record<string, unknown>;
      assert.equal(deadLetter.reason, 'reconciliation-queue-send-failed');
    }
    executedLocalQueueReplayFixtures.add(deadLetterReplayFixture);
    assert.deepEqual([...executedLocalQueueReplayFixtures].sort(), [...LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY].sort());
  });

  it('keeps out-of-order duplicate webhook deliveries stable for the same event id', async () => {
    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const originalPayload = JSON.stringify({
        id: `evt_out_of_order_${index}`,
        type: 'invoice.paid',
        subject: 'parent:demo-active',
        invoiceId: `invoice-out-of-order-${index}`,
      });
      const unrelatedPayload = JSON.stringify({
        id: `evt_out_of_order_unrelated_${index}`,
        type: 'payment_failed',
        subject: 'parent:demo-active',
        invoiceId: `invoice-out-of-order-unrelated-${index}`,
      });
      const originalSignature = await createStripeSignature(originalPayload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');
      const unrelatedSignature = await createStripeSignature(unrelatedPayload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

      const first = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: originalPayload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': originalSignature,
        },
      });
      await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: unrelatedPayload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': unrelatedSignature,
        },
      });
      const replay = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: originalPayload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': originalSignature,
        },
      });

      assert.deepEqual(
        await readJson<WebhookResponse>(first.response),
        await readJson<WebhookResponse>(replay.response)
      );
      assert.equal(harness.queueMessages.length, 2);
    }
  });

  it('fails conflicting webhook payload reuse closed instead of double-accepting it', async () => {
    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const firstPayload = JSON.stringify({
        id: `evt_conflict_property_${index}`,
        type: 'invoice.paid',
        subject: 'parent:demo-active',
      });
      const conflictingPayload = JSON.stringify({
        id: `evt_conflict_property_${index}`,
        type: 'invoice.paid',
        subject: 'parent:other-active',
      });
      const firstSignature = await createStripeSignature(firstPayload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');
      const conflictingSignature = await createStripeSignature(
        conflictingPayload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? ''
      );

      const first = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: firstPayload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': firstSignature,
        },
      });
      const conflicting = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        harness,
        body: conflictingPayload,
        headers: {
          'content-type': 'application/json',
          'stripe-signature': conflictingSignature,
        },
      });

      const firstBody = await readJson<WebhookResponse>(first.response);
      const conflictingBody = await readJson<WebhookResponse>(conflicting.response);
      assert.equal(first.response.status, 202);
      assert.equal(conflicting.response.status, 409);
      assert.equal(firstBody.status, 'accepted');
      assert.equal(conflictingBody.status, 'manual-review');
      assert.equal(conflictingBody.queued, false);
      assert.equal(conflictingBody.conflictReason, 'event-id-payload-mismatch');
      assert.equal(harness.queueMessages.length, 1);
    }
  });
});
