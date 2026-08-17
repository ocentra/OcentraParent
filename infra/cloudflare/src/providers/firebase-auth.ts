import { resolveAuthAdapterMode, type Env } from '../env.js';
import type { ProviderVerificationPort, VerifiedProviderIdentity } from '../auth/verifier.js';
import { parseFirebaseToken, validFirebaseClaims, verifyFirebaseSignature } from './firebase-auth-jwt.js';
import { fetchFirebaseJwkSet, shouldRefreshFirebaseJwkSet } from './firebase-auth-jwks.js';
import type { FirebaseConfig } from './firebase-auth-contract.js';

const FIREBASE_JWKS_URL = 'https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com';
const DEFAULT_CLOCK_SKEW_SECONDS = 60;
const DEFAULT_JWKS_CACHE_SECONDS = 300;
const MAX_JWKS_CACHE_SECONDS = 3600;

function parseBoundedSeconds(value: string | undefined, fallback: number, maximum: number): number | null {
  if (value === undefined || value.trim() === '') {
    return fallback;
  }
  if (!/^\d+$/.test(value)) {
    return null;
  }
  const parsed = Number(value);
  return Number.isSafeInteger(parsed) && parsed > 0 && parsed <= maximum ? parsed : null;
}

function resolveFirebaseConfig(
  env: Pick<Env, 'FIREBASE_PROJECT_ID' | 'FIREBASE_CLOCK_SKEW_SECONDS' | 'FIREBASE_JWKS_CACHE_SECONDS'>
): FirebaseConfig | null {
  const projectId = env.FIREBASE_PROJECT_ID?.trim();
  if (!projectId || !/^[a-z0-9][a-z0-9-]{1,28}[a-z0-9]$/.test(projectId)) {
    return null;
  }

  const issuer = `https://securetoken.google.com/${projectId}`;
  const audience = projectId;

  const clockSkewSeconds = parseBoundedSeconds(env.FIREBASE_CLOCK_SKEW_SECONDS, DEFAULT_CLOCK_SKEW_SECONDS, 300);
  const jwksCacheSeconds = parseBoundedSeconds(
    env.FIREBASE_JWKS_CACHE_SECONDS,
    DEFAULT_JWKS_CACHE_SECONDS,
    MAX_JWKS_CACHE_SECONDS
  );
  if (clockSkewSeconds === null || jwksCacheSeconds === null) {
    return null;
  }

  return { projectId, issuer, audience, jwksUrl: FIREBASE_JWKS_URL, clockSkewSeconds, jwksCacheSeconds };
}

function bearerToken(request: Request): string | null {
  const value = request.headers.get('authorization');
  if (!value) return null;
  return /^Bearer ([A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+)$/i.exec(value)?.[1] ?? null;
}

export function createFirebaseProviderVerificationPort(env: Env): ProviderVerificationPort | undefined {
  if (resolveAuthAdapterMode(env) !== 'provider-verified') {
    return undefined;
  }
  return {
    async verify(request: Request): Promise<VerifiedProviderIdentity | null> {
      const config = resolveFirebaseConfig(env);
      const token = bearerToken(request);
      if (config === null || token === null) return null;
      const parsed = parseFirebaseToken(token);
      if (parsed === null) return null;

      let keys = await fetchFirebaseJwkSet(config);
      if (keys === null) return null;
      let valid = await verifyFirebaseSignature(parsed, keys);
      if (!valid && !keys.some((key) => key.kid === parsed.header.kid)) {
        if (shouldRefreshFirebaseJwkSet(config.jwksUrl)) {
          keys = await fetchFirebaseJwkSet(config, true);
          valid = keys !== null && (await verifyFirebaseSignature(parsed, keys));
        }
      }
      if (!valid || !validFirebaseClaims(parsed.payload, config)) return null;
      return { provider: 'firebase', providerSubject: parsed.payload.sub as string };
    },
  };
}
