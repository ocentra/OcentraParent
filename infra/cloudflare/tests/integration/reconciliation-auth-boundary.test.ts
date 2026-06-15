import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { executeRequest, readJson } from "../../src/testing.js";

interface AuthErrorResponse {
  error: string;
  reason: string;
}

describe("POST /admin/billing/reconciliation auth boundary", () => {
  it("rejects requests that are missing the internal queue signal", async () => {
    const { response } = await executeRequest({
      path: "/admin/billing/reconciliation",
      method: "POST",
      body: {
        requestId: "reconciliation-no-internal-signal",
      },
    });

    const body = await readJson<AuthErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, "forbidden");
    assert.equal(body.reason, "missing-internal-queue-signal");
  });

  it("rejects requests with an incorrect internal shared secret", async () => {
    const { response } = await executeRequest({
      path: "/admin/billing/reconciliation",
      method: "POST",
      headers: {
        "x-ocentra-internal-call": "true",
        "x-ocentra-internal-secret": "wrong-secret",
      },
      body: {
        requestId: "reconciliation-bad-secret",
      },
    });

    const body = await readJson<AuthErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, "forbidden");
    assert.equal(body.reason, "internal-queue-secret-mismatch");
  });
});
