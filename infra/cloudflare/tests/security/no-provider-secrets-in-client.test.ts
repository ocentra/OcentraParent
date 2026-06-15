import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { executeRequest } from "../../src/testing.js";

describe("provider secret exposure", () => {
  it("never echoes provider secrets in public pricing responses", async () => {
    const { response } = await executeRequest({
      path: "/public/pricing",
      envOverrides: {
        STRIPE_SECRET_KEY: "sk_live_price_secret",
        PAYPAL_CLIENT_SECRET: "paypal-price-secret",
      },
    });

    const text = await response.text();
    assert.equal(text.includes("sk_live_price_secret"), false);
    assert.equal(text.includes("paypal-price-secret"), false);
  });

  it("never echoes provider secrets in authenticated billing status responses", async () => {
    const { response } = await executeRequest({
      path: "/auth/billing/status",
      headers: {
        authorization: "Bearer parent:demo-active",
      },
      envOverrides: {
        STRIPE_SECRET_KEY: "sk_live_status_secret",
        STRIPE_WEBHOOK_SECRET: "whsec_status_secret",
      },
    });

    const text = await response.text();
    assert.equal(text.includes("sk_live_status_secret"), false);
    assert.equal(text.includes("whsec_status_secret"), false);
  });

  it("never echoes provider secret refs or raw provider credentials in support-visible admin payloads", async () => {
    const { response } = await executeRequest({
      path: "/admin/billing/accounts?q=review",
      headers: {
        authorization: "Bearer parent:support-agent",
        "x-ocentra-role": "support",
      },
      envOverrides: {
        PAYPAL_CLIENT_SECRET: "paypal-admin-secret",
        ENTITLEMENT_SIGNING_KEY_REF: "signing-key-admin-ref",
        GOOGLE_PLAY_SERVICE_ACCOUNT_REF: "google-play-admin-ref",
      },
    });

    const text = await response.text();
    assert.equal(text.includes("paypal-admin-secret"), false);
    assert.equal(text.includes("signing-key-admin-ref"), false);
    assert.equal(text.includes("google-play-admin-ref"), false);
  });

  it("never exposes child-data markers, evidence refs, or support-bundle markers in client-visible payloads", async () => {
    const statusResponse = await executeRequest({
      path: "/auth/billing/status",
      headers: {
        authorization: "Bearer parent:demo-review",
      },
    });
    const adminResponse = await executeRequest({
      path: "/admin/billing/accounts?q=review",
      headers: {
        authorization: "Bearer parent:support-agent",
        "x-ocentra-role": "support",
      },
    });

    const statusText = await statusResponse.response.text();
    const adminText = await adminResponse.response.text();
    for (const text of [statusText, adminText]) {
      assert.equal(text.includes("child-profile-present"), false);
      assert.equal(text.includes("child-device-001"), false);
      assert.equal(text.includes("evidence://"), false);
      assert.equal(text.includes("support-bundle-secret"), false);
      assert.equal(text.includes("recovery-bundle"), false);
    }
  });
});
