import { isLocalFixtureEnvironment, resolveAuthAdapterMode, type Env } from '../env.js';
import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';
import {
  createAccountIdentityAuthorityStore,
  isVerifiedAccountIdentityAuthorityCapability,
  type VerifiedAccountIdentityAuthorityCapability,
} from '../storage/account-identity-authority-store.js';
import { createAccountIdentityAuthorityCaller } from './account-identity-authority-caller.js';
import { createBrowserSessionStore } from '../storage/account-browser-session-store.js';
import { browserSessionCookieNames, browserSessionRole, readCookie } from '../storage/account-browser-session-codec.js';
import { getAuthStateModel, type AuthState } from './model.js';
import { webhookProviderForPath } from '../routes.js';
import {
  PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS,
  resolveProviderWebhookName,
  verifyProviderWebhook,
  type ProviderWebhookName,
} from './provider-webhook.js';

export interface VerifiedIdentity {
  subject: string;
  state: AuthState;
  role: 'public' | 'parent' | 'support' | 'admin' | 'internal' | 'provider-webhook';
  trustedDevice: boolean;
  authority?: VerifiedAccountIdentityAuthorityCapability;
}

export type AuthFailureResult = { ok: false; response: Response };
export type AuthResult = { ok: true; identity: VerifiedIdentity } | AuthFailureResult;

export interface VerifiedProviderIdentity {
  provider: AccountIdentityProvider;
  providerSubject: string;
}

export type ProviderVerificationResult =
  | { status: 'verified'; identity: VerifiedProviderIdentity }
  | {
      status: 'rejected';
      reason: 'missing-credential' | 'malformed-credential' | 'invalid-credential';
    }
  | {
      status: 'unavailable';
      reason: 'configuration-unavailable' | 'jwks-unavailable' | 'provider-unavailable';
    };

export interface ProviderVerificationPort {
  verify(request: Request): Promise<ProviderVerificationResult>;
}

export interface AuthVerifier {
  verifyPublic(): AuthResult;
  verifyBrowserSession(request: Request): Promise<AuthResult>;
  verifyBrowserRefresh(request: Request): Promise<AuthResult>;
  verifyParentSession(request: Request): Promise<AuthResult>;
  verifyTrustedParentDevice(request: Request): Promise<AuthResult>;
  verifyAdmin(request: Request): Promise<AuthResult>;
  verifySupport(request: Request): Promise<AuthResult>;
  verifyProviderWebhook(provider: ProviderWebhookName, request: Request): Promise<AuthResult>;
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

async function readBrowserCurrentAuthority(env: Env, provider: AccountIdentityProvider, providerSubject: string) {
  try {
    return await createAccountIdentityAuthorityStore(env.ACCOUNT_IDENTITY_D1).readCurrentAuthority(
      provider,
      providerSubject
    );
  } catch {
    return { status: 'manual-required' as const, reason: 'account-identity-d1-unavailable' as const };
  }
}

function requireParentRoleCapability(result: AuthResult, authState: AuthState): AuthResult {
  if (!result.ok) {
    return result;
  }

  const authority = result.identity.authority;
  if (!isVerifiedAccountIdentityAuthorityCapability(authority)) {
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }
  if (authority.role !== 'parent-owner' && authority.role !== 'co-parent-guardian') {
    return forbidden('parent-role-capability-required', authState);
  }
  return result;
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
  const authorityResult = await createAccountIdentityAuthorityCaller(env).resolveVerifiedProviderAuthority(
    request,
    providerVerifier
  );
  if (authorityResult.status === 'trusted') {
    // Provider verification and a subject-keyed Account row do not prove that
    // this request holds the owner-issued credential for the stored device.
    // Keep the provider-only boundary manual-required until that request-bound
    // device credential is verified by its owning runtime.
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }
  if (authorityResult.status === 'rejected') {
    return forbidden(`account-identity-authority-${authorityResult.reason}`, authState);
  }
  if (authorityResult.status === 'not-found') {
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }
  if (authorityResult.status === 'provider-unavailable') {
    return manualRequired(authState, PROVIDER_VERIFICATION_UNAVAILABLE_BLOCKER);
  }
  return manualRequired(authState, authorityResult.reason);
}

async function verifyParentSessionRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const cookieNames = browserSessionCookieNames(!isLocalFixtureEnvironment(env));
  if (readCookie(request, cookieNames.session) !== null) {
    return requireParentRoleCapability(await verifyBrowserSessionRequest(request, env, authState), authState);
  }
  const blocker = authAdapterBlocker(env, providerVerifier);
  if (blocker) {
    return manualRequired(authState, blocker);
  }

  if (resolveAuthAdapterMode(env) !== 'local-safe-fixture') {
    return requireParentRoleCapability(
      await verifyProviderBoundRequest(request, env, authState, providerVerifier!),
      authState
    );
  }
  return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
}

async function verifyBrowserSessionRequest(request: Request, env: Env, authState: AuthState): Promise<AuthResult> {
  const cookieNames = browserSessionCookieNames(!isLocalFixtureEnvironment(env));
  const sessionToken = readCookie(request, cookieNames.session);
  const session = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).read(sessionToken);
  if (session.status === 'missing') return missingHeader(cookieNames.session, authState);
  if (session.status === 'manual-required') return manualRequired(authState, `account-session-${session.reason}`);
  if (session.status === 'rejected') return forbidden(`account-session-${session.reason}`, authState);

  const authorityResult = await readBrowserCurrentAuthority(
    env,
    session.identity.provider,
    session.identity.providerSubject
  );
  if (authorityResult.status !== 'trusted') {
    if (authorityResult.status === 'rejected') {
      return forbidden(`account-identity-authority-${authorityResult.reason}`, authState);
    }
    return manualRequired(authState, 'account-identity-binding-context-manual-required');
  }
  const authority = authorityResult.capability;
  const role = browserSessionRole(authority.role);
  if (role === null) return forbidden('browser-session-role-ineligible', authState);
  if (
    authority.provider !== session.identity.provider ||
    authority.providerSubject !== session.identity.providerSubject ||
    authority.accountId !== session.identity.accountId ||
    authority.sessionId !== session.identity.authoritySessionId ||
    authority.sessionGeneration !== session.identity.authoritySessionGeneration ||
    authority.authorityGeneration !== session.identity.authorityGeneration
  ) {
    return forbidden('account-session-authority-stale', authState);
  }
  return authStateIdentity(authority.providerSubject, authState, role, true, authority);
}

async function verifyBrowserRefreshRequest(request: Request, env: Env, authState: AuthState): Promise<AuthResult> {
  const cookieNames = browserSessionCookieNames(!isLocalFixtureEnvironment(env));
  const refreshToken = readCookie(request, cookieNames.refresh);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const refresh = await store.readRefresh(refreshToken);
  if (refresh.status === 'missing') return missingHeader(cookieNames.refresh, authState);
  if (refresh.status === 'manual-required') return manualRequired(authState, `account-session-${refresh.reason}`);
  if (refresh.status === 'rejected') return forbidden(`account-session-${refresh.reason}`, authState);
  if (!(await store.verifyRefreshCsrf(refreshToken, request.headers.get('x-ocentra-csrf')))) {
    return forbidden('csrf-validation-failed', authState);
  }

  const accessToken = readCookie(request, cookieNames.session);
  if (accessToken !== null) {
    const access = await store.readBinding(accessToken);
    if (access.status === 'manual-required') return manualRequired(authState, `account-session-${access.reason}`);
    if (access.status !== 'active' || access.identity.sessionId !== refresh.identity.sessionId) {
      return forbidden('account-refresh-access-session-mismatch', authState);
    }
  }

  const authorityResult = await readBrowserCurrentAuthority(
    env,
    refresh.identity.provider,
    refresh.identity.providerSubject
  );
  if (authorityResult.status !== 'trusted') {
    if (authorityResult.status === 'rejected') {
      return forbidden(`account-identity-authority-${authorityResult.reason}`, authState);
    }
    return manualRequired(authState, 'account-identity-binding-context-manual-required');
  }
  const authority = authorityResult.capability;
  const role = browserSessionRole(authority.role);
  if (role === null) return forbidden('browser-session-role-ineligible', authState);
  if (
    authority.provider !== refresh.identity.provider ||
    authority.providerSubject !== refresh.identity.providerSubject ||
    authority.accountId !== refresh.identity.accountId ||
    authority.sessionId !== refresh.identity.authoritySessionId ||
    authority.sessionGeneration !== refresh.identity.authoritySessionGeneration ||
    authority.authorityGeneration !== refresh.identity.authorityGeneration
  ) {
    return forbidden('account-session-authority-stale', authState);
  }
  return authStateIdentity(authority.providerSubject, authState, role, true, authority);
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
  return manualRequired(authState, 'admin-authorization-unavailable');
}

async function verifySupportRequest(
  request: Request,
  env: Env,
  authState: AuthState,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<AuthResult> {
  const blocker = authAdapterBlocker(env, providerVerifier);
  if (blocker) {
    return manualRequired(authState, blocker);
  }
  if (resolveAuthAdapterMode(env) === 'local-safe-fixture') {
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }

  const identity = await verifyProviderBoundRequest(request, env, authState, providerVerifier!);
  if (!identity.ok) {
    return identity;
  }
  const authority = identity.identity.authority;
  if (!isVerifiedAccountIdentityAuthorityCapability(authority)) {
    return manualRequired(authState, ACCOUNT_IDENTITY_BINDING_CONTEXT_MANUAL_REQUIRED_BLOCKER);
  }
  if (authority.role !== 'support-admin') {
    return forbidden('support-admin-capability-required', authState);
  }
  return authStateIdentity(authority.providerSubject, authState, 'support', true, authority);
}

async function verifyProviderWebhookRequest(
  provider: ProviderWebhookName,
  request: Request,
  env: Env,
  authState: AuthState
): Promise<AuthResult> {
  let result: Awaited<ReturnType<typeof verifyProviderWebhook>>;
  try {
    result = await verifyProviderWebhook(provider, request.clone(), env);
  } catch {
    return manualRequired(authState, PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.unavailable);
  }
  if (result.status === 'unavailable') {
    return manualRequired(authState, result.blocker);
  }
  if (result.status === 'missing-credential') {
    return missingHeader(result.headerName, authState);
  }
  if (result.status === 'rejected') {
    return {
      ok: false,
      response: json(400, {
        error: result.reason,
        authState,
      }),
    };
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

    async verifyBrowserSession(request: Request): Promise<AuthResult> {
      return verifyBrowserSessionRequest(request, env, 'browser-session-required');
    },

    async verifyBrowserRefresh(request: Request): Promise<AuthResult> {
      return verifyBrowserRefreshRequest(request, env, 'browser-refresh-required');
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

    async verifyProviderWebhook(provider: ProviderWebhookName, request: Request): Promise<AuthResult> {
      return verifyProviderWebhookRequest(provider, request, env, 'provider-webhook-signature-required');
    },

    async verifyInternalQueue(request: Request): Promise<AuthResult> {
      return verifyInternalQueueRequest(request, env, 'internal-queue-only');
    },
  };
}

export function signatureHeaderName(pathname: string): string {
  const provider = webhookProviderForPath(pathname);
  if (!provider) {
    return 'x-goog-signature';
  }
  return providerWebhookHeaderName(provider) ?? 'x-goog-signature';
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
    case 'verifyBrowserSession':
      return verifier.verifyBrowserSession(request);
    case 'verifyBrowserRefresh':
      return verifier.verifyBrowserRefresh(request);
    case 'verifyParentSession':
      return verifier.verifyParentSession(request);
    case 'verifyTrustedParentDevice':
      return verifier.verifyTrustedParentDevice(request);
    case 'verifyAdmin':
      return verifier.verifyAdmin(request);
    case 'verifySupport':
      return verifier.verifySupport(request);
    case 'verifyProviderWebhook': {
      const provider = resolveProviderWebhookName(webhookProviderForPath(new URL(request.url).pathname) ?? '');
      if (!provider) {
        return manualRequired('provider-webhook-signature-required', PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.unsupported);
      }
      return verifier.verifyProviderWebhook(provider, request);
    }
    case 'verifyInternalQueue':
      return verifier.verifyInternalQueue(request);
  }
}
