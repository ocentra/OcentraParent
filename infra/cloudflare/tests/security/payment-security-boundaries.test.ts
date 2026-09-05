import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import worker, { enforceBillingRateLimit } from '../../src/index.js';
import { verifyAuthState } from '../../src/auth/verifier.js';
import { findRoute } from '../../src/routes.js';
import { createStripeSignature, createTestHarness, type CloudflareTestHarness } from '../../src/testing.js';

const log = Logger.instance;
log.register(import.meta.url);

function proofMilestone(result: 'started' | 'blocked' | 'completed', boundary: string): void {
  log.logInfo(
    'payment security proof milestone',
    getStackTrace(),
    {
      owner: 'payment-security-privacy-observability',
      boundary,
      result,
      noClaimReason: null,
      redactionState: 'identifier-free',
    },
    true
  );
}

async function readJsonBody(response: Response): Promise<Record<string, unknown>> {
  return JSON.parse(await response.text()) as Record<string, unknown>;
}

type QueueObservation = {
  acknowledgements: number;
  retries: unknown[];
};

type ObservedQueueRetryOptions = {
  readonly delaySeconds?: number;
};

function createQueueObservation(): QueueObservation {
  return {
    acknowledgements: 0,
    retries: [],
  };
}

function observedQueueBatch(
  body: unknown,
  attempts: number,
  observation: QueueObservation
): Parameters<typeof worker.queue>[0] {
  return {
    queue: 'billing-reconciliation',
    messages: [
      {
        body,
        attempts,
        ack(): void {
          observation.acknowledgements += 1;
        },
        retry(options?: ObservedQueueRetryOptions): void {
          observation.retries.push(options);
        },
      },
    ],
  };
}

function billingQueueOutcomeWrites(harness: CloudflareTestHarness) {
  return harness.bindingState.getAnalyticsWrites().filter((entry) => entry.indexes?.[0] === 'billing-queue-outcome');
}

function requireRecord(value: unknown): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new Error('expected queue record');
  }
  return value as Record<string, unknown>;
}

describe('payment security boundaries', () => {
  it('fails closed until a serialized rate-limit owner is bound for verified webhook ingress', async () => {
    proofMilestone('started', 'provider-webhook-rate-limit');
    const harness = createTestHarness();
    const route = findRoute('/webhooks/stripe', 'POST');
    if (!route) throw new Error('stripe webhook route missing from manifest');
    const secret = harness.env.STRIPE_WEBHOOK_SECRET;
    if (!secret) throw new Error('stripe webhook test secret missing');

    const now = Date.now();
    const payload = JSON.stringify({
      id: 'evt-security-rate-limit',
      type: 'checkout.session.completed',
      created: Math.floor(now / 1000),
    });
    const signature = await createStripeSignature(payload, secret, Math.floor(now / 1000));
    const authResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/stripe', {
        method: 'POST',
        headers: { 'stripe-signature': signature },
        body: payload,
      }),
      harness.env
    );
    assert.equal(authResult.ok, true);
    if (!authResult.ok) return;

    const configured = await enforceBillingRateLimit(harness.env, route, authResult.identity, now);
    assert.ok(configured instanceof Response);
    if (configured instanceof Response) {
      assert.equal(configured.status, 503);
      assert.equal((await readJsonBody(configured)).blocker, 'billing-rate-limit-transaction-owner-unavailable');
    }

    const missingBinding = await enforceBillingRateLimit(
      { ...harness.env, BILLING_RATE_LIMIT_KV: undefined },
      route,
      authResult.identity,
      now
    );
    assert.ok(missingBinding instanceof Response);
    if (missingBinding instanceof Response) {
      assert.equal(missingBinding.status, 503);
      assert.equal((await readJsonBody(missingBinding)).blocker, 'billing-rate-limit-transaction-owner-unavailable');
    }
    proofMilestone('blocked', 'provider-webhook-rate-limit');
    proofMilestone('completed', 'provider-webhook-rate-limit');
  });

  it('does not let an unverified identity satisfy the parent-write rate-limit principal', async () => {
    proofMilestone('started', 'parent-write-rate-limit-principal');
    const harness = createTestHarness();
    const route = findRoute('/auth/billing/referral-invite', 'POST');
    if (!route) throw new Error('referral invite route missing from manifest');
    const result = await enforceBillingRateLimit(harness.env, route, undefined, Date.now());

    assert.ok(result instanceof Response);
    if (result instanceof Response) {
      assert.equal(result.status, 503);
      assert.equal((await readJsonBody(result)).blocker, 'billing-rate-limit-principal-unavailable');
    }
    proofMilestone('blocked', 'parent-write-rate-limit-principal');
    proofMilestone('completed', 'parent-write-rate-limit-principal');
  });

  it('records identifier-free queue retry telemetry with the exact bounded delay', async () => {
    const harness = createTestHarness();
    const observation = createQueueObservation();

    await worker.queue(observedQueueBatch('invalid-queue-body', 2, observation), harness.env);

    assert.deepEqual(observation, {
      acknowledgements: 0,
      retries: [{ delaySeconds: 90 }],
    });
    assert.deepEqual(billingQueueOutcomeWrites(harness), [
      {
        indexes: ['billing-queue-outcome', 'unclassified', 'retry-scheduled'],
        doubles: [2, 90],
      },
    ]);
  });

  it('dead-letters an exhausted provider event with no receipt and records only its payload class', async () => {
    const harness = createTestHarness();
    const observation = createQueueObservation();

    await worker.queue(
      observedQueueBatch(
        {
          action: 'provider-webhook',
          provider: 'stripe',
          eventId: 'evt-missing-receipt-private',
          eventType: 'invoice.paid',
        },
        5,
        observation
      ),
      harness.env
    );

    assert.deepEqual(observation, {
      acknowledgements: 1,
      retries: [],
    });
    assert.equal(harness.deadLetterMessages.length, 1);
    const deadLetter = requireRecord(harness.deadLetterMessages[0]);
    assert.deepEqual(Object.keys(deadLetter).sort(), [
      'disposition',
      'errorCode',
      'failedAt',
      'payloadDigest',
      'payloadKind',
      'reason',
      'sourceQueue',
    ]);
    assert.equal(deadLetter.disposition, 'dead-letter');
    assert.equal(deadLetter.sourceQueue, 'BILLING_RECONCILIATION_QUEUE');
    assert.equal(deadLetter.reason, 'queue-consumer-manual-required');
    assert.equal(deadLetter.payloadKind, 'provider-webhook');
    assert.equal(deadLetter.errorCode, 'billing-queue-operation-failed');
    assert.equal(typeof deadLetter.payloadDigest, 'string');
    assert.equal(typeof deadLetter.failedAt, 'string');
    assert.deepEqual(billingQueueOutcomeWrites(harness), [
      {
        indexes: ['billing-queue-outcome', 'provider-webhook', 'dead-lettered'],
        doubles: [5, 0],
      },
    ]);
  });

  it('keeps an exhausted message retryable and visible when dead-letter custody is unavailable', async () => {
    const harness = createTestHarness({ BILLING_DEAD_LETTER_QUEUE: undefined });
    const observation = createQueueObservation();

    await worker.queue(observedQueueBatch('invalid-queue-body', 5, observation), harness.env);

    assert.deepEqual(observation, {
      acknowledgements: 0,
      retries: [{ delaySeconds: 900 }],
    });
    assert.equal(harness.deadLetterMessages.length, 0);
    assert.deepEqual(billingQueueOutcomeWrites(harness), [
      {
        indexes: ['billing-queue-outcome', 'unclassified', 'dead-letter-unavailable'],
        doubles: [5, 900],
      },
    ]);
  });
});
