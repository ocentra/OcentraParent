import type { ProviderVerificationPort, VerifiedIdentity } from './verifier.js';
import {
  createAccountIdentityAuthorityStore,
  isVerifiedAccountIdentityAuthorityCapability,
} from '../storage/account-identity-authority-store.js';
import { createBrowserSessionStore } from '../storage/account-browser-session-store.js';
import { readCookie, cookieMaxAge } from '../storage/account-browser-session-codec.js';
import { isLocalFixtureEnvironment, parseAllowedOrigins, type Env } from '../env.js';

const SESSION_COOKIE = 'ocentra_session';
const REFRESH_COOKIE = 'ocentra_refresh';
const CSRF_COOKIE = 'ocentra_csrf';
const CSRF_HEADER = 'x-ocentra-csrf';

function json(status: number, body: unknown, headers: HeadersInit = {}): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'content-type': 'application/json; charset=utf-8', ...headers },
  });
}

function cookie(name: string, value: string, maxAge: number, httpOnly: boolean, env: Env): string {
  const secure = isLocalFixtureEnvironment(env) ? '' : '; Secure';
  const httpOnlyPart = httpOnly ? '; HttpOnly' : '';
  return `${name}=${value}; Path=/; SameSite=Lax; Max-Age=${maxAge}${secure}${httpOnlyPart}`;
}

function clearCookie(name: string, httpOnly: boolean, env: Env): string {
  return cookie(name, '', 0, httpOnly, env);
}

function safetyFailure(request: Request, env: Env, csrfRequired: boolean): Response | null {
  const origin = request.headers.get('origin');
  if (!origin || !parseAllowedOrigins(env).includes(origin)) {
    return json(403, { error: 'origin-validation-failed', boundary: 'account-browser-session' });
  }
  const fetchSite = request.headers.get('sec-fetch-site');
  if (fetchSite !== 'same-origin' && fetchSite !== 'same-site') {
    return json(403, { error: 'fetch-metadata-validation-failed', boundary: 'account-browser-session' });
  }
  if (csrfRequired && request.headers.get(CSRF_HEADER) === null) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  return null;
}

function capabilityMissing(): Response {
  return json(503, { error: 'manual-required', blocker: 'account-identity-authority-capability-missing' });
}

function authorityFailure(status: string): Response {
  return json(status === 'rejected' ? 403 : 503, {
    error: status === 'rejected' ? 'account-identity-authority-rejected' : 'manual-required',
    blocker: status,
  });
}

function sessionResponse(identity: VerifiedIdentity, expiresAt: string, csrfToken: string, env: Env): Response {
  const authority = identity.authority;
  if (!isVerifiedAccountIdentityAuthorityCapability(authority)) return capabilityMissing();
  return json(
    200,
    {
      status: 'authenticated',
      providerSubject: authority.providerSubject,
      accountId: authority.accountId,
      householdId: authority.householdId,
      memberId: authority.memberId,
      role: authority.role,
      expiresAt,
      csrfToken,
    },
    { 'cache-control': 'no-store' }
  );
}

export async function loginBrowserSession(
  request: Request,
  env: Env,
  providerVerifier: ProviderVerificationPort | undefined
): Promise<Response> {
  const safety = safetyFailure(request, env, false);
  if (safety) return safety;
  if (providerVerifier === undefined)
    return json(503, { error: 'manual-required', blocker: 'provider-verification-unavailable' });
  const providerIdentity = await providerVerifier.verify(request).catch(() => null);
  if (providerIdentity === null)
    return json(401, { error: 'authentication-required', blocker: 'provider-token-invalid' });
  const authority = await createAccountIdentityAuthorityStore(env.ACCOUNT_IDENTITY_D1).readCurrentAuthority(
    providerIdentity.provider,
    providerIdentity.providerSubject
  );
  if (authority.status !== 'trusted') return authorityFailure(authority.status);
  const created = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).create(authority.capability);
  if (created.status !== 'accepted')
    return json(created.status === 'manual-required' ? 503 : 409, { error: created.status, reason: created.reason });
  if (created.identity === null) return json(503, { error: 'session-custody-conflict' });
  const maxAge = cookieMaxAge(created.identity.expiresAt);
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', cookie(SESSION_COOKIE, created.secrets.sessionToken, maxAge, true, env));
  headers.append('set-cookie', cookie(REFRESH_COOKIE, created.secrets.refreshToken, maxAge, true, env));
  headers.append('set-cookie', cookie(CSRF_COOKIE, created.secrets.csrfToken, maxAge, false, env));
  const identity: VerifiedIdentity = {
    subject: authority.capability.providerSubject,
    state: 'browser-session-required',
    role: authority.capability.role === 'support-admin' ? 'support' : 'parent',
    trustedDevice: true,
    authority: authority.capability,
  };
  const response = sessionResponse(identity, created.identity.expiresAt, created.secrets.csrfToken, env);
  for (const [key, value] of response.headers) if (key !== 'cache-control') headers.set(key, value);
  return new Response(response.body, { status: response.status, headers });
}

export async function refreshBrowserSession(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  const rotated = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).rotate(readCookie(request, REFRESH_COOKIE));
  if (rotated.status !== 'accepted')
    return json(rotated.status === 'manual-required' ? 503 : 401, { error: rotated.status, reason: rotated.reason });
  if (rotated.identity === null) return json(503, { error: 'session-custody-conflict' });
  const maxAge = cookieMaxAge(rotated.identity.expiresAt);
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', cookie(SESSION_COOKIE, rotated.secrets.sessionToken, maxAge, true, env));
  headers.append('set-cookie', cookie(REFRESH_COOKIE, rotated.secrets.refreshToken, maxAge, true, env));
  headers.append('set-cookie', cookie(CSRF_COOKIE, rotated.secrets.csrfToken, maxAge, false, env));
  const response = sessionResponse(identity, rotated.identity.expiresAt, rotated.secrets.csrfToken, env);
  for (const [key, value] of response.headers) if (key !== 'cache-control') headers.set(key, value);
  return new Response(response.body, { status: response.status, headers });
}

export async function logoutBrowserSession(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  if (!(await store.verifyCsrf(readCookie(request, SESSION_COOKIE), request.headers.get(CSRF_HEADER)))) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  const result = await store.logout(readCookie(request, SESSION_COOKIE));
  if (result.status !== 'accepted')
    return json(result.status === 'manual-required' ? 503 : 401, { error: result.status, reason: result.reason });
  const headers = new Headers({ 'cache-control': 'no-store', 'x-ocentra-logout-subject': identity.subject });
  headers.append('set-cookie', clearCookie(SESSION_COOKIE, true, env));
  headers.append('set-cookie', clearCookie(REFRESH_COOKIE, true, env));
  headers.append('set-cookie', clearCookie(CSRF_COOKIE, false, env));
  return new Response(null, {
    status: 204,
    headers,
  });
}

export async function revokeBrowserSessions(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  if (!isVerifiedAccountIdentityAuthorityCapability(identity.authority)) return capabilityMissing();
  if (identity.authority.role !== 'parent-owner') return json(403, { error: 'parent-owner-required' });
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  if (!(await store.verifyCsrf(readCookie(request, SESSION_COOKIE), request.headers.get(CSRF_HEADER)))) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  const result = await store.revokeAll(identity.authority.provider, identity.authority.providerSubject);
  if (result.status !== 'accepted')
    return json(result.status === 'manual-required' ? 503 : 409, { error: result.status, reason: result.reason });
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', clearCookie(SESSION_COOKIE, true, env));
  headers.append('set-cookie', clearCookie(REFRESH_COOKIE, true, env));
  headers.append('set-cookie', clearCookie(CSRF_COOKIE, false, env));
  return new Response(null, {
    status: 204,
    headers,
  });
}
