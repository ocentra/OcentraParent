import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { BillingSupportAdminDisputesResponseSchema } from '../../src/generated/billing-contracts.js';
import { createHmacSignature, createTestHarness, executeRequest, readJson } from '../../src/testing.js';

interface ProviderWebhookResponse {
  provider: string;
}

interface ProviderWebhookErrorResponse {
  error: string;
  blocker?: string;
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

interface BillingInvoicesResponse {
  invoices: Array<{
    invoiceId: string;
    paymentState: string;
  }>;
}

describe('non-stripe provider webhooks', () => {
  it('accepts signed Razorpay payloads', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'rzp_evt_1',
      type: 'subscription.charged',
      subject: 'parent:demo-review',
    });
    const signature = await createHmacSignature(payload, harness.env.RAZORPAY_KEY_SECRET ?? '');

    const { response } = await executeRequest({
      path: '/webhooks/razorpay',
      method: 'POST',
      harness,
      body: payload,
      headers: {
        'content-type': 'application/json',
        'x-razorpay-signature': signature,
      },
    });

    const body = await readJson<ProviderWebhookResponse>(response);
    assert.equal(response.status, 202);
    assert.equal(body.provider, 'razorpay');

    const status = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
    });
    const statusBody = await readJson<BillingStatusResponse>(status.response);
    assert.equal(status.response.status, 200);
    assert.equal(statusBody.accountStatus, 'active');
    assert.equal(statusBody.subscriptionStatus, 'active');
    assert.equal(statusBody.parentVisibleState, 'available');
  });

  it('accepts signed PayPal and Google payloads and explicit Apple authorization', async () => {
    const harness = createTestHarness();

    const paypalPayload = JSON.stringify({
      id: 'paypal_evt_1',
      type: 'BILLING.SUBSCRIPTION.ACTIVATED',
    });
    const paypalTransmissionId = 'paypal-transmission-1';
    const paypalSignature = await createHmacSignature(
      `${paypalTransmissionId}.${paypalPayload}`,
      harness.env.PAYPAL_CLIENT_SECRET ?? ''
    );
    const paypal = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      harness,
      body: paypalPayload,
      headers: {
        'content-type': 'application/json',
        'paypal-transmission-id': paypalTransmissionId,
        'paypal-transmission-sig': paypalSignature,
      },
    });
    const paypalBody = await readJson<ProviderWebhookResponse>(paypal.response);
    assert.equal(paypal.response.status, 202);
    assert.equal(paypalBody.provider, 'paypal');

    const googlePayload = JSON.stringify({
      id: 'google_evt_1',
      type: 'SUBSCRIPTION_RENEWED',
    });
    const googleSignature = await createHmacSignature(googlePayload, harness.env.GOOGLE_PLAY_SERVICE_ACCOUNT_REF ?? '');
    const google = await executeRequest({
      path: '/webhooks/google',
      method: 'POST',
      harness,
      body: googlePayload,
      headers: {
        'content-type': 'application/json',
        'x-goog-signature': googleSignature,
      },
    });
    const googleBody = await readJson<ProviderWebhookResponse>(google.response);
    assert.equal(google.response.status, 202);
    assert.equal(googleBody.provider, 'google');

    const apple = await executeRequest({
      path: '/webhooks/apple',
      method: 'POST',
      harness,
      body: {
        id: 'apple_evt_1',
        type: 'DID_RENEW',
      },
      headers: {
        authorization: `Bearer ${harness.env.APPLE_STORE_KEY_REF}`,
      },
    });
    const appleBody = await readJson<ProviderWebhookResponse>(apple.response);
    assert.equal(apple.response.status, 202);
    assert.equal(appleBody.provider, 'apple');
  });

  it('materializes payment-failed webhooks into grace-visible billing state', async () => {
    const harness = createTestHarness();
    const payload = JSON.stringify({
      id: 'rzp_evt_payment_failed',
      type: 'payment_failed',
      subject: 'parent:demo-active',
    });
    const signature = await createHmacSignature(payload, harness.env.RAZORPAY_KEY_SECRET ?? '');

    const webhook = await executeRequest({
      path: '/webhooks/razorpay',
      method: 'POST',
      harness,
      body: payload,
      headers: {
        'content-type': 'application/json',
        'x-razorpay-signature': signature,
      },
    });
    const webhookBody = await readJson<ProviderWebhookResponse>(webhook.response);
    assert.equal(webhook.response.status, 202);
    assert.equal(webhookBody.provider, 'razorpay');

    const status = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const statusBody = await readJson<BillingStatusResponse>(status.response);
    assert.equal(status.response.status, 200);
    assert.equal(statusBody.accountStatus, 'grace');
    assert.equal(statusBody.subscriptionStatus, 'grace');
    assert.equal(statusBody.parentVisibleState, 'grace');
    assert.notEqual(statusBody.failureState, null);
    assert.ok(statusBody.warnings.includes('provider-webhook-payment-required'));

    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingSnapshotResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'grace');
    assert.equal(snapshotBody.snapshot.parentVisibleState, 'grace');
    assert.equal(snapshotBody.snapshot.signatureState, 'signed');

    const invoices = await executeRequest({
      path: '/auth/billing/invoices',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const invoicesBody = await readJson<BillingInvoicesResponse>(invoices.response);
    const currentInvoice = invoicesBody.invoices.find(
      (invoice) => invoice.invoiceId === 'parent-demo-active-invoice-current'
    );
    assert.equal(invoices.response.status, 200);
    assert.equal(currentInvoice?.paymentState, 'grace');
  });

  it('materializes dispute lifecycle rows and entitlement recovery from provider events', async () => {
    const harness = createTestHarness();
    const disputeId = 'dp_demo_active';
    const invoiceId = 'parent-demo-active-invoice-current';

    const openedPayload = JSON.stringify({
      id: 'paypal_evt_dispute_opened',
      type: 'dispute_open',
      subject: 'parent:demo-active',
      disputeId,
      invoiceId,
    });
    const openedSignature = await createHmacSignature(
      `paypal-dispute-opened.${openedPayload}`,
      harness.env.PAYPAL_CLIENT_SECRET ?? ''
    );
    const opened = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      harness,
      body: openedPayload,
      headers: {
        'content-type': 'application/json',
        'paypal-transmission-id': 'paypal-dispute-opened',
        'paypal-transmission-sig': openedSignature,
      },
    });
    assert.equal(opened.response.status, 202);

    const openedStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const openedStatusBody = await readJson<BillingStatusResponse>(openedStatus.response);
    assert.equal(openedStatus.response.status, 200);
    assert.equal(openedStatusBody.accountStatus, 'manual-review');
    assert.equal(openedStatusBody.subscriptionStatus, 'past-due');
    assert.equal(openedStatusBody.parentVisibleState, 'manual-review');

    const openedDisputes = await executeRequest({
      path: `/admin/billing/disputes?q=${disputeId}`,
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const openedDisputesBody = BillingSupportAdminDisputesResponseSchema.parse(
      await readJson<unknown>(openedDisputes.response)
    );
    assert.equal(openedDisputes.response.status, 200);
    assert.equal(openedDisputesBody.results[0]?.disputeState, 'dispute-opened');

    const wonPayload = JSON.stringify({
      id: 'paypal_evt_dispute_won',
      type: 'dispute_won',
      subject: 'parent:demo-active',
      disputeId,
      invoiceId,
    });
    const wonSignature = await createHmacSignature(
      `paypal-dispute-won.${wonPayload}`,
      harness.env.PAYPAL_CLIENT_SECRET ?? ''
    );
    const won = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      harness,
      body: wonPayload,
      headers: {
        'content-type': 'application/json',
        'paypal-transmission-id': 'paypal-dispute-won',
        'paypal-transmission-sig': wonSignature,
      },
    });
    assert.equal(won.response.status, 202);

    const wonStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const wonStatusBody = await readJson<BillingStatusResponse>(wonStatus.response);
    assert.equal(wonStatus.response.status, 200);
    assert.equal(wonStatusBody.accountStatus, 'active');
    assert.equal(wonStatusBody.subscriptionStatus, 'active');
    assert.equal(wonStatusBody.parentVisibleState, 'available');

    const wonDisputes = await executeRequest({
      path: `/admin/billing/disputes?q=${disputeId}`,
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const wonDisputesBody = BillingSupportAdminDisputesResponseSchema.parse(
      await readJson<unknown>(wonDisputes.response)
    );
    assert.equal(wonDisputes.response.status, 200);
    assert.equal(wonDisputesBody.results[0]?.disputeState, 'dispute-won');
    assert.equal(wonDisputesBody.results[0]?.manualRequired, false);
  });

  it('fails safe when non-stripe provider webhook credentials are missing', async () => {
    const razorpay = await executeRequest({
      path: '/webhooks/razorpay',
      method: 'POST',
      envOverrides: {
        RAZORPAY_KEY_SECRET: '',
      },
      body: {
        id: 'rzp_evt_missing_secret',
      },
      headers: {
        'x-razorpay-signature': 'deadbeef',
      },
    });
    const razorpayBody = await readJson<ProviderWebhookErrorResponse>(razorpay.response);
    assert.equal(razorpay.response.status, 503);
    assert.equal(razorpayBody.error, 'manual-required');
    assert.equal(razorpayBody.blocker, 'razorpay-webhook-secret-missing');

    const paypal = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      envOverrides: {
        PAYPAL_CLIENT_SECRET: '',
      },
      body: {
        id: 'paypal_evt_missing_secret',
      },
      headers: {
        'paypal-transmission-id': 'paypal-missing-secret',
        'paypal-transmission-sig': 'deadbeef',
      },
    });
    const paypalBody = await readJson<ProviderWebhookErrorResponse>(paypal.response);
    assert.equal(paypal.response.status, 503);
    assert.equal(paypalBody.error, 'manual-required');
    assert.equal(paypalBody.blocker, 'paypal-webhook-secret-missing');

    const apple = await executeRequest({
      path: '/webhooks/apple',
      method: 'POST',
      envOverrides: {
        APPLE_STORE_KEY_REF: '',
      },
      body: {
        id: 'apple_evt_missing_secret',
      },
      headers: {
        authorization: 'Bearer missing',
      },
    });
    const appleBody = await readJson<ProviderWebhookErrorResponse>(apple.response);
    assert.equal(apple.response.status, 503);
    assert.equal(appleBody.error, 'manual-required');
    assert.equal(appleBody.blocker, 'apple-store-key-ref-missing');

    const google = await executeRequest({
      path: '/webhooks/google',
      method: 'POST',
      envOverrides: {
        GOOGLE_PLAY_SERVICE_ACCOUNT_REF: '',
      },
      body: {
        id: 'google_evt_missing_secret',
      },
      headers: {
        'x-goog-signature': 'deadbeef',
      },
    });
    const googleBody = await readJson<ProviderWebhookErrorResponse>(google.response);
    assert.equal(google.response.status, 503);
    assert.equal(googleBody.error, 'manual-required');
    assert.equal(googleBody.blocker, 'google-play-service-account-ref-missing');
  });

  it('rejects invalid non-stripe provider signatures and Apple authorization before queueing', async () => {
    const razorpay = await executeRequest({
      path: '/webhooks/razorpay',
      method: 'POST',
      body: {
        id: 'rzp_evt_invalid_signature',
      },
      headers: {
        'x-razorpay-signature': 'deadbeef',
      },
    });
    const razorpayBody = await readJson<ProviderWebhookErrorResponse>(razorpay.response);
    assert.equal(razorpay.response.status, 400);
    assert.equal(razorpayBody.error, 'invalid-razorpay-signature');

    const paypal = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      body: {
        id: 'paypal_evt_invalid_signature',
      },
      headers: {
        'paypal-transmission-id': 'paypal-invalid-signature',
        'paypal-transmission-sig': 'deadbeef',
      },
    });
    const paypalBody = await readJson<ProviderWebhookErrorResponse>(paypal.response);
    assert.equal(paypal.response.status, 400);
    assert.equal(paypalBody.error, 'invalid-paypal-signature');

    const google = await executeRequest({
      path: '/webhooks/google',
      method: 'POST',
      body: {
        id: 'google_evt_invalid_signature',
      },
      headers: {
        'x-goog-signature': 'deadbeef',
      },
    });
    const googleBody = await readJson<ProviderWebhookErrorResponse>(google.response);
    assert.equal(google.response.status, 400);
    assert.equal(googleBody.error, 'invalid-google-signature');

    const apple = await executeRequest({
      path: '/webhooks/apple',
      method: 'POST',
      body: {
        id: 'apple_evt_invalid_authorization',
      },
      headers: {
        authorization: 'Bearer wrong-authorization',
      },
    });
    const appleBody = await readJson<ProviderWebhookErrorResponse>(apple.response);
    assert.equal(apple.response.status, 400);
    assert.equal(appleBody.error, 'invalid-apple-authorization');
  });

  it('rejects missing non-stripe provider webhook auth headers before handler execution', async () => {
    const razorpay = await executeRequest({
      path: '/webhooks/razorpay',
      method: 'POST',
      body: {
        id: 'rzp_evt_missing_header',
      },
    });
    const razorpayBody = await readJson<ProviderWebhookErrorResponse>(razorpay.response);
    assert.equal(razorpay.response.status, 401);
    assert.equal(razorpayBody.error, 'authentication-required');

    const paypal = await executeRequest({
      path: '/webhooks/paypal',
      method: 'POST',
      body: {
        id: 'paypal_evt_missing_header',
      },
    });
    const paypalBody = await readJson<ProviderWebhookErrorResponse>(paypal.response);
    assert.equal(paypal.response.status, 401);
    assert.equal(paypalBody.error, 'authentication-required');

    const google = await executeRequest({
      path: '/webhooks/google',
      method: 'POST',
      body: {
        id: 'google_evt_missing_header',
      },
    });
    const googleBody = await readJson<ProviderWebhookErrorResponse>(google.response);
    assert.equal(google.response.status, 401);
    assert.equal(googleBody.error, 'authentication-required');

    const apple = await executeRequest({
      path: '/webhooks/apple',
      method: 'POST',
      body: {
        id: 'apple_evt_missing_header',
      },
    });
    const appleBody = await readJson<ProviderWebhookErrorResponse>(apple.response);
    assert.equal(apple.response.status, 401);
    assert.equal(appleBody.error, 'authentication-required');
  });
});
