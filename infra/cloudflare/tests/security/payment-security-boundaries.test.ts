import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { enforceBillingRateLimit } from '../../src/index.js';
import { verifyAuthState } from '../../src/auth/verifier.js';
import { findRoute } from '../../src/routes.js';
import { createStripeSignature, createTestHarness } from '../../src/testing.js';

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

    const configured = await enforceBillingRateLimit(
      harness.env,
      route,
      authResult.identity,
      now
    );
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
    const result = await enforceBillingRateLimit(
      harness.env,
      route,
      undefined,
      Date.now()
    );

    assert.ok(result instanceof Response);
    if (result instanceof Response) {
      assert.equal(result.status, 503);
      assert.equal((await readJsonBody(result)).blocker, 'billing-rate-limit-principal-unavailable');
    }
    proofMilestone('blocked', 'parent-write-rate-limit-principal');
    proofMilestone('completed', 'parent-write-rate-limit-principal');
  });
});
