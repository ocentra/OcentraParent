import type { FirebaseConfig, FirebaseJsonWebKey } from './firebase-auth-contract.js';

const MAX_TOKEN_BYTES = 16 * 1024;
const MAX_TOKEN_LIFETIME_SECONDS = 3600;
const FORBIDDEN_AUTHORITY_CLAIMS = new Set([
  'householdId',
  'household_id',
  'memberId',
  'member_id',
  'role',
  'roles',
  'childProfileId',
  'child_profile_id',
  'childDeviceId',
  'child_device_id',
  'deviceId',
  'device_id',
  'trustedDevice',
  'trusted_device',
]);

export interface FirebaseJsonObject {
  [key: string]: unknown;
}

export interface FirebaseParsedToken {
  encodedHeader: string;
  encodedPayload: string;
  signature: Uint8Array;
  header: FirebaseJsonObject;
  payload: FirebaseJsonObject;
}

function isJsonObject(value: unknown): value is FirebaseJsonObject {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function decodeBase64Url(value: string): Uint8Array | null {
  if (!/^[A-Za-z0-9_-]+$/.test(value)) return null;
  const padded = value.replace(/-/g, '+').replace(/_/g, '/') + '='.repeat((4 - (value.length % 4)) % 4);
  try {
    return Uint8Array.from(atob(padded), (character) => character.charCodeAt(0));
  } catch {
    return null;
  }
}

function hasDuplicateObjectKeys(source: string): boolean {
  const containers: Array<{ kind: 'object'; keys: Set<string>; expectingKey: boolean } | { kind: 'array' }> = [];
  for (let index = 0; index < source.length; index += 1) {
    const character = source[index];
    if (/\s/.test(character)) continue;
    if (character === '"') {
      const start = index;
      for (index += 1; index < source.length; index += 1) {
        if (source[index] === '\\') index += 1;
        else if (source[index] === '"') break;
      }
      if (index >= source.length) return true;
      const current = containers[containers.length - 1];
      if (current?.kind === 'object' && current.expectingKey) {
        const key = JSON.parse(source.slice(start, index + 1)) as string;
        if (current.keys.has(key)) return true;
        current.keys.add(key);
        current.expectingKey = false;
      }
      continue;
    }
    if (character === '{') containers.push({ kind: 'object', keys: new Set<string>(), expectingKey: true });
    else if (character === '[') containers.push({ kind: 'array' });
    else if (character === '}' && containers.pop()?.kind !== 'object') return true;
    else if (character === ']' && containers.pop()?.kind !== 'array') return true;
    else if (character === ',' && containers[containers.length - 1]?.kind === 'object') {
      containers[containers.length - 1].expectingKey = true;
    } else if (character === ':' && containers[containers.length - 1]?.kind === 'object') {
      containers[containers.length - 1].expectingKey = false;
    }
  }
  return false;
}

function decodeJsonPart(value: string): FirebaseJsonObject | null {
  const bytes = decodeBase64Url(value);
  if (bytes === null) return null;
  try {
    const source = new TextDecoder().decode(bytes);
    if (hasDuplicateObjectKeys(source)) return null;
    const parsed: unknown = JSON.parse(source);
    return isJsonObject(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

export function parseFirebaseToken(token: string): FirebaseParsedToken | null {
  if (token.length === 0 || new TextEncoder().encode(token).byteLength > MAX_TOKEN_BYTES) return null;
  const parts = token.split('.');
  if (parts.length !== 3 || !parts[0] || !parts[1] || !parts[2]) return null;
  const header = decodeJsonPart(parts[0]);
  const payload = decodeJsonPart(parts[1]);
  const signature = decodeBase64Url(parts[2]);
  if (header === null || payload === null || signature === null || signature.byteLength === 0) return null;
  if (header.alg !== 'RS256' || typeof header.kid !== 'string' || !/^[\x21-\x7e]{1,128}$/.test(header.kid)) {
    return null;
  }
  return { encodedHeader: parts[0], encodedPayload: parts[1], signature, header, payload };
}

function validNumericDate(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
}

function validTimeClaims(payload: FirebaseJsonObject, nowSeconds: number, clockSkewSeconds: number): boolean {
  if (!validNumericDate(payload.exp) || !validNumericDate(payload.iat) || !validNumericDate(payload.auth_time))
    return false;
  if (
    payload.exp <= nowSeconds - clockSkewSeconds ||
    payload.iat > nowSeconds + clockSkewSeconds ||
    payload.iat >= payload.exp ||
    payload.exp - payload.iat > MAX_TOKEN_LIFETIME_SECONDS + clockSkewSeconds ||
    payload.auth_time > payload.iat ||
    payload.auth_time > nowSeconds + clockSkewSeconds
  ) {
    return false;
  }
  return payload.nbf === undefined || (validNumericDate(payload.nbf) && payload.nbf <= nowSeconds + clockSkewSeconds);
}

export function validFirebaseClaims(payload: FirebaseJsonObject, config: FirebaseConfig): boolean {
  return (
    payload.iss === config.issuer &&
    payload.aud === config.audience &&
    typeof payload.sub === 'string' &&
    payload.sub.length > 0 &&
    payload.sub.length <= 128 &&
    ![...FORBIDDEN_AUTHORITY_CLAIMS].some((claim) => Object.prototype.hasOwnProperty.call(payload, claim)) &&
    validTimeClaims(payload, Math.floor(Date.now() / 1000), config.clockSkewSeconds)
  );
}

export async function verifyFirebaseSignature(
  token: FirebaseParsedToken,
  keys: FirebaseJsonWebKey[]
): Promise<boolean> {
  for (const key of keys.filter((candidate) => candidate.kid === token.header.kid)) {
    try {
      const cryptoKey = await crypto.subtle.importKey(
        'jwk',
        key,
        { name: 'RSASSA-PKCS1-v1_5', hash: 'SHA-256' },
        false,
        ['verify']
      );
      if (
        await crypto.subtle.verify(
          { name: 'RSASSA-PKCS1-v1_5' },
          cryptoKey,
          token.signature,
          new TextEncoder().encode(`${token.encodedHeader}.${token.encodedPayload}`)
        )
      ) {
        return true;
      }
    } catch {
      continue;
    }
  }
  return false;
}
