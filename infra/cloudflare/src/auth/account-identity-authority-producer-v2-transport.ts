import {
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
  ACCOUNT_ISSUER_MAX_FIELD_BYTES,
  ACCOUNT_ISSUER_MAX_INNER_BYTES,
  ACCOUNT_ISSUER_MAX_WIRE_BYTES,
  ACCOUNT_ISSUER_PROTOCOL_VERSION,
  ACCOUNT_ISSUER_RECEIPT_TAG,
  ACCOUNT_ISSUER_REQUEST_TAG,
  type AccountIssuerFrame,
  type AccountIssuerReceiptFrame,
  type AccountIssuerRequestFrame,
  isAccountIdentityAuthorityProducerV2KeyId,
  isAccountIdentityAuthorityProducerV2Digest,
  isAccountIdentityAuthorityProducerV2Text,
  accountIssuerTransportDomainBytes,
} from './account-identity-authority-producer-v2-contract.js';

export type AccountIssuerMessageKind =
  | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND
  | typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND;

export function decodeAccountIssuerRequestFrame(value: ArrayBuffer | Uint8Array): AccountIssuerRequestFrame | null {
  const decoded = decodeFrame(value, ACCOUNT_ISSUER_REQUEST_TAG, 4);
  if (decoded === null) return null;
  const [correlationId, idempotencyKey, keyId, innerWire] = decoded.fields;
  if (
    typeof correlationId !== 'string' ||
    typeof idempotencyKey !== 'string' ||
    typeof keyId !== 'string' ||
    !(innerWire instanceof Uint8Array) ||
    !isAccountIdentityAuthorityProducerV2KeyId(keyId)
  ) {
    return null;
  }
  return {
    tag: ACCOUNT_ISSUER_REQUEST_TAG,
    kind: decoded.kind,
    correlationId,
    idempotencyKey,
    keyId,
    innerWire,
  };
}

export function decodeAccountIssuerReceiptFrame(value: ArrayBuffer | Uint8Array): AccountIssuerReceiptFrame | null {
  const decoded = decodeFrame(value, ACCOUNT_ISSUER_RECEIPT_TAG, 5);
  if (decoded === null) return null;
  const [receiptId, correlationId, idempotencyKey, keyId, resultDigest] = decoded.fields;
  if (
    typeof receiptId !== 'string' ||
    typeof correlationId !== 'string' ||
    typeof idempotencyKey !== 'string' ||
    typeof keyId !== 'string' ||
    typeof resultDigest !== 'string' ||
    !isAccountIdentityAuthorityProducerV2KeyId(keyId) ||
    !isAccountIdentityAuthorityProducerV2Digest(resultDigest)
  ) {
    return null;
  }
  return {
    tag: ACCOUNT_ISSUER_RECEIPT_TAG,
    kind: decoded.kind,
    receiptId,
    correlationId,
    idempotencyKey,
    keyId,
    resultDigest,
  };
}

export function decodeAccountIssuerFrame(value: ArrayBuffer | Uint8Array): AccountIssuerFrame | null {
  const bytes = copyBytes(value);
  if (bytes === null || bytes.byteLength === 0 || bytes.byteLength > ACCOUNT_ISSUER_MAX_WIRE_BYTES) return null;
  const tag = bytes[accountIssuerTransportDomainBytes().byteLength + 2];
  if (tag === ACCOUNT_ISSUER_REQUEST_TAG) return decodeAccountIssuerRequestFrame(bytes);
  if (tag === ACCOUNT_ISSUER_RECEIPT_TAG) return decodeAccountIssuerReceiptFrame(bytes);
  return null;
}

export function encodeAccountIssuerRequestFrame(request: {
  readonly kind: AccountIssuerMessageKind;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly keyId: string;
  readonly innerWire: ArrayBuffer | Uint8Array;
}): Uint8Array | null {
  const innerWire = copyBytes(request.innerWire);
  if (
    innerWire === null ||
    innerWire.byteLength === 0 ||
    innerWire.byteLength > ACCOUNT_ISSUER_MAX_INNER_BYTES ||
    !isAccountIdentityAuthorityProducerV2Text(request.correlationId) ||
    !isAccountIdentityAuthorityProducerV2Text(request.idempotencyKey) ||
    !isAccountIdentityAuthorityProducerV2Text(request.keyId) ||
    !isAccountIdentityAuthorityProducerV2KeyId(request.keyId)
  ) {
    return null;
  }
  return encodeFrame(ACCOUNT_ISSUER_REQUEST_TAG, request.kind, [
    encodeText(request.correlationId),
    encodeText(request.idempotencyKey),
    encodeText(request.keyId),
    innerWire,
  ]);
}

export function encodeAccountIssuerReceiptFrame(receipt: {
  readonly kind: AccountIssuerMessageKind;
  readonly receiptId: string;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly keyId: string;
  readonly resultDigest: string;
}): Uint8Array | null {
  if (
    !isAccountIdentityAuthorityProducerV2Text(receipt.receiptId) ||
    !isAccountIdentityAuthorityProducerV2Text(receipt.correlationId) ||
    !isAccountIdentityAuthorityProducerV2Text(receipt.idempotencyKey) ||
    !isAccountIdentityAuthorityProducerV2Text(receipt.keyId) ||
    !isAccountIdentityAuthorityProducerV2Text(receipt.resultDigest) ||
    !isAccountIdentityAuthorityProducerV2KeyId(receipt.keyId) ||
    !isAccountIdentityAuthorityProducerV2Digest(receipt.resultDigest)
  ) {
    return null;
  }
  return encodeFrame(ACCOUNT_ISSUER_RECEIPT_TAG, receipt.kind, [
    encodeText(receipt.receiptId),
    encodeText(receipt.correlationId),
    encodeText(receipt.idempotencyKey),
    encodeText(receipt.keyId),
    encodeText(receipt.resultDigest),
  ]);
}

function decodeFrame(
  value: ArrayBuffer | Uint8Array,
  expectedTag: typeof ACCOUNT_ISSUER_REQUEST_TAG | typeof ACCOUNT_ISSUER_RECEIPT_TAG,
  fieldCount: 4 | 5
): { readonly kind: AccountIssuerMessageKind; readonly fields: ReadonlyArray<string | Uint8Array> } | null {
  const bytes = copyBytes(value);
  if (bytes === null || bytes.byteLength === 0 || bytes.byteLength > ACCOUNT_ISSUER_MAX_WIRE_BYTES) return null;
  const cursor = new FrameCursor(bytes);
  if (!cursor.readExact(accountIssuerTransportDomainBytes())) return null;
  if (cursor.readU16() !== ACCOUNT_ISSUER_PROTOCOL_VERSION) return null;
  if (cursor.readByte() !== expectedTag) return null;
  const kind = cursor.readByte();
  if (
    kind !== ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND &&
    kind !== ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND
  ) {
    return null;
  }
  if (cursor.readText() !== ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE) return null;

  const fields: Array<string | Uint8Array> = [];
  for (let index = 0; index < fieldCount; index += 1) {
    const field =
      expectedTag === ACCOUNT_ISSUER_REQUEST_TAG && index === 3
        ? cursor.readField(ACCOUNT_ISSUER_MAX_INNER_BYTES)
        : cursor.readText();
    if (field === null) return null;
    fields.push(field);
  }
  return cursor.finished ? { kind, fields } : null;
}

function encodeFrame(
  tag: typeof ACCOUNT_ISSUER_REQUEST_TAG | typeof ACCOUNT_ISSUER_RECEIPT_TAG,
  kind: AccountIssuerMessageKind,
  fields: ReadonlyArray<Uint8Array>
): Uint8Array | null {
  const domain = accountIssuerTransportDomainBytes();
  const service = new TextEncoder().encode(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE);
  const encodedFields = fields.map((field, index) => {
    const max =
      tag === ACCOUNT_ISSUER_REQUEST_TAG && index === 3
        ? ACCOUNT_ISSUER_MAX_INNER_BYTES
        : ACCOUNT_ISSUER_MAX_FIELD_BYTES;
    return field.byteLength > 0 && field.byteLength <= max ? field : null;
  });
  if (encodedFields.some((field) => field === null)) return null;
  const total =
    domain.byteLength +
    2 +
    2 +
    4 +
    service.byteLength +
    encodedFields.reduce((sum, field) => sum + 4 + (field?.byteLength ?? 0), 0);
  if (total > ACCOUNT_ISSUER_MAX_WIRE_BYTES) return null;
  const wire = new Uint8Array(total);
  let offset = 0;
  wire.set(domain, offset);
  offset += domain.byteLength;
  new DataView(wire.buffer).setUint16(offset, ACCOUNT_ISSUER_PROTOCOL_VERSION, false);
  offset += 2;
  wire[offset++] = tag;
  wire[offset++] = kind;
  writeField(wire, service, offset);
  offset += 4 + service.byteLength;
  for (const field of encodedFields) {
    if (field === null) return null;
    writeField(wire, field, offset);
    offset += 4 + field.byteLength;
  }
  return wire;
}

function encodeText(value: string): Uint8Array {
  return new TextEncoder().encode(value);
}

function writeField(target: Uint8Array, value: Uint8Array, offset: number): void {
  new DataView(target.buffer).setUint32(offset, value.byteLength, false);
  target.set(value, offset + 4);
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

class FrameCursor {
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

  public readU16(): number | null {
    if (this.offset + 2 > this.bytes.byteLength) return null;
    const value = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.offset, 2).getUint16(0, false);
    this.offset += 2;
    return value;
  }

  public readExact(expected: Uint8Array): boolean {
    const actual = this.bytes.slice(this.offset, this.offset + expected.byteLength);
    if (actual.byteLength !== expected.byteLength || !actual.every((byte, index) => byte === expected[index]))
      return false;
    this.offset += expected.byteLength;
    return true;
  }

  public readText(): string | null {
    const field = this.readField(ACCOUNT_ISSUER_MAX_FIELD_BYTES);
    if (field === null) return null;
    try {
      const text = new TextDecoder('utf-8', { fatal: true }).decode(field);
      return isAccountIdentityAuthorityProducerV2Text(text) ? text : null;
    } catch {
      return null;
    }
  }

  public readField(maxBytes: number): Uint8Array | null {
    if (this.offset + 4 > this.bytes.byteLength) return null;
    const length = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.offset, 4).getUint32(0, false);
    this.offset += 4;
    if (length === 0 || length > maxBytes || this.offset + length > this.bytes.byteLength) return null;
    const value = this.bytes.slice(this.offset, this.offset + length);
    this.offset += length;
    return value;
  }
}
