import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { createStripeSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

interface WebhookErrorResponse {
  error: string;
  missingHeader?: string;
  blocker?: string;
}

interface WebhookAcceptedResponse {
  status: string;
  provider: string;
  queued: boolean;
  proofIdFamily: string;
  eventId: string;
  eventType: string;
  conflictReason?: string;
}

interface BillingStatusResponse {
  accountStatus: string;
  subscriptionStatus: string;
  parentVisibleState: string;
  failureState: unknown;
  warnings: ReadonlyArray<string>;
}

interface BillingSnapshotResponse {
  snapshot: {
    subscriptionStatus: string;
    parentVisibleState: string;
    signatureState: string;
  };
}

describe('POST /webhooks/stripe', () => {
  it('rejects missing webhook signatures', async () => {
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      body: {
        id: 'evt_missing',
      },
    });

    const body = await readJson<WebhookErrorResponse>(response);
    assert.equal(response.status, 401);
    assert.equal(body.missingHeader, 'stripe-signature');
  });

  it('rejects syntactically valid but incorrect webhook signatures', async () => {
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      body: {
        id: 'evt_invalid',
      },
      headers: {
        'stripe-signature': `t=1710000000,v1=${'0'.repeat(64)}`,
      },
    });

    const body = await readJson<WebhookErrorResponse>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'invalid-stripe-signature');
  });

  it('fails safe when the Stripe webhook secret binding is missing', async () => {
    const harness = createTestHarness({
      STRIPE_WEBHOOK_SECRET: '',
    });

    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: {
        id: 'evt_missing_secret',
      },
      headers: {
        'stripe-signature': `t=1710000000,v1=${'0'.repeat(64)}`,
      },
    });

    const body = await readJson<WebhookErrorResponse>(response);
    assert.equal(response.status, 503);
    assert.equal(body.error, 'manual-required');
    assert.equal(body.blocker, 'stripe-webhook-secret-missing');
  });

  it('keeps webhook auth manual-required when the auth adapter mode is unresolved or unknown', async () => {
    const rejectedWebhookFixtureSecret = ['whsec', 'auth', 'boundary', 'fixture'].join('_');

    for (const authAdapterMode of ['account-auth-adapter-manual-required', 'future-provider-adapter'] as const) {
      const { response } = await executeRequest({
        path: '/webhooks/stripe',
        method: 'POST',
        body: {
          id: `evt_${authAdapterMode}`,
        },
        headers: {
          'stripe-signature': `t=1710000000,v1=${'0'.repeat(64)}`,
        },
        envOverrides: {
          AUTH_ADAPTER_MODE: authAdapterMode,
          STRIPE_WEBHOOK_SECRET: rejectedWebhookFixtureSecret,
        },
      });

      const text = await response.text();
      const body = JSON.parse(text) as WebhookErrorResponse & { authState?: unknown };
      assert.equal(response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'provider-webhook-signature-required');
      assert.equal(
        body.blocker,
        authAdapterMode === 'account-auth-adapter-manual-required'
          ? 'account-auth-adapter-manual-required'
          : 'unsupported-auth-adapter-mode'
      );
      assert.equal(text.includes(rejectedWebhookFixtureSecret), false);
      assert.equal(text.includes('stripe-signature'), false);
    }
  });

  it('accepts valid signed payloads and queues them for reconciliation', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'evt_valid',
      type: 'checkout.session.completed',
      subject: 'parent:demo-grace',
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

    const body = await readJson<WebhookAcceptedResponse>(response);
    assert.equal(response.status, 202);
    assert.equal(body.status, 'accepted');
    assert.equal(body.provider, 'stripe');
    assert.equal(body.queued, true);
    assert.equal(body.proofIdFamily, 'payment-route.webhook-stripe');
    assert.equal(body.eventId, 'evt_valid');
    assert.equal(body.eventType, 'checkout.session.completed');
    assert.equal(harness.queueMessages.length, 1);

    const queuedEvent = harness.queueMessages[0] as Record<string, unknown>;
    assert.equal(queuedEvent.provider, 'stripe');
    assert.equal(queuedEvent.eventId, 'evt_valid');
    assert.equal(queuedEvent.eventType, 'checkout.session.completed');
    assert.equal(queuedEvent.subject, 'parent:demo-grace');
    assert.equal(typeof queuedEvent.receivedAt, 'string');
    assert.equal(new Date(String(queuedEvent.receivedAt)).toISOString(), queuedEvent.receivedAt);

    const status = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });
    const statusBody = await readJson<BillingStatusResponse>(status.response);
    assert.equal(status.response.status, 200);
    assert.equal(statusBody.accountStatus, 'active');
    assert.equal(statusBody.subscriptionStatus, 'active');
    assert.equal(statusBody.parentVisibleState, 'available');
    assert.equal(statusBody.failureState, null);
    assert.ok(statusBody.warnings.includes('provider-webhook-synced'));

    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-grace',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingSnapshotResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'active');
    assert.equal(snapshotBody.snapshot.parentVisibleState, 'available');
    assert.equal(snapshotBody.snapshot.signatureState, 'signed');
  });

  it('deduplicates repeated signed payloads and fails closed on conflicting event payload reuse', async () => {
    const harness = createTestHarness();
    const firstPayload = JSON.stringify({
      id: 'evt_conflict_guard',
      type: 'invoice.paid',
      subject: 'parent:demo-grace',
    });
    const firstSignature = await createStripeSignature(firstPayload, harness.env.STRIPE_WEBHOOK_SECRET ?? '');

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
    const replay = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      harness,
      body: firstPayload,
      headers: {
        'content-type': 'application/json',
        'stripe-signature': firstSignature,
      },
    });

    assert.deepEqual(
      await readJson<WebhookAcceptedResponse>(first.response),
      await readJson<WebhookAcceptedResponse>(replay.response)
    );
    assert.equal(harness.queueMessages.length, 1);

    const conflictingPayload = JSON.stringify({
      id: 'evt_conflict_guard',
      type: 'customer.subscription.deleted',
      subject: 'parent:demo-grace',
    });
    const conflictingSignature = await createStripeSignature(
      conflictingPayload,
      harness.env.STRIPE_WEBHOOK_SECRET ?? ''
    );
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

    const conflictingBody = await readJson<WebhookAcceptedResponse>(conflicting.response);
    assert.equal(conflicting.response.status, 409);
    assert.equal(conflictingBody.status, 'manual-review');
    assert.equal(conflictingBody.provider, 'stripe');
    assert.equal(conflictingBody.queued, false);
    assert.equal(conflictingBody.eventId, 'evt_conflict_guard');
    assert.equal(conflictingBody.eventType, 'customer.subscription.deleted');
    assert.equal(conflictingBody.conflictReason, 'event-id-payload-mismatch');
    assert.equal(harness.queueMessages.length, 1);
  });
});
