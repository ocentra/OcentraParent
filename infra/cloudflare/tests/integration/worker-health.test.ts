import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { createTestHarness, executeRequest, readJson } from '../../src/testing.js';

const log = Logger.instance;
log.register(import.meta.url);

describe('GET /health', () => {
  it('succeeds through the worker boundary and summarizes binding health', async () => {
    const harness = createTestHarness({
      BILLING_AUDIT_R2: undefined,
    });
    const { response } = await executeRequest({
      path: '/health',
      harness,
    });

    const body = await readJson<any>(response);
    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.equal(body.service, 'cloudflare-control-plane');
    assert.equal(body.bindingStatus, 'ready');
    assert.equal(body.missingBindingCount, 0);
    assert.equal(body.seedSummary.pricingPlanCount, 3);
    assert.equal(body.seedSummary.referralFixtureCount, 2);
  });

  it('does not require auth to reach the health route', async () => {
    const { response } = await executeRequest({
      path: '/health',
    });

    assert.equal(response.status, 200);
  });

  it('does not disclose provider secrets, binding internals, or child-data fields', async () => {
    const stripeSecretKey = ['sk', 'live', 'health', 'fixture'].join('_');
    const stripeWebhookSecret = ['whsec', 'health', 'fixture'].join('_');
    const { response } = await executeRequest({
      path: '/health',
      envOverrides: {
        STRIPE_SECRET_KEY: stripeSecretKey,
        STRIPE_WEBHOOK_SECRET: stripeWebhookSecret,
      },
    });

    const text = await response.text();
    assert.equal(text.includes(stripeSecretKey), false);
    assert.equal(text.includes(stripeWebhookSecret), false);
    assert.equal(text.includes('childActivityCustody'), false);
    assert.equal(text.includes('ownerSubject'), false);
    assert.equal(text.includes('BILLING_D1'), false);
  });

  it('fails closed on missing required environment values without exposing validation details', async () => {
    const { response } = await executeRequest({
      path: '/health',
      envOverrides: {
        APP_ORIGIN: undefined as unknown as string,
      },
    });

    const body = await readJson<{ error: string; status: string }>(response);
    assert.equal(response.status, 500);
    assert.deepEqual(body, {
      error: 'environment-validation-failed',
      status: 'manual-required',
    });
    assert.equal(JSON.stringify(body).includes('APP_ORIGIN'), false);
  });
});
