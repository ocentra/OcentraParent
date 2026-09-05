import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { redactHeaders, redactPayload } from '../../src/security/redaction.js';

describe('redaction helpers', () => {
  it('redacts auth, session, and provider headers while preserving support-safe request identity', () => {
    const headers = new Headers({
      authorization: 'Bearer secret-token',
      cookie: 'session=abc',
      'x-session-token': 'session-token-123',
      'x-ocentra-csrf': 'csrf-token-123',
      'x-goog-signature': 'google-signature-123',
      'x-ocentra-role': 'support',
      'x-request-id': 'req-123',
    });

    assert.deepEqual(redactHeaders(headers), {
      authorization: '[redacted]',
      cookie: '[redacted]',
      'x-goog-signature': '[redacted]',
      'x-ocentra-csrf': '[redacted]',
      'x-ocentra-role': 'support',
      'x-request-id': 'req-123',
      'x-session-token': '[redacted]',
    });
  });

  it('redacts signing refs, child-data markers, evidence refs, recovery bundles, and local paths', () => {
    const payload = redactPayload({
      requestId: 'req-123',
      handlerKey: 'billing-status',
      auditReference: 'audit:billing:demo-active',
      ENTITLEMENT_SIGNING_KEY_REF: 'signing-key-test-ref',
      GOOGLE_PLAY_SERVICE_ACCOUNT_REF: 'google-play-service-account-test-ref',
      rawEvidenceRef: 'evidence://billing/demo-active',
      recoveryBundlePath: 'E:\\OcentraParent\\recovery-bundle\\bundle.zip',
      childActivityCustody: 'child-activity-present',
      childDeviceId: 'child-device-001',
      childName: 'child-name-001',
      policyText: 'private policy text',
      rawWebhookBody: '{"childId":"child-001"}',
      localDeviceSecretMarker: 'local-device-secret-123',
      stripeWebhookSecret: 'whsec_123',
      nested: {
        apiKey: 'sk_live_abc',
        filePath: 'C:\\Users\\sujan\\secrets\\billing.json',
        safe: 'visible',
      },
    }) as Record<string, unknown>;

    assert.deepEqual(payload, {
      requestId: 'req-123',
      handlerKey: 'billing-status',
      auditReference: 'audit:billing:demo-active',
      ENTITLEMENT_SIGNING_KEY_REF: '[redacted]',
      GOOGLE_PLAY_SERVICE_ACCOUNT_REF: '[redacted]',
      rawEvidenceRef: '[redacted]',
      recoveryBundlePath: '[redacted]',
      childActivityCustody: '[redacted]',
      childDeviceId: '[redacted]',
      childName: '[redacted]',
      policyText: '[redacted]',
      rawWebhookBody: '[redacted]',
      localDeviceSecretMarker: '[redacted]',
      stripeWebhookSecret: '[redacted]',
      nested: {
        apiKey: '[redacted]',
        filePath: '[redacted]',
        safe: 'visible',
      },
    });
  });

  it('redacts secret-like string values inside arrays without erasing harmless route context', () => {
    const payload = redactPayload({
      routeKey: 'stripe-webhook',
      attempts: ['whsec_nested_secret', '/Users/sujan/private/recovery-bundle.txt', 'plain-visible-value'],
    }) as Record<string, unknown>;

    assert.deepEqual(payload, {
      routeKey: 'stripe-webhook',
      attempts: ['[redacted]', '[redacted]', 'plain-visible-value'],
    });
  });
});
