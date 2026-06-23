import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

interface ErrorResponse {
  error: string;
}

describe('interactive billing boundary', () => {
  it('rejects checkout without explicit origin and csrf proof', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
      body: {
        requestId: 'checkout-no-csrf',
        planId: 'family-core',
      },
    });

    const body = await readJson<ErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'origin-validation-failed');
  });

  it('rejects checkout with missing csrf even when the origin is allowed', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/checkout',
      method: 'POST',
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer parent:demo-active',
      },
      body: {
        requestId: 'checkout-missing-csrf',
        planId: 'family-core',
      },
    });

    const body = await readJson<ErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'csrf-validation-failed');
  });

  it('rejects change-plan without an allowlisted origin and cancel without csrf proof', async () => {
    const changePlan = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
      body: {
        requestId: 'change-plan-no-origin',
        planId: 'family-max',
        abuseGateState: 'passed-turnstile',
      },
    });
    const changePlanBody = await readJson<ErrorResponse>(changePlan.response);
    assert.equal(changePlan.response.status, 403);
    assert.equal(changePlanBody.error, 'origin-validation-failed');

    const cancel = await executeRequest({
      path: '/auth/billing/cancel',
      method: 'POST',
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer parent:demo-active',
      },
      body: {
        requestId: 'cancel-no-csrf',
        abuseGateState: 'trusted-authenticated-session',
      },
    });
    const cancelBody = await readJson<ErrorResponse>(cancel.response);
    assert.equal(cancel.response.status, 403);
    assert.equal(cancelBody.error, 'csrf-validation-failed');
  });
});
