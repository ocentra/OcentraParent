import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';

const SESSION_TOKEN_PATTERN = /^[A-Za-z0-9_-]{43}\.[A-Za-z0-9_-]{43}$/;
const SESSION_ID_PATTERN = /^[A-Za-z0-9_-]{43}$/;
const DIGEST_HEX_PATTERN = /^[a-f0-9]{64}$/;
const ISO_TIMESTAMP_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/;
const AUTHORITY_TEXT_PATTERN = /^[^\u0000-\u001f\u007f]{1,256}$/;

export interface BrowserSessionSecrets {
  readonly sessionToken: string;
  readonly refreshToken: string;
  readonly csrfToken: string;
}

export interface BrowserSessionRow {
  session_id: string;
  session_token_digest: string;
  refresh_token_digest: string;
  csrf_token_digest: string;
  provider: AccountIdentityProvider;
  provider_subject: string;
  role: 'parent-owner' | 'co-parent-guardian' | 'support-admin';
  account_id: string;
  household_id: string | null;
  member_id: string | null;
  device_id: string | null;
  child_profile_id: string | null;
  child_device_id: string | null;
  authority_session_id: string;
  authority_session_generation: number;
  authority_generation: number;
  support_receipt_id: string | null;
  support_provider_subject: string | null;
  support_account_id: string | null;
  support_member_id: string | null;
  support_household_id: string | null;
  support_device_id: string | null;
  support_child_profile_id: string | null;
  support_child_device_id: string | null;
  support_scope: 'read-only' | 'household' | 'device-control' | null;
  support_issuer: string | null;
  support_issued_at: string | null;
  support_expires_at: string | null;
  support_revocation_state: 'active' | 'revoked' | null;
  support_audit_identity: string | null;
  issued_at: string;
  access_expires_at: string;
  refresh_expires_at: string;
  revoke_generation: number;
  refresh_generation: number;
  status: 'active' | 'revoked';
  last_seen_at: string;
  revoked_at: string | null;
  created_at: string;
  updated_at: string;
}

function record(value: unknown): Record<string, unknown> | null {
  return value !== null && typeof value === 'object' && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null;
}

function text(value: unknown): value is string {
  return typeof value === 'string' && AUTHORITY_TEXT_PATTERN.test(value);
}

function nullableText(value: unknown): value is string | null {
  return value === null || text(value);
}

function generation(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value > 0;
}

function timestamp(value: unknown): value is string {
  return typeof value === 'string' && ISO_TIMESTAMP_PATTERN.test(value) && Number.isFinite(Date.parse(value));
}

export function isBrowserSessionTimestamp(value: unknown): value is string {
  return timestamp(value);
}

function nullableTimestamp(value: unknown): value is string | null {
  return value === null || timestamp(value);
}

function provider(value: unknown): value is AccountIdentityProvider {
  return value === 'authjs' || value === 'firebase';
}

function role(value: unknown): value is BrowserSessionRow['role'] {
  return value === 'parent-owner' || value === 'co-parent-guardian' || value === 'support-admin';
}

function nullableSupportScope(value: unknown): value is BrowserSessionRow['support_scope'] {
  return value === null || value === 'read-only' || value === 'household' || value === 'device-control';
}

function nullableSupportRevocationState(value: unknown): value is BrowserSessionRow['support_revocation_state'] {
  return value === null || value === 'active' || value === 'revoked';
}

/**
 * D1's generic row decoder is not a runtime schema. Every authority-bearing
 * row must pass this exact shape/type check before it can become a session
 * identity or participate in a custody mutation.
 */
export function isBrowserSessionRow(value: unknown): value is BrowserSessionRow {
  const row = record(value);
  if (row === null) return false;
  const baseShape =
    isSessionId(row.session_id) &&
    isDigestValue(row.session_token_digest) &&
    isDigestValue(row.refresh_token_digest) &&
    isDigestValue(row.csrf_token_digest) &&
    provider(row.provider) &&
    text(row.provider_subject) &&
    role(row.role) &&
    text(row.account_id) &&
    nullableText(row.household_id) &&
    nullableText(row.member_id) &&
    nullableText(row.device_id) &&
    nullableText(row.child_profile_id) &&
    nullableText(row.child_device_id) &&
    text(row.authority_session_id) &&
    generation(row.authority_session_generation) &&
    generation(row.authority_generation) &&
    nullableText(row.support_receipt_id) &&
    nullableText(row.support_provider_subject) &&
    nullableText(row.support_account_id) &&
    nullableText(row.support_member_id) &&
    nullableText(row.support_household_id) &&
    nullableText(row.support_device_id) &&
    nullableText(row.support_child_profile_id) &&
    nullableText(row.support_child_device_id) &&
    nullableSupportScope(row.support_scope) &&
    nullableText(row.support_issuer) &&
    nullableTimestamp(row.support_issued_at) &&
    nullableTimestamp(row.support_expires_at) &&
    nullableSupportRevocationState(row.support_revocation_state) &&
    nullableText(row.support_audit_identity) &&
    timestamp(row.issued_at) &&
    timestamp(row.access_expires_at) &&
    timestamp(row.refresh_expires_at) &&
    generation(row.revoke_generation) &&
    generation(row.refresh_generation) &&
    (row.status === 'active' || row.status === 'revoked') &&
    timestamp(row.last_seen_at) &&
    nullableTimestamp(row.revoked_at) &&
    timestamp(row.created_at) &&
    timestamp(row.updated_at);
  if (!baseShape) return false;
  const session = row as BrowserSessionRow;

  const issuedAt = Date.parse(session.issued_at);
  const createdAt = Date.parse(session.created_at);
  const accessExpiresAt = Date.parse(session.access_expires_at);
  const refreshExpiresAt = Date.parse(session.refresh_expires_at);
  const lastSeenAt = Date.parse(session.last_seen_at);
  const updatedAt = Date.parse(session.updated_at);
  const revokedAt = session.revoked_at === null ? null : Date.parse(session.revoked_at);
  const supportIssuedAt = session.support_issued_at === null ? null : Date.parse(session.support_issued_at);
  const supportExpiresAt = session.support_expires_at === null ? null : Date.parse(session.support_expires_at);
  const supportFields = [
    session.support_receipt_id,
    session.support_provider_subject,
    session.support_account_id,
    session.support_member_id,
    session.support_household_id,
    session.support_device_id,
    session.support_child_profile_id,
    session.support_child_device_id,
    session.support_scope,
    session.support_issuer,
    session.support_issued_at,
    session.support_expires_at,
    session.support_revocation_state,
    session.support_audit_identity,
  ];
  const supportComplete =
    session.role === 'support-admin' &&
    supportFields.every((field) => field !== null) &&
    session.support_provider_subject === session.provider_subject &&
    session.support_account_id === session.account_id &&
    session.support_member_id === session.member_id &&
    session.support_household_id === session.household_id &&
    session.support_device_id === session.device_id &&
    session.support_child_profile_id === session.child_profile_id &&
    session.support_child_device_id === session.child_device_id &&
    session.support_revocation_state === 'active' &&
    supportIssuedAt !== null &&
    supportExpiresAt !== null &&
    supportIssuedAt <= issuedAt &&
    supportExpiresAt > issuedAt &&
    supportExpiresAt > supportIssuedAt;
  const supportAbsent = session.role !== 'support-admin' && supportFields.every((field) => field === null);
  const distinctDigests =
    new Set([session.session_token_digest, session.refresh_token_digest, session.csrf_token_digest]).size === 3;
  const orderedLifetime =
    createdAt <= issuedAt &&
    issuedAt <= lastSeenAt &&
    lastSeenAt <= updatedAt &&
    issuedAt < accessExpiresAt &&
    accessExpiresAt < refreshExpiresAt &&
    (session.status === 'active'
      ? session.revoked_at === null
      : session.revoked_at !== null && revokedAt !== null && createdAt <= revokedAt && revokedAt <= updatedAt);
  return distinctDigests && orderedLifetime && (supportComplete || supportAbsent);
}

function isDigestValue(value: unknown): value is string {
  return typeof value === 'string' && DIGEST_HEX_PATTERN.test(value);
}

export interface BrowserSessionIdentity {
  readonly sessionId: string;
  readonly provider: AccountIdentityProvider;
  readonly providerSubject: string;
  readonly role: 'parent-owner' | 'co-parent-guardian' | 'support-admin';
  readonly accountId: string;
  readonly authoritySessionId: string;
  readonly authoritySessionGeneration: number;
  readonly authorityGeneration: number;
  readonly issuedAt: string;
  readonly accessExpiresAt: string;
  readonly refreshExpiresAt: string;
  readonly revokeGeneration: number;
  readonly refreshGeneration: number;
}

function base64Url(bytes: Uint8Array): string {
  let binary = '';
  for (const value of bytes) binary += String.fromCharCode(value);
  return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/g, '');
}

export function newOpaqueValue(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return base64Url(bytes);
}

export function newSessionId(): string {
  return newOpaqueValue();
}

export function isSessionId(value: unknown): value is string {
  return typeof value === 'string' && SESSION_ID_PATTERN.test(value);
}

export function sessionCookieValue(sessionId: string, secret: string): string {
  return `${sessionId}.${secret}`;
}

export function parseSessionCookie(value: string | null): { sessionId: string; secret: string } | null {
  if (value === null || !SESSION_TOKEN_PATTERN.test(value)) return null;
  const separator = value.indexOf('.');
  const sessionId = value.slice(0, separator);
  const secret = value.slice(separator + 1);
  return sessionId.length === 43 && secret.length === 43 ? { sessionId, secret } : null;
}

export async function sha256Hex(value: string): Promise<string> {
  const bytes = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(value));
  return Array.from(new Uint8Array(bytes), (part) => part.toString(16).padStart(2, '0')).join('');
}

export function isDigest(value: string): boolean {
  return DIGEST_HEX_PATTERN.test(value);
}

export function constantTimeEqual(left: string, right: string): boolean {
  const length = Math.max(left.length, right.length);
  let difference = left.length ^ right.length;
  for (let index = 0; index < length; index += 1) {
    difference |= (left.charCodeAt(index) || 0) ^ (right.charCodeAt(index) || 0);
  }
  return difference === 0;
}

export function browserSessionRole(role: string): 'parent' | 'support' | null {
  if (role === 'parent-owner' || role === 'co-parent-guardian') return 'parent';
  if (role === 'support-admin') return 'support';
  return null;
}

export interface BrowserSessionCookieNames {
  readonly session: string;
  readonly refresh: string;
  readonly csrf: string;
}

export function browserSessionCookieNames(useHostPrefix: boolean): BrowserSessionCookieNames {
  const prefix = useHostPrefix ? '__Host-' : '';
  return {
    session: `${prefix}ocentra_session`,
    refresh: `${prefix}ocentra_refresh`,
    csrf: `${prefix}ocentra_csrf`,
  };
}

export function sessionIdentity(row: BrowserSessionRow): BrowserSessionIdentity {
  return {
    sessionId: row.session_id,
    provider: row.provider,
    providerSubject: row.provider_subject,
    role: row.role,
    accountId: row.account_id,
    authoritySessionId: row.authority_session_id,
    authoritySessionGeneration: row.authority_session_generation,
    authorityGeneration: row.authority_generation,
    issuedAt: row.issued_at,
    accessExpiresAt: row.access_expires_at,
    refreshExpiresAt: row.refresh_expires_at,
    revokeGeneration: row.revoke_generation,
    refreshGeneration: row.refresh_generation,
  };
}

export function nowIso(nowMs = Date.now()): string {
  return new Date(nowMs).toISOString();
}

export function cookieMaxAge(expiresAt: string, nowMs = Date.now()): number {
  const remaining = Date.parse(expiresAt) - nowMs;
  return Math.max(0, Math.floor(remaining / 1000));
}

export function readCookie(request: Request, name: string): string | null {
  const source = request.headers.get('cookie');
  if (!source) return null;
  for (const part of source.split(';')) {
    const separator = part.indexOf('=');
    if (separator < 0 || part.slice(0, separator).trim() !== name) continue;
    return part.slice(separator + 1).trim() || null;
  }
  return null;
}
