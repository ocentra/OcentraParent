import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER,
  INTERNAL_SECRET_HEADER,
  UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER,
  signatureHeaderName,
  verifyAuthState,
} from '../../src/auth/verifier.js';
import { validateAuthBoundaryRoute, type AuthBoundaryRouteLike } from '../../src/auth/model.js';
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
    STRIPE_WEBHOOK_SECRET: ['whsec', 'auth', 'boundary', 'fixture'].join('_'),
    STRIPE_WEBHOOK_TOLERANCE_SECONDS: '300',
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

  it('does not accept a caller-supplied bearer parent session without bound account authority', async () => {
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

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'parent-session-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('does not infer household authority from a caller-supplied bearer subject', async () => {
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

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'parent-session-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('keeps parent-session routes manual-required until an account auth adapter is bound', async () => {
    const env = createAuthTestEnv();
    const result = await verifyAuthState(
      'parent-session-required',
      new Request('https://cloudflare.local/auth/billing/status'),
      env
    );

    assert.equal(result.ok, false);
    if (!result.ok) {
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'parent-session-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('keeps trusted-device routes manual-required until a current browser authority is bound', async () => {
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
      const body = await readJsonBody(result.response);
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('keeps admin routes manual-required until an account admin authority is bound', async () => {
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
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'admin-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
      assert.equal('authorization' in body, false);
    }
  });

  it('does not mint support or admin authority from caller-supplied role headers', async () => {
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

    assert.equal(supportResult.ok, false);
    if (!supportResult.ok) {
      const body = await readJsonBody(supportResult.response);
      assert.equal(supportResult.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'support-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
    assert.equal(adminResult.ok, false);
    if (!adminResult.ok) {
      const body = await readJsonBody(adminResult.response);
      assert.equal(adminResult.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'support-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('keeps support-owned routes manual-required without an owner-issued authority capability', async () => {
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
      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'support-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
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
      assert.equal(invalidResult.response.status, 400);
    }
    assert.equal(wrongProviderHeaderResult.ok, false);
    if (!wrongProviderHeaderResult.ok) {
      const body = await readJsonBody(wrongProviderHeaderResult.response);
      assert.equal(wrongProviderHeaderResult.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.blocker, 'google-provider-verifier-unavailable');
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
      assert.deepEqual(Object.keys(body).sort(), ['authState', 'blocker', 'error']);
      assert.equal(body.authState, 'support-required');
      assert.equal(body.error, 'manual-required');
      assert.equal(body.blocker, 'account-identity-binding-context-manual-required');
    }
  });

  it('rejects privileged route definitions that omit an audit event', () => {
    const route: AuthBoundaryRouteLike = {
      path: '/admin/billing/accounts',
      method: 'GET',
      authState: 'support-required',
      auditEvent: '   ',
      auditRule: 'support-read',
      routeGroup: 'admin',
      routeBoundary: 'private',
    };

    assert.equal(validateAuthBoundaryRoute(route), 'admin-support-routes-require-audit-event');
  });
});
