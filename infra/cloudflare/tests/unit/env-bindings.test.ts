import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { createTestHarness } from "../../src/testing.js";
import { getBindingHealth, getMissingBindings, validateEnv } from "../../src/env.js";

describe("env validation", () => {
  it("fails validation when required environment values are missing", () => {
    const env = createTestHarness().env;

    assert.ok(
      validateEnv({
        ...env,
        ENVIRONMENT: "",
      }).includes("missing required env: ENVIRONMENT"),
    );
    assert.ok(
      validateEnv({
        ...env,
        APP_ORIGIN: "",
      }).includes("missing required env: APP_ORIGIN"),
    );
    assert.ok(
      validateEnv({
        ...env,
        CORS_ALLOWED_ORIGINS: "",
      }).includes("missing required env: CORS_ALLOWED_ORIGINS"),
    );
    assert.ok(
      validateEnv({
        ...env,
        ENTITLEMENT_SIGNING_KEY_REF: "",
      }).includes("missing required env: ENTITLEMENT_SIGNING_KEY_REF"),
    );
  });

  it("reports malformed request max bytes", () => {
    const env = createTestHarness().env;
    const errors = validateEnv({
      ...env,
      REQUEST_MAX_BYTES: "abc",
    });

    assert.ok(errors.includes("REQUEST_MAX_BYTES must be a positive integer when provided"));
  });

  it("tracks required and optional bindings separately from hard env validation", () => {
    const env = createTestHarness({
      BILLING_AUDIT_R2: undefined,
      ANALYTICS: undefined,
    }).env;

    assert.deepEqual(getMissingBindings(env), []);
    assert.deepEqual(getBindingHealth(env), {
      BILLING_D1: "configured",
      BILLING_DO: "configured",
      REFERRAL_DO: "configured",
      ENTITLEMENT_SNAPSHOT_DO: "configured",
      BILLING_RECONCILIATION_QUEUE: "configured",
      BILLING_DEAD_LETTER_QUEUE: "configured",
      BILLING_RATE_LIMIT_KV: "configured",
      BILLING_CONFIG_KV: "configured",
      BILLING_AUDIT_R2: "missing",
      ANALYTICS: "missing",
    });
  });

  it("keeps optional bindings optional instead of turning them into hard validation failures", () => {
    const env = createTestHarness({
      BILLING_AUDIT_R2: undefined,
      ANALYTICS: undefined,
    }).env;

    assert.deepEqual(validateEnv(env), []);
  });

  it("reports unknown or misspelled env keys as validation failures", () => {
    const env = {
      ...createTestHarness().env,
      BILLING_CONFIG_KVV: {} as object,
    };

    assert.ok(validateEnv(env).includes("unknown env key: BILLING_CONFIG_KVV"));
  });
});
