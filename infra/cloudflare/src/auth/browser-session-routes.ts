import type { ProviderVerificationPort, VerifiedIdentity } from './verifier.js';
import {
  createAccountIdentityAuthorityCaller,
  type VerifiedProviderAuthorityResult,
} from './account-identity-authority-caller.js';
import { isVerifiedAccountIdentityAuthorityCapability } from '../storage/account-identity-authority-store.js';
import { createBrowserSessionStore } from '../storage/account-browser-session-store.js';
import {
  browserSessionCookieNames,
  browserSessionRole,
  cookieMaxAge,
  readCookie,
} from '../storage/account-browser-session-codec.js';
import { isLocalFixtureEnvironment, parseAllowedOrigins, type Env } from '../env.js';

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

function cookieNames(env: Env) {
  return browserSessionCookieNames(!isLocalFixtureEnvironment(env));
}

function requestCorrelationId(request: Request): string | undefined {
  return (
    request.headers.get('x-ocentra-request-id') ??
    request.headers.get('x-request-id') ??
    request.headers.get('cf-ray') ??
    undefined
  );
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

function authorityFailure(result: VerifiedProviderAuthorityResult): Response {
  switch (result.status) {
    case 'rejected':
      return json(403, { error: 'account-identity-authority-rejected' });
    case 'provider-unavailable':
      return json(503, { error: 'manual-required', blocker: 'provider-verification-unavailable' });
    case 'not-found':
      return json(503, { error: 'manual-required', blocker: 'account-identity-binding-context-manual-required' });
    case 'manual-required':
      return json(503, { error: 'manual-required', blocker: result.reason });
    case 'trusted':
      return capabilityMissing();
  }
}

function sessionResponse(identity: VerifiedIdentity, expiresAt: string, csrfToken: string): Response {
  const authority = identity.authority;
  if (!isVerifiedAccountIdentityAuthorityCapability(authority)) return capabilityMissing();
  if (browserSessionRole(authority.role) === null) {
    return json(403, { error: 'browser-session-role-ineligible' });
  }
  return json(
    200,
    {
      status: 'authenticated',
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
  const authorityResult = await createAccountIdentityAuthorityCaller(env).resolveVerifiedProviderAuthority(
    request,
    providerVerifier
  );
  if (authorityResult.status !== 'trusted') return authorityFailure(authorityResult);
  const authority = authorityResult.capability;
  const role = browserSessionRole(authority.role);
  if (role === null) return json(403, { error: 'browser-session-role-ineligible' });
  const nowMs = Date.now();
  const created = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).create(
    authority,
    nowMs,
    requestCorrelationId(request)
  );
  if (created.status !== 'accepted') {
    return json(created.status === 'manual-required' ? 503 : 409, {
      error: created.status,
      reason: created.reason,
    });
  }
  if (created.identity === null) return json(503, { error: 'session-custody-conflict' });
  const names = cookieNames(env);
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append(
    'set-cookie',
    cookie(
      names.session,
      created.secrets.sessionToken,
      cookieMaxAge(created.identity.accessExpiresAt, nowMs),
      true,
      env
    )
  );
  headers.append(
    'set-cookie',
    cookie(
      names.refresh,
      created.secrets.refreshToken,
      cookieMaxAge(created.identity.refreshExpiresAt, nowMs),
      true,
      env
    )
  );
  headers.append(
    'set-cookie',
    cookie(names.csrf, created.secrets.csrfToken, cookieMaxAge(created.identity.refreshExpiresAt, nowMs), false, env)
  );
  const identity: VerifiedIdentity = {
    subject: authority.providerSubject,
    state: 'browser-session-required',
    role,
    trustedDevice: true,
    authority,
  };
  const response = sessionResponse(identity, created.identity.accessExpiresAt, created.secrets.csrfToken);
  for (const [key, value] of response.headers) if (key !== 'cache-control') headers.set(key, value);
  return new Response(response.body, { status: response.status, headers });
}

export async function refreshBrowserSession(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const refreshToken = readCookie(request, names.refresh);
  if (!(await store.verifyRefreshCsrf(refreshToken, request.headers.get(CSRF_HEADER)))) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  const nowMs = Date.now();
  const rotated = await store.rotate(refreshToken, nowMs, requestCorrelationId(request));
  if (rotated.status !== 'accepted') {
    return json(rotated.status === 'manual-required' ? 503 : 401, {
      error: rotated.status,
      reason: rotated.reason,
    });
  }
  if (rotated.identity === null) return json(503, { error: 'session-custody-conflict' });
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append(
    'set-cookie',
    cookie(
      names.session,
      rotated.secrets.sessionToken,
      cookieMaxAge(rotated.identity.accessExpiresAt, nowMs),
      true,
      env
    )
  );
  headers.append(
    'set-cookie',
    cookie(
      names.refresh,
      rotated.secrets.refreshToken,
      cookieMaxAge(rotated.identity.refreshExpiresAt, nowMs),
      true,
      env
    )
  );
  headers.append(
    'set-cookie',
    cookie(names.csrf, rotated.secrets.csrfToken, cookieMaxAge(rotated.identity.refreshExpiresAt, nowMs), false, env)
  );
  const response = sessionResponse(identity, rotated.identity.accessExpiresAt, rotated.secrets.csrfToken);
  for (const [key, value] of response.headers) if (key !== 'cache-control') headers.set(key, value);
  return new Response(response.body, { status: response.status, headers });
}

export async function logoutBrowserSession(request: Request, env: Env, _identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const sessionToken = readCookie(request, names.session);
  if (!(await store.verifyCsrf(sessionToken, request.headers.get(CSRF_HEADER)))) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  const result = await store.logout(sessionToken, Date.now(), requestCorrelationId(request));
  if (result.status !== 'accepted') {
    return json(result.status === 'manual-required' ? 503 : 401, {
      error: result.status,
      reason: result.reason,
    });
  }
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', clearCookie(names.session, true, env));
  headers.append('set-cookie', clearCookie(names.refresh, true, env));
  headers.append('set-cookie', clearCookie(names.csrf, false, env));
  return new Response(null, { status: 204, headers });
}

export async function revokeBrowserSessions(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const safety = safetyFailure(request, env, true);
  if (safety) return safety;
  if (!isVerifiedAccountIdentityAuthorityCapability(identity.authority)) return capabilityMissing();
  if (identity.authority.role !== 'parent-owner') return json(403, { error: 'parent-owner-required' });
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const sessionToken = readCookie(request, names.session);
  if (!(await store.verifyCsrf(sessionToken, request.headers.get(CSRF_HEADER)))) {
    return json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' });
  }
  const result = await store.revokeAll(
    identity.authority.provider,
    identity.authority.providerSubject,
    Date.now(),
    requestCorrelationId(request)
  );
  if (result.status !== 'accepted') {
    return json(result.status === 'manual-required' ? 503 : 409, {
      error: result.status,
      reason: result.reason,
    });
  }
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', clearCookie(names.session, true, env));
  headers.append('set-cookie', clearCookie(names.refresh, true, env));
  headers.append('set-cookie', clearCookie(names.csrf, false, env));
  return new Response(null, { status: 204, headers });
}
