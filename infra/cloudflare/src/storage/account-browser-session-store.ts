import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';
import type { D1Database } from '@cloudflare/workers-types';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import type { VerifiedAccountIdentityAuthorityCapability } from './account-identity-authority-store.js';
import {
  browserSessionRole,
  constantTimeEqual,
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

const ACCESS_LIFETIME_MS = 30 * 60 * 1000;
// Refresh rotation renews access only; this fixed family lifetime is never extended.
const REFRESH_LIFETIME_MS = 30 * 24 * 60 * 60 * 1000;
const log = Logger.instance;
log.register(import.meta.url);

export type BrowserSessionReadResult =
  | { status: 'active'; identity: BrowserSessionIdentity; row: BrowserSessionRow }
  | { status: 'missing' }
  | { status: 'rejected'; reason: 'malformed' | 'invalid' | 'expired' | 'revoked' | 'role-ineligible' }
  | { status: 'manual-required'; reason: 'binding-missing' | 'schema-missing' | 'd1-unavailable' };

export type BrowserSessionMutationResult =
  | { status: 'accepted'; identity: BrowserSessionIdentity | null; secrets: BrowserSessionSecrets }
  | {
      status: 'rejected';
      reason: 'missing' | 'malformed' | 'invalid' | 'expired' | 'revoked' | 'replay' | 'conflict' | 'role-ineligible';
    }
  | { status: 'manual-required'; reason: 'binding-missing' | 'schema-missing' | 'd1-unavailable' };

export interface BrowserSessionStore {
  create(
    authority: VerifiedAccountIdentityAuthorityCapability,
    nowMs?: number,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
  read(sessionToken: string | null, nowMs?: number): Promise<BrowserSessionReadResult>;
  readBinding(sessionToken: string | null, nowMs?: number): Promise<BrowserSessionReadResult>;
  readRefresh(refreshToken: string | null, nowMs?: number): Promise<BrowserSessionReadResult>;
  rotate(
    refreshToken: string | null,
    nowMs?: number,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
  verifyCsrf(sessionToken: string | null, csrfToken: string | null, nowMs?: number): Promise<boolean>;
  verifyRefreshCsrf(refreshToken: string | null, csrfToken: string | null, nowMs?: number): Promise<boolean>;
  logout(
    sessionToken: string | null,
    nowMs?: number,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
  revokeAll(
    provider: AccountIdentityProvider,
    providerSubject: string,
    nowMs?: number,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
}

const SELECT_BY_ID_SQL = `
SELECT session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
       provider, provider_subject, role, account_id, authority_session_id,
       authority_session_generation, authority_generation, issued_at,
       access_expires_at, refresh_expires_at, revoke_generation,
       refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
FROM ocentra_account_browser_sessions WHERE session_id = ? LIMIT 1`;

const SELECT_BY_SUBJECT_REVOKED_AT_SQL = `
SELECT session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
       provider, provider_subject, role, account_id, authority_session_id,
       authority_session_generation, authority_generation, issued_at,
       access_expires_at, refresh_expires_at, revoke_generation,
       refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
FROM ocentra_account_browser_sessions
WHERE provider = ? AND provider_subject = ? AND revoked_at = ?`;

const SELECT_FENCE_SQL = `
SELECT revoke_generation FROM ocentra_account_browser_session_fences
WHERE provider = ? AND provider_subject = ? LIMIT 1`;

const ENSURE_FENCE_SQL = `
INSERT INTO ocentra_account_browser_session_fences
  (provider, provider_subject, revoke_generation, updated_at)
VALUES (?, ?, 1, ?)
ON CONFLICT(provider, provider_subject) DO NOTHING`;

const INSERT_SQL = `
INSERT INTO ocentra_account_browser_sessions (
  session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
  provider, provider_subject, role, account_id, authority_session_id,
  authority_session_generation, authority_generation, issued_at,
  access_expires_at, refresh_expires_at, revoke_generation,
  refresh_generation, status, last_seen_at, revoked_at, created_at, updated_at
)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, fence.revoke_generation,
       1, 'active', ?, NULL, ?, ?
FROM ocentra_account_browser_session_fences AS fence
WHERE fence.provider = ? AND fence.provider_subject = ?`;

const REVOKE_SQL = `
UPDATE ocentra_account_browser_sessions
SET status = 'revoked', revoked_at = ?, updated_at = ?
WHERE session_id = ? AND status = 'active'`;

const ADVANCE_FENCE_SQL = `
UPDATE ocentra_account_browser_session_fences
SET revoke_generation = revoke_generation + 1, updated_at = ?
WHERE provider = ? AND provider_subject = ?`;

const REVOKE_ALL_SQL = `
UPDATE ocentra_account_browser_sessions
SET status = 'revoked', revoked_at = ?, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND status = 'active'`;

const ROTATE_SQL = `
UPDATE ocentra_account_browser_sessions
SET session_token_digest = ?, refresh_token_digest = ?, csrf_token_digest = ?,
    access_expires_at = ?, refresh_generation = refresh_generation + 1,
    last_seen_at = ?, updated_at = ?
WHERE session_id = ? AND refresh_token_digest = ? AND status = 'active'
  AND refresh_expires_at > ?
  AND revoke_generation = (
    SELECT revoke_generation FROM ocentra_account_browser_session_fences
    WHERE provider = ocentra_account_browser_sessions.provider
      AND provider_subject = ocentra_account_browser_sessions.provider_subject
  )`;

const AUDIT_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`;

interface FenceRow {
  revoke_generation: number;
}

function missingSchema(error: unknown): boolean {
  const message = String(error instanceof Error ? error.message : error).toLowerCase();
  return message.includes('no such table') || message.includes('no such column') || message.includes('does not exist');
}

function changes(result: { meta?: { changes?: number } }): number {
  return typeof result.meta?.changes === 'number' ? result.meta.changes : 0;
}

function safeSubject(value: string): boolean {
  return value.length > 0 && value.length <= 256 && !/[\u0000-\u001f\u007f]/.test(value);
}

function safeCorrelation(value: string | undefined, action: string): string {
  if (value && /^[A-Za-z0-9._:-]{1,128}$/.test(value)) return value;
  return `${action}-${newSessionId().slice(0, 16)}`;
}

function manualStorageFailure(error: unknown): {
  status: 'manual-required';
  reason: 'schema-missing' | 'd1-unavailable';
} {
  log.logWarn('account browser session storage unavailable', getStackTrace(), {
    owner: 'account-identity-family-plan',
    boundary: 'browser-session-custody',
    result: 'blocked',
    reason: missingSchema(error) ? 'schema-missing' : 'd1-unavailable',
    redactionState: 'tokens-and-storage-error-omitted',
  });
  return { status: 'manual-required', reason: missingSchema(error) ? 'schema-missing' : 'd1-unavailable' };
}

async function audit(
  database: D1Database,
  row: Pick<BrowserSessionRow, 'session_id' | 'provider' | 'provider_subject'>,
  action: 'created' | 'refreshed' | 'logout' | 'global-revoke' | 'replay-rejected',
  result: 'accepted' | 'rejected',
  reason: string,
  requestCorrelation: string,
  now: string
): Promise<void> {
  const sessionRefDigest = await sha256Hex(`ocentra/account-browser-session/audit/session:${row.session_id}`);
  const actorRefDigest = await sha256Hex(
    `ocentra/account-browser-session/audit/actor:${row.provider}:${row.provider_subject}`
  );
  await database
    .prepare(AUDIT_SQL)
    .bind(
      newSessionId(),
      sessionRefDigest,
      row.provider,
      actorRefDigest,
      action,
      result,
      reason,
      requestCorrelation,
      now
    )
    .run();
}

function expired(row: BrowserSessionRow, nowMs: number, credential: 'access' | 'refresh'): boolean {
  const expiry = Date.parse(credential === 'access' ? row.access_expires_at : row.refresh_expires_at);
  return !Number.isFinite(expiry) || expiry <= nowMs;
}

async function rowById(database: D1Database, sessionId: string): Promise<BrowserSessionRow | null> {
  return database.prepare(SELECT_BY_ID_SQL).bind(sessionId).first<BrowserSessionRow>();
}

async function isCurrentFence(database: D1Database, row: BrowserSessionRow): Promise<boolean> {
  const fence = await database.prepare(SELECT_FENCE_SQL).bind(row.provider, row.provider_subject).first<FenceRow>();
  return fence !== null && fence.revoke_generation === row.revoke_generation;
}

function mutationFromRow(row: BrowserSessionRow, secrets: BrowserSessionSecrets): BrowserSessionMutationResult {
  return { status: 'accepted', identity: sessionIdentity(row), secrets };
}

async function readWithSecret(
  database: D1Database,
  value: string | null,
  digestField: 'session_token_digest' | 'refresh_token_digest',
  nowMs: number,
  credential: 'access' | 'refresh' | 'binding'
): Promise<{ row: BrowserSessionRow; digest: string } | BrowserSessionReadResult> {
  const parsed = parseSessionCookie(value);
  if (value !== null && parsed === null) return { status: 'rejected', reason: 'malformed' };
  if (parsed === null) return { status: 'missing' };
  const row = await rowById(database, parsed.sessionId);
  if (row === null) return { status: 'rejected', reason: 'invalid' };
  const digest = await sha256Hex(value!);
  if (!isDigest(digest) || !constantTimeEqual(digest, row[digestField])) {
    return { status: 'rejected', reason: 'invalid' };
  }
  if (row.status !== 'active' || !(await isCurrentFence(database, row))) {
    return { status: 'rejected', reason: 'revoked' };
  }
  if (credential !== 'binding' && expired(row, nowMs, credential)) {
    return { status: 'rejected', reason: 'expired' };
  }
  if (browserSessionRole(row.role) === null) {
    return { status: 'rejected', reason: 'role-ineligible' };
  }
  return { row, digest };
}

export function createBrowserSessionStore(database: D1Database | undefined): BrowserSessionStore {
  const unavailable = (): BrowserSessionMutationResult => ({ status: 'manual-required', reason: 'binding-missing' });
  if (database === undefined) {
    return {
      create: async () => unavailable(),
      read: async () => ({ status: 'manual-required', reason: 'binding-missing' }),
      readBinding: async () => ({ status: 'manual-required', reason: 'binding-missing' }),
      readRefresh: async () => ({ status: 'manual-required', reason: 'binding-missing' }),
      rotate: async () => unavailable(),
      verifyCsrf: async () => false,
      verifyRefreshCsrf: async () => false,
      logout: async () => unavailable(),
      revokeAll: async () => unavailable(),
    };
  }

  return {
    async create(authority, nowMs = Date.now(), requestCorrelation) {
      if (browserSessionRole(authority.role) === null) return { status: 'rejected', reason: 'role-ineligible' };
      const issuedAt = nowIso(nowMs);
      const accessExpiresAt = nowIso(nowMs + ACCESS_LIFETIME_MS);
      const refreshExpiresAt = nowIso(nowMs + REFRESH_LIFETIME_MS);
      const sessionId = newSessionId();
      const secrets = {
        sessionToken: sessionCookieValue(sessionId, newOpaqueValue()),
        refreshToken: sessionCookieValue(sessionId, newOpaqueValue()),
        csrfToken: sessionCookieValue(sessionId, newOpaqueValue()),
      } satisfies BrowserSessionSecrets;
      try {
        await database.batch([
          database.prepare(ENSURE_FENCE_SQL).bind(authority.provider, authority.providerSubject, issuedAt),
          database
            .prepare(INSERT_SQL)
            .bind(
              sessionId,
              await sha256Hex(secrets.sessionToken),
              await sha256Hex(secrets.refreshToken),
              await sha256Hex(secrets.csrfToken),
              authority.provider,
              authority.providerSubject,
              authority.role,
              authority.accountId,
              authority.sessionId,
              authority.sessionGeneration,
              authority.authorityGeneration,
              issuedAt,
              accessExpiresAt,
              refreshExpiresAt,
              issuedAt,
              issuedAt,
              issuedAt,
              authority.provider,
              authority.providerSubject
            ),
        ]);
        const row = await rowById(database, sessionId);
        if (row === null) return { status: 'rejected', reason: 'conflict' };
        await audit(
          database,
          row,
          'created',
          'accepted',
          'session-created',
          safeCorrelation(requestCorrelation, 'session-create'),
          issuedAt
        );
        return mutationFromRow(row, secrets);
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async read(sessionToken, nowMs = Date.now()) {
      try {
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'access');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async readBinding(sessionToken, nowMs = Date.now()) {
      try {
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'binding');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async readRefresh(refreshToken, nowMs = Date.now()) {
      try {
        const result = await readWithSecret(database, refreshToken, 'refresh_token_digest', nowMs, 'refresh');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async rotate(refreshToken, nowMs = Date.now(), requestCorrelation) {
      const now = nowIso(nowMs);
      try {
        const session = await readWithSecret(database, refreshToken, 'refresh_token_digest', nowMs, 'refresh');
        if (!('row' in session)) {
          if (session.status === 'manual-required') return session;
          return { status: 'rejected', reason: session.reason === 'missing' ? 'missing' : session.reason };
        }
        const row = session.row;
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
            nowIso(nowMs + ACCESS_LIFETIME_MS),
            now,
            now,
            row.session_id,
            session.digest,
            now
          )
          .run();
        if (changes(updated) !== 1) {
          await audit(
            database,
            row,
            'replay-rejected',
            'rejected',
            'refresh-replay',
            safeCorrelation(requestCorrelation, 'session-refresh'),
            now
          );
          return { status: 'rejected', reason: 'replay' };
        }
        const current = await rowById(database, row.session_id);
        if (current === null) return { status: 'rejected', reason: 'conflict' };
        await audit(
          database,
          current,
          'refreshed',
          'accepted',
          'refresh-rotated',
          safeCorrelation(requestCorrelation, 'session-refresh'),
          now
        );
        return mutationFromRow(current, secrets);
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async verifyCsrf(sessionToken, csrfToken, nowMs = Date.now()) {
      try {
        const session = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'access');
        if (!('row' in session)) return false;
        const parsed = parseSessionCookie(csrfToken);
        if (parsed === null || parsed.sessionId !== session.row.session_id) return false;
        const digest = await sha256Hex(csrfToken!);
        return constantTimeEqual(digest, session.row.csrf_token_digest);
      } catch (error) {
        manualStorageFailure(error);
        return false;
      }
    },

    async verifyRefreshCsrf(refreshToken, csrfToken, nowMs = Date.now()) {
      try {
        const session = await readWithSecret(database, refreshToken, 'refresh_token_digest', nowMs, 'refresh');
        if (!('row' in session)) return false;
        const parsed = parseSessionCookie(csrfToken);
        if (parsed === null || parsed.sessionId !== session.row.session_id) return false;
        const digest = await sha256Hex(csrfToken!);
        return constantTimeEqual(digest, session.row.csrf_token_digest);
      } catch (error) {
        manualStorageFailure(error);
        return false;
      }
    },

    async logout(sessionToken, nowMs = Date.now(), requestCorrelation) {
      const session = await this.read(sessionToken, nowMs);
      if (session.status === 'manual-required') return session;
      if (session.status === 'missing') return { status: 'rejected', reason: 'missing' };
      if (session.status === 'rejected') return { status: 'rejected', reason: session.reason };
      try {
        const now = nowIso(nowMs);
        const updated = await database.prepare(REVOKE_SQL).bind(now, now, session.row.session_id).run();
        if (changes(updated) !== 1) return { status: 'rejected', reason: 'conflict' };
        await audit(
          database,
          session.row,
          'logout',
          'accepted',
          'session-logout',
          safeCorrelation(requestCorrelation, 'session-logout'),
          now
        );
        return {
          status: 'accepted',
          identity: session.identity,
          secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
        };
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async revokeAll(provider, providerSubject, nowMs = Date.now(), requestCorrelation) {
      if (!safeSubject(providerSubject) || (provider !== 'authjs' && provider !== 'firebase')) {
        return { status: 'rejected', reason: 'invalid' };
      }
      const now = nowIso(nowMs);
      try {
        await database.batch([
          database.prepare(ENSURE_FENCE_SQL).bind(provider, providerSubject, now),
          database.prepare(ADVANCE_FENCE_SQL).bind(now, provider, providerSubject),
          database.prepare(REVOKE_ALL_SQL).bind(now, now, provider, providerSubject),
        ]);
        const rows = await database
          .prepare(SELECT_BY_SUBJECT_REVOKED_AT_SQL)
          .bind(provider, providerSubject, now)
          .all<BrowserSessionRow>();
        for (const row of rows.results ?? []) {
          await audit(
            database,
            row,
            'global-revoke',
            'accepted',
            'global-revoke',
            safeCorrelation(requestCorrelation, 'session-revoke'),
            now
          );
        }
        return {
          status: 'accepted',
          identity: rows.results?.[0] ? sessionIdentity(rows.results[0]) : null,
          secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
        };
      } catch (error) {
        return manualStorageFailure(error);
      }
    },
  };
}
