import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, it } from 'node:test';
import type {
  AnalyticsEngineDataset,
  D1Database,
  DurableObjectNamespace,
  KVNamespace,
  Queue,
  R2Bucket,
} from '@cloudflare/workers-types';
import type { Env } from '../../src/env.js';
import {
  getBindingHealth,
  getBindingOwnership,
  getMissingBindings,
  parseAllowedOrigins,
  validateEnv,
} from '../../src/env.js';

const wranglerDevConfig = readFileSync(resolve(import.meta.dirname, '../../wrangler.toml'), 'utf8');
const wranglerProductionConfig = readFileSync(resolve(import.meta.dirname, '../../wrangler.production.toml'), 'utf8');
const devVarsExample = readFileSync(resolve(import.meta.dirname, '../../.dev.vars.example'), 'utf8');

function parseDevVarsExample(content: string): Record<string, string> {
  return Object.fromEntries(
    content
      .split(/\r?\n/)
      .filter(Boolean)
      .map((line) => {
        const separatorIndex = line.indexOf('=');
        return [line.slice(0, separatorIndex), line.slice(separatorIndex + 1)];
      })
  );
}

function createEnv(overrides: Partial<Env> = {}): Env {
  return {
    ENVIRONMENT: 'test',
    APP_ORIGIN: 'http://localhost:3000',
    CORS_ALLOWED_ORIGINS: 'http://localhost:3000',
    REQUEST_MAX_BYTES: '1048576',
    BILLING_ROUTE_KILL_SWITCH: 'false',
    AUTH_ADAPTER_MODE: 'local-safe-fixture',
    INTERNAL_QUEUE_SHARED_SECRET: 'queue-secret',
    STRIPE_WEBHOOK_TOLERANCE_SECONDS: '300',
    ENTITLEMENT_SIGNING_KEY_REF: 'signing-key-test-ref',
    BILLING_D1: {} as D1Database,
    ACCOUNT_IDENTITY_D1: {} as D1Database,
    BILLING_DO: {} as DurableObjectNamespace,
    REFERRAL_DO: {} as DurableObjectNamespace,
    ENTITLEMENT_SNAPSHOT_DO: {} as DurableObjectNamespace,
    BILLING_RECONCILIATION_QUEUE: {} as Queue,
    BILLING_DEAD_LETTER_QUEUE: {} as Queue,
    BILLING_RATE_LIMIT_KV: {} as KVNamespace,
    BILLING_CONFIG_KV: {} as KVNamespace,
    BILLING_AUDIT_R2: {} as R2Bucket,
    ANALYTICS: {} as AnalyticsEngineDataset,
    ...overrides,
  };
}

function createProductionEnv(overrides: Partial<Env> = {}): Env {
  return createEnv({
    ENVIRONMENT: 'production',
    AUTH_ADAPTER_MODE: 'account-auth-adapter-manual-required',
    INTERACTIVE_CSRF_TOKEN: 'csrf-test-token',
    ...overrides,
  });
}

describe('env validation', () => {
  it('fails validation when required environment values are missing', () => {
    const env = createEnv();

    assert.ok(
      validateEnv({
        ...env,
        ENVIRONMENT: '',
      }).includes('missing required env: ENVIRONMENT')
    );
    assert.ok(
      validateEnv({
        ...env,
        APP_ORIGIN: '',
      }).includes('missing required env: APP_ORIGIN')
    );
    assert.ok(
      validateEnv({
        ...env,
        CORS_ALLOWED_ORIGINS: '',
      }).includes('missing required env: CORS_ALLOWED_ORIGINS')
    );
    assert.ok(
      validateEnv({
        ...env,
        ENTITLEMENT_SIGNING_KEY_REF: '',
      }).includes('missing required env: ENTITLEMENT_SIGNING_KEY_REF')
    );
  });

  it('fails closed for missing or non-string origin configuration', () => {
    const missingAppOriginErrors = validateEnv(createEnv({ APP_ORIGIN: undefined as unknown as string }));
    assert.deepEqual(
      missingAppOriginErrors.filter((error) => error.includes('APP_ORIGIN')),
      ['missing required env: APP_ORIGIN']
    );

    const nonStringAppOriginErrors = validateEnv(createEnv({ APP_ORIGIN: 42 as unknown as string }));
    assert.deepEqual(
      nonStringAppOriginErrors.filter((error) => error.includes('APP_ORIGIN')),
      ['missing required env: APP_ORIGIN']
    );

    const missingCorsErrors = validateEnv(createEnv({ CORS_ALLOWED_ORIGINS: undefined as unknown as string }));
    assert.deepEqual(parseAllowedOrigins(createEnv({ CORS_ALLOWED_ORIGINS: undefined as unknown as string })), []);
    assert.equal(missingCorsErrors.includes('missing required env: CORS_ALLOWED_ORIGINS'), true);
    assert.equal(missingCorsErrors.includes('CORS_ALLOWED_ORIGINS must include at least one origin'), true);

    const nonStringCorsErrors = validateEnv(
      createEnv({ CORS_ALLOWED_ORIGINS: { origin: 'test' } as unknown as string })
    );
    assert.equal(nonStringCorsErrors.includes('missing required env: CORS_ALLOWED_ORIGINS'), true);
    assert.equal(nonStringCorsErrors.includes('CORS_ALLOWED_ORIGINS must include at least one origin'), true);
  });

  it('rejects malformed app and comma-list origins without throwing', () => {
    for (const appOrigin of [
      'ftp://localhost:3000',
      'https://localhost:3000/path',
      'https://user:pass@localhost:3000',
    ]) {
      assert.equal(
        validateEnv(createEnv({ APP_ORIGIN: appOrigin })).includes('APP_ORIGIN must be a valid http(s) origin'),
        true
      );
    }

    const corsErrors = validateEnv(
      createEnv({ CORS_ALLOWED_ORIGINS: 'https://localhost:3000,not-an-origin,https://localhost:3000/path' })
    );
    assert.equal(corsErrors.includes('CORS_ALLOWED_ORIGINS must contain only valid http(s) origins'), true);
  });

  it('preserves wildcard origin behavior for local, test, and development environments', () => {
    for (const environment of ['local', 'test', 'development'] as const) {
      assert.deepEqual(
        validateEnv(
          createEnv({
            ENVIRONMENT: environment,
            APP_ORIGIN: '*',
            CORS_ALLOWED_ORIGINS: '*, https://localhost:3000',
            INTERACTIVE_CSRF_TOKEN: 'csrf-test-token',
          })
        ),
        [],
        environment
      );
    }
  });

  it('reports malformed request max bytes', () => {
    const env = createEnv();
    const errors = validateEnv({
      ...env,
      REQUEST_MAX_BYTES: 'abc',
    });

    assert.ok(errors.includes('REQUEST_MAX_BYTES must be a positive integer when provided'));
  });

  it('tracks required and optional bindings separately from hard env validation', () => {
    const env = createEnv({
      ACCOUNT_IDENTITY_D1: undefined,
      BILLING_AUDIT_R2: undefined,
      ANALYTICS: undefined,
    });

    assert.deepEqual(getMissingBindings(env), []);
    assert.deepEqual(getBindingHealth(env), {
      BILLING_D1: 'configured',
      BILLING_DO: 'configured',
      REFERRAL_DO: 'configured',
      ENTITLEMENT_SNAPSHOT_DO: 'configured',
      BILLING_RECONCILIATION_QUEUE: 'configured',
      BILLING_DEAD_LETTER_QUEUE: 'configured',
      BILLING_RATE_LIMIT_KV: 'configured',
      BILLING_CONFIG_KV: 'configured',
      ACCOUNT_IDENTITY_D1: 'missing',
      BILLING_AUDIT_R2: 'missing',
      ANALYTICS: 'missing',
    });
  });

  it('keeps optional bindings optional instead of turning them into hard validation failures', () => {
    const env = createEnv({
      BILLING_AUDIT_R2: undefined,
      ANALYTICS: undefined,
    });

    assert.deepEqual(validateEnv(env), []);
  });

  it('reports unknown or misspelled env keys as validation failures', () => {
    const env = {
      ...createEnv(),
      BILLING_CONFIG_KVV: {} as object,
    };

    assert.ok(validateEnv(env).includes('unknown env key: BILLING_CONFIG_KVV'));
  });

  it('rejects an unclassified environment instead of treating it as a safe test/live mode', () => {
    const previewValue = 'csrf-preview-token';

    assert.ok(
      validateEnv(
        createEnv({
          ENVIRONMENT: 'preview',
          INTERACTIVE_CSRF_TOKEN: previewValue,
        })
      ).includes('ENVIRONMENT must be one of local, test, development, production')
    );
  });

  it('freezes one owner and one purpose per binding with child-data storage forbidden', () => {
    const ownership = getBindingOwnership();

    assert.deepEqual(Object.keys(ownership).sort(), [
      'ACCOUNT_IDENTITY_D1',
      'ANALYTICS',
      'BILLING_AUDIT_R2',
      'BILLING_CONFIG_KV',
      'BILLING_D1',
      'BILLING_DEAD_LETTER_QUEUE',
      'BILLING_DO',
      'BILLING_RATE_LIMIT_KV',
      'BILLING_RECONCILIATION_QUEUE',
      'ENTITLEMENT_SNAPSHOT_DO',
      'REFERRAL_DO',
    ]);

    for (const binding of Object.values(ownership)) {
      assert.notEqual(binding.owner.trim(), '');
      assert.notEqual(binding.purpose.trim(), '');
      assert.equal(binding.childDataStorage, 'forbidden');
      assert.match(binding.privacyBoundary, /no child telemetry|no child telemetry, raw child data/i);
    }
  });

  it('keeps queue and dead-letter ownership explicit', () => {
    const ownership = getBindingOwnership();

    assert.deepEqual(ownership.BILLING_RECONCILIATION_QUEUE, {
      owner: 'billing-reconciliation-jobs',
      purpose: 'retry, provider polling, and reconciliation jobs',
      bindingFamily: 'queue',
      privacyBoundary: 'redacted reconciliation payloads only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
      queueRole: 'producer',
      pairedQueueBinding: 'BILLING_DEAD_LETTER_QUEUE',
    });
    assert.deepEqual(ownership.BILLING_DEAD_LETTER_QUEUE, {
      owner: 'billing-dead-letter-ops',
      purpose: 'dead-letter capture and operator replay workflow for reconciliation failures',
      bindingFamily: 'queue',
      privacyBoundary: 'redacted dead-letter payloads only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
      queueRole: 'dead-letter',
      pairedQueueBinding: 'BILLING_RECONCILIATION_QUEUE',
    });
  });

  it('keeps durable object ownership explicit and single-purpose', () => {
    const ownership = getBindingOwnership();

    assert.deepEqual(ownership.BILLING_DO, {
      owner: 'billing-control-do',
      purpose: 'serialized billing-state writes and idempotency coordination',
      bindingFamily: 'durable-object',
      privacyBoundary: 'billing write coordination only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
    assert.deepEqual(ownership.REFERRAL_DO, {
      owner: 'referral-control-do',
      purpose: 'serialized referral qualification, abuse review, and credit lifecycle coordination',
      bindingFamily: 'durable-object',
      privacyBoundary: 'referral coordination only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
    assert.deepEqual(ownership.ENTITLEMENT_SNAPSHOT_DO, {
      owner: 'entitlement-snapshot-do',
      purpose: 'snapshot issuance coordination and replay-safe signing workflow',
      bindingFamily: 'durable-object',
      privacyBoundary: 'entitlement snapshot coordination only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
  });

  it('marks optional audit R2 as manual-required instead of runtime-ready', () => {
    const ownership = getBindingOwnership();

    assert.deepEqual(ownership.BILLING_AUDIT_R2, {
      owner: 'support-audit-export',
      purpose: 'support-safe audit and export bundles only',
      bindingFamily: 'r2',
      privacyBoundary: 'support-safe audit/export artifacts only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'manual-required',
      rejectedUse: 'must not become a telemetry dump or general-purpose child-data storage',
    });
    assert.equal(validateEnv(createEnv({ BILLING_AUDIT_R2: undefined })).includes('BILLING_AUDIT_R2'), false);
  });

  it('keeps D1 and KV bindings inside explicit privacy boundaries', () => {
    const ownership = getBindingOwnership();

    assert.deepEqual(ownership.BILLING_D1, {
      owner: 'billing-ledger-read-model',
      purpose: 'queryable billing ledgers, support/admin views, and reconciliation read models',
      bindingFamily: 'd1',
      privacyBoundary: 'billing, support, and reconciliation records only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
    assert.deepEqual(ownership.ACCOUNT_IDENTITY_D1, {
      owner: 'account-identity-authority',
      purpose: 'durable provider-subject to current account/member/role/device/session authority mapping',
      bindingFamily: 'd1',
      privacyBoundary:
        'provider subject, account/member/device identifiers, current household target binding, session provenance, and support receipt metadata only; no child telemetry or raw claims',
      childDataStorage: 'forbidden',
      readinessState: 'manual-required',
      rejectedUse: 'must not accept caller-supplied household, child, device, role, session, or receipt authority',
    });
    assert.deepEqual(ownership.BILLING_RATE_LIMIT_KV, {
      owner: 'billing-rate-limit-guard',
      purpose: 'rate limits and lightweight abuse counters',
      bindingFamily: 'kv',
      privacyBoundary: 'rate-limit counters and low-risk abuse state only; no child telemetry or raw child data',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
    assert.deepEqual(ownership.BILLING_CONFIG_KV, {
      owner: 'billing-runtime-config',
      purpose: 'low-risk runtime flags and rollout config',
      bindingFamily: 'kv',
      privacyBoundary:
        'rollout flags and non-secret config only; no child telemetry, raw child data, or provider secrets',
      childDataStorage: 'forbidden',
      readinessState: 'required',
    });
  });

  it('keeps development and production wrangler bindings explicit and aligned with the storage binding model', () => {
    for (const config of [wranglerDevConfig, wranglerProductionConfig]) {
      assert.match(config, /binding = "BILLING_D1"/);
      assert.match(config, /binding = "ACCOUNT_IDENTITY_D1"/);
      assert.match(config, /binding = "BILLING_RATE_LIMIT_KV"/);
      assert.match(config, /binding = "BILLING_CONFIG_KV"/);
      assert.match(config, /binding = "BILLING_RECONCILIATION_QUEUE"/);
      assert.match(config, /binding = "BILLING_DEAD_LETTER_QUEUE"/);
      assert.match(config, /binding = "BILLING_AUDIT_R2"/);
      assert.match(config, /binding = "ANALYTICS"/);
      assert.match(config, /name = "BILLING_DO"/);
      assert.match(config, /name = "REFERRAL_DO"/);
      assert.match(config, /name = "ENTITLEMENT_SNAPSHOT_DO"/);
      assert.match(config, /ENTITLEMENT_SIGNING_KEY_REF = "manual-required-/);
    }
  });

  it('rejects wildcard production origins in the checked-in wrangler production config', () => {
    const productionOriginMatch = wranglerProductionConfig.match(/APP_ORIGIN = "([^"]+)"/);
    const productionCorsMatch = wranglerProductionConfig.match(/CORS_ALLOWED_ORIGINS = "([^"]+)"/);

    assert.ok(productionOriginMatch);
    assert.ok(productionCorsMatch);
    assert.equal(productionOriginMatch[1], 'https://parent.ocentra.com');
    assert.equal(productionCorsMatch[1], 'https://parent.ocentra.com');
    assert.doesNotMatch(productionOriginMatch[1], /\*/);
    assert.doesNotMatch(productionCorsMatch[1], /\*/);
  });

  it('rejects wildcard app and comma-list origins in production after trimming', () => {
    const appOriginErrors = validateEnv(createProductionEnv({ APP_ORIGIN: 'https://*.ocentra.com' }));
    assert.equal(appOriginErrors.includes('APP_ORIGIN must not contain a wildcard in production'), true);

    const corsOriginErrors = validateEnv(
      createProductionEnv({ CORS_ALLOWED_ORIGINS: 'https://parent.ocentra.com, *, https://*.ocentra.com' })
    );
    assert.equal(corsOriginErrors.includes('CORS_ALLOWED_ORIGINS must not contain a wildcard in production'), true);
  });

  it('keeps .dev.vars.example secret values placeholder-only and server-side', () => {
    const vars = parseDevVarsExample(devVarsExample);

    assert.equal(vars.ENVIRONMENT, 'development');
    assert.equal(vars.APP_ORIGIN, 'http://localhost:3000');
    assert.equal(vars.CORS_ALLOWED_ORIGINS, 'http://localhost:3000');
    assert.equal(vars.BILLING_ROUTE_KILL_SWITCH, 'false');
    assert.equal(vars.AUTH_ADAPTER_MODE, 'account-auth-adapter-manual-required');
    assert.equal(vars.INTERNAL_QUEUE_SHARED_SECRET, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.STRIPE_SECRET_KEY, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.STRIPE_WEBHOOK_SECRET, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.STRIPE_WEBHOOK_TOLERANCE_SECONDS, '300');
    assert.equal(vars.RAZORPAY_KEY_ID, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.RAZORPAY_KEY_SECRET, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.PAYPAL_CLIENT_ID, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.PAYPAL_CLIENT_SECRET, 'REPLACE_WITH_LOCAL_ONLY_SECRET');
    assert.equal(vars.APPLE_STORE_KEY_REF, 'REPLACE_WITH_LOCAL_ONLY_SECRET_REF');
    assert.equal(vars.GOOGLE_PLAY_SERVICE_ACCOUNT_REF, 'REPLACE_WITH_LOCAL_ONLY_SECRET_REF');
    assert.equal(vars.ENTITLEMENT_SIGNING_KEY_REF, 'REPLACE_WITH_LOCAL_ONLY_SECRET_REF');
  });
});
