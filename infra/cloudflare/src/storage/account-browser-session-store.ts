import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';
import type { D1Database } from '@cloudflare/workers-types';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import type { VerifiedAccountIdentityAuthorityCapability } from './account-identity-authority-store.js';
import {
  isDigest,
  newOpaqueValue,
  newSessionId,
  nowIso,
  parseSessionCookie,
  sessionCookieValue,
  sessionIdentity,
  sha256Hex,
  type BrowserSessionIdentity,
  type BrowserSessionRow,
  type BrowserSessionSecrets,
} from './account-browser-session-codec.js';

const SESSION_LIFETIME_MS = 8 * 60 * 60 * 1000;
const log = Logger.instance;
log.register(import.meta.url);

export type BrowserSessionReadResult =
  | { status: 'active'; identity: BrowserSessionIdentity; row: BrowserSessionRow }
  | { status: 'missing' }
  | { status: 'rejected'; reason: 'malformed' | 'invalid' | 'expired' | 'revoked' }
  | { status: 'manual-required'; reason: 'binding-missing' | 'schema-missing' };

export type BrowserSessionMutationResult =
  | { status: 'accepted'; identity: BrowserSessionIdentity | null; secrets: BrowserSessionSecrets }
  | { status: 'rejected'; reason: 'missing' | 'malformed' | 'invalid' | 'expired' | 'revoked' | 'replay' | 'conflict' }
  | { status: 'manual-required'; reason: 'binding-missing' | 'schema-missing' };

export interface BrowserSessionStore {
  create(authority: VerifiedAccountIdentityAuthorityCapability, nowMs?: number): Promise<BrowserSessionMutationResult>;
  read(sessionToken: string | null, nowMs?: number): Promise<BrowserSessionReadResult>;
  rotate(refreshToken: string | null, nowMs?: number): Promise<BrowserSessionMutationResult>;
  verifyCsrf(sessionToken: string | null, csrfToken: string | null, nowMs?: number): Promise<boolean>;
  logout(sessionToken: string | null, nowMs?: number): Promise<BrowserSessionMutationResult>;
  revokeAll(
    provider: AccountIdentityProvider,
    providerSubject: string,
    nowMs?: number
  ): Promise<BrowserSessionMutationResult>;
}

const SELECT_BY_ID_SQL = `
SELECT session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
       provider, provider_subject, account_id, authority_session_id,
       authority_session_generation, authority_generation, issued_at, expires_at,
       refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
FROM ocentra_account_browser_sessions WHERE session_id = ? LIMIT 1`;

const SELECT_BY_SUBJECT_SQL = `
SELECT session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
       provider, provider_subject, account_id, authority_session_id,
       authority_session_generation, authority_generation, issued_at, expires_at,
       refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
FROM ocentra_account_browser_sessions
WHERE provider = ? AND provider_subject = ? AND status = 'active'`;

const INSERT_SQL = `
INSERT INTO ocentra_account_browser_sessions (
  session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
  provider, provider_subject, account_id, authority_session_id,
  authority_session_generation, authority_generation, issued_at, expires_at,
  refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 'active', ?, NULL, ?, ?)`;

const REVOKE_SQL = `
UPDATE ocentra_account_browser_sessions
SET status = 'revoked', revoked_at = ?, updated_at = ?
WHERE session_id = ? AND status = 'active'`;

const REVOKE_ALL_SQL = `
UPDATE ocentra_account_browser_sessions
SET status = 'revoked', revoked_at = ?, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND status = 'active'`;

const ROTATE_SQL = `
UPDATE ocentra_account_browser_sessions
SET session_token_digest = ?, refresh_token_digest = ?, csrf_token_digest = ?,
    refresh_generation = refresh_generation + 1, last_seen_at = ?, updated_at = ?
WHERE session_id = ? AND refresh_token_digest = ? AND status = 'active' AND expires_at > ?`;

const AUDIT_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_id, provider, provider_subject, action, result, reason, correlation_id, occurred_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`;

function missingSchema(error: unknown): boolean {
  return (
    String(error instanceof Error ? error.message : error)
      .toLowerCase()
      .includes('no such table') &&
    String(error instanceof Error ? error.message : error).includes('ocentra_account_browser_')
  );
}

function changes(result: { meta?: { changes?: number } }): number {
  return typeof result.meta?.changes === 'number' ? result.meta.changes : 0;
}

function safeSubject(value: string): boolean {
  return value.length > 0 && value.length <= 256 && !/[\u0000-\u001f\u007f]/.test(value);
}

function correlationId(action: string): string {
  return `${action}-${newSessionId().slice(0, 16)}`;
}

async function audit(
  database: D1Database,
  row: Pick<BrowserSessionRow, 'session_id' | 'provider' | 'provider_subject'>,
  action: 'created' | 'refreshed' | 'logout' | 'global-revoke' | 'replay-rejected',
  result: 'accepted' | 'rejected',
  reason: string,
  now: string
): Promise<void> {
  await database
    .prepare(AUDIT_SQL)
    .bind(
      newSessionId(),
      row.session_id,
      row.provider,
      row.provider_subject,
      action,
      result,
      reason,
      correlationId(action),
      now
    )
    .run();
}

function expired(row: BrowserSessionRow, nowMs: number): boolean {
  const expiry = Date.parse(row.expires_at);
  return !Number.isFinite(expiry) || expiry <= nowMs;
}

async function rowById(database: D1Database, sessionId: string): Promise<BrowserSessionRow | null> {
  return database.prepare(SELECT_BY_ID_SQL).bind(sessionId).first<BrowserSessionRow>();
}

function mutationFromRow(row: BrowserSessionRow, secrets: BrowserSessionSecrets): BrowserSessionMutationResult {
  return { status: 'accepted', identity: sessionIdentity(row), secrets };
}

async function readWithSecret(
  database: D1Database,
  value: string | null,
  digestField: 'session_token_digest' | 'refresh_token_digest',
  nowMs: number
): Promise<{ row: BrowserSessionRow; digest: string } | BrowserSessionReadResult> {
  const parsed = parseSessionCookie(value);
  if (value !== null && parsed === null) return { status: 'rejected', reason: 'malformed' };
  if (parsed === null) return { status: 'missing' };
  const row = await rowById(database, parsed.sessionId);
  if (row === null) return { status: 'rejected', reason: 'invalid' };
  const digest = await sha256Hex(value!);
  if (!isDigest(digest) || digest !== row[digestField]) return { status: 'rejected', reason: 'invalid' };
  if (row.status !== 'active') return { status: 'rejected', reason: 'revoked' };
  if (expired(row, nowMs)) return { status: 'rejected', reason: 'expired' };
  return { row, digest };
}

export function createBrowserSessionStore(database: D1Database | undefined): BrowserSessionStore {
  const unavailable = (): BrowserSessionMutationResult => ({ status: 'manual-required', reason: 'binding-missing' });
  if (database === undefined) {
    return {
      create: async () => unavailable(),
      read: async () => ({ status: 'manual-required', reason: 'binding-missing' }),
      rotate: async () => unavailable(),
      verifyCsrf: async () => false,
      logout: async () => unavailable(),
      revokeAll: async () => unavailable(),
    };
  }

  return {
    async create(authority, nowMs = Date.now()) {
      const issuedAt = nowIso(nowMs);
      const expiresAt = nowIso(nowMs + SESSION_LIFETIME_MS);
      const sessionId = newSessionId();
      const secrets = {
        sessionToken: sessionCookieValue(sessionId, newOpaqueValue()),
        refreshToken: sessionCookieValue(sessionId, newOpaqueValue()),
        csrfToken: sessionCookieValue(sessionId, newOpaqueValue()),
      } satisfies BrowserSessionSecrets;
      try {
        await database
          .prepare(INSERT_SQL)
          .bind(
            sessionId,
            await sha256Hex(secrets.sessionToken),
            await sha256Hex(secrets.refreshToken),
            await sha256Hex(secrets.csrfToken),
            authority.provider,
            authority.providerSubject,
            authority.accountId,
            authority.sessionId,
            authority.sessionGeneration,
            authority.authorityGeneration,
            issuedAt,
            expiresAt,
            issuedAt,
            issuedAt,
            issuedAt
          )
          .run();
        const row = await rowById(database, sessionId);
        if (row === null) return { status: 'rejected', reason: 'conflict' };
        await audit(database, row, 'created', 'accepted', 'session-created', issuedAt);
        return mutationFromRow(row, secrets);
      } catch (error) {
        if (missingSchema(error)) return { status: 'manual-required', reason: 'schema-missing' };
        log.logError('account browser session creation failed', getStackTrace(), {
          owner: 'account-identity-family-plan',
          boundary: 'browser-session-custody',
          result: 'blocked',
          reason: 'storage-error',
          redactionState: 'tokens-omitted',
        });
        throw error;
      }
    },

    async read(sessionToken, nowMs = Date.now()) {
      try {
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs);
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        if (missingSchema(error)) return { status: 'manual-required', reason: 'schema-missing' };
        throw error;
      }
    },

    async rotate(refreshToken, nowMs = Date.now()) {
      const now = nowIso(nowMs);
      try {
        const parsed = parseSessionCookie(refreshToken);
        if (refreshToken !== null && parsed === null) return { status: 'rejected', reason: 'malformed' };
        if (parsed === null) return { status: 'rejected', reason: 'missing' };
        const row = await rowById(database, parsed.sessionId);
        if (row === null) return { status: 'rejected', reason: 'invalid' };
        const suppliedDigest = await sha256Hex(refreshToken);
        if (row.status !== 'active') return { status: 'rejected', reason: 'revoked' };
        if (expired(row, nowMs)) return { status: 'rejected', reason: 'expired' };
        if (suppliedDigest !== row.refresh_token_digest) {
          await database.prepare(REVOKE_SQL).bind(now, now, row.session_id).run();
          await audit(database, row, 'replay-rejected', 'rejected', 'refresh-replay', now);
          return { status: 'rejected', reason: 'replay' };
        }
        const secrets = {
          sessionToken: sessionCookieValue(row.session_id, newOpaqueValue()),
          refreshToken: sessionCookieValue(row.session_id, newOpaqueValue()),
          csrfToken: sessionCookieValue(row.session_id, newOpaqueValue()),
        } satisfies BrowserSessionSecrets;
        const updated = await database
          .prepare(ROTATE_SQL)
          .bind(
            await sha256Hex(secrets.sessionToken),
            await sha256Hex(secrets.refreshToken),
            await sha256Hex(secrets.csrfToken),
            now,
            now,
            row.session_id,
            suppliedDigest,
            now
          )
          .run();
        if (changes(updated) !== 1) return { status: 'rejected', reason: 'conflict' };
        const current = await rowById(database, row.session_id);
        if (current === null) return { status: 'rejected', reason: 'conflict' };
        await audit(database, current, 'refreshed', 'accepted', 'refresh-rotated', now);
        return mutationFromRow(current, secrets);
      } catch (error) {
        if (missingSchema(error)) return { status: 'manual-required', reason: 'schema-missing' };
        throw error;
      }
    },

    async verifyCsrf(sessionToken, csrfToken, nowMs = Date.now()) {
      const session = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs);
      if (!('row' in session)) return false;
      const parsed = parseSessionCookie(csrfToken);
      if (parsed === null || parsed.sessionId !== session.row.session_id) return false;
      return (await sha256Hex(csrfToken!)) === session.row.csrf_token_digest;
    },

    async logout(sessionToken, nowMs = Date.now()) {
      const session = await this.read(sessionToken, nowMs);
      if (session.status === 'manual-required') return { status: 'manual-required', reason: session.reason };
      if (session.status === 'missing') return { status: 'rejected', reason: 'missing' };
      if (session.status === 'rejected') return { status: 'rejected', reason: session.reason };
      const now = nowIso(nowMs);
      const updated = await database.prepare(REVOKE_SQL).bind(now, now, session.row.session_id).run();
      if (changes(updated) !== 1) return { status: 'rejected', reason: 'conflict' };
      await audit(database, session.row, 'logout', 'accepted', 'session-logout', now);
      return {
        status: 'accepted',
        identity: session.identity,
        secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
      };
    },

    async revokeAll(provider, providerSubject, nowMs = Date.now()) {
      if (!safeSubject(providerSubject)) return { status: 'rejected', reason: 'invalid' };
      const now = nowIso(nowMs);
      const rows = await database
        .prepare(SELECT_BY_SUBJECT_SQL)
        .bind(provider, providerSubject)
        .all<BrowserSessionRow>();
      const updated = await database.prepare(REVOKE_ALL_SQL).bind(now, now, provider, providerSubject).run();
      for (const row of rows.results ?? [])
        await audit(database, row, 'global-revoke', 'accepted', 'global-revoke', now);
      return {
        status: 'accepted',
        identity: rows.results?.[0] ? sessionIdentity(rows.results[0]) : null,
        secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
      };
    },
  };
}
