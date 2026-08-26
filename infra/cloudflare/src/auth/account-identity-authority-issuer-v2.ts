import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { decodeJsonRejectingDuplicateKeys } from './account-identity-authority-json-decoder.js';
import { decodeAccountIssuerRequestFrame } from './account-identity-authority-producer-v2-transport.js';
import {
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
  accountIdentityAuthorityProducerV2ReceiptIdDomainBytes,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_ENROLLMENT_GENERATION,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES,
  type AccountIdentityAuthorityProducerV2Claims,
  type AccountIdentityAuthorityProducerV2Operation,
  type AccountIdentityAuthorityProducerV2Receipt,
  type ParsedAccountIdentityAuthorityProducerV2Wire,
  isAccountIdentityAuthorityProducerV2Digest,
  isAccountIdentityAuthorityProducerV2KeyId,
  isAccountIdentityAuthorityProducerV2ReceiptId,
  isAccountIdentityAuthorityProducerV2ServiceBindingId,
  isAccountIdentityAuthorityProducerV2Text,
  parseAccountIdentityAuthorityProducerV2Wire,
} from './account-identity-authority-producer-v2-contract.js';
import type {
  AccountIdentityAuthorityIssuerV2InboundReceipt,
  AccountIdentityAuthorityIssuerV2Store,
  AccountIdentityAuthorityIssuerV2Verifier,
} from '../storage/account-identity-authority-issuer-v2.js';

const log = Logger.instance;
log.register(import.meta.url);

const MILLIS_UTC_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/u;
const P256_HALF_ORDER = Uint8Array.from([
  0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xde, 0x73, 0x7d,
  0x56, 0xd3, 0x8b, 0xcf, 0x42, 0x79, 0xdc, 0xe5, 0x61, 0x7e, 0x31, 0x92, 0xa8,
]);

export type AccountIdentityAuthorityIssuerV2VerificationReason =
  | 'invalid-wire'
  | 'invalid-key'
  | 'invalid-key-id'
  | 'invalid-signature'
  | 'invalid-payload'
  | 'authority-expired'
  | 'unsupported-operation'
  | 'currentness-mismatch'
  | 'receipt-replay-conflict';

export type AccountIdentityAuthorityIssuerV2WireVerification =
  | {
      readonly status: 'trusted';
      readonly envelope: VerifiedAccountIdentityAuthorityIssuerV2Envelope;
    }
  | { readonly status: 'rejected'; readonly reason: AccountIdentityAuthorityIssuerV2VerificationReason };

export type AccountIdentityAuthorityIssuerV2CurrentVerification =
  | {
      readonly status: 'trusted';
      readonly envelope: VerifiedAccountIdentityAuthorityIssuerV2Envelope;
      readonly receipt: 'recorded' | 'duplicate';
    }
  | {
      readonly status: 'manual-required';
      readonly reason: 'account-identity-issuer-v2-schema-missing' | 'account-identity-issuer-v2-unavailable';
    }
  | {
      readonly status: 'rejected';
      readonly reason: AccountIdentityAuthorityIssuerV2VerificationReason;
    };

export interface VerifiedAccountIdentityAuthorityIssuerV2Envelope {
  readonly operation: AccountIdentityAuthorityProducerV2Operation;
  readonly receiptId: string;
  readonly keyId: string;
  readonly serviceBindingId: string;
  readonly keyGeneration: number;
  readonly enrollmentGeneration: number;
  readonly authorityGeneration: number;
  readonly sessionGeneration: number;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly issuedAt: string;
  readonly expiresAt: string;
  readonly payloadDigest: string;
  readonly authorityPayloadDigest: string;
  readonly claims: AccountIdentityAuthorityProducerV2Claims | null;
  readonly receiptPayload: AccountIdentityAuthorityProducerV2Receipt | null;
  readonly wireDigest: string;
}

export async function verifyAccountIdentityAuthorityProducerV2Wire(
  wire: ArrayBuffer | Uint8Array,
  publicKey: ArrayBuffer | Uint8Array,
  nowMs: number = Date.now()
): Promise<AccountIdentityAuthorityIssuerV2WireVerification> {
  const parsed = parseAccountIdentityAuthorityProducerV2Wire(wire);
  if (parsed === null) return rejected('invalid-wire');
  const keyBytes = copyBytes(publicKey);
  if (keyBytes === null || !isCanonicalPublicKey(keyBytes)) return rejected('invalid-key');
  if (!isValidClock(nowMs)) return rejected('authority-expired');

  let expectedKeyId: string;
  try {
    expectedKeyId = await deriveAccountIdentityAuthorityProducerV2KeyId(keyBytes);
  } catch {
    return rejected('invalid-key-id');
  }
  if (expectedKeyId !== parsed.keyId || !isAccountIdentityAuthorityProducerV2KeyId(parsed.keyId)) {
    return rejected('invalid-key-id');
  }
  if (!isLowS(parsed.signature)) return rejected('invalid-signature');

  let signatureValid = false;
  try {
    const cryptoKey = await crypto.subtle.importKey(
      'raw',
      asArrayBuffer(keyBytes),
      { name: 'ECDSA', namedCurve: 'P-256' },
      false,
      ['verify']
    );
    signatureValid = await crypto.subtle.verify(
      { name: 'ECDSA', hash: 'SHA-256' },
      cryptoKey,
      asArrayBuffer(parsed.signature),
      asArrayBuffer(parsed.signingBytes)
    );
  } catch {
    return rejected('invalid-key');
  }
  if (!signatureValid) return rejected('invalid-signature');

  const lifetime = validateLifetime(parsed, nowMs);
  if (lifetime !== null) return rejected(lifetime);

  const payloadText = decodeUtf8(parsed.payload);
  if (payloadText === null) return rejected('invalid-payload');
  const decoded = decodeJsonRejectingDuplicateKeys(payloadText);
  if (decoded.status !== 'decoded' || JSON.stringify(decoded.value) !== payloadText) {
    return rejected('invalid-payload');
  }

  const payloadDigest = await sha256Prefixed(parsed.payload);
  if (parsed.operation === 'IssueCurrentAuthority') {
    const claims = parseClaims(decoded.value);
    if (claims === null || !isAccountIdentityAuthorityProducerV2ReceiptId(parsed.receiptId)) {
      return rejected('invalid-payload');
    }
    const expectedReceiptId = await deriveReceiptId(parsed.correlationId, parsed.idempotencyKey, payloadDigest);
    if (expectedReceiptId !== parsed.receiptId) return rejected('invalid-payload');
    return trustedEnvelope(parsed, payloadDigest, payloadDigest, claims, null, wire);
  }

  const receipt = parseReceipt(decoded.value);
  if (
    receipt === null ||
    receipt.receiptId !== parsed.receiptId ||
    receipt.keyId !== parsed.keyId ||
    receipt.serviceBindingId !== parsed.serviceBindingId ||
    receipt.keyGeneration !== parsed.keyGeneration ||
    receipt.enrollmentGeneration !== parsed.enrollmentGeneration ||
    receipt.authorityGeneration !== parsed.authorityGeneration ||
    receipt.sessionGeneration !== parsed.sessionGeneration ||
    receipt.correlationId !== parsed.correlationId ||
    receipt.idempotencyKey !== parsed.idempotencyKey
  ) {
    return rejected('invalid-payload');
  }
  return trustedEnvelope(parsed, payloadDigest, receipt.payloadDigest, null, receipt, wire);
}

export async function verifyAccountIdentityAuthorityProducerV2Currentness(
  store: AccountIdentityAuthorityIssuerV2Store,
  wire: ArrayBuffer | Uint8Array,
  nowMs: number = Date.now()
): Promise<AccountIdentityAuthorityIssuerV2CurrentVerification> {
  const parsed = parseAccountIdentityAuthorityProducerV2Wire(wire);
  if (parsed === null) return rejectedCurrent('invalid-wire');
  let current: Awaited<ReturnType<AccountIdentityAuthorityIssuerV2Store['readCurrentVerifier']>>;
  try {
    current = await store.readCurrentVerifier(parsed.serviceBindingId);
  } catch {
    return manualRequired('account-identity-issuer-v2-unavailable');
  }
  if (current.status === 'manual-required') {
    return manualRequired(current.reason);
  }
  if (current.status !== 'current') return rejectedCurrent('currentness-mismatch');
  if (!matchesCurrentVerifier(parsed, current.verifier)) return rejectedCurrent('currentness-mismatch');

  const verified = await verifyAccountIdentityAuthorityProducerV2Wire(wire, current.verifier.publicKey, nowMs);
  if (verified.status !== 'trusted') return verified;
  if (!matchesAuthorityBinding(verified.envelope, current.verifier)) return rejectedCurrent('currentness-mismatch');

  const inbound: AccountIdentityAuthorityIssuerV2InboundReceipt = {
    receiptId: verified.envelope.receiptId,
    operation: verified.envelope.operation,
    accountId: current.verifier.accountId,
    householdId: current.verifier.householdId,
    provider: current.verifier.provider,
    providerSubject: current.verifier.providerSubject,
    serviceBindingId: verified.envelope.serviceBindingId,
    correlationId: verified.envelope.correlationId,
    idempotencyKey: verified.envelope.idempotencyKey,
    payloadDigest: verified.envelope.payloadDigest,
    authorityPayloadDigest: verified.envelope.authorityPayloadDigest,
    keyId: verified.envelope.keyId,
    keyGeneration: verified.envelope.keyGeneration,
    enrollmentGeneration: verified.envelope.enrollmentGeneration,
    authorityGeneration: verified.envelope.authorityGeneration,
    sessionGeneration: verified.envelope.sessionGeneration,
    issuedAt: verified.envelope.issuedAt,
    expiresAt: verified.envelope.expiresAt,
    wireDigest: verified.envelope.wireDigest,
  };
  let receiptResult: Awaited<ReturnType<AccountIdentityAuthorityIssuerV2Store['recordInboundReceipt']>>;
  try {
    receiptResult = await store.recordInboundReceipt(inbound, current.verifier);
  } catch {
    return manualRequired('account-identity-issuer-v2-unavailable');
  }
  if (receiptResult.status === 'manual-required') return manualRequired(receiptResult.reason);
  if (receiptResult.status === 'currentness-mismatch') return rejectedCurrent('currentness-mismatch');
  if (receiptResult.status === 'conflict') return rejectedCurrent('receipt-replay-conflict');
  log.logInfo(
    'account issuer v2 inbound receipt accepted',
    getStackTrace(),
    {
      owner: 'cloudflare-wp05-account-identity-authority-issuer-v2',
      boundary: 'account-issuer-v2-inbound-receipt',
      result: receiptResult.status,
      operation: verified.envelope.operation,
      serviceBindingId: verified.envelope.serviceBindingId,
      keyId: verified.envelope.keyId,
      redactionState: 'redacted',
    },
    false
  );
  return { status: 'trusted', envelope: verified.envelope, receipt: receiptResult.status };
}

/**
 * Verify the complete protected request boundary. The outer frame is only a
 * routing envelope; the inner v2 bytes remain the sole signed authority
 * payload. Both layers must nevertheless agree before D1 currentness or
 * inbound-receipt state is touched.
 */
export function verifyAccountIdentityAuthorityProducerV2RequestFrame(
  store: AccountIdentityAuthorityIssuerV2Store,
  frame: ArrayBuffer | Uint8Array,
  nowMs: number = Date.now()
): Promise<AccountIdentityAuthorityIssuerV2CurrentVerification> {
  const request = decodeAccountIssuerRequestFrame(frame);
  if (request === null) return Promise.resolve(rejectedCurrent('invalid-wire'));
  const parsed = parseAccountIdentityAuthorityProducerV2Wire(request.innerWire);
  if (parsed === null) return Promise.resolve(rejectedCurrent('invalid-wire'));
  const expectedKind =
    parsed.operation === 'IssueCurrentAuthority'
      ? ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND
      : ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND;
  if (
    request.kind !== expectedKind ||
    request.correlationId !== parsed.correlationId ||
    request.idempotencyKey !== parsed.idempotencyKey ||
    request.keyId !== parsed.keyId
  ) {
    return Promise.resolve(rejectedCurrent('invalid-wire'));
  }
  return verifyAccountIdentityAuthorityProducerV2Currentness(store, request.innerWire, nowMs);
}

export async function deriveAccountIdentityAuthorityProducerV2KeyId(
  publicKey: ArrayBuffer | Uint8Array
): Promise<string> {
  const keyBytes = copyBytes(publicKey);
  if (keyBytes === null || !isCanonicalPublicKey(keyBytes)) throw new Error('invalid public key');
  const domain = new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN);
  const input = new Uint8Array(domain.byteLength + keyBytes.byteLength);
  input.set(domain);
  input.set(keyBytes, domain.byteLength);
  const digest = new Uint8Array(await crypto.subtle.digest('SHA-256', input));
  return `${ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX}${toHex(digest)}`;
}

function parseClaims(value: unknown): AccountIdentityAuthorityProducerV2Claims | null {
  const record = exactObject(value, [
    'accountId',
    'householdId',
    'provider',
    'providerSubject',
    'memberId',
    'deviceId',
    'sessionId',
  ]);
  if (record === null) return null;
  const accountId = textField(record.accountId);
  const householdId = textField(record.householdId);
  const provider = textField(record.provider);
  const providerSubject = textField(record.providerSubject);
  const memberId = textField(record.memberId);
  const deviceId = textField(record.deviceId);
  const sessionId = textField(record.sessionId);
  if (
    accountId === null ||
    householdId === null ||
    provider === null ||
    providerSubject === null ||
    memberId === null ||
    deviceId === null ||
    sessionId === null ||
    (provider !== 'authjs' && provider !== 'firebase')
  ) {
    return null;
  }
  return {
    accountId,
    householdId,
    provider,
    providerSubject,
    memberId,
    deviceId,
    sessionId,
  };
}

function parseReceipt(value: unknown): AccountIdentityAuthorityProducerV2Receipt | null {
  const record = exactObject(value, [
    'receiptId',
    'operation',
    'accountId',
    'householdId',
    'serviceBindingId',
    'correlationId',
    'idempotencyKey',
    'payloadDigest',
    'keyId',
    'keyGeneration',
    'enrollmentGeneration',
    'authorityGeneration',
    'sessionGeneration',
    'issuedAt',
    'expiresAt',
  ]);
  if (record === null || record.operation !== 'IssueCurrentAuthority') return null;
  const receiptId = textField(record.receiptId);
  const accountId = textField(record.accountId);
  const householdId = textField(record.householdId);
  const serviceBindingId = textField(record.serviceBindingId);
  const correlationId = textField(record.correlationId);
  const idempotencyKey = textField(record.idempotencyKey);
  const payloadDigest = textField(record.payloadDigest);
  const keyId = textField(record.keyId);
  const issuedAt = textField(record.issuedAt);
  const expiresAt = textField(record.expiresAt);
  const keyGeneration = generationField(record.keyGeneration);
  const enrollmentGeneration = generationField(record.enrollmentGeneration);
  const authorityGeneration = generationField(record.authorityGeneration);
  const sessionGeneration = generationField(record.sessionGeneration);
  const issuedTimestamp = issuedAt === null ? null : parseTimestamp(issuedAt);
  const expiresTimestamp = expiresAt === null ? null : parseTimestamp(expiresAt);
  if (
    receiptId === null ||
    accountId === null ||
    householdId === null ||
    serviceBindingId === null ||
    correlationId === null ||
    idempotencyKey === null ||
    payloadDigest === null ||
    keyId === null ||
    issuedAt === null ||
    expiresAt === null ||
    keyGeneration === null ||
    enrollmentGeneration === null ||
    authorityGeneration === null ||
    sessionGeneration === null ||
    issuedTimestamp === null ||
    expiresTimestamp === null ||
    expiresTimestamp <= issuedTimestamp ||
    !isAccountIdentityAuthorityProducerV2ReceiptId(receiptId) ||
    !isAccountIdentityAuthorityProducerV2Digest(payloadDigest) ||
    !isAccountIdentityAuthorityProducerV2KeyId(keyId) ||
    !isAccountIdentityAuthorityProducerV2ServiceBindingId(serviceBindingId)
  ) {
    return null;
  }
  return {
    receiptId,
    operation: 'IssueCurrentAuthority',
    accountId,
    householdId,
    serviceBindingId,
    correlationId,
    idempotencyKey,
    payloadDigest,
    keyId,
    keyGeneration,
    enrollmentGeneration,
    authorityGeneration,
    sessionGeneration,
    issuedAt,
    expiresAt,
  };
}

function exactObject(value: unknown, keys: ReadonlyArray<string>): Record<string, unknown> | null {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  const actualKeys = Object.keys(record);
  return actualKeys.length === keys.length && actualKeys.every((key, index) => key === keys[index]) ? record : null;
}

function textField(value: unknown): string | null {
  return typeof value === 'string' && isAccountIdentityAuthorityProducerV2Text(value) ? value : null;
}

function generationField(value: unknown): number | null {
  return isSafeGeneration(value) ? value : null;
}

function matchesCurrentVerifier(
  parsed: ParsedAccountIdentityAuthorityProducerV2Wire,
  current: AccountIdentityAuthorityIssuerV2Verifier
): boolean {
  return (
    current.status === 'active' &&
    current.service === ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE &&
    parsed.serviceBindingId === current.serviceBindingId &&
    parsed.keyId === current.keyId &&
    parsed.keyGeneration === current.keyGeneration &&
    parsed.enrollmentGeneration === current.enrollmentGeneration &&
    parsed.authorityGeneration === current.authorityGeneration &&
    parsed.sessionGeneration === current.sessionGeneration
  );
}

function matchesAuthorityBinding(
  envelope: VerifiedAccountIdentityAuthorityIssuerV2Envelope,
  current: AccountIdentityAuthorityIssuerV2Verifier
): boolean {
  if (
    envelope.serviceBindingId !== current.serviceBindingId ||
    envelope.keyId !== current.keyId ||
    envelope.keyGeneration !== current.keyGeneration ||
    envelope.enrollmentGeneration !== current.enrollmentGeneration ||
    envelope.authorityGeneration !== current.authorityGeneration ||
    envelope.sessionGeneration !== current.sessionGeneration
  ) {
    return false;
  }
  if (envelope.claims !== null) {
    return (
      envelope.claims.accountId === current.accountId &&
      envelope.claims.householdId === current.householdId &&
      envelope.claims.provider === current.provider &&
      envelope.claims.providerSubject === current.providerSubject
    );
  }
  const receipt = envelope.receiptPayload;
  return receipt !== null && receipt.accountId === current.accountId && receipt.householdId === current.householdId;
}

function trustedEnvelope(
  parsed: ParsedAccountIdentityAuthorityProducerV2Wire,
  payloadDigest: string,
  authorityPayloadDigest: string,
  claims: AccountIdentityAuthorityProducerV2Claims | null,
  receiptPayload: AccountIdentityAuthorityProducerV2Receipt | null,
  wire: ArrayBuffer | Uint8Array
): Promise<AccountIdentityAuthorityIssuerV2WireVerification> {
  const wireBytes = copyBytes(wire);
  if (wireBytes === null) return Promise.resolve(rejected('invalid-wire'));
  return sha256Prefixed(wireBytes).then((wireDigest) => ({
    status: 'trusted',
    envelope: Object.freeze({
      operation: parsed.operation,
      receiptId: parsed.receiptId,
      keyId: parsed.keyId,
      serviceBindingId: parsed.serviceBindingId,
      keyGeneration: parsed.keyGeneration,
      enrollmentGeneration: parsed.enrollmentGeneration,
      authorityGeneration: parsed.authorityGeneration,
      sessionGeneration: parsed.sessionGeneration,
      correlationId: parsed.correlationId,
      idempotencyKey: parsed.idempotencyKey,
      issuedAt: parsed.issuedAt,
      expiresAt: parsed.expiresAt,
      payloadDigest,
      authorityPayloadDigest,
      claims,
      receiptPayload,
      wireDigest,
    }),
  }));
}

function validateLifetime(
  parsed: ParsedAccountIdentityAuthorityProducerV2Wire,
  nowMs: number
): AccountIdentityAuthorityIssuerV2VerificationReason | null {
  const issuedAt = parseTimestamp(parsed.issuedAt);
  const expiresAt = parseTimestamp(parsed.expiresAt);
  if (issuedAt === null || expiresAt === null) return 'invalid-wire';
  if (
    issuedAt > nowMs + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS * 1_000 ||
    expiresAt <= issuedAt ||
    expiresAt <= nowMs ||
    expiresAt > issuedAt + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS * 1_000
  ) {
    return 'authority-expired';
  }
  return null;
}

function parseTimestamp(value: string): number | null {
  if (!MILLIS_UTC_PATTERN.test(value)) return null;
  const timestamp = Date.parse(value);
  return Number.isSafeInteger(timestamp) && timestamp >= 0 && new Date(timestamp).toISOString() === value
    ? timestamp
    : null;
}

function isCanonicalPublicKey(value: Uint8Array): boolean {
  return (
    value.byteLength === ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES &&
    value[0] === 0x04 &&
    value.slice(1).some((byte) => byte !== 0)
  );
}

function isLowS(signature: Uint8Array): boolean {
  if (signature.byteLength !== ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES) return false;
  const s = signature.slice(32);
  if (!s.some((byte) => byte !== 0)) return false;
  for (let index = 0; index < P256_HALF_ORDER.byteLength; index += 1) {
    if (s[index] < P256_HALF_ORDER[index]) return true;
    if (s[index] > P256_HALF_ORDER[index]) return false;
  }
  return true;
}

function isSafeGeneration(value: unknown): value is number {
  return (
    typeof value === 'number' &&
    Number.isSafeInteger(value) &&
    value > 0 &&
    value <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_ENROLLMENT_GENERATION
  );
}

function isValidClock(value: number): boolean {
  return Number.isSafeInteger(value) && value >= 0;
}

function decodeUtf8(value: Uint8Array): string | null {
  try {
    return new TextDecoder('utf-8', { fatal: true }).decode(value);
  } catch {
    return null;
  }
}

function copyBytes(value: ArrayBuffer | Uint8Array): Uint8Array | null {
  try {
    const source = value instanceof Uint8Array ? value : new Uint8Array(value);
    const copy = new Uint8Array(source.byteLength);
    copy.set(source);
    return copy;
  } catch {
    return null;
  }
}

async function sha256Prefixed(value: Uint8Array): Promise<string> {
  return `sha256:${toHex(new Uint8Array(await crypto.subtle.digest('SHA-256', asArrayBuffer(value))))}`;
}

async function deriveReceiptId(correlationId: string, idempotencyKey: string, payloadDigest: string): Promise<string> {
  const encoder = new TextEncoder();
  const domain = accountIdentityAuthorityProducerV2ReceiptIdDomainBytes();
  const fields = [encoder.encode(correlationId), encoder.encode(idempotencyKey), encoder.encode(payloadDigest)];
  const input = new Uint8Array(domain.byteLength + fields.reduce((total, field) => total + 8 + field.byteLength, 0));
  input.set(domain);
  const view = new DataView(input.buffer);
  let offset = domain.byteLength;
  for (const field of fields) {
    view.setBigUint64(offset, BigInt(field.byteLength), false);
    offset += 8;
    input.set(field, offset);
    offset += field.byteLength;
  }
  return `sha256:receipt:${toHex(new Uint8Array(await crypto.subtle.digest('SHA-256', input)))}`;
}

function toHex(value: Uint8Array): string {
  return Array.from(value, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

function asArrayBuffer(value: Uint8Array): ArrayBuffer {
  const copy = new Uint8Array(new ArrayBuffer(value.byteLength));
  copy.set(value);
  return copy.buffer;
}

function rejected(
  reason: AccountIdentityAuthorityIssuerV2VerificationReason
): AccountIdentityAuthorityIssuerV2WireVerification {
  log.logWarn(
    'account issuer v2 wire rejected',
    getStackTrace(),
    {
      owner: 'cloudflare-wp05-account-identity-authority-issuer-v2',
      boundary: 'account-issuer-v2-verifier',
      result: 'rejected',
      noClaimReason: reason,
      redactionState: 'redacted',
    },
    false
  );
  return { status: 'rejected', reason };
}

function rejectedCurrent(
  reason: AccountIdentityAuthorityIssuerV2VerificationReason
): AccountIdentityAuthorityIssuerV2CurrentVerification {
  log.logWarn(
    'account issuer v2 currentness rejected',
    getStackTrace(),
    {
      owner: 'cloudflare-wp05-account-identity-authority-issuer-v2',
      boundary: 'account-issuer-v2-currentness',
      result: 'rejected',
      noClaimReason: reason,
      redactionState: 'redacted',
    },
    false
  );
  return { status: 'rejected', reason };
}

function manualRequired(
  reason: 'account-identity-issuer-v2-schema-missing' | 'account-identity-issuer-v2-unavailable'
): AccountIdentityAuthorityIssuerV2CurrentVerification {
  log.logWarn(
    'account issuer v2 currentness unavailable',
    getStackTrace(),
    {
      owner: 'cloudflare-wp05-account-identity-authority-issuer-v2',
      boundary: 'account-issuer-v2-d1-currentness',
      result: 'blocked',
      noClaimReason: reason,
      redactionState: 'redacted',
    },
    false
  );
  return { status: 'manual-required', reason };
}
