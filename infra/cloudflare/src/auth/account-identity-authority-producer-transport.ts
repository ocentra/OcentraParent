import {
  AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema,
  type AccountIdentityCurrentMemberDeviceAuthorityHandoff,
} from '@ocentra-parent/schema-domain/account-identity-authority';

const DOMAIN_SEPARATOR = new TextEncoder().encode('ocentra.account-authority-producer.signing.v1\0');
const SCHEMA_VERSION = 'ocentra.account-authority-producer.v1';
const AUDIENCE = 'ocentra.account.authority';
const ENVIRONMENT = 'account-owned';
const SIGNATURE_ALGORITHM = 'ed25519';
const SIGNATURE_BYTES = 64;
const MAX_FIELD_BYTES = 1_024;
const MAX_PAYLOAD_BYTES = 16 * 1_024;
const MAX_LIFETIME_MS = 5 * 60 * 1_000;
const MAX_FUTURE_ISSUED_SKEW_MS = 30 * 1_000;
const MAX_WIRE_BYTES = MAX_PAYLOAD_BYTES + MAX_FIELD_BYTES * 7 + 128 + SIGNATURE_BYTES;
const KEY_ID_PATTERN = /^sha256:[0-9a-f]{64}$/;
const MILLIS_UTC_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/;

export const ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE = 'account-identity-authority-source-unavailable' as const;

export type AccountIdentityAuthorityProducerUnavailable = {
  readonly status: 'unavailable';
  readonly reason: typeof ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE;
};

export type AccountIdentityAuthorityProducerVerificationError =
  'invalid-wire' | 'invalid-payload' | 'authority-expired' | 'verification-key-unavailable' | 'signature-invalid';

export type AccountIdentityAuthorityProducerKeyResolution =
  | { readonly status: 'trusted'; readonly keyId: string; readonly publicKey: ArrayBuffer }
  | { readonly status: 'unavailable' }
  | { readonly status: 'rejected' };

/**
 * Account owns this adapter. Cloudflare must never obtain a key from env,
 * Firebase, D1 caller data, a request header, or a fixture. Until Account
 * supplies an authenticated durable implementation, the runtime stays typed
 * unavailable and no writer mutation is mounted.
 */
export interface AccountIdentityAuthorityProducerKeyRegistry {
  resolveVerificationKey(keyId: string): Promise<AccountIdentityAuthorityProducerKeyResolution>;
}

/** The verified handoff is intentionally opaque and has no serialisation path. */
export interface VerifiedAccountIdentityAuthorityProducerHandoff {
  readonly __verifiedAccountIdentityAuthorityProducerHandoff: unique symbol;
}

const HANDOFFS = new WeakMap<
  VerifiedAccountIdentityAuthorityProducerHandoff,
  AccountIdentityCurrentMemberDeviceAuthorityHandoff
>();

export type AccountIdentityAuthorityProducerVerificationResult =
  | { readonly status: 'verified'; readonly handoff: VerifiedAccountIdentityAuthorityProducerHandoff }
  | AccountIdentityAuthorityProducerUnavailable
  | { readonly status: 'rejected'; readonly reason: AccountIdentityAuthorityProducerVerificationError };

export function accountIdentityAuthorityProducerUnavailable(): AccountIdentityAuthorityProducerUnavailable {
  return { status: 'unavailable', reason: ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE };
}

export async function verifyAccountIdentityAuthorityProducerWire(
  wire: Uint8Array,
  keyRegistry: AccountIdentityAuthorityProducerKeyRegistry,
  now = Date.now()
): Promise<AccountIdentityAuthorityProducerVerificationResult> {
  const parsed = parseWire(wire, now);
  if (parsed.status !== 'parsed') return parsed;

  let resolution: AccountIdentityAuthorityProducerKeyResolution;
  try {
    resolution = await keyRegistry.resolveVerificationKey(parsed.keyId);
  } catch {
    return accountIdentityAuthorityProducerUnavailable();
  }
  if (resolution.status === 'unavailable') return accountIdentityAuthorityProducerUnavailable();
  if (resolution.status !== 'trusted' || resolution.keyId !== parsed.keyId) {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }

  const keyBytes = new Uint8Array(resolution.publicKey);
  if (keyBytes.byteLength !== 32 || (await expectedKeyId(keyBytes)) !== parsed.keyId) {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }

  let publicKey: CryptoKey;
  try {
    publicKey = await crypto.subtle.importKey('raw', keyBytes, 'Ed25519', false, ['verify']);
  } catch {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }
  let valid: boolean;
  try {
    valid = await crypto.subtle.verify('Ed25519', publicKey, parsed.signature, parsed.signingBytes);
  } catch {
    return { status: 'rejected', reason: 'signature-invalid' };
  }
  if (!valid) return { status: 'rejected', reason: 'signature-invalid' };

  const handoff: VerifiedAccountIdentityAuthorityProducerHandoff = Object.freeze({
    __verifiedAccountIdentityAuthorityProducerHandoff: Symbol('verified-account-authority') as never,
  });
  HANDOFFS.set(handoff, parsed.handoff);
  return { status: 'verified', handoff };
}

function parseWire(
  wire: Uint8Array,
  now: number
):
  | {
      readonly status: 'parsed';
      readonly keyId: string;
      readonly signature: Uint8Array;
      readonly signingBytes: Uint8Array;
      readonly handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff;
    }
  | { readonly status: 'rejected'; readonly reason: AccountIdentityAuthorityProducerVerificationError } {
  if (wire.byteLength > MAX_WIRE_BYTES || wire.byteLength <= SIGNATURE_BYTES) {
    return { status: 'rejected', reason: 'invalid-wire' };
  }
  const signingLength = wire.byteLength - SIGNATURE_BYTES;
  const signingBytes = wire.slice(0, signingLength);
  const signature = wire.slice(signingLength);
  if (!startsWith(signingBytes, DOMAIN_SEPARATOR)) return { status: 'rejected', reason: 'invalid-wire' };

  const cursor = new Cursor(signingBytes.slice(DOMAIN_SEPARATOR.byteLength));
  const fields: string[] = [];
  for (let index = 0; index < 8; index += 1) {
    const value = cursor.read(index === 7 ? 'bytes' : 'string');
    if (value === null) return { status: 'rejected', reason: 'invalid-wire' };
    if (typeof value === 'string') fields.push(value);
  }
  const payload = cursor.lastBytes;
  if (!cursor.finished || payload === null) return { status: 'rejected', reason: 'invalid-wire' };
  if (
    fields[0] !== SCHEMA_VERSION ||
    fields[1] !== AUDIENCE ||
    fields[2] !== ENVIRONMENT ||
    fields[3] !== SIGNATURE_ALGORITHM
  ) {
    return { status: 'rejected', reason: 'invalid-wire' };
  }
  const keyId = fields[4];
  if (!KEY_ID_PATTERN.test(keyId)) return { status: 'rejected', reason: 'invalid-wire' };

  const issuedAt = parseTimestamp(fields[5]);
  const expiresAt = parseTimestamp(fields[6]);
  if (
    issuedAt === null ||
    expiresAt === null ||
    issuedAt >= expiresAt ||
    issuedAt > now + MAX_FUTURE_ISSUED_SKEW_MS ||
    expiresAt <= now ||
    expiresAt > issuedAt + MAX_LIFETIME_MS
  ) {
    return { status: 'rejected', reason: 'authority-expired' };
  }

  let payloadValue: unknown;
  try {
    payloadValue = JSON.parse(new TextDecoder('utf-8', { fatal: true }).decode(payload));
  } catch {
    return { status: 'rejected', reason: 'invalid-payload' };
  }
  const parsed = AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema.safeParse(payloadValue);
  if (!parsed.success || JSON.stringify(parsed.data) !== new TextDecoder().decode(payload)) {
    return { status: 'rejected', reason: 'invalid-payload' };
  }
  return { status: 'parsed', keyId, signature, signingBytes, handoff: parsed.data };
}

function parseTimestamp(value: string): number | null {
  if (!MILLIS_UTC_PATTERN.test(value)) return null;
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) && new Date(timestamp).toISOString() === value ? timestamp : null;
}

async function expectedKeyId(publicKey: Uint8Array): Promise<string> {
  const digest = new Uint8Array(await crypto.subtle.digest('SHA-256', publicKey));
  return `sha256:${Array.from(digest, (byte) => byte.toString(16).padStart(2, '0')).join('')}`;
}

function startsWith(value: Uint8Array, prefix: Uint8Array): boolean {
  return prefix.every((byte, index) => value[index] === byte);
}

class Cursor {
  private offset = 0;
  public lastBytes: Uint8Array | null = null;

  public constructor(private readonly bytes: Uint8Array) {}

  public get finished(): boolean {
    return this.offset === this.bytes.byteLength;
  }

  public read(kind: 'string' | 'bytes'): string | Uint8Array | null {
    if (this.offset + 4 > this.bytes.byteLength) return null;
    const length = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.offset, 4).getUint32(0);
    this.offset += 4;
    if (length > MAX_PAYLOAD_BYTES || this.offset + length > this.bytes.byteLength) return null;
    const value = this.bytes.slice(this.offset, this.offset + length);
    this.offset += length;
    if (kind === 'bytes') {
      this.lastBytes = value;
      return value;
    }
    if (value.byteLength === 0 || value.byteLength > MAX_FIELD_BYTES) return null;
    try {
      const text = new TextDecoder('utf-8', { fatal: true }).decode(value);
      return text.length > 0 && text.length <= MAX_FIELD_BYTES ? text : null;
    } catch {
      return null;
    }
  }
}
