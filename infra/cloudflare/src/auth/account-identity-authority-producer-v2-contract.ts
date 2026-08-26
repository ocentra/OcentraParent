/**
 * Account-owned protected issuer v2 wire contract.
 *
 * This module is deliberately limited to the versioned edge contract. It does
 * not mint authority, create keys, or provide a signing fallback. The Account
 * owner supplies the signed bytes; Cloudflare only parses and verifies them.
 */

export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION = 'ocentra.account-authority-producer.v2' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN =
  'ocentra.account-authority-producer.signing.v2\0' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE = 'ocentra.account.authority.v2' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT = 'account-owned' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM = 'ecdsa-p256-sha256-p1363' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN = 'ocentra.account-issuer.transport.v2\0' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE =
  'ocentra.account-authority-producer.cloudflare.v2' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN =
  'ocentra.account-authority-producer.key-id.v2\0' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_DOMAIN =
  'ocentra.account-authority-producer.receipt-id.v2\0' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_DOMAIN =
  'ocentra.account-authority-producer.v2.binding\0' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX = 'sha256:ecdsa-p256:' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX = 'sha256:receipt:' as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_PREFIX = 'sha256:binding:' as const;

export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES = 64 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES = 65 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES = 1_024 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES = 16_384 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION = 9_007_199_254_740_991 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS = 300 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS = 30 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES =
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES +
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES * 13 +
  256;

export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND = 6 as const;
export const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND = 7 as const;

export const ACCOUNT_ISSUER_PROTOCOL_VERSION = 2 as const;
export const ACCOUNT_ISSUER_REQUEST_TAG = 1 as const;
export const ACCOUNT_ISSUER_RECEIPT_TAG = 2 as const;
export const ACCOUNT_ISSUER_MAX_FIELD_BYTES = 1_024 as const;
export const ACCOUNT_ISSUER_MAX_INNER_BYTES = 65_536 as const;
export const ACCOUNT_ISSUER_MAX_WIRE_BYTES = 131_072 as const;

export type AccountIdentityAuthorityProducerV2Operation = 'IssueCurrentAuthority' | 'AcknowledgeReceipt';

export interface AccountIdentityAuthorityProducerV2Claims {
  readonly accountId: string;
  readonly householdId: string;
  readonly provider: string;
  readonly providerSubject: string;
  readonly memberId: string;
  readonly deviceId: string;
  readonly sessionId: string;
}

export interface AccountIdentityAuthorityProducerV2Receipt {
  readonly receiptId: string;
  readonly operation: 'IssueCurrentAuthority';
  readonly accountId: string;
  readonly householdId: string;
  readonly serviceBindingId: string;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly payloadDigest: string;
  readonly keyId: string;
  readonly keyGeneration: number;
  readonly enrollmentGeneration: number;
  readonly authorityGeneration: number;
  readonly sessionGeneration: number;
  readonly issuedAt: string;
  readonly expiresAt: string;
}

export interface ParsedAccountIdentityAuthorityProducerV2Wire {
  readonly operation: AccountIdentityAuthorityProducerV2Operation;
  readonly signingBytes: Uint8Array;
  readonly signature: Uint8Array;
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
  readonly payload: Uint8Array;
}

export interface AccountIssuerRequestFrame {
  readonly tag: typeof ACCOUNT_ISSUER_REQUEST_TAG;
  readonly kind:
    | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND
    | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly keyId: string;
  readonly innerWire: Uint8Array;
}

export interface AccountIssuerReceiptFrame {
  readonly tag: typeof ACCOUNT_ISSUER_RECEIPT_TAG;
  readonly kind:
    | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND
    | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND;
  readonly receiptId: string;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly keyId: string;
  readonly resultDigest: string;
}

export type AccountIssuerFrame = AccountIssuerRequestFrame | AccountIssuerReceiptFrame;

const KEY_ID_PATTERN = /^sha256:ecdsa-p256:[0-9a-f]{64}$/u;
const RECEIPT_ID_PATTERN = /^sha256:receipt:[0-9a-f]{64}$/u;
const SERVICE_BINDING_ID_PATTERN = /^sha256:binding:[0-9a-f]{64}$/u;
const DIGEST_PATTERN = /^sha256:[0-9a-f]{64}$/u;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/u;

export function isAccountIdentityAuthorityProducerV2KeyId(value: string): boolean {
  return KEY_ID_PATTERN.test(value);
}

export function isAccountIdentityAuthorityProducerV2ReceiptId(value: string): boolean {
  return RECEIPT_ID_PATTERN.test(value);
}

export function isAccountIdentityAuthorityProducerV2ServiceBindingId(value: string): boolean {
  return SERVICE_BINDING_ID_PATTERN.test(value);
}

export function isAccountIdentityAuthorityProducerV2Digest(value: string): boolean {
  return DIGEST_PATTERN.test(value);
}

export function isAccountIdentityAuthorityProducerV2Text(value: string): boolean {
  const byteLength = new TextEncoder().encode(value).byteLength;
  return (
    value.trim().length > 0 &&
    byteLength <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES &&
    !CONTROL_CHARACTER_PATTERN.test(value)
  );
}

export function accountIdentityAuthorityProducerV2DomainBytes(): Uint8Array {
  return new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN);
}

export function accountIdentityAuthorityProducerV2KeyIdDomainBytes(): Uint8Array {
  return new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN);
}

export function accountIdentityAuthorityProducerV2ReceiptIdDomainBytes(): Uint8Array {
  return new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_DOMAIN);
}

export async function deriveAccountIdentityAuthorityProducerV2ServiceBindingId(
  service: string,
  accountId: string,
  householdId: string
): Promise<string> {
  const encoder = new TextEncoder();
  const fields = [encoder.encode(service), encoder.encode(accountId), encoder.encode(householdId)];
  const domain = encoder.encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_DOMAIN);
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
  const digest = new Uint8Array(await crypto.subtle.digest('SHA-256', input));
  return `${ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE_BINDING_ID_PREFIX}${Array.from(digest, (byte) => byte.toString(16).padStart(2, '0')).join('')}`;
}

export function accountIssuerTransportDomainBytes(): Uint8Array {
  return new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN);
}

export function parseAccountIdentityAuthorityProducerV2Wire(
  value: ArrayBuffer | Uint8Array
): ParsedAccountIdentityAuthorityProducerV2Wire | null {
  const wire = copyBytes(value);
  if (
    wire === null ||
    wire.byteLength <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES ||
    wire.byteLength > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES
  ) {
    return null;
  }

  const signingLength = wire.byteLength - ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES;
  const signingBytes = wire.slice(0, signingLength);
  const signature = wire.slice(signingLength);
  const cursor = new WireCursor(signingBytes);
  if (!cursor.readExact(accountIdentityAuthorityProducerV2DomainBytes())) return null;

  const operation = operationFromMessageKind(cursor.readByte());
  if (operation === null) return null;
  if (!readExpectedText(cursor, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION)) return null;
  if (!readExpectedText(cursor, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE)) return null;
  if (!readExpectedText(cursor, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT)) return null;
  if (!readExpectedText(cursor, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM)) return null;
  if (!readExpectedText(cursor, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE)) return null;

  const receiptId = cursor.readText();
  const keyId = cursor.readText();
  const serviceBindingId = cursor.readText();
  const keyGeneration = cursor.readGeneration();
  const enrollmentGeneration = cursor.readGeneration();
  const authorityGeneration = cursor.readGeneration();
  const sessionGeneration = cursor.readGeneration();
  const correlationId = cursor.readText();
  const idempotencyKey = cursor.readText();
  const issuedAt = cursor.readText();
  const expiresAt = cursor.readText();
  const payload = cursor.readField(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES);

  if (
    receiptId === null ||
    keyId === null ||
    serviceBindingId === null ||
    keyGeneration === null ||
    enrollmentGeneration === null ||
    authorityGeneration === null ||
    sessionGeneration === null ||
    correlationId === null ||
    idempotencyKey === null ||
    issuedAt === null ||
    expiresAt === null ||
    payload === null ||
    !cursor.finished ||
    !isAccountIdentityAuthorityProducerV2ReceiptId(receiptId) ||
    !isAccountIdentityAuthorityProducerV2KeyId(keyId) ||
    !isAccountIdentityAuthorityProducerV2ServiceBindingId(serviceBindingId)
  ) {
    return null;
  }

  return Object.freeze({
    operation,
    signingBytes,
    signature,
    receiptId,
    keyId,
    serviceBindingId,
    keyGeneration,
    enrollmentGeneration,
    authorityGeneration,
    sessionGeneration,
    correlationId,
    idempotencyKey,
    issuedAt,
    expiresAt,
    payload,
  });
}

function readExpectedText(cursor: WireCursor, expected: string): boolean {
  return cursor.readText() === expected;
}

function operationFromMessageKind(value: number | null): AccountIdentityAuthorityProducerV2Operation | null {
  if (value === ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND) return 'IssueCurrentAuthority';
  if (value === ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND) return 'AcknowledgeReceipt';
  return null;
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

function bytesEqual(left: Uint8Array, right: Uint8Array): boolean {
  return left.byteLength === right.byteLength && left.every((byte, index) => byte === right[index]);
}

class WireCursor {
  private offset = 0;

  public constructor(private readonly bytes: Uint8Array) {}

  public get finished(): boolean {
    return this.offset === this.bytes.byteLength;
  }

  public readByte(): number | null {
    const value = this.bytes[this.offset];
    if (value === undefined) return null;
    this.offset += 1;
    return value;
  }

  public readExact(expected: Uint8Array): boolean {
    const value = this.bytes.slice(this.offset, this.offset + expected.byteLength);
    if (!bytesEqual(value, expected)) return false;
    this.offset += expected.byteLength;
    return true;
  }

  public readText(): string | null {
    const field = this.readField(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES);
    if (field === null) return null;
    try {
      const value = new TextDecoder('utf-8', { fatal: true }).decode(field);
      return isAccountIdentityAuthorityProducerV2Text(value) ? value : null;
    } catch {
      return null;
    }
  }

  public readGeneration(): number | null {
    const field = this.readField(8);
    if (field === null || field.byteLength !== 8) return null;
    const value = new DataView(field.buffer, field.byteOffset, field.byteLength).getBigUint64(0, false);
    const number = Number(value);
    return Number.isSafeInteger(number) && number > 0 && number <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
      ? number
      : null;
  }

  public readField(maxBytes: number): Uint8Array | null {
    if (this.offset + 4 > this.bytes.byteLength) return null;
    const length = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.offset, 4).getUint32(0, false);
    this.offset += 4;
    if (length === 0 || length > maxBytes || this.offset + length > this.bytes.byteLength) return null;
    const field = this.bytes.slice(this.offset, this.offset + length);
    this.offset += length;
    return field;
  }
}
