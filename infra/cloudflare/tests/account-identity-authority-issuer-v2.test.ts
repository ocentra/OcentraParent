import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { encodeAccountIssuerRequestFrame } from '../src/auth/account-identity-authority-producer-v2-transport.js';
import {
  deriveAccountIdentityAuthorityProducerV2KeyId,
  verifyAccountIdentityAuthorityProducerV2Currentness,
  verifyAccountIdentityAuthorityProducerV2RequestFrame,
  verifyAccountIdentityAuthorityProducerV2Wire,
} from '../src/auth/account-identity-authority-issuer-v2.js';
import {
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_PREFIX,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES,
  accountIdentityAuthorityProducerV2DomainBytes,
} from '../src/auth/account-identity-authority-producer-v2-contract.js';
import { createAccountIdentityAuthorityIssuerV2Store } from '../src/storage/account-identity-authority-issuer-v2.js';

const ZERO_HEX = '0'.repeat(64);
const KEY_ID = `${ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX}${ZERO_HEX}`;
const RECEIPT_ID = `${ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX}${ZERO_HEX}`;
const SERVICE_BINDING_ID = `${ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_PREFIX}${ZERO_HEX}`;
const NOW_MS = Date.parse('2026-08-28T00:01:00.000Z');

function p256Generator(): Uint8Array {
  return Uint8Array.from([
    0x04, 0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4, 0x40, 0xf2, 0x77, 0x03,
    0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96, 0x4f, 0xe3, 0x42, 0xe2, 0xfe,
    0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb, 0x4a, 0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce,
    0xcb, 0xb6, 0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
  ]);
}

function appendField(target: number[], value: string | Uint8Array): void {
  const bytes = typeof value === 'string' ? new TextEncoder().encode(value) : value;
  const length = bytes.byteLength;
  target.push((length >>> 24) & 0xff, (length >>> 16) & 0xff, (length >>> 8) & 0xff, length & 0xff);
  target.push(...bytes);
}

function appendGeneration(target: number[], value: number): void {
  const bytes = new Uint8Array(8);
  new DataView(bytes.buffer).setBigUint64(0, BigInt(value), false);
  appendField(target, bytes);
}

function minimallyParseableWire(): Uint8Array {
  const signingBytes: number[] = [
    ...accountIdentityAuthorityProducerV2DomainBytes(),
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND,
  ];
  for (const value of [
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
  ]) {
    appendField(signingBytes, value);
  }
  appendField(signingBytes, RECEIPT_ID);
  appendField(signingBytes, KEY_ID);
  appendField(signingBytes, SERVICE_BINDING_ID);
  for (const generation of [1, 2, 3, 4]) appendGeneration(signingBytes, generation);
  appendField(signingBytes, 'correlation-1');
  appendField(signingBytes, 'idempotency-1');
  appendField(signingBytes, '2026-08-28T00:00:00.000Z');
  appendField(signingBytes, '2026-08-28T00:05:00.000Z');
  appendField(signingBytes, '{}');
  return Uint8Array.from([...signingBytes, ...new Uint8Array(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES)]);
}

describe('account identity authority issuer v2 boundary', () => {
  it('derives algorithm-aware key ids only from canonical P-256 material', async () => {
    const publicKey = p256Generator();
    const keyId = await deriveAccountIdentityAuthorityProducerV2KeyId(publicKey);

    assert.match(keyId, /^sha256:ecdsa-p256:[0-9a-f]{64}$/u);
    assert.equal(keyId, await deriveAccountIdentityAuthorityProducerV2KeyId(publicKey));
    await assert.rejects(
      () => deriveAccountIdentityAuthorityProducerV2KeyId(new Uint8Array(64)),
      /invalid public key/u
    );
    assert.equal(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN.endsWith('\0'), true);
  });

  it('rejects malformed wire or noncanonical key before accepting authority', async () => {
    assert.deepEqual(await verifyAccountIdentityAuthorityProducerV2Wire(new Uint8Array(), p256Generator(), NOW_MS), {
      status: 'rejected',
      reason: 'invalid-wire',
    });
    assert.deepEqual(
      await verifyAccountIdentityAuthorityProducerV2Wire(minimallyParseableWire(), new Uint8Array(64), NOW_MS),
      { status: 'rejected', reason: 'invalid-key' }
    );
  });

  it('keeps D1 currentness and outer-frame mismatches fail-closed without a live owner binding', async () => {
    const store = createAccountIdentityAuthorityIssuerV2Store(undefined);
    assert.deepEqual(await store.readCurrentVerifier(SERVICE_BINDING_ID), {
      status: 'manual-required',
      reason: 'account-identity-issuer-v2-schema-missing',
    });
    assert.deepEqual(await verifyAccountIdentityAuthorityProducerV2Currentness(store, new Uint8Array(), NOW_MS), {
      status: 'rejected',
      reason: 'invalid-wire',
    });
    assert.deepEqual(
      await verifyAccountIdentityAuthorityProducerV2Currentness(store, minimallyParseableWire(), NOW_MS),
      { status: 'manual-required', reason: 'account-identity-issuer-v2-schema-missing' }
    );

    const frame = encodeAccountIssuerRequestFrame({
      kind: ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND,
      correlationId: 'outer-correlation',
      idempotencyKey: 'idempotency-1',
      keyId: KEY_ID,
      innerWire: minimallyParseableWire(),
    });
    assert.notEqual(frame, null);
    if (frame === null) return;
    assert.deepEqual(await verifyAccountIdentityAuthorityProducerV2RequestFrame(store, frame, NOW_MS), {
      status: 'rejected',
      reason: 'invalid-wire',
    });
  });
});
