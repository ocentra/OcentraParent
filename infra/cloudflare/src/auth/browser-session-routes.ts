import type { ProviderVerificationPort, VerifiedIdentity } from './verifier.js';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
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
  newOpaqueValue,
  readCookie,
} from '../storage/account-browser-session-codec.js';
import { isLocalFixtureEnvironment, parseAllowedOrigins, type Env } from '../env.js';

const CSRF_HEADER = 'x-ocentra-csrf';
const REQUEST_CORRELATION_HEADER = 'x-ocentra-request-id';
const log = Logger.instance;
log.register(import.meta.url);

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

function requestCorrelationId(request: Request): string {
  const candidate =
    request.headers.get('x-ocentra-request-id') ??
    request.headers.get('x-request-id') ??
    request.headers.get('cf-ray') ??
    null;
  return candidate && /^[A-Za-z0-9._:-]{1,128}$/.test(candidate)
    ? candidate
    : `browser-route-${newOpaqueValue().slice(0, 24)}`;
}

function milestone(
  action: 'login' | 'refresh' | 'logout' | 'global-revoke',
  result: 'started' | 'accepted' | 'rejected' | 'manual-required',
  correlationId: string,
  reason?: string
): void {
  log.logInfo(
    'account browser session route milestone',
    getStackTrace(),
    {
      owner: 'account-identity-family-plan',
      boundary: 'browser-session-route',
      action,
      result,
      reason: reason ?? null,
      correlationId,
      redactionState: 'provider-subject-and-session-identifiers-omitted',
    },
    true
  );
}

function withCorrelation(response: Response, correlationId: string): Response {
  const headers = new Headers(response.headers);
  headers.set(REQUEST_CORRELATION_HEADER, correlationId);
  return new Response(response.body, { status: response.status, statusText: response.statusText, headers });
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
  const correlation = requestCorrelationId(request);
  milestone('login', 'started', correlation);
  const safety = safetyFailure(request, env, false);
  if (safety) {
    milestone('login', 'rejected', correlation, 'request-safety');
    return withCorrelation(safety, correlation);
  }
  const authorityResult = await createAccountIdentityAuthorityCaller(env).resolveVerifiedProviderAuthority(
    request,
    providerVerifier
  );
  if (authorityResult.status !== 'trusted') {
    milestone(
      'login',
      authorityResult.status === 'manual-required' ? 'manual-required' : 'rejected',
      correlation,
      authorityResult.status
    );
    return withCorrelation(authorityFailure(authorityResult), correlation);
  }
  const authority = authorityResult.capability;
  const role = browserSessionRole(authority.role);
  if (role === null) {
    milestone('login', 'rejected', correlation, 'role-ineligible');
    return withCorrelation(json(403, { error: 'browser-session-role-ineligible' }), correlation);
  }
  const nowMs = Date.now();
  const created = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).create(authority, nowMs, correlation);
  if (created.status !== 'accepted') {
    milestone(
      'login',
      created.status === 'manual-required' ? 'manual-required' : 'rejected',
      correlation,
      created.reason
    );
    return withCorrelation(
      json(created.status === 'manual-required' ? 503 : 409, {
        error: created.status,
        reason: created.reason,
      }),
      correlation
    );
  }
  if (created.identity === null) {
    milestone('login', 'manual-required', correlation, 'session-custody-conflict');
    return withCorrelation(json(503, { error: 'session-custody-conflict' }), correlation);
  }
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
  milestone('login', 'accepted', correlation);
  headers.set(REQUEST_CORRELATION_HEADER, correlation);
  return new Response(response.body, { status: response.status, headers });
}

export async function refreshBrowserSession(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const correlation = requestCorrelationId(request);
  milestone('refresh', 'started', correlation);
  const safety = safetyFailure(request, env, true);
  if (safety) {
    milestone('refresh', 'rejected', correlation, 'request-safety');
    return withCorrelation(safety, correlation);
  }
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const refreshToken = readCookie(request, names.refresh);
  if (!(await store.verifyRefreshCsrf(refreshToken, request.headers.get(CSRF_HEADER)))) {
    milestone('refresh', 'rejected', correlation, 'csrf-validation-failed');
    return withCorrelation(
      json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' }),
      correlation
    );
  }
  const nowMs = Date.now();
  const rotated = await store.rotate(refreshToken, nowMs, correlation);
  if (rotated.status !== 'accepted') {
    milestone(
      'refresh',
      rotated.status === 'manual-required' ? 'manual-required' : 'rejected',
      correlation,
      rotated.reason
    );
    return withCorrelation(
      json(rotated.status === 'manual-required' ? 503 : 401, {
        error: rotated.status,
        reason: rotated.reason,
      }),
      correlation
    );
  }
  if (rotated.identity === null) {
    milestone('refresh', 'manual-required', correlation, 'session-custody-conflict');
    return withCorrelation(json(503, { error: 'session-custody-conflict' }), correlation);
  }
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
  milestone('refresh', 'accepted', correlation);
  headers.set(REQUEST_CORRELATION_HEADER, correlation);
  return new Response(response.body, { status: response.status, headers });
}

export async function logoutBrowserSession(request: Request, env: Env, _identity: VerifiedIdentity): Promise<Response> {
  const correlation = requestCorrelationId(request);
  milestone('logout', 'started', correlation);
  const safety = safetyFailure(request, env, true);
  if (safety) {
    milestone('logout', 'rejected', correlation, 'request-safety');
    return withCorrelation(safety, correlation);
  }
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const refreshToken = readCookie(request, names.refresh);
  if (!(await store.verifyRefreshCsrf(refreshToken, request.headers.get(CSRF_HEADER)))) {
    milestone('logout', 'rejected', correlation, 'csrf-validation-failed');
    return withCorrelation(
      json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' }),
      correlation
    );
  }
  const result = await store.logoutRefresh(refreshToken, Date.now(), correlation);
  if (result.status !== 'accepted') {
    milestone(
      'logout',
      result.status === 'manual-required' ? 'manual-required' : 'rejected',
      correlation,
      result.reason
    );
    return withCorrelation(
      json(result.status === 'manual-required' ? 503 : 401, {
        error: result.status,
        reason: result.reason,
      }),
      correlation
    );
  }
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', clearCookie(names.session, true, env));
  headers.append('set-cookie', clearCookie(names.refresh, true, env));
  headers.append('set-cookie', clearCookie(names.csrf, false, env));
  headers.set(REQUEST_CORRELATION_HEADER, correlation);
  milestone('logout', 'accepted', correlation);
  return new Response(null, { status: 204, headers });
}

export async function revokeBrowserSessions(request: Request, env: Env, identity: VerifiedIdentity): Promise<Response> {
  const correlation = requestCorrelationId(request);
  milestone('global-revoke', 'started', correlation);
  const safety = safetyFailure(request, env, true);
  if (safety) {
    milestone('global-revoke', 'rejected', correlation, 'request-safety');
    return withCorrelation(safety, correlation);
  }
  if (!isVerifiedAccountIdentityAuthorityCapability(identity.authority)) {
    milestone('global-revoke', 'manual-required', correlation, 'authority-capability-missing');
    return withCorrelation(capabilityMissing(), correlation);
  }
  if (identity.authority.role !== 'parent-owner') {
    milestone('global-revoke', 'rejected', correlation, 'parent-owner-required');
    return withCorrelation(json(403, { error: 'parent-owner-required' }), correlation);
  }
  const names = cookieNames(env);
  const store = createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1);
  const refreshToken = readCookie(request, names.refresh);
  if (!(await store.verifyRefreshCsrf(refreshToken, request.headers.get(CSRF_HEADER)))) {
    milestone('global-revoke', 'rejected', correlation, 'csrf-validation-failed');
    return withCorrelation(
      json(403, { error: 'csrf-validation-failed', boundary: 'account-browser-session' }),
      correlation
    );
  }
  const result = await store.revokeAll(identity.authority, Date.now(), correlation);
  if (result.status !== 'accepted') {
    milestone(
      'global-revoke',
      result.status === 'manual-required' ? 'manual-required' : 'rejected',
      correlation,
      result.reason
    );
    return withCorrelation(
      json(result.status === 'manual-required' ? 503 : 409, {
        error: result.status,
        reason: result.reason,
      }),
      correlation
    );
  }
  const headers = new Headers({ 'cache-control': 'no-store' });
  headers.append('set-cookie', clearCookie(names.session, true, env));
  headers.append('set-cookie', clearCookie(names.refresh, true, env));
  headers.append('set-cookie', clearCookie(names.csrf, false, env));
  headers.set(REQUEST_CORRELATION_HEADER, correlation);
  milestone('global-revoke', 'accepted', correlation);
  return new Response(null, { status: 204, headers });
}
