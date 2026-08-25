import type { FirebaseConfig, FirebaseJsonWebKey } from './firebase-auth-contract.js';

const MAX_JWKS_BYTES = 128 * 1024;
const JWKS_REQUEST_TIMEOUT_MS = 5000;
const MAX_CACHE_ENTRIES = 4;
const UNKNOWN_KID_REFRESH_BACKOFF_MS = 30_000;

interface FirebaseJwkSet {
  keys: FirebaseJsonWebKey[];
}

interface CachedJwkSet {
  expiresAt: number;
  keys: FirebaseJsonWebKey[];
}

const jwksCache = new Map<string, CachedJwkSet>();
const jwksRefreshAfter = new Map<string, number>();
const jwksFetchAfter = new Map<string, number>();
const jwksInFlight = new Map<string, Promise<FirebaseJsonWebKey[] | null>>();

function parseJwkSet(value: unknown): FirebaseJwkSet | null {
  if (typeof value !== 'object' || value === null || !Array.isArray((value as { keys?: unknown }).keys)) return null;
  const rawKeys = (value as { keys: unknown[] }).keys;
  if (rawKeys.length === 0 || rawKeys.length > 32) return null;
  const keys = rawKeys
    .filter((key): key is Record<string, unknown> => typeof key === 'object' && key !== null && !Array.isArray(key))
    .filter(
      (key) =>
        key.kty === 'RSA' &&
        typeof key.kid === 'string' &&
        /^[\x21-\x7e]{1,128}$/.test(key.kid) &&
        (key.alg === undefined || key.alg === 'RS256') &&
        (key.use === undefined || key.use === 'sig')
    )
    .map((key) => key as FirebaseJsonWebKey);
  return keys.length > 0 ? { keys } : null;
}

function cacheJwkSet(url: string, result: FirebaseJwkSet, ttlSeconds: number, now: number): void {
  while (jwksCache.size >= MAX_CACHE_ENTRIES && !jwksCache.has(url)) {
    const oldest = jwksCache.keys().next().value;
    if (oldest === undefined) break;
    jwksCache.delete(oldest);
  }
  jwksCache.delete(url);
  jwksCache.set(url, { keys: result.keys, expiresAt: now + ttlSeconds * 1000 });
}

async function readBoundedBody(response: Response): Promise<string | null> {
  const contentType = response.headers.get('content-type')?.split(';', 1)[0]?.trim().toLowerCase();
  const contentLength = response.headers.get('content-length');
  if (
    contentType !== 'application/json' ||
    (contentLength !== null && (!/^\d+$/.test(contentLength) || Number(contentLength) > MAX_JWKS_BYTES))
  ) {
    return null;
  }
  if (!response.body) return null;
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let totalBytes = 0;
  while (true) {
    const chunk = await reader.read();
    if (chunk.done) break;
    totalBytes += chunk.value.byteLength;
    if (totalBytes > MAX_JWKS_BYTES) {
      await reader.cancel();
      return null;
    }
    chunks.push(chunk.value);
  }
  const bodyBytes = new Uint8Array(totalBytes);
  let offset = 0;
  for (const chunk of chunks) {
    bodyBytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return new TextDecoder().decode(bodyBytes);
}

async function fetchFirebaseJwkSetUnshared(
  config: FirebaseConfig,
  forceRefresh = false
): Promise<FirebaseJsonWebKey[] | null> {
  const now = Date.now();
  const cached = jwksCache.get(config.jwksUrl);
  if (!forceRefresh && cached && cached.expiresAt > now) return cached.keys;
  if (!forceRefresh && now < (jwksFetchAfter.get(config.jwksUrl) ?? 0)) {
    return cached && cached.expiresAt > now ? cached.keys : null;
  }
  let timeout: ReturnType<typeof setTimeout> | undefined;
  let loaded = false;
  try {
    const controller = new AbortController();
    timeout = setTimeout(() => controller.abort(), JWKS_REQUEST_TIMEOUT_MS);
    const response = await fetch(config.jwksUrl, {
      headers: { accept: 'application/json' },
      redirect: 'error',
      signal: controller.signal,
    });
    if (!response.ok) return null;
    const body = await readBoundedBody(response);
    if (body === null) return null;
    const result = parseJwkSet(JSON.parse(body));
    if (result === null) return null;
    cacheJwkSet(config.jwksUrl, result, config.jwksCacheSeconds, now);
    jwksFetchAfter.delete(config.jwksUrl);
    loaded = true;
    return result.keys;
  } catch {
    return null;
  } finally {
    if (!loaded) jwksFetchAfter.set(config.jwksUrl, Date.now() + UNKNOWN_KID_REFRESH_BACKOFF_MS);
    if (timeout !== undefined) clearTimeout(timeout);
  }
}

export async function fetchFirebaseJwkSet(
  config: FirebaseConfig,
  forceRefresh = false
): Promise<FirebaseJsonWebKey[] | null> {
  const existing = jwksInFlight.get(config.jwksUrl);
  if (existing !== undefined) return existing;
  const request = fetchFirebaseJwkSetUnshared(config, forceRefresh);
  jwksInFlight.set(config.jwksUrl, request);
  try {
    return await request;
  } finally {
    if (jwksInFlight.get(config.jwksUrl) === request) jwksInFlight.delete(config.jwksUrl);
  }
}

export function shouldRefreshFirebaseJwkSet(url: string): boolean {
  const now = Date.now();
  const nextRefresh = jwksRefreshAfter.get(url) ?? 0;
  if (now < nextRefresh) return false;
  jwksRefreshAfter.set(url, now + UNKNOWN_KID_REFRESH_BACKOFF_MS);
  return true;
}
