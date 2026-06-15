import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { createStripeSignature, createTestHarness, executeRequest } from "../../src/testing.js";

describe("stripe webhook fuzz smoke", () => {
  it("accepts a spread of signed JSON payload shapes without surfacing 5xx responses", async () => {
    const harness = createTestHarness();
    const eventTypes = [
      "invoice.paid",
      "checkout.session.completed",
      "payment_failed",
      "dispute_open",
      "dispute_won",
    ] as const;

    for (let index = 0; index < 10; index += 1) {
      const payload = JSON.stringify({
        id: `evt_fuzz_${index}`,
        type: eventTypes[index % eventTypes.length],
        subject: "parent:demo-active",
        invoiceId: "parent-demo-active-invoice-current",
        disputeId: `dp_fuzz_${index}`,
        data: {
          object: {
            amount_total: 1000 + index,
            metadata: {
              familyRef: `family:fuzz-${index}`,
              subject: "parent:demo-active",
            },
          },
        },
      });
      const signature = await createStripeSignature(payload, harness.env.STRIPE_WEBHOOK_SECRET ?? "");

      const { response } = await executeRequest({
        path: "/webhooks/stripe",
        method: "POST",
        harness,
        body: payload,
        headers: {
          "content-type": "application/json",
          "stripe-signature": signature,
        },
      });

      assert.equal(response.status, 202);
    }
  });
});
