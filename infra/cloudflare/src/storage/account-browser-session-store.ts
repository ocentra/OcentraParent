import type { D1Database } from '@cloudflare/workers-types';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import {
  isVerifiedAccountIdentityAuthorityCapability,
  type VerifiedAccountIdentityAuthorityCapability,
} from './account-identity-authority-store.js';
import {
  browserSessionRole,
  constantTimeEqual,
  isBrowserSessionRow,
  isBrowserSessionTimestamp,
  isDigest,
  isSessionId,
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
const SESSION_CUSTODY_SCHEMA_VERSION = 7;
const log = Logger.instance;
log.register(import.meta.url);

const CURRENT_AUTHORITY_CONDITIONS_SQL = `
  authority.mapping_status = 'active'
  AND authority.account_state = 'active'
  AND authority.membership_state = 'active'
  AND authority.account_id = session.account_id
  AND authority.household_id = session.household_id
  AND authority.member_id = session.member_id
  AND authority.role = session.role
  AND authority.device_id = session.device_id
  AND authority.child_profile_id = session.child_profile_id
  AND authority.child_device_id = session.child_device_id
  AND authority.device_trust_state = 'trusted'
  AND authority.session_freshness_state = 'fresh'
  AND authority.session_id = session.authority_session_id
  AND authority.session_generation = session.authority_session_generation
  AND authority.session_expires_at > ?
  AND authority.authority_generation = session.authority_generation
  AND authority.pairing_state = 'paired'
  AND authority.install_state = 'installed'
  AND authority.lifecycle_state = 'active'
  AND authority.revocation_state = 'active'
  AND authority.support_receipt_id IS session.support_receipt_id
  AND authority.support_provider_subject IS session.support_provider_subject
  AND authority.support_account_id IS session.support_account_id
  AND authority.support_member_id IS session.support_member_id
  AND authority.support_household_id IS session.support_household_id
  AND authority.support_device_id IS session.support_device_id
  AND authority.support_child_profile_id IS session.support_child_profile_id
  AND authority.support_child_device_id IS session.support_child_device_id
  AND authority.support_scope IS session.support_scope
  AND authority.support_issuer IS session.support_issuer
  AND authority.support_issued_at IS session.support_issued_at
  AND authority.support_expires_at IS session.support_expires_at
  AND authority.support_revocation_state IS session.support_revocation_state
  AND authority.support_audit_identity IS session.support_audit_identity
  AND (
    (authority.role <> 'support-admin' AND authority.support_receipt_id IS NULL)
    OR (
      authority.support_receipt_id IS NOT NULL
      AND authority.support_provider_subject = authority.provider_subject
      AND authority.support_account_id = authority.account_id
      AND authority.support_member_id = authority.member_id
      AND authority.support_household_id = authority.household_id
      AND authority.support_device_id = authority.device_id
      AND authority.support_child_profile_id = authority.child_profile_id
      AND authority.support_child_device_id = authority.child_device_id
      AND authority.support_revocation_state = 'active'
      AND authority.support_issued_at <= ?
      AND authority.support_expires_at > ?
    )
  )`;

export type BrowserSessionReadResult =
  | { status: 'active'; identity: BrowserSessionIdentity; row: BrowserSessionRow }
  | { status: 'missing' }
  | { status: 'rejected'; reason: 'malformed' | 'invalid' | 'expired' | 'revoked' | 'role-ineligible' }
  | { status: 'manual-required'; reason: 'binding-missing' | 'schema-missing' | 'schema-invalid' | 'd1-unavailable' };

export type BrowserSessionMutationResult =
  | { status: 'accepted'; identity: BrowserSessionIdentity | null; secrets: BrowserSessionSecrets }
  | {
      status: 'rejected';
      reason: 'missing' | 'malformed' | 'invalid' | 'expired' | 'revoked' | 'replay' | 'conflict' | 'role-ineligible';
    }
  | {
      status: 'manual-required';
      reason: 'binding-missing' | 'schema-missing' | 'schema-invalid' | 'd1-unavailable';
    };

export interface BrowserSessionStore {
  create(
    authority: VerifiedAccountIdentityAuthorityCapability,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
  read(sessionToken: string | null): Promise<BrowserSessionReadResult>;
  readBinding(sessionToken: string | null): Promise<BrowserSessionReadResult>;
  readRefresh(refreshToken: string | null): Promise<BrowserSessionReadResult>;
  rotate(refreshToken: string | null, requestCorrelation?: string): Promise<BrowserSessionMutationResult>;
  verifyCsrf(sessionToken: string | null, csrfToken: string | null): Promise<boolean>;
  verifyRefreshCsrf(refreshToken: string | null, csrfToken: string | null): Promise<boolean>;
  logoutRefresh(refreshToken: string | null, requestCorrelation?: string): Promise<BrowserSessionMutationResult>;
  logout(sessionToken: string | null, requestCorrelation?: string): Promise<BrowserSessionMutationResult>;
  revokeAll(
    authority: VerifiedAccountIdentityAuthorityCapability,
    requestCorrelation?: string
  ): Promise<BrowserSessionMutationResult>;
}

const SELECT_CURRENT_BY_ID_SQL = `
SELECT session.session_id, session.session_token_digest, session.refresh_token_digest, session.csrf_token_digest,
       session.provider, session.provider_subject, session.role, session.account_id,
       session.household_id, session.member_id, session.device_id, session.child_profile_id, session.child_device_id,
       session.authority_session_id, session.authority_session_generation, session.authority_generation,
       session.support_receipt_id, session.support_provider_subject, session.support_account_id,
       session.support_member_id, session.support_household_id, session.support_device_id,
       session.support_child_profile_id, session.support_child_device_id, session.support_scope,
       session.support_issuer, session.support_issued_at, session.support_expires_at,
       session.support_revocation_state, session.support_audit_identity, session.issued_at,
       session.access_expires_at, session.refresh_expires_at, session.revoke_generation,
       session.refresh_generation, session.status, session.last_seen_at, session.revoked_at,
       session.created_at, session.updated_at
FROM ocentra_account_browser_sessions AS session
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = session.provider AND authority.provider_subject = session.provider_subject
WHERE session.session_id = ?
  AND ${CURRENT_AUTHORITY_CONDITIONS_SQL}
LIMIT 1`;

const SELECT_ACTIVE_BY_SUBJECT_SQL = `
SELECT session.session_id, session.session_token_digest, session.refresh_token_digest, session.csrf_token_digest,
       session.provider, session.provider_subject, session.role, session.account_id,
       session.household_id, session.member_id, session.device_id, session.child_profile_id, session.child_device_id,
       session.authority_session_id, session.authority_session_generation, session.authority_generation,
       session.support_receipt_id, session.support_provider_subject, session.support_account_id,
       session.support_member_id, session.support_household_id, session.support_device_id,
       session.support_child_profile_id, session.support_child_device_id, session.support_scope,
       session.support_issuer, session.support_issued_at, session.support_expires_at,
       session.support_revocation_state, session.support_audit_identity, session.issued_at,
       session.access_expires_at, session.refresh_expires_at, session.revoke_generation,
       session.refresh_generation, session.status, session.last_seen_at, session.revoked_at,
       session.created_at, session.updated_at
FROM ocentra_account_browser_sessions AS session
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = session.provider AND authority.provider_subject = session.provider_subject
WHERE session.provider = ? AND session.provider_subject = ?
  AND session.status = 'active' AND session.revoke_generation = ?
  AND ${CURRENT_AUTHORITY_CONDITIONS_SQL}`;

const SELECT_FENCE_SQL = `
SELECT revoke_generation, updated_at FROM ocentra_account_browser_session_fences
WHERE provider = ? AND provider_subject = ? LIMIT 1`;

const SELECT_CUSTODY_SCHEMA_SQL = `
SELECT schema_name, schema_version, applied_at
FROM ocentra_account_browser_session_schema
WHERE schema_name = 'browser-session-custody' LIMIT 1`;

const SELECT_CUSTODY_SCHEMA_DDL_SQL = `
SELECT sql FROM sqlite_schema
WHERE type = 'table' AND name = 'ocentra_account_browser_session_schema' LIMIT 1`;

const ENSURE_FENCE_SQL = `
INSERT INTO ocentra_account_browser_session_fences
  (provider, provider_subject, revoke_generation, updated_at)
VALUES (?, ?, 1, ?)
ON CONFLICT(provider, provider_subject) DO NOTHING`;

const INSERT_SQL = `
INSERT INTO ocentra_account_browser_sessions (
  session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
  provider, provider_subject, role, account_id, household_id, member_id, device_id,
  child_profile_id, child_device_id, authority_session_id, authority_session_generation,
  authority_generation, support_receipt_id, support_provider_subject, support_account_id,
  support_member_id, support_household_id, support_device_id, support_child_profile_id,
  support_child_device_id, support_scope, support_issuer, support_issued_at,
  support_expires_at, support_revocation_state, support_audit_identity, issued_at,
  access_expires_at, refresh_expires_at, revoke_generation, refresh_generation,
  status, last_seen_at, revoked_at, created_at, updated_at
)
SELECT ?, ?, ?, ?,
       authority.provider, authority.provider_subject, authority.role, authority.account_id,
       authority.household_id, authority.member_id, authority.device_id,
       authority.child_profile_id, authority.child_device_id, authority.session_id,
       authority.session_generation, authority.authority_generation,
       authority.support_receipt_id, authority.support_provider_subject, authority.support_account_id,
       authority.support_member_id, authority.support_household_id, authority.support_device_id,
       authority.support_child_profile_id, authority.support_child_device_id, authority.support_scope,
       authority.support_issuer, authority.support_issued_at, authority.support_expires_at,
       authority.support_revocation_state, authority.support_audit_identity,
       ?, ?, ?, fence.revoke_generation, 1, 'active', ?, NULL, ?, ?
FROM ocentra_account_browser_session_fences AS fence
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = ? AND authority.provider_subject = ?
WHERE fence.provider = authority.provider AND fence.provider_subject = authority.provider_subject
  AND authority.mapping_status = 'active'
  AND authority.account_state = 'active'
  AND authority.membership_state = 'active'
  AND authority.account_id = ?
  AND authority.household_id = ?
  AND authority.member_id = ?
  AND authority.role = ?
  AND authority.device_id = ?
  AND authority.child_profile_id = ?
  AND authority.child_device_id = ?
  AND authority.device_trust_state = 'trusted'
  AND authority.session_freshness_state = 'fresh'
  AND authority.session_id = ?
  AND authority.session_generation = ?
  AND authority.session_expires_at > ?
  AND authority.authority_generation = ?
  AND authority.pairing_state = 'paired'
  AND authority.install_state = 'installed'
  AND authority.lifecycle_state = 'active'
  AND authority.revocation_state = 'active'
  AND authority.support_receipt_id IS ?
  AND authority.support_provider_subject IS ?
  AND authority.support_account_id IS ?
  AND authority.support_member_id IS ?
  AND authority.support_household_id IS ?
  AND authority.support_device_id IS ?
  AND authority.support_child_profile_id IS ?
  AND authority.support_child_device_id IS ?
  AND authority.support_scope IS ?
  AND authority.support_issuer IS ?
  AND authority.support_issued_at IS ?
  AND authority.support_expires_at IS ?
  AND authority.support_revocation_state IS ?
  AND authority.support_audit_identity IS ?
  AND (
    (authority.role <> 'support-admin' AND authority.support_receipt_id IS NULL)
    OR (
      authority.support_receipt_id IS NOT NULL
      AND authority.support_provider_subject = authority.provider_subject
      AND authority.support_account_id = authority.account_id
      AND authority.support_member_id = authority.member_id
      AND authority.support_household_id = authority.household_id
      AND authority.support_device_id = authority.device_id
      AND authority.support_child_profile_id = authority.child_profile_id
      AND authority.support_child_device_id = authority.child_device_id
      AND authority.support_revocation_state = 'active'
      AND authority.support_issued_at <= ?
      AND authority.support_expires_at > ?
    )
  )`;

const REVOKE_SQL = `
UPDATE ocentra_account_browser_sessions AS session
SET status = 'revoked', revoked_at = ?, updated_at = ?
WHERE session.session_id = ? AND session.status = 'active'
  AND EXISTS (
    SELECT 1 FROM ocentra_account_identity_current_authority AS authority
    WHERE authority.provider = session.provider AND authority.provider_subject = session.provider_subject
      AND ${CURRENT_AUTHORITY_CONDITIONS_SQL}
  )`;

const ADVANCE_FENCE_SQL = `
UPDATE ocentra_account_browser_session_fences
SET revoke_generation = revoke_generation + 1, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND revoke_generation = ?
  AND EXISTS (
    SELECT 1 FROM ocentra_account_identity_current_authority AS authority
    WHERE authority.provider = ? AND authority.provider_subject = ?
      AND authority.mapping_status = 'active'
      AND authority.account_state = 'active'
      AND authority.membership_state = 'active'
      AND authority.account_id = ?
      AND authority.household_id = ?
      AND authority.member_id = ?
      AND authority.role = ?
      AND authority.device_id = ?
      AND authority.child_profile_id = ?
      AND authority.child_device_id = ?
      AND authority.device_trust_state = 'trusted'
      AND authority.session_freshness_state = 'fresh'
      AND authority.session_id = ?
      AND authority.session_generation = ?
      AND authority.session_expires_at > ?
      AND authority.authority_generation = ?
      AND authority.pairing_state = 'paired'
      AND authority.install_state = 'installed'
      AND authority.lifecycle_state = 'active'
      AND authority.revocation_state = 'active'
      AND authority.support_receipt_id IS ?
      AND authority.support_provider_subject IS ?
      AND authority.support_account_id IS ?
      AND authority.support_member_id IS ?
      AND authority.support_household_id IS ?
      AND authority.support_device_id IS ?
      AND authority.support_child_profile_id IS ?
      AND authority.support_child_device_id IS ?
      AND authority.support_scope IS ?
      AND authority.support_issuer IS ?
      AND authority.support_issued_at IS ?
      AND authority.support_expires_at IS ?
      AND authority.support_revocation_state IS ?
      AND authority.support_audit_identity IS ?
      AND (
        (authority.role <> 'support-admin' AND authority.support_receipt_id IS NULL)
        OR (
          authority.support_receipt_id IS NOT NULL
          AND authority.support_provider_subject = authority.provider_subject
          AND authority.support_account_id = authority.account_id
          AND authority.support_member_id = authority.member_id
          AND authority.support_household_id = authority.household_id
          AND authority.support_device_id = authority.device_id
          AND authority.support_child_profile_id = authority.child_profile_id
          AND authority.support_child_device_id = authority.child_device_id
          AND authority.support_revocation_state = 'active'
          AND authority.support_issued_at <= ?
          AND authority.support_expires_at > ?
        )
      )
  )`;

const REVOKE_ALL_SQL = `
UPDATE ocentra_account_browser_sessions
SET status = 'revoked', revoked_at = ?, updated_at = ?, revoke_generation = ?
WHERE provider = ? AND provider_subject = ? AND status = 'active' AND revoke_generation = ?
  AND EXISTS (
    SELECT 1 FROM ocentra_account_browser_session_fences AS fence
    WHERE fence.provider = ocentra_account_browser_sessions.provider
      AND fence.provider_subject = ocentra_account_browser_sessions.provider_subject
      AND fence.revoke_generation = ?
  )`;

const ROTATE_SQL = `
UPDATE ocentra_account_browser_sessions AS session
SET session_token_digest = ?, refresh_token_digest = ?, csrf_token_digest = ?,
    access_expires_at = ?, refresh_generation = refresh_generation + 1,
    last_seen_at = ?, updated_at = ?
WHERE session.session_id = ? AND session.refresh_token_digest = ? AND session.status = 'active'
  AND session.refresh_expires_at > ?
  AND session.refresh_generation = ?
  AND NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_session_consumed_refresh
    WHERE refresh_token_digest = ?
  )
  AND revoke_generation = (
    SELECT revoke_generation FROM ocentra_account_browser_session_fences
    WHERE provider = session.provider
      AND provider_subject = session.provider_subject
  )
  AND EXISTS (
    SELECT 1 FROM ocentra_account_identity_current_authority AS authority
    WHERE authority.provider = session.provider AND authority.provider_subject = session.provider_subject
      AND ${CURRENT_AUTHORITY_CONDITIONS_SQL}
  )`;

const CONSUME_REFRESH_SQL = `
-- This statement follows ROTATE_SQL in one sequential D1 batch. The INSERT
-- captures the old digest at refresh_generation - 1, while both the source
-- row and the fallback probe match the post-rotation digest/generation/clock.
INSERT INTO ocentra_account_browser_session_consumed_refresh
  (refresh_token_digest, session_id, refresh_generation, consumed_at)
SELECT ?, ?, refresh_generation - 1, ?
FROM ocentra_account_browser_sessions
WHERE session_id = ? AND refresh_token_digest = ? AND status = 'active'
  AND refresh_generation = ? AND updated_at = ?
UNION ALL
SELECT NULL, NULL, NULL, NULL
WHERE NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_sessions
    WHERE session_id = ? AND refresh_token_digest = ? AND status = 'active'
      AND refresh_generation = ? AND updated_at = ?
  )`;

const SELECT_CONSUMED_REFRESH_SQL = `
SELECT refresh_token_digest, session_id, refresh_generation, consumed_at
FROM ocentra_account_browser_session_consumed_refresh
WHERE refresh_token_digest = ? LIMIT 1`;

const AUDIT_ACTIVE_SESSION_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?
FROM ocentra_account_browser_sessions
WHERE session_id = ? AND status = 'active' AND updated_at = ? AND refresh_generation = ?
UNION ALL
SELECT NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
WHERE NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_sessions
    WHERE session_id = ? AND status = 'active' AND updated_at = ? AND refresh_generation = ?
  )`;

const AUDIT_ROTATED_SESSION_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?
FROM ocentra_account_browser_sessions
WHERE session_id = ? AND status = 'active' AND updated_at = ? AND refresh_generation = ?
  AND refresh_token_digest = ?
UNION ALL
SELECT NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
WHERE NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_sessions
    WHERE session_id = ? AND status = 'active' AND updated_at = ? AND refresh_generation = ?
      AND refresh_token_digest = ?
  )`;

const AUDIT_REVOKED_SESSION_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?
FROM ocentra_account_browser_sessions
WHERE session_id = ? AND status = 'revoked' AND revoked_at = ? AND updated_at = ?
  AND revoke_generation = ?
  AND EXISTS (
    SELECT 1 FROM ocentra_account_browser_session_fences AS fence
    WHERE fence.provider = ocentra_account_browser_sessions.provider
      AND fence.provider_subject = ocentra_account_browser_sessions.provider_subject
      AND fence.revoke_generation = ?
  )
UNION ALL
SELECT NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
WHERE NOT EXISTS (
  SELECT 1 FROM ocentra_account_browser_sessions
  WHERE session_id = ? AND status = 'revoked' AND revoked_at = ? AND updated_at = ?
    AND revoke_generation = ?
    AND EXISTS (
      SELECT 1 FROM ocentra_account_browser_session_fences AS fence
      WHERE fence.provider = ocentra_account_browser_sessions.provider
        AND fence.provider_subject = ocentra_account_browser_sessions.provider_subject
        AND fence.revoke_generation = ?
    )
)`;

const AUDIT_REVOKED_SINGLE_SESSION_SQL = `
INSERT INTO ocentra_account_browser_session_audit
  (audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?
FROM ocentra_account_browser_sessions
WHERE session_id = ? AND status = 'revoked' AND revoked_at = ? AND updated_at = ?
  AND revoke_generation = ?
UNION ALL
SELECT NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
WHERE NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_sessions
    WHERE session_id = ? AND status = 'revoked' AND revoked_at = ? AND updated_at = ?
      AND revoke_generation = ?
  )`;

const REVOKE_OUTCOME_SQL = `
INSERT INTO ocentra_account_browser_session_revoke_outcomes
  (outcome_id, provider, scope_ref_digest, actor_ref_digest, action, result, reason,
   revoke_generation, correlation_id, occurred_at)
SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
FROM ocentra_account_browser_session_fences
WHERE changes() = 1 AND provider = ? AND provider_subject = ? AND revoke_generation = ?
UNION ALL
SELECT NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
WHERE changes() <> 1 OR NOT EXISTS (
    SELECT 1 FROM ocentra_account_browser_session_fences
    WHERE provider = ? AND provider_subject = ? AND revoke_generation = ?
  )`;

interface FenceRow {
  revoke_generation: number;
  updated_at: string;
}

interface ConsumedRefreshRow {
  refresh_token_digest: string;
  session_id: string;
  refresh_generation: number;
  consumed_at: string;
}

class InvalidBrowserSessionRowError extends Error {
  constructor(table: string) {
    super(`invalid ${table} row`);
    this.name = 'InvalidBrowserSessionRowError';
  }
}

class MissingBrowserSessionSchemaError extends Error {
  constructor() {
    super('browser session custody schema is not migrated');
    this.name = 'MissingBrowserSessionSchemaError';
  }
}

class InvalidBrowserSessionSchemaError extends Error {
  constructor() {
    super('browser session custody schema version is invalid');
    this.name = 'InvalidBrowserSessionSchemaError';
  }
}

interface CurrentAuthorityBinding {
  provider: string;
  providerSubject: string;
  accountId: string;
  householdId: string;
  memberId: string;
  role: string;
  deviceId: string;
  childProfileId: string;
  childDeviceId: string;
  authoritySessionId: string;
  authoritySessionGeneration: number;
  authorityGeneration: number;
  supportReceiptId: string | null;
  supportProviderSubject: string | null;
  supportAccountId: string | null;
  supportMemberId: string | null;
  supportHouseholdId: string | null;
  supportDeviceId: string | null;
  supportChildProfileId: string | null;
  supportChildDeviceId: string | null;
  supportScope: string | null;
  supportIssuer: string | null;
  supportIssuedAt: string | null;
  supportExpiresAt: string | null;
  supportRevocationState: string | null;
  supportAuditIdentity: string | null;
}

function missingSchema(error: unknown): boolean {
  if (error instanceof MissingBrowserSessionSchemaError) return true;
  const message = String(error instanceof Error ? error.message : error).toLowerCase();
  return message.includes('no such table') || message.includes('no such column') || message.includes('does not exist');
}

function invalidSchema(error: unknown): boolean {
  return error instanceof InvalidBrowserSessionRowError || error instanceof InvalidBrowserSessionSchemaError;
}

function changes(result: { meta?: { changes?: number } }): number {
  return typeof result.meta?.changes === 'number' ? result.meta.changes : 0;
}

function decodeSessionRow(value: unknown): BrowserSessionRow | null {
  if (value === null) return null;
  if (!isBrowserSessionRow(value)) throw new InvalidBrowserSessionRowError('browser session');
  return value;
}

function decodeFenceRow(value: unknown): FenceRow | null {
  if (value === null) return null;
  if (
    typeof value !== 'object' ||
    Array.isArray(value) ||
    typeof (value as { revoke_generation?: unknown }).revoke_generation !== 'number' ||
    !Number.isSafeInteger((value as { revoke_generation: number }).revoke_generation) ||
    (value as { revoke_generation: number }).revoke_generation <= 0 ||
    !isBrowserSessionTimestamp((value as { updated_at?: unknown }).updated_at)
  ) {
    throw new InvalidBrowserSessionRowError('browser session fence');
  }
  return value as FenceRow;
}

function decodeConsumedRefreshRow(value: unknown): ConsumedRefreshRow | null {
  if (value === null) return null;
  const row = value as Record<string, unknown>;
  if (
    typeof value !== 'object' ||
    Array.isArray(value) ||
    typeof row.refresh_token_digest !== 'string' ||
    !isDigest(row.refresh_token_digest) ||
    !isSessionId(row.session_id) ||
    typeof row.refresh_generation !== 'number' ||
    !Number.isSafeInteger(row.refresh_generation) ||
    row.refresh_generation <= 0 ||
    typeof row.consumed_at !== 'string' ||
    !isBrowserSessionTimestamp(row.consumed_at)
  ) {
    throw new InvalidBrowserSessionRowError('consumed refresh');
  }
  return value as ConsumedRefreshRow;
}

function safeCorrelation(value: string | undefined, action: string): string {
  if (value && /^[A-Za-z0-9._:-]{1,128}$/.test(value)) return value;
  return `${action}-${newSessionId().slice(0, 16)}`;
}

function manualStorageFailure(error: unknown): {
  status: 'manual-required';
  reason: 'schema-missing' | 'schema-invalid' | 'd1-unavailable';
} {
  log.logWarn('account browser session storage unavailable', getStackTrace(), {
    owner: 'account-identity-family-plan',
    boundary: 'browser-session-custody',
    result: 'blocked',
    reason: missingSchema(error) ? 'schema-missing' : invalidSchema(error) ? 'schema-invalid' : 'd1-unavailable',
    redactionState: 'tokens-and-storage-error-omitted',
  });
  return {
    status: 'manual-required',
    reason: missingSchema(error) ? 'schema-missing' : invalidSchema(error) ? 'schema-invalid' : 'd1-unavailable',
  };
}

async function auditBindings(
  row: Pick<BrowserSessionRow, 'session_id' | 'provider' | 'provider_subject'>,
  action: 'created' | 'refreshed' | 'logout' | 'global-revoke' | 'replay-rejected',
  result: 'accepted' | 'rejected',
  reason: string,
  requestCorrelation: string,
  now: string
): Promise<ReadonlyArray<string>> {
  const sessionRefDigest = await sha256Hex(`ocentra/account-browser-session/audit/session:${row.session_id}`);
  const actorRefDigest = await sha256Hex(
    `ocentra/account-browser-session/audit/actor:${row.provider}:${row.provider_subject}`
  );
  return [
    newSessionId(),
    sessionRefDigest,
    row.provider,
    actorRefDigest,
    action,
    result,
    reason,
    requestCorrelation,
    now,
  ];
}

async function requireCustodySchema(database: D1Database): Promise<void> {
  const value = await database.prepare(SELECT_CUSTODY_SCHEMA_SQL).first<unknown>();
  if (value === null) throw new MissingBrowserSessionSchemaError();
  const ddl = await database.prepare(SELECT_CUSTODY_SCHEMA_DDL_SQL).first<unknown>();
  const ddlText =
    typeof ddl === 'object' && ddl !== null && !Array.isArray(ddl) && typeof (ddl as { sql?: unknown }).sql === 'string'
      ? (ddl as { sql: string }).sql.replace(/\s+/g, ' ').trim().toUpperCase()
      : null;
  if (
    typeof value !== 'object' ||
    Array.isArray(value) ||
    (value as { schema_name?: unknown }).schema_name !== 'browser-session-custody' ||
    (value as { schema_version?: unknown }).schema_version !== SESSION_CUSTODY_SCHEMA_VERSION ||
    !isBrowserSessionTimestamp((value as { applied_at?: unknown }).applied_at) ||
    ddlText === null ||
    !ddlText.includes('STRICT') ||
    !ddlText.includes("SCHEMA_NAME TEXT NOT NULL PRIMARY KEY CHECK (SCHEMA_NAME = 'BROWSER-SESSION-CUSTODY')") ||
    !ddlText.includes('SCHEMA_VERSION INTEGER NOT NULL CHECK (SCHEMA_VERSION = 7)')
  ) {
    throw new InvalidBrowserSessionSchemaError();
  }
}

async function scopeDigest(provider: string, providerSubject: string): Promise<string> {
  return sha256Hex(`ocentra/account-browser-session/audit/scope:${provider}:${providerSubject}`);
}

async function actorDigest(provider: string, providerSubject: string): Promise<string> {
  return sha256Hex(`ocentra/account-browser-session/audit/actor:${provider}:${providerSubject}`);
}

function expired(row: BrowserSessionRow, nowMs: number, credential: 'access' | 'refresh'): boolean {
  const expiry = Date.parse(credential === 'access' ? row.access_expires_at : row.refresh_expires_at);
  return !Number.isFinite(expiry) || expiry <= nowMs;
}

async function currentRowById(
  database: D1Database,
  sessionId: string,
  nowMs: number
): Promise<BrowserSessionRow | null> {
  const now = nowIso(nowMs);
  return decodeSessionRow(
    await database.prepare(SELECT_CURRENT_BY_ID_SQL).bind(sessionId, now, now, now).first<unknown>()
  );
}

async function consumedRefreshByDigest(
  database: D1Database,
  refreshTokenDigest: string
): Promise<ConsumedRefreshRow | null> {
  return decodeConsumedRefreshRow(
    await database.prepare(SELECT_CONSUMED_REFRESH_SQL).bind(refreshTokenDigest).first<unknown>()
  );
}

async function isCurrentFence(database: D1Database, row: BrowserSessionRow): Promise<boolean> {
  const fence = decodeFenceRow(
    await database.prepare(SELECT_FENCE_SQL).bind(row.provider, row.provider_subject).first<unknown>()
  );
  return fence !== null && fence.revoke_generation === row.revoke_generation;
}

function mutationFromRow(row: BrowserSessionRow, secrets: BrowserSessionSecrets): BrowserSessionMutationResult {
  return { status: 'accepted', identity: sessionIdentity(row), secrets };
}

function authorityBindingFromRow(row: BrowserSessionRow): CurrentAuthorityBinding | null {
  if (browserSessionRole(row.role) === null) return null;
  if (
    row.household_id === null ||
    row.member_id === null ||
    row.device_id === null ||
    row.child_profile_id === null ||
    row.child_device_id === null
  ) {
    return null;
  }
  return {
    provider: row.provider,
    providerSubject: row.provider_subject,
    accountId: row.account_id,
    householdId: row.household_id,
    memberId: row.member_id,
    role: row.role,
    deviceId: row.device_id,
    childProfileId: row.child_profile_id,
    childDeviceId: row.child_device_id,
    authoritySessionId: row.authority_session_id,
    authoritySessionGeneration: row.authority_session_generation,
    authorityGeneration: row.authority_generation,
    supportReceiptId: row.support_receipt_id,
    supportProviderSubject: row.support_provider_subject,
    supportAccountId: row.support_account_id,
    supportMemberId: row.support_member_id,
    supportHouseholdId: row.support_household_id,
    supportDeviceId: row.support_device_id,
    supportChildProfileId: row.support_child_profile_id,
    supportChildDeviceId: row.support_child_device_id,
    supportScope: row.support_scope,
    supportIssuer: row.support_issuer,
    supportIssuedAt: row.support_issued_at,
    supportExpiresAt: row.support_expires_at,
    supportRevocationState: row.support_revocation_state,
    supportAuditIdentity: row.support_audit_identity,
  };
}

function authorityBindingFromCapability(
  authority: VerifiedAccountIdentityAuthorityCapability
): CurrentAuthorityBinding {
  return {
    provider: authority.provider,
    providerSubject: authority.providerSubject,
    accountId: authority.accountId,
    householdId: authority.householdId,
    memberId: authority.memberId,
    role: authority.role,
    deviceId: authority.deviceId,
    childProfileId: authority.childProfileId,
    childDeviceId: authority.childDeviceId,
    authoritySessionId: authority.sessionId,
    authoritySessionGeneration: authority.sessionGeneration,
    authorityGeneration: authority.authorityGeneration,
    supportReceiptId: authority.supportReceiptId,
    supportProviderSubject: authority.supportProviderSubject,
    supportAccountId: authority.supportAccountId,
    supportMemberId: authority.supportMemberId,
    supportHouseholdId: authority.supportHouseholdId,
    supportDeviceId: authority.supportDeviceId,
    supportChildProfileId: authority.supportChildProfileId,
    supportChildDeviceId: authority.supportChildDeviceId,
    supportScope: authority.supportScope,
    supportIssuer: authority.supportIssuer,
    supportIssuedAt: authority.supportIssuedAt,
    supportExpiresAt: authority.supportExpiresAt,
    supportRevocationState: authority.supportRevocationState,
    supportAuditIdentity: authority.supportAuditIdentity,
  };
}

type RefreshRotationLookup =
  | { status: 'current'; row: BrowserSessionRow; digest: string }
  | { status: 'consumed'; row: BrowserSessionRow | null; consumed: ConsumedRefreshRow }
  | BrowserSessionReadResult;

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
  const row = await currentRowById(database, parsed.sessionId, nowMs);
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

async function readRefreshForRotation(
  database: D1Database,
  refreshToken: string | null,
  nowMs: number
): Promise<RefreshRotationLookup> {
  const parsed = parseSessionCookie(refreshToken);
  if (refreshToken !== null && parsed === null) return { status: 'rejected', reason: 'malformed' };
  if (parsed === null) return { status: 'missing' };
  const digest = await sha256Hex(refreshToken!);
  if (!isDigest(digest)) return { status: 'rejected', reason: 'invalid' };
  const consumed = await consumedRefreshByDigest(database, digest);
  const row = await currentRowById(database, parsed.sessionId, nowMs);
  if (consumed !== null) {
    return { status: 'consumed', row, consumed };
  }
  if (row === null || !constantTimeEqual(digest, row.refresh_token_digest)) {
    return { status: 'rejected', reason: 'invalid' };
  }
  if (row.status !== 'active' || !(await isCurrentFence(database, row))) {
    return { status: 'rejected', reason: 'revoked' };
  }
  if (expired(row, nowMs, 'refresh')) return { status: 'rejected', reason: 'expired' };
  if (browserSessionRole(row.role) === null) return { status: 'rejected', reason: 'role-ineligible' };
  return { status: 'current', row, digest };
}

async function activeRowsForGeneration(
  database: D1Database,
  provider: string,
  providerSubject: string,
  revokeGeneration: number,
  nowMs: number
): Promise<BrowserSessionRow[]> {
  const result = await database
    .prepare(SELECT_ACTIVE_BY_SUBJECT_SQL)
    .bind(provider, providerSubject, revokeGeneration, nowIso(nowMs), nowIso(nowMs), nowIso(nowMs))
    .all<unknown>();
  return (result.results ?? []).map((row) => {
    const decoded = decodeSessionRow(row);
    if (decoded === null) throw new InvalidBrowserSessionRowError('browser session');
    return decoded;
  });
}

async function revokeFamilyAtomic(
  database: D1Database,
  binding: CurrentAuthorityBinding,
  expectedGeneration: number,
  now: string,
  requestCorrelation: string,
  action: 'global-revoke' | 'refresh-replay',
  auditAction: 'global-revoke' | 'replay-rejected',
  auditResult: 'accepted' | 'rejected',
  reason: string
): Promise<BrowserSessionMutationResult> {
  if (!Number.isSafeInteger(expectedGeneration) || expectedGeneration <= 0) {
    return { status: 'manual-required', reason: 'd1-unavailable' };
  }
  const nextGeneration = expectedGeneration + 1;
  if (!Number.isSafeInteger(nextGeneration)) {
    return { status: 'manual-required', reason: 'd1-unavailable' };
  }
  const nowMs = Date.parse(now);
  if (!Number.isFinite(nowMs)) {
    return { status: 'manual-required', reason: 'd1-unavailable' };
  }
  const rows = await activeRowsForGeneration(
    database,
    binding.provider,
    binding.providerSubject,
    expectedGeneration,
    nowMs
  );
  const correlation = safeCorrelation(
    requestCorrelation,
    action === 'global-revoke' ? 'session-revoke' : 'session-replay'
  );
  const scopeRefDigest = await scopeDigest(binding.provider, binding.providerSubject);
  const actorRefDigest = await actorDigest(binding.provider, binding.providerSubject);
  const statements = [
    database.prepare(ENSURE_FENCE_SQL).bind(binding.provider, binding.providerSubject, now),
    database
      .prepare(ADVANCE_FENCE_SQL)
      .bind(
        now,
        binding.provider,
        binding.providerSubject,
        expectedGeneration,
        binding.provider,
        binding.providerSubject,
        binding.accountId,
        binding.householdId,
        binding.memberId,
        binding.role,
        binding.deviceId,
        binding.childProfileId,
        binding.childDeviceId,
        binding.authoritySessionId,
        binding.authoritySessionGeneration,
        now,
        binding.authorityGeneration,
        binding.supportReceiptId,
        binding.supportProviderSubject,
        binding.supportAccountId,
        binding.supportMemberId,
        binding.supportHouseholdId,
        binding.supportDeviceId,
        binding.supportChildProfileId,
        binding.supportChildDeviceId,
        binding.supportScope,
        binding.supportIssuer,
        binding.supportIssuedAt,
        binding.supportExpiresAt,
        binding.supportRevocationState,
        binding.supportAuditIdentity,
        now,
        now
      ),
    database
      .prepare(REVOKE_OUTCOME_SQL)
      .bind(
        newSessionId(),
        binding.provider,
        scopeRefDigest,
        actorRefDigest,
        action,
        auditResult,
        reason,
        nextGeneration,
        correlation,
        now,
        binding.provider,
        binding.providerSubject,
        nextGeneration,
        binding.provider,
        binding.providerSubject,
        nextGeneration
      ),
    database
      .prepare(REVOKE_ALL_SQL)
      .bind(now, now, nextGeneration, binding.provider, binding.providerSubject, expectedGeneration, nextGeneration),
  ];
  for (const row of rows) {
    const audit = await auditBindings(row, auditAction, auditResult, reason, correlation, now);
    statements.push(
      database
        .prepare(AUDIT_REVOKED_SESSION_SQL)
        .bind(
          ...audit,
          row.session_id,
          now,
          now,
          nextGeneration,
          nextGeneration,
          row.session_id,
          now,
          now,
          nextGeneration,
          nextGeneration
        )
    );
  }
  const results = await database.batch(statements);
  if (changes(results[1]) !== 1 || changes(results[2]) !== 1) {
    return { status: 'rejected', reason: action === 'refresh-replay' ? 'replay' : 'conflict' };
  }
  if (action === 'refresh-replay') {
    return {
      status: 'rejected',
      reason: 'replay',
    };
  }
  const first = rows[0];
  const identity = first
    ? sessionIdentity({
        ...first,
        revoke_generation: nextGeneration,
        status: 'revoked',
        revoked_at: now,
        updated_at: now,
      })
    : null;
  return {
    status: 'accepted',
    identity,
    secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
  };
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
      logoutRefresh: async () => unavailable(),
      logout: async () => unavailable(),
      revokeAll: async () => unavailable(),
    };
  }

  return {
    async create(authority, requestCorrelation) {
      if (!isVerifiedAccountIdentityAuthorityCapability(authority)) {
        return { status: 'rejected', reason: 'invalid' };
      }
      if (browserSessionRole(authority.role) === null) return { status: 'rejected', reason: 'role-ineligible' };
      const nowMs = Date.now();
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
        await requireCustodySchema(database);
        const correlation = safeCorrelation(requestCorrelation, 'session-create');
        const sessionTokenDigest = await sha256Hex(secrets.sessionToken);
        const refreshTokenDigest = await sha256Hex(secrets.refreshToken);
        const csrfTokenDigest = await sha256Hex(secrets.csrfToken);
        const audit = await auditBindings(
          {
            session_id: sessionId,
            provider: authority.provider,
            provider_subject: authority.providerSubject,
          },
          'created',
          'accepted',
          'session-created',
          correlation,
          issuedAt
        );
        const results = await database.batch([
          database.prepare(ENSURE_FENCE_SQL).bind(authority.provider, authority.providerSubject, issuedAt),
          database
            .prepare(INSERT_SQL)
            .bind(
              sessionId,
              sessionTokenDigest,
              refreshTokenDigest,
              csrfTokenDigest,
              issuedAt,
              accessExpiresAt,
              refreshExpiresAt,
              issuedAt,
              issuedAt,
              issuedAt,
              authority.provider,
              authority.providerSubject,
              authority.accountId,
              authority.householdId,
              authority.memberId,
              authority.role,
              authority.deviceId,
              authority.childProfileId,
              authority.childDeviceId,
              authority.sessionId,
              authority.sessionGeneration,
              issuedAt,
              authority.authorityGeneration,
              authority.supportReceiptId,
              authority.supportProviderSubject,
              authority.supportAccountId,
              authority.supportMemberId,
              authority.supportHouseholdId,
              authority.supportDeviceId,
              authority.supportChildProfileId,
              authority.supportChildDeviceId,
              authority.supportScope,
              authority.supportIssuer,
              authority.supportIssuedAt,
              authority.supportExpiresAt,
              authority.supportRevocationState,
              authority.supportAuditIdentity,
              issuedAt,
              issuedAt
            ),
          database.prepare(AUDIT_ACTIVE_SESSION_SQL).bind(...audit, sessionId, issuedAt, 1, sessionId, issuedAt, 1),
        ]);
        if (changes(results[1]) !== 1) return { status: 'rejected', reason: 'invalid' };
        if (changes(results[2]) !== 1) return { status: 'manual-required', reason: 'd1-unavailable' };
        const row = await currentRowById(database, sessionId, nowMs);
        if (row === null) return { status: 'rejected', reason: 'conflict' };
        return mutationFromRow(row, secrets);
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async read(sessionToken) {
      const nowMs = Date.now();
      try {
        await requireCustodySchema(database);
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'access');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async readBinding(sessionToken) {
      const nowMs = Date.now();
      try {
        await requireCustodySchema(database);
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'binding');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async readRefresh(refreshToken) {
      const nowMs = Date.now();
      try {
        await requireCustodySchema(database);
        const result = await readWithSecret(database, refreshToken, 'refresh_token_digest', nowMs, 'refresh');
        if ('row' in result) return { status: 'active', identity: sessionIdentity(result.row), row: result.row };
        return result;
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async rotate(refreshToken, requestCorrelation) {
      const nowMs = Date.now();
      const now = nowIso(nowMs);
      try {
        await requireCustodySchema(database);
        const session = await readRefreshForRotation(database, refreshToken, nowMs);
        if (session.status === 'consumed') {
          if (
            session.row === null ||
            session.row.session_id !== session.consumed.session_id ||
            session.consumed.refresh_generation >= session.row.refresh_generation
          ) {
            return { status: 'manual-required', reason: 'd1-unavailable' };
          }
          const binding = authorityBindingFromRow(session.row);
          if (binding === null) return { status: 'manual-required', reason: 'd1-unavailable' };
          const replay = await revokeFamilyAtomic(
            database,
            binding,
            session.row.revoke_generation,
            now,
            safeCorrelation(requestCorrelation, 'session-replay'),
            'refresh-replay',
            'replay-rejected',
            'rejected',
            'refresh-replay'
          );
          return replay.status === 'manual-required' ? replay : { status: 'rejected', reason: 'replay' };
        }
        if (!('row' in session)) {
          if (session.status === 'manual-required') return session;
          return { status: 'rejected', reason: session.status === 'missing' ? 'missing' : session.reason };
        }
        const row = session.row;
        const secrets = {
          sessionToken: sessionCookieValue(row.session_id, newOpaqueValue()),
          refreshToken: sessionCookieValue(row.session_id, newOpaqueValue()),
          csrfToken: sessionCookieValue(row.session_id, newOpaqueValue()),
        } satisfies BrowserSessionSecrets;
        const correlation = safeCorrelation(requestCorrelation, 'session-refresh');
        const nextSessionTokenDigest = await sha256Hex(secrets.sessionToken);
        const nextRefreshTokenDigest = await sha256Hex(secrets.refreshToken);
        const nextCsrfTokenDigest = await sha256Hex(secrets.csrfToken);
        const audit = await auditBindings(row, 'refreshed', 'accepted', 'refresh-rotated', correlation, now);
        const results = await database.batch([
          database
            .prepare(ROTATE_SQL)
            .bind(
              nextSessionTokenDigest,
              nextRefreshTokenDigest,
              nextCsrfTokenDigest,
              nowIso(nowMs + ACCESS_LIFETIME_MS),
              now,
              now,
              row.session_id,
              session.digest,
              now,
              row.refresh_generation,
              session.digest,
              now,
              now,
              now
            ),
          database
            .prepare(CONSUME_REFRESH_SQL)
            .bind(
              session.digest,
              row.session_id,
              now,
              row.session_id,
              nextRefreshTokenDigest,
              row.refresh_generation + 1,
              now,
              row.session_id,
              nextRefreshTokenDigest,
              row.refresh_generation + 1,
              now
            ),
          database
            .prepare(AUDIT_ROTATED_SESSION_SQL)
            .bind(
              ...audit,
              row.session_id,
              now,
              row.refresh_generation + 1,
              nextRefreshTokenDigest,
              row.session_id,
              now,
              row.refresh_generation + 1,
              nextRefreshTokenDigest
            ),
        ]);
        if (changes(results[0]) !== 1) {
          const consumed = await consumedRefreshByDigest(database, session.digest);
          if (consumed !== null) {
            const binding = authorityBindingFromRow(row);
            if (binding === null) return { status: 'manual-required', reason: 'd1-unavailable' };
            const replay = await revokeFamilyAtomic(
              database,
              binding,
              row.revoke_generation,
              now,
              correlation,
              'refresh-replay',
              'replay-rejected',
              'rejected',
              'refresh-replay'
            );
            return replay.status === 'manual-required' ? replay : { status: 'rejected', reason: 'replay' };
          }
          return { status: 'rejected', reason: 'conflict' };
        }
        if (changes(results[1]) !== 1 || changes(results[2]) !== 1) {
          return { status: 'manual-required', reason: 'd1-unavailable' };
        }
        const current = await currentRowById(database, row.session_id, nowMs);
        if (current === null) return { status: 'rejected', reason: 'conflict' };
        return mutationFromRow(current, secrets);
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async verifyCsrf(sessionToken, csrfToken) {
      const nowMs = Date.now();
      try {
        await requireCustodySchema(database);
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

    async verifyRefreshCsrf(refreshToken, csrfToken) {
      const nowMs = Date.now();
      try {
        await requireCustodySchema(database);
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

    async logout(sessionToken, requestCorrelation) {
      const nowMs = Date.now();
      let session: BrowserSessionReadResult;
      try {
        await requireCustodySchema(database);
        const result = await readWithSecret(database, sessionToken, 'session_token_digest', nowMs, 'access');
        session =
          'row' in result ? { status: 'active', identity: sessionIdentity(result.row), row: result.row } : result;
      } catch (error) {
        return manualStorageFailure(error);
      }
      if (session.status === 'manual-required') return session;
      if (session.status === 'missing') return { status: 'rejected', reason: 'missing' };
      if (session.status === 'rejected') return { status: 'rejected', reason: session.reason };
      try {
        const now = nowIso(nowMs);
        const correlation = safeCorrelation(requestCorrelation, 'session-logout');
        const audit = await auditBindings(session.row, 'logout', 'accepted', 'session-logout', correlation, now);
        const results = await database.batch([
          database.prepare(REVOKE_SQL).bind(now, now, session.row.session_id, now, now, now),
          database
            .prepare(AUDIT_REVOKED_SINGLE_SESSION_SQL)
            .bind(
              ...audit,
              session.row.session_id,
              now,
              now,
              session.row.revoke_generation,
              session.row.session_id,
              now,
              now,
              session.row.revoke_generation
            ),
        ]);
        if (changes(results[0]) !== 1) return { status: 'rejected', reason: 'conflict' };
        if (changes(results[1]) !== 1) return { status: 'manual-required', reason: 'd1-unavailable' };
        return {
          status: 'accepted',
          identity: session.identity,
          secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
        };
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async logoutRefresh(refreshToken, requestCorrelation) {
      const nowMs = Date.now();
      let session: BrowserSessionReadResult;
      try {
        await requireCustodySchema(database);
        const result = await readWithSecret(database, refreshToken, 'refresh_token_digest', nowMs, 'refresh');
        session =
          'row' in result ? { status: 'active', identity: sessionIdentity(result.row), row: result.row } : result;
      } catch (error) {
        return manualStorageFailure(error);
      }
      if (session.status === 'manual-required') return session;
      if (session.status === 'missing') return { status: 'rejected', reason: 'missing' };
      if (session.status === 'rejected') return { status: 'rejected', reason: session.reason };
      try {
        const now = nowIso(nowMs);
        const correlation = safeCorrelation(requestCorrelation, 'session-logout');
        const audit = await auditBindings(session.row, 'logout', 'accepted', 'session-logout', correlation, now);
        const results = await database.batch([
          database.prepare(REVOKE_SQL).bind(now, now, session.row.session_id, now, now, now),
          database
            .prepare(AUDIT_REVOKED_SINGLE_SESSION_SQL)
            .bind(
              ...audit,
              session.row.session_id,
              now,
              now,
              session.row.revoke_generation,
              session.row.session_id,
              now,
              now,
              session.row.revoke_generation
            ),
        ]);
        if (changes(results[0]) !== 1) return { status: 'rejected', reason: 'conflict' };
        if (changes(results[1]) !== 1) return { status: 'manual-required', reason: 'd1-unavailable' };
        return {
          status: 'accepted',
          identity: session.identity,
          secrets: { sessionToken: '', refreshToken: '', csrfToken: '' },
        };
      } catch (error) {
        return manualStorageFailure(error);
      }
    },

    async revokeAll(authority, requestCorrelation) {
      if (!isVerifiedAccountIdentityAuthorityCapability(authority)) {
        return { status: 'rejected', reason: 'invalid' };
      }
      if (authority.role !== 'parent-owner') {
        return { status: 'rejected', reason: 'role-ineligible' };
      }
      if (
        authority.supportScope !== null ||
        authority.supportIssuer !== null ||
        authority.supportAuditIdentity !== null
      ) {
        return { status: 'rejected', reason: 'invalid' };
      }
      const nowMs = Date.now();
      const now = nowIso(nowMs);
      try {
        await requireCustodySchema(database);
        const fence = decodeFenceRow(
          await database.prepare(SELECT_FENCE_SQL).bind(authority.provider, authority.providerSubject).first<unknown>()
        );
        const binding = authorityBindingFromCapability(authority);
        return await revokeFamilyAtomic(
          database,
          binding,
          fence?.revoke_generation ?? 1,
          now,
          safeCorrelation(requestCorrelation, 'session-revoke'),
          'global-revoke',
          'global-revoke',
          'accepted',
          'global-revoke'
        );
      } catch (error) {
        return manualStorageFailure(error);
      }
    },
  };
}
