import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER,
  INTERNAL_SECRET_HEADER,
  UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER,
  signatureHeaderName,
  verifyAuthState,
} from '../../src/auth/verifier.js';
import type { Env } from '../../src/env.js';

async function readJsonBody(response: Response): Promise<Record<string, unknown>> {
  return JSON.parse(await response.text()) as Record<string, unknown>;
}

function createAuthTestEnv(overrides: Partial<Env> = {}): Env {
  const internalQueueSharedSecret = ['internal', 'test', 'credential'].join('-');
  return {
    ENVIRONMENT: 'test',
    APP_ORIGIN: 'http://localhost:3000',
    CORS_ALLOWED_ORIGINS: 'http://localhost:3000',
    REQUEST_MAX_BYTES: '2048',
    AUTH_ADAPTER_MODE: 'local-safe-fixture',
    INTERNAL_QUEUE_SHARED_SECRET: internalQueueSharedSecret,
    ENTITLEMENT_SIGNING_KEY_REF: 'signing-key-test-ref',
    ...overrides,
  };
}

describe('auth boundary', () => {
  it('accepts public routes without any auth headers', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState('public', new Request('https://cloudflare.local/health'), env);

    assert.equal(result.ok, true);
    if (result.ok) {
      assert.equal(result.identity.subject, 'public');
      assert.equal(result.identity.role, 'public');
      assert.equal(result.identity.trustedDevice, false);
    }
  });

  it('accepts bearer parent sessions', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'parent-session-required',
      new Request('https://cloudflare.local/auth/billing/status', {
        headers: {
          authorization: 'Bearer parent:demo-active',
        },
      }),
      env
    );

    assert.equal(result.ok, true);
    if (result.ok) {
      assert.equal(result.identity.subject, 'parent:demo-active');
      assert.equal(result.identity.role, 'parent');
    }
  });

  it('preserves household subject prefixes for downstream role gating', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'parent-session-required',
      new Request('https://cloudflare.local/auth/billing/checkout', {
        headers: {
          authorization: 'Bearer guardian:demo-guardian',
        },
      }),
      env
    );

    assert.equal(result.ok, true);
    if (result.ok) {
      assert.equal(result.identity.subject, 'guardian:demo-guardian');
    }
  });

  it('rejects parent-session routes without an authorization header', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'parent-session-required',
      new Request('https://cloudflare.local/auth/billing/status'),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 401);
      assert.equal(body.error, 'authentication-required');
      assert.equal(body.authState, 'parent-session-required');
      assert.equal(body.missingHeader, 'authorization');
    }
  });

  it('rejects trusted-device routes without the device header', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'trusted-parent-device-required',
      new Request('https://cloudflare.local/auth/billing/entitlement-snapshot', {
        headers: {
          authorization: 'Bearer parent:demo-active',
        },
      }),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      assert.equal(result.response.status, 403);
    }
  });

  it('rejects admin routes that only satisfy parent-session auth', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'admin-required',
      new Request('https://cloudflare.local/admin/billing/refunds', {
        method: 'POST',
        headers: {
          authorization: 'Bearer parent:demo-active',
        },
      }),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 403);
      assert.equal(body.error, 'forbidden');
      assert.equal(body.authState, 'admin-required');
      assert.equal(body.reason, 'admin-role-required');
      assert.equal('authorization' in body, false);
    }
  });

  it('accepts support and admin roles on support-owned routes', async () => {
    const env = createAuthTestEnv();
    const supportResult = await verifyAuthState(
      'support-required',
      new Request('https://cloudflare.local/admin/billing/accounts', {
        headers: {
          authorization: 'Bearer parent:support-agent',
          'x-ocentra-role': 'support',
        },
      }),
      env
    );
    const adminResult = await verifyAuthState(
      'support-required',
      new Request('https://cloudflare.local/admin/billing/accounts', {
        headers: {
          authorization: 'Bearer parent:admin-agent',
          'x-ocentra-role': 'admin',
        },
      }),
      env
    );

    assert.equal(supportResult.ok, true);
    if (supportResult.ok) {
      assert.equal(supportResult.identity.role, 'support');
    }
    assert.equal(adminResult.ok, true);
    if (adminResult.ok) {
      assert.equal(adminResult.identity.role, 'admin');
    }
  });

  it('rejects support-owned routes without support or admin authority', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'support-required',
      new Request('https://cloudflare.local/admin/billing/accounts', {
        headers: {
          authorization: 'Bearer parent:demo-active',
          'x-ocentra-role': 'parent',
        },
      }),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 403);
      assert.equal(body.error, 'forbidden');
      assert.equal(body.authState, 'support-required');
      assert.equal(body.reason, 'support-role-required');
    }
  });

  it('rejects missing, malformed, and wrong-provider webhook signature headers', async () => {
    const env = createAuthTestEnv();
    const missingResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/stripe', {
        method: 'POST',
      }),
      env
    );
    const invalidResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/stripe', {
        method: 'POST',
        headers: {
          'stripe-signature': 'definitely-not-a-real-signature',
        },
      }),
      env
    );
    const wrongProviderHeaderResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/google', {
        method: 'POST',
        headers: {
          [signatureHeaderName('/webhooks/stripe')]: 't=1710000000,v1=abcdef',
        },
      }),
      env
    );

    assert.equal(missingResult.ok, false);
    if (!missingResult.ok) {
      const body = await readJsonBody(missingResult.response);
      assert.equal(missingResult.response.status, 401);
      assert.equal(body.missingHeader, 'stripe-signature');
    }
    assert.equal(invalidResult.ok, false);
    if (!invalidResult.ok) {
      assert.equal(invalidResult.response.status, 403);
    }
    assert.equal(wrongProviderHeaderResult.ok, false);
    if (!wrongProviderHeaderResult.ok) {
      const body = await readJsonBody(wrongProviderHeaderResult.response);
      assert.equal(wrongProviderHeaderResult.response.status, 401);
      assert.equal(body.missingHeader, 'x-goog-signature');
    }
  });

  it('marks unresolved account-auth adapter states as manual-required for private parent and role routes', async () => {
    const env = createAuthTestEnv({
      AUTH_ADAPTER_MODE: 'account-auth-adapter-manual-required',
    });

    const parentResult = await verifyAuthState(
      'parent-session-required',
      new Request('https://cloudflare.local/auth/billing/status', {
        headers: {
          authorization: 'Bearer parent:demo-active',
        },
      }),
      env
    );
    const adminResult = await verifyAuthState(
      'admin-required',
      new Request('https://cloudflare.local/admin/billing/refunds', {
        method: 'POST',
        headers: {
          authorization: 'Bearer parent:demo-admin',
          'x-ocentra-role': 'admin',
        },
      }),
      env
    );

    for (const result of [parentResult, adminResult]) {
      assert.equal(result.ok, false);
      if (!result.ok) {
        const body = await readJsonBody(result.response);
        assert.equal(result.response.status, 503);
        assert.equal(body.error, 'manual-required');
        assert.equal(body.blocker, ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER);
      }
    }
  });

  it('treats unknown auth adapter modes as manual-required instead of assuming a provider', async () => {
    const env = createAuthTestEnv({
      AUTH_ADAPTER_MODE: 'future-provider-adapter',
    });
    const result = await verifyAuthState(
      'support-required',
      new Request('https://cloudflare.local/admin/billing/accounts', {
        headers: {
          authorization: 'Bearer parent:support-agent',
          'x-ocentra-role': 'support',
        },
      }),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'support-required');
      assert.equal(body.blocker, UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER);
    }
  });

  it('treats unresolved and unknown auth adapter modes as manual-required for provider webhooks too', async () => {
    const unresolvedEnv = createAuthTestEnv({
      AUTH_ADAPTER_MODE: 'account-auth-adapter-manual-required',
    });
    const futureEnv = createAuthTestEnv({
      AUTH_ADAPTER_MODE: 'future-provider-adapter',
    });

    const unresolvedResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/stripe', {
        method: 'POST',
        headers: {
          'stripe-signature': `t=1710000000,v1=${'0'.repeat(64)}`,
        },
      }),
      unresolvedEnv
    );
    const futureResult = await verifyAuthState(
      'provider-webhook-signature-required',
      new Request('https://cloudflare.local/webhooks/stripe', {
        method: 'POST',
        headers: {
          'stripe-signature': `t=1710000000,v1=${'0'.repeat(64)}`,
        },
      }),
      futureEnv
    );

    assert.equal(unresolvedResult.ok, false);
    if (!unresolvedResult.ok) {
      const body = await readJsonBody(unresolvedResult.response);
      assert.equal(unresolvedResult.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'provider-webhook-signature-required');
      assert.equal(body.blocker, ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER);
    }

    assert.equal(futureResult.ok, false);
    if (!futureResult.ok) {
      const body = await readJsonBody(futureResult.response);
      assert.equal(futureResult.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'provider-webhook-signature-required');
      assert.equal(body.blocker, UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER);
    }
  });

  it('rejects internal-only routes without the queue signal or with the wrong shared secret', async () => {
    const env = createAuthTestEnv();
    const missingSignalResult = await verifyAuthState(
      'internal-queue-only',
      new Request('https://cloudflare.local/admin/billing/reconciliation', {
        method: 'POST',
      }),
      env
    );
    const wrongSecretResult = await verifyAuthState(
      'internal-queue-only',
      new Request('https://cloudflare.local/admin/billing/reconciliation', {
        method: 'POST',
        headers: {
          'x-ocentra-internal-call': 'true',
          [INTERNAL_SECRET_HEADER]: 'wrong-secret',
        },
      }),
      env
    );

    assert.equal(missingSignalResult.ok, false);
    if (!missingSignalResult.ok) {
      const body = await readJsonBody(missingSignalResult.response);
      assert.equal(missingSignalResult.response.status, 403);
      assert.equal(body.reason, 'missing-internal-queue-signal');
    }
    assert.equal(wrongSecretResult.ok, false);
    if (!wrongSecretResult.ok) {
      const body = await readJsonBody(wrongSecretResult.response);
      assert.equal(wrongSecretResult.response.status, 403);
      assert.equal(body.reason, 'internal-queue-secret-mismatch');
    }
  });

  it('never silently downgrades stronger auth states to weaker ones', async () => {
    const env = createAuthTestEnv();
    const supportOnAdminResult = await verifyAuthState(
      'admin-required',
      new Request('https://cloudflare.local/admin/billing/refunds', {
        method: 'POST',
        headers: {
          authorization: 'Bearer parent:support-agent',
          'x-ocentra-role': 'support',
        },
      }),
      env
    );
    const untrustedDeviceResult = await verifyAuthState(
      'trusted-parent-device-required',
      new Request('https://cloudflare.local/auth/billing/license-check', {
        method: 'POST',
        headers: {
          authorization: 'Bearer parent:demo-active',
          'x-ocentra-role': 'admin',
        },
      }),
      env
    );

    assert.equal(supportOnAdminResult.ok, false);
    assert.equal(untrustedDeviceResult.ok, false);
  });

  it('keeps admin and support rejection payloads support-safe and audit-oriented', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'support-required',
      new Request('https://cloudflare.local/admin/billing/accounts', {
        headers: {
          authorization: 'Bearer parent:demo-active',
        },
      }),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.deepEqual(Object.keys(body).sort(), ['authState', 'error', 'reason']);
      assert.equal(body.authState, 'support-required');
      assert.equal(body.error, 'forbidden');
      assert.equal(body.reason, 'support-role-required');
    }
  });
});
