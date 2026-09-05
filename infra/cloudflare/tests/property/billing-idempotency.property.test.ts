import type { Queue } from '@cloudflare/workers-types';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { createStripeSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

const ACCOUNT_IDENTITY_BINDING_UNAVAILABLE = {
  error: 'manual-required',
  authState: 'parent-session-required',
  blocker: 'account-identity-binding-context-manual-required',
} as const;

const WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE = {
  status: 'manual-required',
  authState: 'provider-webhook-signature-required',
  blocker: 'billing-rate-limit-transaction-owner-unavailable',
} as const;

const RECONCILIATION_CONTRACT_UNAVAILABLE = {
  status: 'manual-required',
  handlerKey: 'admin-billing-reconciliation',
  authState: 'internal-queue-only',
  requestModel: 'AdminBillingReconciliationRequest',
  responseModel: 'BillingSupportAdminReconciliationSummary',
  contractState: 'manual-required',
  contractSide: 'request',
  contractBlocker: 'reconciliation-request-contract-not-generated',
  proofIdFamily: 'payment-route.reconciliation',
  actorRole: 'internal',
  message: 'Route dispatch is disabled until its request, response, and owner execution contracts are bound.',
} as const;

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

async function assertJsonResponse(response: Response, status: number, expected: unknown): Promise<void> {
  assert.equal(response.status, status);
  assert.deepEqual(await readJson<unknown>(response), expected);
}

describe('billing write admission and idempotency boundaries', () => {
  it('keeps repeated checkout writes blocked before caller-supplied parent authority can mutate state', async () => {
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

      await assertJsonResponse(first.response, 503, ACCOUNT_IDENTITY_BINDING_UNAVAILABLE);
      await assertJsonResponse(second.response, 503, ACCOUNT_IDENTITY_BINDING_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });

  it('keeps repeated change-plan writes blocked before caller-supplied parent authority can mutate state', async () => {
    for (let index = 0; index < 12; index += 1) {
      const parentAuthorizationValue = 'parent:demo-active';
      const harness = createTestHarness();
      const first = await executeRequest({
        path: '/auth/billing/change-plan',
        method: 'POST',
        harness,
        headers: {
          origin: 'http://localhost:3000',
          authorization: `Bearer ${parentAuthorizationValue}`,
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
          authorization: `Bearer ${token}`,
          'x-ocentra-csrf': 'interactive-parent-session',
        },
        body: {
          requestId: `change-plan-property-${index}`,
          planId: 'family-max',
          abuseGateState: 'passed-turnstile',
        },
      });

      await assertJsonResponse(first.response, 503, ACCOUNT_IDENTITY_BINDING_UNAVAILABLE);
      await assertJsonResponse(second.response, 503, ACCOUNT_IDENTITY_BINDING_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });

  it('keeps repeated verified Stripe deliveries blocked until a serialized rate-limit owner is bound', async () => {
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
      const signature = await createStripeSignature(
        payload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        Math.floor(Date.now() / 1000)
      );

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

      await assertJsonResponse(first.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      await assertJsonResponse(second.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });

  it('keeps reconciliation blocked before queueing while its request contract is unavailable', async () => {
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

      await assertJsonResponse(first.response, 501, RECONCILIATION_CONTRACT_UNAVAILABLE);
      await assertJsonResponse(second.response, 501, RECONCILIATION_CONTRACT_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });

  it('does not persist out-of-order deliveries while the serialized rate-limit owner is unavailable', async () => {
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
      const signatureTimestamp = Math.floor(Date.now() / 1000);
      const originalSignature = await createStripeSignature(
        originalPayload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        signatureTimestamp
      );
      const unrelatedSignature = await createStripeSignature(
        unrelatedPayload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        signatureTimestamp
      );

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
      const unrelated = await executeRequest({
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

      await assertJsonResponse(first.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      await assertJsonResponse(unrelated.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      await assertJsonResponse(replay.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });

  it('does not compare conflicting payload reuse before serialized rate-limit admission succeeds', async () => {
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
      const signatureTimestamp = Math.floor(Date.now() / 1000);
      const firstSignature = await createStripeSignature(
        firstPayload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        signatureTimestamp
      );
      const conflictingSignature = await createStripeSignature(
        conflictingPayload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? '',
        signatureTimestamp
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

      await assertJsonResponse(first.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      await assertJsonResponse(conflicting.response, 503, WEBHOOK_RATE_LIMIT_OWNER_UNAVAILABLE);
      assert.equal(harness.queueMessages.length, 0);
      assert.equal(harness.deadLetterMessages.length, 0);
    }
  });
});
