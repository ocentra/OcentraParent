import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  loginBrowserSession,
  logoutBrowserSession,
  refreshBrowserSession,
  revokeBrowserSessions,
} from '../../src/auth/browser-session-routes.js';
import { createAuthVerifier, type VerifiedIdentity } from '../../src/auth/verifier.js';
import { createFirebaseProviderVerificationPort } from '../../src/providers/firebase-auth.js';
import { createTestHarness, readJson } from '../../src/testing.js';
import type { Env } from '../../src/env.js';

function publicIdentity(env: Env): VerifiedIdentity {
  const result = createAuthVerifier(env).verifyPublic();
  if (!result.ok) throw new Error('public verifier unexpectedly rejected');
  return result.identity;
}

function safeRequest(path: string, headers: HeadersInit = {}): Request {
  return new Request(`https://cloudflare.local${path}`, {
    method: 'POST',
    headers: {
      origin: 'http://localhost:3000',
      'sec-fetch-site': 'same-origin',
      ...headers,
    },
  });
}

describe('account browser session routes', () => {
  it('keeps provider-only login manual-required and ignores a legacy trusted-device header', async () => {
    const harness = createTestHarness({ ACCOUNT_IDENTITY_D1: undefined });
    harness.env.AUTH_ADAPTER_MODE = 'provider-verified';
    const response = await loginBrowserSession(
      safeRequest('/auth/session/login', {
        authorization: 'Bearer provider-only-credential.invalid',
        'x-ocentra-trusted-device': 'true',
        'x-ocentra-request-id': 'route-login-owner-missing',
      }),
      harness.env,
      createFirebaseProviderVerificationPort(harness.env)
    );

    const body = await readJson<Record<string, unknown>>(response);
    assert.equal(response.status, 503);
    assert.equal(body.error, 'manual-required');
    assert.equal(body.blocker, 'provider-verification-unavailable');
    assert.equal(response.headers.get('x-ocentra-request-id'), 'route-login-owner-missing');
    assert.equal(response.headers.has('set-cookie'), false);
    assert.equal(JSON.stringify(body).includes('provider-only-credential'), false);
  });

  it('requires the refresh credential and its CSRF binding before rotation or logout', async () => {
    const harness = createTestHarness({ ACCOUNT_IDENTITY_D1: undefined });
    const identity = publicIdentity(harness.env);
    const refreshResponse = await refreshBrowserSession(
      safeRequest('/auth/session/refresh', { 'x-ocentra-request-id': 'route-refresh-missing' }),
      harness.env,
      identity
    );
    const logoutResponse = await logoutBrowserSession(
      safeRequest('/auth/session/logout', { 'x-ocentra-request-id': 'route-logout-missing' }),
      harness.env,
      identity
    );

    assert.equal(refreshResponse.status, 403);
    assert.deepEqual(await readJson(refreshResponse), {
      error: 'csrf-validation-failed',
      boundary: 'account-browser-session',
    });
    assert.equal(refreshResponse.headers.get('x-ocentra-request-id'), 'route-refresh-missing');
    assert.equal(logoutResponse.status, 403);
    assert.deepEqual(await readJson(logoutResponse), {
      error: 'csrf-validation-failed',
      boundary: 'account-browser-session',
    });
    assert.equal(logoutResponse.headers.get('x-ocentra-request-id'), 'route-logout-missing');
  });

  it('requires a non-forgeable parent-owner capability for global revoke', async () => {
    const harness = createTestHarness({ ACCOUNT_IDENTITY_D1: undefined });
    const response = await revokeBrowserSessions(
      safeRequest('/auth/session/revoke', {
        'x-ocentra-csrf': 'caller-supplied-csrf',
        'x-ocentra-request-id': 'route-revoke-no-capability',
      }),
      harness.env,
      publicIdentity(harness.env)
    );

    const body = await readJson<Record<string, unknown>>(response);
    assert.equal(response.status, 503);
    assert.equal(body.error, 'manual-required');
    assert.equal(body.blocker, 'account-identity-authority-capability-missing');
    assert.equal(response.headers.get('x-ocentra-request-id'), 'route-revoke-no-capability');
    assert.equal(response.headers.has('set-cookie'), false);
  });
});
