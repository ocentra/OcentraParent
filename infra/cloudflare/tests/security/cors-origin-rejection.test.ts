import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

describe('cors origin rejection', () => {
  it('rejects non-allowlisted origins', async () => {
    const { response } = await executeRequest({
      path: '/public/pricing',
      headers: {
        origin: 'https://evil.example',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'cors-origin-rejected');
    assert.equal(response.headers.get('access-control-allow-origin'), 'http://localhost:3000');
  });

  it('allows configured origins to reach the next guard', async () => {
    const { response } = await executeRequest({
      path: '/public/pricing',
      headers: {
        origin: 'http://localhost:3000',
      },
    });

    assert.equal(response.status, 200);
    assert.equal(response.headers.get('access-control-allow-origin'), 'http://localhost:3000');
  });

  it('fails closed for empty or wildcard allow-list configurations instead of silently allowing all origins', async () => {
    const empty = await executeRequest({
      path: '/public/pricing',
      headers: {
        origin: 'https://evil.example',
      },
      envOverrides: {
        CORS_ALLOWED_ORIGINS: '',
      },
    });
    const wildcard = await executeRequest({
      path: '/public/pricing',
      headers: {
        origin: 'https://evil.example',
      },
      envOverrides: {
        CORS_ALLOWED_ORIGINS: '*',
      },
    });

    const emptyBody = await readJson<any>(empty.response);
    const wildcardBody = await readJson<any>(wildcard.response);
    assert.equal(empty.response.status, 500);
    assert.equal(wildcard.response.status, 403);
    assert.equal(emptyBody.error, 'environment-validation-failed');
    assert.equal(wildcardBody.error, 'cors-origin-rejected');
  });
});
