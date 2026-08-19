import {
  AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema,
  type AccountIdentityCurrentMemberDeviceAuthorityHandoff,
  type AccountIdentityProvider,
} from '@ocentra-parent/schema-domain/account-identity-authority';

import { decodeJsonRejectingDuplicateKeys } from './account-identity-authority-json-decoder';

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
const ACCOUNT_OWNED_BINDING = Symbol('account-owned-authority-service-binding');
const WIRE_AUTHENTICATED_HANDOFF = Symbol('wire-authenticated-account-authority-handoff');

export const ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE = 'account-identity-authority-source-unavailable' as const;

export type AccountIdentityAuthorityProducerUnavailable = {
  readonly status: 'unavailable';
  readonly reason: typeof ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE;
};

type WireResolution =
  | { readonly status: 'trusted'; readonly wire: ArrayBuffer }
  | { readonly status: 'unavailable' }
  | { readonly status: 'rejected' };

type KeyResolution =
  | { readonly status: 'trusted'; readonly keyId: string; readonly publicKey: ArrayBuffer }
  | { readonly status: 'unavailable' }
  | { readonly status: 'rejected' };

interface AccountOwnedAuthorityServiceBinding {
  readonly [ACCOUNT_OWNED_BINDING]: true;
  resolveCurrentAuthorityWire(provider: AccountIdentityProvider, providerSubject: string): Promise<WireResolution>;
  resolveVerificationKey(keyId: string): Promise<KeyResolution>;
}

interface AccountOwnedAuthorityTransportConsumer {
  resolveCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityCurrentMemberDeviceAuthorityHandoff | null>;
}

const TRUSTED_BINDINGS = new WeakSet<AccountOwnedAuthorityServiceBinding>();
const AUTHENTICATED_HANDOFFS = new WeakMap<
  WireAuthenticatedHandoff,
  AccountIdentityCurrentMemberDeviceAuthorityHandoff
>();

class WireAuthenticatedHandoff {
  public readonly [WIRE_AUTHENTICATED_HANDOFF] = true;

  private constructor() {}

  public static issue(handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff): WireAuthenticatedHandoff {
    const marker = Object.freeze(new WireAuthenticatedHandoff());
    AUTHENTICATED_HANDOFFS.set(marker, handoff);
    return marker;
  }
}

type VerificationError =
  'invalid-wire' | 'invalid-payload' | 'authority-expired' | 'verification-key-unavailable' | 'signature-invalid';

type VerificationResult =
  | { readonly status: 'wire-authenticated'; readonly handoff: WireAuthenticatedHandoff }
  | AccountIdentityAuthorityProducerUnavailable
  | { readonly status: 'rejected'; readonly reason: VerificationError };

/**
 * Only this module may eventually wrap the real Account-owned Cloudflare
 * service binding. It intentionally has no current caller: Account has not
 * supplied an authenticated binding or durable registry yet.
 */
function registerAccountOwnedServiceBinding(
  binding: Omit<AccountOwnedAuthorityServiceBinding, typeof ACCOUNT_OWNED_BINDING>
): AccountOwnedAuthorityTransportConsumer {
  const trusted: AccountOwnedAuthorityServiceBinding = Object.freeze({
    [ACCOUNT_OWNED_BINDING]: true,
    resolveCurrentAuthorityWire: binding.resolveCurrentAuthorityWire.bind(binding),
    resolveVerificationKey: binding.resolveVerificationKey.bind(binding),
  });
  TRUSTED_BINDINGS.add(trusted);
  return Object.freeze({
    resolveCurrentAuthority(provider, providerSubject) {
      return resolveCurrentAuthority(trusted, provider, providerSubject);
    },
  });
}

export function accountIdentityAuthorityProducerUnavailable(): AccountIdentityAuthorityProducerUnavailable {
  return { status: 'unavailable', reason: ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE };
}

function isTrustedBinding(value: unknown): value is AccountOwnedAuthorityServiceBinding {
  return (
    typeof value === 'object' &&
    value !== null &&
    (value as Partial<AccountOwnedAuthorityServiceBinding>)[ACCOUNT_OWNED_BINDING] === true &&
    TRUSTED_BINDINGS.has(value as AccountOwnedAuthorityServiceBinding)
  );
}

async function resolveCurrentAuthority(
  binding: AccountOwnedAuthorityServiceBinding,
  provider: AccountIdentityProvider,
  providerSubject: string
): Promise<AccountIdentityCurrentMemberDeviceAuthorityHandoff | null> {
  if (!isTrustedBinding(binding)) return null;
  let resolution: WireResolution;
  try {
    resolution = await binding.resolveCurrentAuthorityWire(provider, providerSubject);
  } catch {
    return null;
  }
  if (resolution.status !== 'trusted') return null;
  const wire = readBuffer(resolution.wire);
  if (wire === null) return null;
  const verified = await verifyWire(wire, binding);
  if (verified.status !== 'wire-authenticated') return null;
  const handoff = consumeAuthenticatedHandoff(verified.handoff);
  if (
    handoff === null ||
    handoff.mapping.provider !== provider ||
    handoff.mapping.providerSubject !== providerSubject
  ) {
    return null;
  }
  return handoff;
}

async function verifyWire(wire: Uint8Array, binding: AccountOwnedAuthorityServiceBinding): Promise<VerificationResult> {
  const now = trustedNow();
  if (now === null) return accountIdentityAuthorityProducerUnavailable();
  const parsed = parseWire(wire, now);
  if (parsed.status !== 'parsed') return parsed;

  let resolution: KeyResolution;
  try {
    resolution = await binding.resolveVerificationKey(parsed.keyId);
  } catch {
    return accountIdentityAuthorityProducerUnavailable();
  }
  if (resolution.status === 'unavailable') return accountIdentityAuthorityProducerUnavailable();
  if (resolution.status !== 'trusted' || resolution.keyId !== parsed.keyId) {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }
  const keyBytes = readBuffer(resolution.publicKey);
  if (keyBytes === null || keyBytes.byteLength !== 32) {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }

  let resolvedKeyId: string;
  try {
    resolvedKeyId = await expectedKeyId(keyBytes);
  } catch {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }
  if (resolvedKeyId !== parsed.keyId) {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }

  let publicKey: CryptoKey;
  try {
    publicKey = await crypto.subtle.importKey('raw', keyBytes, 'Ed25519', false, ['verify']);
  } catch {
    return { status: 'rejected', reason: 'verification-key-unavailable' };
  }

  try {
    const valid = await crypto.subtle.verify('Ed25519', publicKey, parsed.signature, parsed.signingBytes);
    return valid
      ? { status: 'wire-authenticated', handoff: WireAuthenticatedHandoff.issue(parsed.handoff) }
      : { status: 'rejected', reason: 'signature-invalid' };
  } catch {
    return { status: 'rejected', reason: 'signature-invalid' };
  }
}

function consumeAuthenticatedHandoff(
  marker: WireAuthenticatedHandoff
): AccountIdentityCurrentMemberDeviceAuthorityHandoff | null {
  if (marker[WIRE_AUTHENTICATED_HANDOFF] !== true) return null;
  const handoff = AUTHENTICATED_HANDOFFS.get(marker) ?? null;
  AUTHENTICATED_HANDOFFS.delete(marker);
  return handoff;
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
  | { readonly status: 'rejected'; readonly reason: VerificationError } {
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
    fields[3] !== SIGNATURE_ALGORITHM ||
    !KEY_ID_PATTERN.test(fields[4])
  ) {
    return { status: 'rejected', reason: 'invalid-wire' };
  }

  const issuedAt = parseTimestamp(fields[5]);
  const expiresAt = parseTimestamp(fields[6]);
  if (issuedAt === null || expiresAt === null) return { status: 'rejected', reason: 'invalid-wire' };
  if (
    issuedAt >= expiresAt ||
    issuedAt > now + MAX_FUTURE_ISSUED_SKEW_MS ||
    expiresAt <= now ||
    expiresAt > issuedAt + MAX_LIFETIME_MS
  ) {
    return { status: 'rejected', reason: 'authority-expired' };
  }

  const payloadText = decodeUtf8(payload);
  if (payloadText === null) return { status: 'rejected', reason: 'invalid-payload' };
  const decodedPayload = decodeJsonRejectingDuplicateKeys(payloadText);
  if (decodedPayload.status !== 'decoded') return { status: 'rejected', reason: 'invalid-payload' };
  const parsed = AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema.safeParse(decodedPayload.value);
  if (!parsed.success || JSON.stringify(parsed.data) !== payloadText || !hasRustAuthorityShape(parsed.data)) {
    return { status: 'rejected', reason: 'invalid-payload' };
  }
  return { status: 'parsed', keyId: fields[4], signature, signingBytes, handoff: parsed.data };
}

function hasRustAuthorityShape(handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff): boolean {
  const member = handoff.member;
  const binding = handoff.binding;
  const receipt = member.supportReceipt;
  return (
    handoff.schemaVersion === 'v0.1' &&
    handoff.mapping.status === 'active' &&
    handoff.mapping.accountId === member.accountId &&
    member.accountId === binding.accountId &&
    member.householdId === binding.householdId &&
    member.accountState === 'active' &&
    member.membershipState === 'active' &&
    member.deviceTrustState === 'trusted' &&
    member.sessionFreshnessState === 'fresh' &&
    Number.isSafeInteger(member.sessionGeneration) &&
    member.sessionGeneration > 0 &&
    member.sessionExpiresAt.trim().length > 0 &&
    (member.role !== 'support-admin' || receipt !== null) &&
    (receipt === null ||
      (receipt.issuedAt.trim().length > 0 &&
        receipt.expiresAt.trim().length > 0 &&
        receipt.revocationState === 'active')) &&
    binding.pairingState === 'paired' &&
    binding.installState === 'installed' &&
    binding.lifecycleState === 'active' &&
    binding.revocationState === 'active' &&
    Number.isSafeInteger(member.authorityGeneration) &&
    member.authorityGeneration > 0 &&
    member.authorityGeneration === binding.authorityGeneration
  );
}

function trustedNow(): number | null {
  const now = Date.now();
  return Number.isSafeInteger(now) && now >= 0 ? now : null;
}

function parseTimestamp(value: string): number | null {
  if (!MILLIS_UTC_PATTERN.test(value)) return null;
  const timestamp = Date.parse(value);
  return Number.isSafeInteger(timestamp) && timestamp >= 0 && new Date(timestamp).toISOString() === value
    ? timestamp
    : null;
}

async function expectedKeyId(publicKey: Uint8Array): Promise<string> {
  const digest = new Uint8Array(await crypto.subtle.digest('SHA-256', publicKey));
  return `sha256:${Array.from(digest, (byte) => byte.toString(16).padStart(2, '0')).join('')}`;
}

function decodeUtf8(value: Uint8Array): string | null {
  try {
    return new TextDecoder('utf-8', { fatal: true }).decode(value);
  } catch {
    return null;
  }
}

function readBuffer(value: ArrayBuffer): Uint8Array | null {
  try {
    return new Uint8Array(value);
  } catch {
    return null;
  }
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
    const text = decodeUtf8(value);
    return text !== null && text.length > 0 ? text : null;
  }
}
