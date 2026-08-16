import type { Queue } from '@cloudflare/workers-types';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { createTestHarness, executeRequest, readJson } from '../../src/testing.js';

interface AuthErrorResponse {
  error: string;
  reason: string;
}

interface ReconciliationAcceptedResponse {
  status: string;
  queued: boolean;
}

function createThrowingQueue(message: string): Queue {
  return {
    send: async (): Promise<void> => {
      throw new Error(message);
    },
    sendBatch: async (): Promise<void> => {
      throw new Error(message);
    },
  } as unknown as Queue;
}

describe('POST /admin/billing/reconciliation auth boundary', () => {
  it('rejects requests that are missing the internal queue signal', async () => {
    const { response } = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      body: {
        requestId: 'reconciliation-no-internal-signal',
      },
    });

    const body = await readJson<AuthErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'forbidden');
    assert.equal(body.reason, 'missing-internal-queue-signal');
  });

  it('rejects requests with an incorrect internal shared secret', async () => {
    const { response } = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'wrong-secret',
      },
      body: {
        requestId: 'reconciliation-bad-secret',
      },
    });

    const body = await readJson<AuthErrorResponse>(response);
    assert.equal(response.status, 403);
    assert.equal(body.error, 'forbidden');
    assert.equal(body.reason, 'internal-queue-secret-mismatch');
  });

  it('dead-letters reconciliation work when the primary queue binding is absent', async () => {
    const harness = createTestHarness();
    harness.env.BILLING_RECONCILIATION_QUEUE = undefined;

    const { response } = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      harness,
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-missing-queue',
      },
    });

    const body = await readJson<ReconciliationAcceptedResponse>(response);
    assert.equal(response.status, 202);
    assert.equal(body.status, 'accepted');
    assert.equal(body.queued, false);
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 1);

    const deadLetter = harness.deadLetterMessages[0] as Record<string, unknown>;
    assert.equal(deadLetter.disposition, 'dead-letter');
    assert.equal(deadLetter.sourceQueue, 'BILLING_RECONCILIATION_QUEUE');
    assert.equal(deadLetter.reason, 'reconciliation-queue-missing');
    assert.equal(deadLetter.errorMessage, null);
    assert.equal(typeof deadLetter.failedAt, 'string');

    const payload = deadLetter.payload as Record<string, unknown>;
    assert.equal(payload.action, 'reconciliation');
    assert.equal(payload.requestId, 'reconciliation-missing-queue');
    assert.equal(payload.actorRole, 'internal');
  });

  it('dead-letters reconciliation work when the primary queue send fails', async () => {
    const harness = createTestHarness();
    harness.env.BILLING_RECONCILIATION_QUEUE = createThrowingQueue('simulated-reconciliation-queue-failure');

    const { response } = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      harness,
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-queue-send-failure',
      },
    });

    const body = await readJson<ReconciliationAcceptedResponse>(response);
    assert.equal(response.status, 202);
    assert.equal(body.status, 'accepted');
    assert.equal(body.queued, false);
    assert.equal(harness.queueMessages.length, 0);
    assert.equal(harness.deadLetterMessages.length, 1);

    const deadLetter = harness.deadLetterMessages[0] as Record<string, unknown>;
    assert.equal(deadLetter.disposition, 'dead-letter');
    assert.equal(deadLetter.sourceQueue, 'BILLING_RECONCILIATION_QUEUE');
    assert.equal(deadLetter.reason, 'reconciliation-queue-send-failed');
    assert.equal(deadLetter.errorMessage, 'simulated-reconciliation-queue-failure');
    assert.equal(typeof deadLetter.failedAt, 'string');

    const payload = deadLetter.payload as Record<string, unknown>;
    assert.equal(payload.action, 'reconciliation');
    assert.equal(payload.requestId, 'reconciliation-queue-send-failure');
    assert.equal(payload.actorRole, 'internal');
  });
});
