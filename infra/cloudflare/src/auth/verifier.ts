import { isLocalFixtureEnvironment, resolveAuthAdapterMode, type Env } from '../env.js';
import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';
import {
  createAccountIdentityAuthorityStore,
  type VerifiedAccountIdentityAuthorityCapability,
} from '../storage/account-identity-authority-store.js';
import { getAuthStateModel, type AuthState } from './model.js';

export interface VerifiedIdentity {
  subject: string;
  state: AuthState;
  role: 'public' | 'parent' | 'support' | 'admin' | 'internal' | 'provider-webhook';
  trustedDevice: boolean;
  authority?: VerifiedAccountIdentityAuthorityCapability;
}

export type AuthFailureResult = { ok: false; response: Response };
export type AuthResult = { ok: true; identity: VerifiedIdentity } | AuthFailureResult;
type BearerIdentityResult = { ok: true; token: string; trustedDevice: boolean } | AuthFailureResult;

export interface VerifiedProviderIdentity {
  provider: AccountIdentityProvider;
  providerSubject: string;
}

export interface ProviderVerificationPort {
  verify(request: Request): Promise<VerifiedProviderIdentity | null>;
}

export interface AuthVerifier {
  verifyPublic(): AuthResult;
  verifyParentSession(request: Request): Promise<AuthResult>;
  verifyTrustedParentDevice(request: Request): Promise<AuthResult>;
  verifyAdmin(request: Request): Promise<AuthResult>;
  verifySupport(request: Request): Promise<AuthResult>;
  verifyProviderWebhook(provider: string, request: Request): Promise<AuthResult>;
  verifyInternalQueue(request: Request): Promise<AuthResult>;
}

export const INTERNAL_SECRET_HEADER = 'x-ocentra-internal-secret';
export const ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER = 'account-auth-adapter-manual-required';
export const UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER = 'unsupported-auth-adapter-mode';
export const PROVIDER_VERIFICATION_UNAVAILABLE_BLOCKER = 'provider-verification-unavailable';
export const ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER =
  'account-identity-binding-context-manual-required';

function json(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: {
      'content-type': 'application/json; charset=utf-8',
    },
  });
}

function missingHeader(headerName: string, state: AuthState): AuthFailureResult {
  return {
    ok: false,
    response: json(401, {
      error: 'authentication-required',
      authState: state,
      missingHeader: headerName,
    }),
  };
}

function forbidden(reason: string, state: AuthState): AuthFailureResult {
  return {
    ok: false,
    response: json(403, {
      error: 'forbidden',
      authState: state,
      reason,
    }),
  };
}

function manualRequired(
  authState: AuthState,
  blocker = ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER
): AuthFailureResult {
  return {
    ok: false,
    response: json(503, {
      error: 'manual-required',
      authState,
      blocker,
    }),
  };
}

function authStateIdentity(
  subject: string,
  state: AuthState,
  role: VerifiedIdentity['role'],
  trustedDevice: boolean,
  authority?: VerifiedAccountIdentityAuthorityCapability
): AuthResult {
  return {
    ok: true,
    identity: {
      subject,
      state,
      role,
      trustedDevice,
      authority,
    },
  };
}

function normalizeSubject(token: string): string {
  const sanitized = token.replace(/[^A-Za-z0-9:_-]/g, '-').slice(0, 64);
  if (sanitized.length === 0) {
    return 'parent:unknown';
  }
  if (
    sanitized.startsWith('parent:') ||
    sanitized.startsWith('guardian:') ||
    sanitized.startsWith('child:') ||
    sanitized.startsWith('member:')
  ) {
    return sanitized;
  }
  return `parent:${sanitized}`;
}

function parseBearerToken(headerValue: string | null): string | null {
  if (!headerValue) {
    return null;
  }
  const [scheme, value] = headerValue.split(/\s+/, 2);
  if (scheme?.toLowerCase() !== 'bearer' || !value) {
    return null;
  }
  return value.trim();
}

function parseStripeSignatureHeader(signatureHeader: string): {
  timestamp: string;
  signatures: ReadonlyArray<string>;
} | null {
  const parts = signatureHeader.split(',').map((entry) => entry.trim());
  const timestamp = parts.find((entry) => entry.startsWith('t='))?.slice(2);
  const signatures = parts
    .filter((entry) => entry.startsWith('v1='))
    .map((entry) => entry.slice(3))
    .filter((entry) => /^[a-f0-9]{64}$/i.test(entry));

  if (!timestamp || !/^\d+$/.test(timestamp) || signatures.length === 0) {
    return null;
  }

  return {
    timestamp,
    signatures,
  };
}

function safeEqualHex(left: string, right: string): boolean {
  if (left.length !== right.length) {
    return false;
  }
  let diff = 0;
  for (let index = 0; index < left.length; index += 1) {
    diff |= left.charCodeAt(index) ^ right.charCodeAt(index);
  }
  return diff === 0;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (value) => value.toString(16).padStart(2, '0')).join('');
}

function providerFromPathname(pathname: string): string | null {
  if (!pathname.startsWith('/webhooks/')) {
    return null;
  }
  const provider = pathname.slice('/webhooks/'.length).trim();
  return provider.length > 0 ? provider : null;
}

function providerWebhookHeaderName(provider: string): string | null {
  switch (provider) {
    case 'stripe':
      return 'stripe-signature';
    case 'paypal':
      return 'paypal-transmission-id';
    case 'razorpay':
      return 'x-razorpay-signature';
    case 'apple':
      return 'authorization';
    case 'google':
      return 'x-goog-signature';
    default:
      return null;
  }
}

function isManualRequiredAdapterMode(mode: string): boolean {
  return mode === ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER || mode.startsWith('account-auth-adapter');
}

function authAdapterBlocker(env: Env, providerVerifier: ProviderVerificationPort | undefined): string | null {
  const mode = resolveAuthAdapterMode(env);
  if (mode === 'local-safe-fixture') {
    return isLocalFixtureEnvironment(env) ? null : ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER;
  }
  if (isManualRequiredAdapterMode(mode)) {
    return ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER;
  }
  if (mode === 'provider-verified') {
    return providerVerifier === undefined ? PROVIDER_VERIFICATION_UNAVAILABLE_BLOCKER : null;
  }
  return UNSUPPORTED_AUTH_ADAPTER_MODE_BLOCKER;
}

async function verifyProviderBoundRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort
): Promise<AuthResult> {
  let verifiedIdentity: VerifiedProviderIdentity | null;
  try {
    verifiedIdentity = await providerVerifier.verify(request);
  } catch {
    verifiedIdentity = null;
  }
  if (verifiedIdentity === null) {
    return manualRequired(authState, PROVIDER_VERIFICATION_UNAVAILABLE_BLOCKER);
  }

  const authorityResult = await createAccountIdentityAuthorityStore(env.ACCOUNT_IDENTITY_D1).readCurrentAuthority(
    verifiedIdentity.provider,
    verifiedIdentity.providerSubject
  );
  if (authorityResult.status === 'trusted') {
    const role =
      authorityResult.capability.role === 'support-admin'
        ? 'support'
        : authorityResult.capability.role === 'child-profile' ||
            authorityResult.capability.role === 'child-device-agent'
          ? 'public'
          : 'parent';
    return authStateIdentity(
      authorityResult.capability.providerSubject,
      authState,
      role,
      true,
      authorityResult.capability
    );
  }
  if (authorityResult.status === 'rejected') {
    return forbidden(`account-identity-authority-${authorityResult.reason}`, authState);
  }
  if (authorityResult.status === 'not-found') {
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }
  return manualRequired(authState, authorityResult.reason);
}

function extractBearerIdentity(request: Request, authState: AuthState): BearerIdentityResult {
  const token = parseBearerToken(request.headers.get('authorization'));
  if (!token) {
    const failure = missingHeader('authorization', authState);
    return {
      ok: false,
      response: failure.response,
    };
  }

  return {
    ok: true,
    token,
    trustedDevice: request.headers.get('x-ocentra-trusted-device') === 'true',
  };
}

async function verifyParentSessionRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const blocker = authAdapterBlocker(env, providerVerifier);
  if (blocker) {
    return manualRequired(authState, blocker);
  }

  if (resolveAuthAdapterMode(env) !== 'local-safe-fixture') {
    return verifyProviderBoundRequest(request, env, authState, providerVerifier!);
  }

  const bearerIdentity = extractBearerIdentity(request, authState);
  if (!bearerIdentity.ok) {
    return bearerIdentity;
  }

  return authStateIdentity(normalizeSubject(bearerIdentity.token), authState, 'parent', bearerIdentity.trustedDevice);
}

async function verifyTrustedParentDeviceRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const identity = await verifyParentSessionRequest(request, env, authState, providerVerifier);
  if (!identity.ok) {
    return identity;
  }

  if (!identity.identity.trustedDevice) {
    return forbidden('trusted-parent-device-required', authState);
  }

  return identity;
}

async function verifyAdminRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const identity = await verifyParentSessionRequest(request, env, authState, providerVerifier);
  if (!identity.ok) {
    return identity;
  }

  if (resolveAuthAdapterMode(env) !== 'local-safe-fixture') {
    return manualRequired(authState, 'admin-authorization-unavailable');
  }

  if (request.headers.get('x-ocentra-role') !== 'admin') {
    return forbidden('admin-role-required', authState);
  }

  return authStateIdentity(identity.identity.subject, authState, 'admin', identity.identity.trustedDevice);
}

async function verifySupportRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const identity = await verifyParentSessionRequest(request, env, authState, providerVerifier);
  if (!identity.ok) {
    return identity;
  }

  if (resolveAuthAdapterMode(env) !== 'local-safe-fixture') {
    return manualRequired(authState, 'support-authorization-unavailable');
  }

  const roleHeader = request.headers.get('x-ocentra-role');
  if (roleHeader !== 'support' && roleHeader !== 'admin') {
    return forbidden('support-role-required', authState);
  }

  return authStateIdentity(
    identity.identity.subject,
    authState,
    roleHeader === 'admin' ? 'admin' : 'support',
    identity.identity.trustedDevice
  );
}

function verifyProviderWebhookRequest(provider: string, request: Request, env: Env, authState: AuthState): AuthResult {
  const blocker = authAdapterBlocker(env, undefined);
  if (blocker) {
    return manualRequired(authState, blocker);
  }

  const headerName = providerWebhookHeaderName(provider);
  if (!headerName) {
    return manualRequired(authState, 'unsupported-provider-webhook');
  }

  const headerValue = request.headers.get(headerName);
  if (!headerValue) {
    return missingHeader(headerName, authState);
  }
  if (!hasWebhookSignatureSyntax(new URL(request.url).pathname, headerValue)) {
    return forbidden('invalid-provider-webhook-signature-header', authState);
  }

  return authStateIdentity('provider-webhook', authState, 'provider-webhook', false);
}

function verifyInternalQueueRequest(request: Request, env: Env, authState: AuthState): AuthResult {
  if (request.headers.get('x-ocentra-internal-call') !== 'true') {
    return forbidden('missing-internal-queue-signal', authState);
  }
  if (!env.INTERNAL_QUEUE_SHARED_SECRET?.trim()) {
    return manualRequired(authState, 'internal-queue-secret-unavailable');
  }
  if (request.headers.get(INTERNAL_SECRET_HEADER) !== env.INTERNAL_QUEUE_SHARED_SECRET) {
    return forbidden('internal-queue-secret-mismatch', authState);
  }
  return authStateIdentity('internal-queue', authState, 'internal', false);
}

export function createAuthVerifier(env: Env, providerVerifier?: ProviderVerificationPort): AuthVerifier {
  return {
    verifyPublic(): AuthResult {
      return authStateIdentity('public', 'public', 'public', false);
    },

    async verifyParentSession(request: Request): Promise<AuthResult> {
      return verifyParentSessionRequest(request, env, 'parent-session-required', providerVerifier);
    },

    async verifyTrustedParentDevice(request: Request): Promise<AuthResult> {
      return verifyTrustedParentDeviceRequest(request, env, 'trusted-parent-device-required', providerVerifier);
    },

    async verifyAdmin(request: Request): Promise<AuthResult> {
      return verifyAdminRequest(request, env, 'admin-required', providerVerifier);
    },

    async verifySupport(request: Request): Promise<AuthResult> {
      return verifySupportRequest(request, env, 'support-required', providerVerifier);
    },

    async verifyProviderWebhook(provider: string, request: Request): Promise<AuthResult> {
      return verifyProviderWebhookRequest(provider, request, env, 'provider-webhook-signature-required');
    },

    async verifyInternalQueue(request: Request): Promise<AuthResult> {
      return verifyInternalQueueRequest(request, env, 'internal-queue-only');
    },
  };
}

export function signatureHeaderName(pathname: string): string {
  const provider = providerFromPathname(pathname);
  if (!provider) {
    return 'x-goog-signature';
  }
  return providerWebhookHeaderName(provider) ?? 'x-goog-signature';
}

function hasWebhookSignatureSyntax(pathname: string, signatureValue: string | null): boolean {
  if (!signatureValue || signatureValue.trim().length === 0) {
    return false;
  }
  if (pathname.endsWith('/stripe')) {
    return parseStripeSignatureHeader(signatureValue) !== null;
  }
  return true;
}

export async function verifyStripeWebhookSignature(
  payload: string,
  signatureHeader: string,
  secret: string,
  timestampToleranceSeconds: string | undefined
): Promise<boolean> {
  const parsed = parseStripeSignatureHeader(signatureHeader);
  if (!parsed) {
    return false;
  }
  if (
    timestampToleranceSeconds === undefined ||
    !/^\d+$/.test(timestampToleranceSeconds) ||
    Number(timestampToleranceSeconds) <= 0 ||
    Number(timestampToleranceSeconds) > 86_400
  ) {
    return false;
  }
  const timestampSeconds = Number(parsed.timestamp);
  const nowSeconds = Math.floor(Date.now() / 1000);
  if (
    !Number.isSafeInteger(timestampSeconds) ||
    Math.abs(nowSeconds - timestampSeconds) > Number(timestampToleranceSeconds)
  ) {
    return false;
  }

  const key = await crypto.subtle.importKey(
    'raw',
    new TextEncoder().encode(secret),
    {
      name: 'HMAC',
      hash: 'SHA-256',
    },
    false,
    ['sign']
  );
  const signed = await crypto.subtle.sign('HMAC', key, new TextEncoder().encode(`${parsed.timestamp}.${payload}`));
  const actualSignature = bytesToHex(new Uint8Array(signed));
  return parsed.signatures.some((expected) => safeEqualHex(expected, actualSignature));
}

export async function verifyAuthState(
  authState: AuthState,
  request: Request,
  env: Env,
  providerVerifier?: ProviderVerificationPort
): Promise<AuthResult> {
  const verifier = createAuthVerifier(env, providerVerifier);
  const authModel = getAuthStateModel(authState);

  switch (authModel.adapterMethod) {
    case 'verifyPublic':
      return verifier.verifyPublic();
    case 'verifyParentSession':
      return verifier.verifyParentSession(request);
    case 'verifyTrustedParentDevice':
      return verifier.verifyTrustedParentDevice(request);
    case 'verifyAdmin':
      return verifier.verifyAdmin(request);
    case 'verifySupport':
      return verifier.verifySupport(request);
    case 'verifyProviderWebhook': {
      const provider = providerFromPathname(new URL(request.url).pathname);
      return verifier.verifyProviderWebhook(provider ?? 'unsupported', request);
    }
    case 'verifyInternalQueue':
      return verifier.verifyInternalQueue(request);
  }
}
