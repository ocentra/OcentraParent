import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

const sameSiteHeaders = {
  origin: 'http://localhost:3000',
  'sec-fetch-site': 'same-origin',
};

describe('account browser session worker boundary', () => {
  it('keeps login unavailable until the Account-owned request contract and device owner are bound', async () => {
    const response = await executeRequest({
      path: '/auth/session/login',
      method: 'POST',
      headers: {
        ...sameSiteHeaders,
        'x-ocentra-trusted-device': 'true',
        'x-ocentra-request-id': 'real-session-login-contract',
      },
      body: { provider: 'firebase' },
    });
    const body = await readJson<Record<string, unknown>>(response.response);

    assert.equal(response.response.status, 501);
    assert.equal(body.status, 'manual-required');
    assert.equal(body.handlerKey, 'account-session-login');
    assert.equal(body.contractSide, 'request');
    assert.equal(body.contractBlocker, 'account-session-request-contract-owned-by-account-identity');
    assert.equal(response.response.headers.has('set-cookie'), false);
    assert.equal(JSON.stringify(body).includes('trusted-device'), false);
  });

  it('fails closed on refresh, logout, and global revoke without a durable browser credential binding', async () => {
    for (const path of ['/auth/session/refresh', '/auth/session/logout', '/auth/session/revoke']) {
      const result = await executeRequest({
        path,
        method: 'POST',
        headers: sameSiteHeaders,
      });
      const body = await readJson<Record<string, unknown>>(result.response);

      assert.equal(result.response.status, 503);
      assert.equal(body.error, 'manual-required');
      assert.equal(body.authState, 'browser-refresh-required');
      assert.equal(body.blocker, 'account-session-binding-missing');
      assert.equal(result.response.headers.has('set-cookie'), false);
    }
  });

  it('does not expose bearer values or route internals when the real worker rejects the boundary', async () => {
    const bearer = 'Bearer provider-only-real-runtime-credential';
    const result = await executeRequest({
      path: '/auth/session/login',
      method: 'POST',
      headers: {
        ...sameSiteHeaders,
        authorization: bearer,
      },
      body: { provider: 'firebase', credential: bearer },
    });
    const text = await result.response.text();

    assert.equal(result.response.status, 501);
    assert.equal(text.includes(bearer), false);
    assert.equal(text.includes('ACCOUNT_IDENTITY_D1'), false);
    assert.equal(text.includes('sessionToken'), false);
    assert.equal(text.includes('refreshToken'), false);
  });
});
