import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  loginBrowserSession,
  logoutBrowserSession,
  refreshBrowserSession,
  revokeBrowserSessions,
} from '../../src/auth/browser-session-routes.js';
import { createAuthVerifier, type VerifiedIdentity } from '../../src/auth/verifier.js';
import { createTestHarness, readJson } from '../../src/testing.js';
import type { Env } from '../../src/env.js';

function publicIdentity(env: Env): VerifiedIdentity {
  const result = createAuthVerifier(env).verifyPublic();
  if (!result.ok) throw new Error('public verifier unexpectedly rejected');
  return result.identity;
}

function request(path: string, headers: HeadersInit = {}): Request {
  return new Request(`https://cloudflare.local${path}`, {
    method: 'POST',
    headers,
  });
}

describe('account browser session request safety', () => {
  it('rejects cross-origin and cross-site requests for every session mutation', async () => {
    const harness = createTestHarness();
    const identity = publicIdentity(harness.env);
    const headers = {
      origin: 'https://attacker.invalid',
      'sec-fetch-site': 'cross-site',
      'x-ocentra-csrf': 'caller-supplied-token',
      'x-ocentra-trusted-device': 'true',
    };

    const responses = await Promise.all([
      loginBrowserSession(request('/auth/session/login', headers), harness.env, undefined),
      refreshBrowserSession(request('/auth/session/refresh', headers), harness.env, identity),
      logoutBrowserSession(request('/auth/session/logout', headers), harness.env, identity),
      revokeBrowserSessions(request('/auth/session/revoke', headers), harness.env, identity),
    ]);

    for (const response of responses) {
      assert.equal(response.status, 403);
      assert.deepEqual(await readJson(response), {
        error: 'origin-validation-failed',
        boundary: 'account-browser-session',
      });
      assert.equal(response.headers.has('set-cookie'), false);
    }
  });

  it('requires same-site fetch metadata and a CSRF header on state-changing refresh flows', async () => {
    const harness = createTestHarness();
    const identity = publicIdentity(harness.env);
    const sameOrigin = { origin: 'http://localhost:3000' };

    const loginResponse = await loginBrowserSession(request('/auth/session/login', sameOrigin), harness.env, undefined);
    assert.equal(loginResponse.status, 403);
    assert.deepEqual(await readJson(loginResponse), {
      error: 'fetch-metadata-validation-failed',
      boundary: 'account-browser-session',
    });

    const refreshResponse = await refreshBrowserSession(
      request('/auth/session/refresh', { ...sameOrigin, 'sec-fetch-site': 'same-origin' }),
      harness.env,
      identity
    );
    const logoutResponse = await logoutBrowserSession(
      request('/auth/session/logout', { ...sameOrigin, 'sec-fetch-site': 'same-origin' }),
      harness.env,
      identity
    );
    const revokeResponse = await revokeBrowserSessions(
      request('/auth/session/revoke', { ...sameOrigin, 'sec-fetch-site': 'same-origin' }),
      harness.env,
      identity
    );

    for (const response of [refreshResponse, logoutResponse, revokeResponse]) {
      assert.equal(response.status, 403);
      assert.deepEqual(await readJson(response), {
        error: 'csrf-validation-failed',
        boundary: 'account-browser-session',
      });
      assert.equal(response.headers.has('set-cookie'), false);
    }
  });

  it('does not reflect an untrusted correlation value into the response', async () => {
    const harness = createTestHarness();
    const untrustedCorrelation = 'a'.repeat(129);
    const response = await loginBrowserSession(
      request('/auth/session/login', {
        origin: 'http://localhost:3000',
        'sec-fetch-site': 'same-origin',
        'x-ocentra-request-id': untrustedCorrelation,
      }),
      harness.env,
      undefined
    );

    assert.equal(response.status, 503);
    assert.match(response.headers.get('x-ocentra-request-id') ?? '', /^browser-route-[A-Za-z0-9_-]{1,24}$/);
    assert.notEqual(response.headers.get('x-ocentra-request-id'), untrustedCorrelation);
  });
});
