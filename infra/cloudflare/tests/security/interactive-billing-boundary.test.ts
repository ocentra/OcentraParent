import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { executeRequest, readJson } from '../../src/testing.js';

const ACCOUNT_IDENTITY_BINDING_UNAVAILABLE = {
  error: 'manual-required',
  authState: 'parent-session-required',
  blocker: 'account-identity-binding-context-manual-required',
} as const;

async function assertAccountIdentityBindingUnavailable(response: Response): Promise<void> {
  assert.equal(response.status, 503);
  assert.deepEqual(await readJson<unknown>(response), ACCOUNT_IDENTITY_BINDING_UNAVAILABLE);
}

describe('interactive billing boundary', () => {
  it('does not treat a caller-supplied parent token as authority before origin and csrf evaluation', async () => {
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

    await assertAccountIdentityBindingUnavailable(response);
  });

  it('does not let an allowed origin bypass the missing Account identity binding', async () => {
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

    await assertAccountIdentityBindingUnavailable(response);
  });

  it('keeps change-plan and cancel blocked before caller-minted interactive proofs are considered', async () => {
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
    await assertAccountIdentityBindingUnavailable(changePlan.response);

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
    await assertAccountIdentityBindingUnavailable(cancel.response);
  });
});
