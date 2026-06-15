import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { executeRequest, readJson } from "../../src/testing.js";

describe("request smuggling guards", () => {
  it("rejects invalid content length values instead of coercing them", async () => {
    const { response } = await executeRequest({
      path: "/health",
      headers: {
        "content-length": "-1",
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, "invalid-content-length");
  });

  it("rejects transfer-encoding framing before route dispatch", async () => {
    const { response } = await executeRequest({
      path: "/webhooks/stripe",
      method: "POST",
      headers: {
        "transfer-encoding": "chunked",
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, "unsupported-transfer-encoding");
  });
});
