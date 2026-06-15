import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  createStripeSignature,
  createTestHarness,
  executeRequest,
  readJson,
} from "../../src/testing.js";

describe("billing write idempotency", () => {
  it("reuses durable-object outcomes for repeated hosted checkout session writes and keeps one audit row", async () => {
    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const requestId = `checkout-idempotent-${index}`;
      const first = await executeRequest({
        path: "/auth/billing/checkout",
        method: "POST",
        harness,
        headers: {
          origin: "http://localhost:3000",
          authorization: "Bearer parent:demo-active",
          "x-ocentra-csrf": "interactive-parent-session",
        },
        body: {
          requestId,
          planId: "family-core",
          successPath: "/family/billing/checkout/success",
          cancelPath: "/family/billing/checkout/cancel",
          abuseGateState: "passed-turnstile",
        },
      });
      const second = await executeRequest({
        path: "/auth/billing/checkout",
        method: "POST",
        harness,
        headers: {
          origin: "http://localhost:3000",
          authorization: "Bearer parent:demo-active",
          "x-ocentra-csrf": "interactive-parent-session",
        },
        body: {
          requestId,
          planId: "family-core",
          successPath: "/family/billing/checkout/success",
          cancelPath: "/family/billing/checkout/cancel",
          abuseGateState: "passed-turnstile",
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));

      const audit = await executeRequest({
        path: `/admin/billing/audit?q=${requestId}`,
        harness,
        headers: {
          authorization: "Bearer parent:admin-agent",
          "x-ocentra-role": "admin",
        },
      });
      const auditBody = await readJson<{
        results: Array<{
          eventId: string;
        }>;
      }>(audit.response);
      const matchingEvents = auditBody.results.filter(
        (event) => event.eventId === `billing-checkout-session:${requestId}`,
      );
      assert.equal(matchingEvents.length, 1);
    }
  });

  it("reuses durable-object outcomes for repeated change-plan writes per subject", async () => {
    for (let index = 0; index < 12; index += 1) {
      const token = "parent:demo-active";
      const harness = createTestHarness();
      const first = await executeRequest({
        path: "/auth/billing/change-plan",
        method: "POST",
        harness,
        headers: {
          origin: "http://localhost:3000",
          authorization: `Bearer ${token}`,
          "x-ocentra-csrf": "interactive-parent-session",
        },
        body: {
          requestId: `change-plan-property-${index}`,
          planId: "family-max",
          abuseGateState: "passed-turnstile",
        },
      });
      const second = await executeRequest({
        path: "/auth/billing/change-plan",
        method: "POST",
        harness,
        headers: {
          origin: "http://localhost:3000",
          authorization: `Bearer ${token}`,
          "x-ocentra-csrf": "interactive-parent-session",
        },
        body: {
          requestId: `change-plan-property-${index}`,
          planId: "family-max",
          abuseGateState: "passed-turnstile",
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));
      assert.equal(harness.queueMessages.length, 1);
    }
  });

  it("reuses durable-object outcomes for repeated stripe webhook deliveries", async () => {
    const eventTypes = [
      "invoice.paid",
      "checkout.session.completed",
      "payment_failed",
      "dispute_open",
    ] as const;

    for (let index = 0; index < 12; index += 1) {
      const harness = createTestHarness();
      const payload = JSON.stringify({
        id: `evt_idempotent_${index}`,
        type: eventTypes[index % eventTypes.length],
        subject: "parent:demo-active",
        invoiceId: "parent-demo-active-invoice-current",
        disputeId: `dp_idempotent_${index}`,
      });
      const signature = await createStripeSignature(
        payload,
        harness.env.STRIPE_WEBHOOK_SECRET ?? "",
      );

      const first = await executeRequest({
        path: "/webhooks/stripe",
        method: "POST",
        harness,
        body: payload,
        headers: {
          "content-type": "application/json",
          "stripe-signature": signature,
        },
      });
      const second = await executeRequest({
        path: "/webhooks/stripe",
        method: "POST",
        harness,
        body: payload,
        headers: {
          "content-type": "application/json",
          "stripe-signature": signature,
        },
      });

      assert.deepEqual(await readJson<unknown>(first.response), await readJson<unknown>(second.response));
      assert.equal(harness.queueMessages.length, 1);
    }
  });
});
