import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

describe('request limits', () => {
  it('rejects payloads larger than the configured limit', async () => {
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      headers: {
        'content-length': '11',
      },
      envOverrides: {
        REQUEST_MAX_BYTES: '10',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 413);
    assert.equal(body.error, 'payload-too-large');
    assert.equal(body.maxBytes, 10);
  });

  it('allows payloads at the configured limit to reach the next dispatch stage', async () => {
    const { response } = await executeRequest({
      path: '/health',
      method: 'POST',
      headers: {
        'content-length': '10',
      },
      envOverrides: {
        REQUEST_MAX_BYTES: '10',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 404);
    assert.equal(body.error, 'route-not-found');
  });

  it('rejects ambiguous content length headers', async () => {
    const { response } = await executeRequest({
      path: '/health',
      headers: {
        'content-length': '1, 2',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'ambiguous-content-length');
  });

  it('fails closed when a state-changing request body arrives without content-length metadata', async () => {
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      body: JSON.stringify({ event: 'missing-length' }),
      autoContentLength: false,
      headers: {
        'stripe-signature': 't=1710000000,v1=abcdef',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 400);
    assert.equal(body.error, 'missing-content-length');
  });

  it('keeps oversized rejection payloads redacted and free of request echoes', async () => {
    const { response } = await executeRequest({
      path: '/webhooks/stripe',
      method: 'POST',
      headers: {
        'content-length': '11',
        authorization: 'Bearer should-not-leak',
      },
      envOverrides: {
        REQUEST_MAX_BYTES: '10',
      },
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 413);
    assert.equal(body.error, 'payload-too-large');
    assert.equal('authorization' in body, false);
    assert.equal('requestHeaders' in body, false);
    assert.equal('body' in body, false);
  });
});
