import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { redactHeaders, redactPayload } from '../../src/security/redaction.js';

const log = Logger.instance;
log.register(import.meta.url);

describe('security redaction', () => {
  it('redacts provider secrets, signing refs, child-data markers, and evidence references', () => {
    const value = redactPayload({
      requestId: 'req-123',
      auditReference: 'audit:ok',
      ENTITLEMENT_SIGNING_KEY_REF: 'signing-key-test-ref',
      rawEvidenceRef: 'evidence://billing/demo-active',
      childProfileMarker: 'child-profile-present',
      provider: {
        webhookSecret: 'whsec_nested',
        apiKey: 'sk_live_nested',
        recoveryBundlePath: '/Users/sujan/private/recovery-bundle.txt',
      },
    }) as Record<string, unknown>;

    assert.deepEqual(value, {
      requestId: 'req-123',
      auditReference: 'audit:ok',
      ENTITLEMENT_SIGNING_KEY_REF: '[redacted]',
      rawEvidenceRef: '[redacted]',
      childProfileMarker: '[redacted]',
      provider: {
        webhookSecret: '[redacted]',
        apiKey: '[redacted]',
        recoveryBundlePath: '[redacted]',
      },
    });
  });

  it('redacts cookies, auth headers, session tokens, and provider signatures before error output', () => {
    const headers = redactHeaders(
      new Headers({
        authorization: 'Bearer should-not-leak',
        cookie: 'session=123',
        'x-session-token': 'session-token-123',
        'stripe-signature': 't=1710000000,v1=abcdef',
        origin: 'http://localhost:3000',
        'x-request-id': 'req-123',
      })
    );

    assert.deepEqual(headers, {
      authorization: '[redacted]',
      cookie: '[redacted]',
      origin: 'http://localhost:3000',
      'stripe-signature': '[redacted]',
      'x-request-id': 'req-123',
      'x-session-token': '[redacted]',
    });
  });

  it('redacts provider credential identifiers and key references by field name', () => {
    const value = redactPayload({
      RAZORPAY_KEY_ID: 'rzp-live-key-id',
      PAYPAL_CLIENT_ID: 'paypal-client-id',
      APPLE_STORE_KEY_REF: 'apple-store-key-ref',
      GOOGLE_PLAY_SERVICE_ACCOUNT_REF: 'google-service-account-ref',
      ENTITLEMENT_SIGNING_KEY_REF: 'entitlement-signing-key-ref',
    }) as Record<string, unknown>;

    assert.deepEqual(value, {
      RAZORPAY_KEY_ID: '[redacted]',
      PAYPAL_CLIENT_ID: '[redacted]',
      APPLE_STORE_KEY_REF: '[redacted]',
      GOOGLE_PLAY_SERVICE_ACCOUNT_REF: '[redacted]',
      ENTITLEMENT_SIGNING_KEY_REF: '[redacted]',
    });
  });
});
