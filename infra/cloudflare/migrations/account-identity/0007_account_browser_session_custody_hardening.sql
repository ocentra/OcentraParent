-- Forward-only custody hardening for D1 databases that already applied 0005
-- and 0006. Those migrations remain historical and are intentionally not
-- edited in place: D1 will not replay an applied migration. This migration
-- rebuilds every browser-session table as STRICT, copies only rows that pass
-- the complete custody invariants, and records rejected legacy rows without
-- retaining raw provider subjects, session identifiers, or bearer material.

CREATE TABLE IF NOT EXISTS ocentra_account_browser_session_schema_quarantine (
  quarantine_id TEXT NOT NULL PRIMARY KEY CHECK (
    length(quarantine_id) = 32 AND quarantine_id NOT GLOB '*[^0-9a-f]*'
  ),
  source_table TEXT NOT NULL CHECK (
    source_table IN (
      'ocentra_account_browser_session_fences',
      'ocentra_account_browser_sessions',
      'ocentra_account_browser_session_audit',
      'ocentra_account_browser_session_consumed_refresh',
      'ocentra_account_browser_session_revoke_outcomes'
    )
  ),
  reason TEXT NOT NULL CHECK (reason = 'invalid-legacy-row'),
  quarantined_at TEXT NOT NULL CHECK (julianday(quarantined_at) IS NOT NULL)
) STRICT;

DROP INDEX IF EXISTS idx_ocentra_account_browser_sessions_subject;
ALTER TABLE ocentra_account_browser_sessions RENAME TO ocentra_account_browser_sessions_legacy_0007;

CREATE VIEW ocentra_account_browser_sessions_legacy_validity_0007 AS
SELECT
  rowid AS legacy_rowid,
  CASE
    WHEN typeof(session_id) = 'text'
      AND length(session_id) = 43
      AND session_id NOT GLOB '*[^A-Za-z0-9_-]*'
      AND typeof(session_token_digest) = 'text'
      AND length(session_token_digest) = 64
      AND session_token_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(refresh_token_digest) = 'text'
      AND length(refresh_token_digest) = 64
      AND refresh_token_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(csrf_token_digest) = 'text'
      AND length(csrf_token_digest) = 64
      AND csrf_token_digest NOT GLOB '*[^0-9a-f]*'
      AND session_token_digest <> refresh_token_digest
      AND session_token_digest <> csrf_token_digest
      AND refresh_token_digest <> csrf_token_digest
      AND typeof(provider) = 'text'
      AND provider IN ('authjs', 'firebase')
      AND typeof(provider_subject) = 'text'
      AND length(provider_subject) BETWEEN 1 AND 256
      AND typeof(role) = 'text'
      AND role IN ('parent-owner', 'co-parent-guardian', 'support-admin')
      AND typeof(account_id) = 'text'
      AND length(account_id) BETWEEN 1 AND 256
      AND typeof(household_id) = 'text'
      AND length(household_id) BETWEEN 1 AND 256
      AND typeof(member_id) = 'text'
      AND length(member_id) BETWEEN 1 AND 256
      AND typeof(device_id) = 'text'
      AND length(device_id) BETWEEN 1 AND 256
      AND typeof(child_profile_id) = 'text'
      AND length(child_profile_id) BETWEEN 1 AND 256
      AND typeof(child_device_id) = 'text'
      AND length(child_device_id) BETWEEN 1 AND 256
      AND typeof(authority_session_id) = 'text'
      AND length(authority_session_id) BETWEEN 1 AND 256
      AND typeof(authority_session_generation) = 'integer'
      AND authority_session_generation > 0
      AND authority_session_generation <= 9007199254740991
      AND typeof(authority_generation) = 'integer'
      AND authority_generation > 0
      AND authority_generation <= 9007199254740991
      AND typeof(issued_at) = 'text'
      AND julianday(issued_at) IS NOT NULL
      AND typeof(access_expires_at) = 'text'
      AND julianday(access_expires_at) IS NOT NULL
      AND typeof(refresh_expires_at) = 'text'
      AND julianday(refresh_expires_at) IS NOT NULL
      AND typeof(revoke_generation) = 'integer'
      AND revoke_generation > 0
      AND revoke_generation <= 9007199254740991
      AND typeof(refresh_generation) = 'integer'
      AND refresh_generation > 0
      AND refresh_generation <= 9007199254740991
      AND typeof(status) = 'text'
      AND status IN ('active', 'revoked')
      AND typeof(last_seen_at) = 'text'
      AND julianday(last_seen_at) IS NOT NULL
      AND (revoked_at IS NULL OR (typeof(revoked_at) = 'text' AND julianday(revoked_at) IS NOT NULL))
      AND typeof(created_at) = 'text'
      AND julianday(created_at) IS NOT NULL
      AND typeof(updated_at) = 'text'
      AND julianday(updated_at) IS NOT NULL
      AND julianday(created_at) <= julianday(issued_at)
      AND julianday(issued_at) <= julianday(last_seen_at)
      AND julianday(last_seen_at) <= julianday(updated_at)
      AND julianday(issued_at) < julianday(access_expires_at)
      AND julianday(access_expires_at) < julianday(refresh_expires_at)
      AND (
        (status = 'active' AND revoked_at IS NULL)
        OR (
          status = 'revoked'
          AND revoked_at IS NOT NULL
          AND julianday(created_at) <= julianday(revoked_at)
          AND julianday(revoked_at) <= julianday(updated_at)
        )
      )
      AND (
        (
          role = 'support-admin'
          AND typeof(support_receipt_id) = 'text'
          AND length(support_receipt_id) BETWEEN 1 AND 256
          AND typeof(support_provider_subject) = 'text'
          AND length(support_provider_subject) BETWEEN 1 AND 256
          AND typeof(support_account_id) = 'text'
          AND length(support_account_id) BETWEEN 1 AND 256
          AND typeof(support_member_id) = 'text'
          AND length(support_member_id) BETWEEN 1 AND 256
          AND typeof(support_household_id) = 'text'
          AND length(support_household_id) BETWEEN 1 AND 256
          AND typeof(support_device_id) = 'text'
          AND length(support_device_id) BETWEEN 1 AND 256
          AND typeof(support_child_profile_id) = 'text'
          AND length(support_child_profile_id) BETWEEN 1 AND 256
          AND typeof(support_child_device_id) = 'text'
          AND length(support_child_device_id) BETWEEN 1 AND 256
          AND typeof(support_scope) = 'text'
          AND support_scope IN ('read-only', 'household', 'device-control')
          AND typeof(support_issuer) = 'text'
          AND length(support_issuer) BETWEEN 1 AND 256
          AND typeof(support_issued_at) = 'text'
          AND julianday(support_issued_at) IS NOT NULL
          AND typeof(support_expires_at) = 'text'
          AND julianday(support_expires_at) IS NOT NULL
          AND julianday(support_issued_at) < julianday(support_expires_at)
          AND typeof(support_revocation_state) = 'text'
          AND support_revocation_state = 'active'
          AND typeof(support_audit_identity) = 'text'
          AND length(support_audit_identity) BETWEEN 1 AND 256
          AND support_provider_subject = provider_subject
          AND support_account_id = account_id
          AND support_member_id = member_id
          AND support_household_id = household_id
          AND support_device_id = device_id
          AND support_child_profile_id = child_profile_id
          AND support_child_device_id = child_device_id
        )
        OR (
          role <> 'support-admin'
          AND support_receipt_id IS NULL
          AND support_provider_subject IS NULL
          AND support_account_id IS NULL
          AND support_member_id IS NULL
          AND support_household_id IS NULL
          AND support_device_id IS NULL
          AND support_child_profile_id IS NULL
          AND support_child_device_id IS NULL
          AND support_scope IS NULL
          AND support_issuer IS NULL
          AND support_issued_at IS NULL
          AND support_expires_at IS NULL
          AND support_revocation_state IS NULL
          AND support_audit_identity IS NULL
        )
      )
    THEN 1
    ELSE 0
  END AS is_valid
FROM ocentra_account_browser_sessions_legacy_0007;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT lower(hex(randomblob(16))), 'ocentra_account_browser_sessions', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_sessions_legacy_0007 AS session
JOIN ocentra_account_browser_sessions_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = session.rowid
WHERE validity.is_valid = 0;

-- Any rejected legacy row aborts this migration transaction. The preceding
-- quarantine attempt is rolled back with the failed migration under D1's
-- migration transaction semantics; no invalid row is silently retained or
-- presented as usable custody, and the sentinel below is never published.
INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT NULL, 'ocentra_account_browser_sessions', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_sessions_legacy_0007 AS session
JOIN ocentra_account_browser_sessions_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = session.rowid
WHERE validity.is_valid = 0;

CREATE TABLE ocentra_account_browser_sessions (
  session_id TEXT NOT NULL PRIMARY KEY CHECK (
    length(session_id) = 43 AND session_id NOT GLOB '*[^A-Za-z0-9_-]*'
  ),
  session_token_digest TEXT NOT NULL UNIQUE CHECK (
    length(session_token_digest) = 64 AND session_token_digest NOT GLOB '*[^0-9a-f]*'
  ),
  refresh_token_digest TEXT NOT NULL UNIQUE CHECK (
    length(refresh_token_digest) = 64 AND refresh_token_digest NOT GLOB '*[^0-9a-f]*'
  ),
  csrf_token_digest TEXT NOT NULL CHECK (
    length(csrf_token_digest) = 64 AND csrf_token_digest NOT GLOB '*[^0-9a-f]*'
  ),
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL CHECK (length(provider_subject) BETWEEN 1 AND 256),
  role TEXT NOT NULL CHECK (role IN ('parent-owner', 'co-parent-guardian', 'support-admin')),
  account_id TEXT NOT NULL CHECK (length(account_id) BETWEEN 1 AND 256),
  household_id TEXT NOT NULL CHECK (length(household_id) BETWEEN 1 AND 256),
  member_id TEXT NOT NULL CHECK (length(member_id) BETWEEN 1 AND 256),
  device_id TEXT NOT NULL CHECK (length(device_id) BETWEEN 1 AND 256),
  child_profile_id TEXT NOT NULL CHECK (length(child_profile_id) BETWEEN 1 AND 256),
  child_device_id TEXT NOT NULL CHECK (length(child_device_id) BETWEEN 1 AND 256),
  authority_session_id TEXT NOT NULL CHECK (length(authority_session_id) BETWEEN 1 AND 256),
  authority_session_generation INTEGER NOT NULL CHECK (
    authority_session_generation > 0 AND authority_session_generation <= 9007199254740991
  ),
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
  support_receipt_id TEXT,
  support_provider_subject TEXT,
  support_account_id TEXT,
  support_member_id TEXT,
  support_household_id TEXT,
  support_device_id TEXT,
  support_child_profile_id TEXT,
  support_child_device_id TEXT,
  support_scope TEXT,
  support_issuer TEXT,
  support_issued_at TEXT,
  support_expires_at TEXT,
  support_revocation_state TEXT,
  support_audit_identity TEXT,
  issued_at TEXT NOT NULL CHECK (julianday(issued_at) IS NOT NULL),
  access_expires_at TEXT NOT NULL CHECK (julianday(access_expires_at) IS NOT NULL),
  refresh_expires_at TEXT NOT NULL CHECK (julianday(refresh_expires_at) IS NOT NULL),
  revoke_generation INTEGER NOT NULL CHECK (
    revoke_generation > 0 AND revoke_generation <= 9007199254740991
  ),
  refresh_generation INTEGER NOT NULL CHECK (
    refresh_generation > 0 AND refresh_generation <= 9007199254740991
  ),
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  last_seen_at TEXT NOT NULL CHECK (julianday(last_seen_at) IS NOT NULL),
  revoked_at TEXT CHECK (revoked_at IS NULL OR julianday(revoked_at) IS NOT NULL),
  created_at TEXT NOT NULL CHECK (julianday(created_at) IS NOT NULL),
  updated_at TEXT NOT NULL CHECK (julianday(updated_at) IS NOT NULL),
  CHECK (
    session_token_digest <> refresh_token_digest AND
    session_token_digest <> csrf_token_digest AND
    refresh_token_digest <> csrf_token_digest
  ),
  CHECK (
    julianday(created_at) <= julianday(issued_at) AND
    julianday(issued_at) <= julianday(last_seen_at) AND
    julianday(last_seen_at) <= julianday(updated_at) AND
    julianday(issued_at) < julianday(access_expires_at) AND
    julianday(access_expires_at) < julianday(refresh_expires_at)
  ),
  CHECK (
    (status = 'active' AND revoked_at IS NULL) OR
    (status = 'revoked' AND revoked_at IS NOT NULL AND julianday(created_at) <= julianday(revoked_at) AND julianday(revoked_at) <= julianday(updated_at))
  ),
  CHECK (
    (
      role = 'support-admin' AND
      support_receipt_id IS NOT NULL AND support_provider_subject IS NOT NULL AND
      support_account_id IS NOT NULL AND support_member_id IS NOT NULL AND
      support_household_id IS NOT NULL AND support_device_id IS NOT NULL AND
      support_child_profile_id IS NOT NULL AND support_child_device_id IS NOT NULL AND
      support_scope IN ('read-only', 'household', 'device-control') AND
      support_issuer IS NOT NULL AND support_issued_at IS NOT NULL AND
      support_expires_at IS NOT NULL AND julianday(support_issued_at) < julianday(support_expires_at) AND
      support_revocation_state = 'active' AND support_audit_identity IS NOT NULL
      AND support_provider_subject = provider_subject
      AND support_account_id = account_id
      AND support_member_id = member_id
      AND support_household_id = household_id
      AND support_device_id = device_id
      AND support_child_profile_id = child_profile_id
      AND support_child_device_id = child_device_id
    )
    OR
    (
      role <> 'support-admin' AND support_receipt_id IS NULL AND support_provider_subject IS NULL AND
      support_account_id IS NULL AND support_member_id IS NULL AND support_household_id IS NULL AND
      support_device_id IS NULL AND support_child_profile_id IS NULL AND support_child_device_id IS NULL AND
      support_scope IS NULL AND support_issuer IS NULL AND support_issued_at IS NULL AND
      support_expires_at IS NULL AND support_revocation_state IS NULL AND support_audit_identity IS NULL
    )
  )
) STRICT;

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
SELECT session_id, session_token_digest, refresh_token_digest, csrf_token_digest,
       provider, provider_subject, role, account_id, household_id, member_id, device_id,
       child_profile_id, child_device_id, authority_session_id, authority_session_generation,
       authority_generation, support_receipt_id, support_provider_subject, support_account_id,
       support_member_id, support_household_id, support_device_id, support_child_profile_id,
       support_child_device_id, support_scope, support_issuer, support_issued_at,
       support_expires_at, support_revocation_state, support_audit_identity, issued_at,
       access_expires_at, refresh_expires_at, revoke_generation, refresh_generation,
       status, last_seen_at, revoked_at, created_at, updated_at
FROM ocentra_account_browser_sessions_legacy_0007 AS session
JOIN ocentra_account_browser_sessions_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = session.rowid
WHERE validity.is_valid = 1;

DROP VIEW ocentra_account_browser_sessions_legacy_validity_0007;
DROP TABLE ocentra_account_browser_sessions_legacy_0007;

CREATE INDEX idx_ocentra_account_browser_sessions_subject
  ON ocentra_account_browser_sessions (provider, provider_subject, status);

DROP INDEX IF EXISTS idx_ocentra_account_browser_session_audit_session;
ALTER TABLE ocentra_account_browser_session_audit RENAME TO ocentra_account_browser_session_audit_legacy_0007;

CREATE VIEW ocentra_account_browser_session_audit_legacy_validity_0007 AS
SELECT
  rowid AS legacy_rowid,
  CASE
    WHEN typeof(audit_id) = 'text'
      AND length(audit_id) BETWEEN 1 AND 256
      AND typeof(session_ref_digest) = 'text'
      AND length(session_ref_digest) = 64
      AND session_ref_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(provider) = 'text'
      AND provider IN ('authjs', 'firebase')
      AND typeof(actor_ref_digest) = 'text'
      AND length(actor_ref_digest) = 64
      AND actor_ref_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(action) = 'text'
      AND action IN ('created', 'refreshed', 'logout', 'global-revoke', 'replay-rejected')
      AND typeof(result) = 'text'
      AND result IN ('accepted', 'rejected')
      AND typeof(reason) = 'text'
      AND length(reason) BETWEEN 1 AND 256
      AND typeof(correlation_id) = 'text'
      AND length(correlation_id) BETWEEN 1 AND 128
      AND typeof(occurred_at) = 'text'
      AND julianday(occurred_at) IS NOT NULL
    THEN 1
    ELSE 0
  END AS is_valid
FROM ocentra_account_browser_session_audit_legacy_0007;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT lower(hex(randomblob(16))), 'ocentra_account_browser_session_audit', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_audit_legacy_0007 AS audit
JOIN ocentra_account_browser_session_audit_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = audit.rowid
WHERE validity.is_valid = 0;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT NULL, 'ocentra_account_browser_session_audit', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_audit_legacy_0007 AS audit
JOIN ocentra_account_browser_session_audit_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = audit.rowid
WHERE validity.is_valid = 0;

CREATE TABLE ocentra_account_browser_session_audit (
  audit_id TEXT NOT NULL PRIMARY KEY CHECK (length(audit_id) BETWEEN 1 AND 256),
  session_ref_digest TEXT NOT NULL CHECK (
    length(session_ref_digest) = 64 AND session_ref_digest NOT GLOB '*[^0-9a-f]*'
  ),
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  actor_ref_digest TEXT NOT NULL CHECK (
    length(actor_ref_digest) = 64 AND actor_ref_digest NOT GLOB '*[^0-9a-f]*'
  ),
  action TEXT NOT NULL CHECK (action IN ('created', 'refreshed', 'logout', 'global-revoke', 'replay-rejected')),
  result TEXT NOT NULL CHECK (result IN ('accepted', 'rejected')),
  reason TEXT NOT NULL CHECK (length(reason) BETWEEN 1 AND 256),
  correlation_id TEXT NOT NULL CHECK (length(correlation_id) BETWEEN 1 AND 128),
  occurred_at TEXT NOT NULL CHECK (julianday(occurred_at) IS NOT NULL)
) STRICT;

INSERT INTO ocentra_account_browser_session_audit (
  audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at
)
SELECT audit_id, session_ref_digest, provider, actor_ref_digest, action, result, reason, correlation_id, occurred_at
FROM ocentra_account_browser_session_audit_legacy_0007 AS audit
JOIN ocentra_account_browser_session_audit_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = audit.rowid
WHERE validity.is_valid = 1;

DROP VIEW ocentra_account_browser_session_audit_legacy_validity_0007;
DROP TABLE ocentra_account_browser_session_audit_legacy_0007;

CREATE INDEX idx_ocentra_account_browser_session_audit_session
  ON ocentra_account_browser_session_audit (session_ref_digest, occurred_at);

DROP INDEX IF EXISTS idx_ocentra_account_browser_session_consumed_refresh_session;
ALTER TABLE ocentra_account_browser_session_consumed_refresh
  RENAME TO ocentra_account_browser_session_consumed_refresh_legacy_0007;

CREATE VIEW ocentra_account_browser_session_consumed_refresh_legacy_validity_0007 AS
SELECT
  rowid AS legacy_rowid,
  CASE
    WHEN typeof(refresh_token_digest) = 'text'
      AND length(refresh_token_digest) = 64
      AND refresh_token_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(session_id) = 'text'
      AND length(session_id) = 43
      AND session_id NOT GLOB '*[^A-Za-z0-9_-]*'
      AND typeof(refresh_generation) = 'integer'
      AND refresh_generation > 0
      AND refresh_generation <= 9007199254740991
      AND typeof(consumed_at) = 'text'
      AND julianday(consumed_at) IS NOT NULL
    THEN 1
    ELSE 0
  END AS is_valid
FROM ocentra_account_browser_session_consumed_refresh_legacy_0007;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT lower(hex(randomblob(16))), 'ocentra_account_browser_session_consumed_refresh', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_consumed_refresh_legacy_0007 AS consumed
JOIN ocentra_account_browser_session_consumed_refresh_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = consumed.rowid
WHERE validity.is_valid = 0;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT NULL, 'ocentra_account_browser_session_consumed_refresh', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_consumed_refresh_legacy_0007 AS consumed
JOIN ocentra_account_browser_session_consumed_refresh_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = consumed.rowid
WHERE validity.is_valid = 0;

CREATE TABLE ocentra_account_browser_session_consumed_refresh (
  refresh_token_digest TEXT NOT NULL PRIMARY KEY CHECK (
    length(refresh_token_digest) = 64 AND refresh_token_digest NOT GLOB '*[^0-9a-f]*'
  ),
  session_id TEXT NOT NULL CHECK (
    length(session_id) = 43 AND session_id NOT GLOB '*[^A-Za-z0-9_-]*'
  ),
  refresh_generation INTEGER NOT NULL CHECK (
    refresh_generation > 0 AND refresh_generation <= 9007199254740991
  ),
  consumed_at TEXT NOT NULL CHECK (julianday(consumed_at) IS NOT NULL)
) STRICT;

INSERT INTO ocentra_account_browser_session_consumed_refresh (
  refresh_token_digest, session_id, refresh_generation, consumed_at
)
SELECT consumed.refresh_token_digest, consumed.session_id, consumed.refresh_generation, consumed.consumed_at
FROM ocentra_account_browser_session_consumed_refresh_legacy_0007 AS consumed
JOIN ocentra_account_browser_session_consumed_refresh_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = consumed.rowid
WHERE validity.is_valid = 1;

DROP VIEW ocentra_account_browser_session_consumed_refresh_legacy_validity_0007;
DROP TABLE ocentra_account_browser_session_consumed_refresh_legacy_0007;

CREATE INDEX idx_ocentra_account_browser_session_consumed_refresh_session
  ON ocentra_account_browser_session_consumed_refresh (session_id, consumed_at);

DROP INDEX IF EXISTS idx_ocentra_account_browser_session_revoke_outcomes_scope;
ALTER TABLE ocentra_account_browser_session_revoke_outcomes
  RENAME TO ocentra_account_browser_session_revoke_outcomes_legacy_0007;

CREATE VIEW ocentra_account_browser_session_revoke_outcomes_legacy_validity_0007 AS
SELECT
  rowid AS legacy_rowid,
  CASE
    WHEN typeof(outcome_id) = 'text'
      AND length(outcome_id) BETWEEN 1 AND 256
      AND typeof(provider) = 'text'
      AND provider IN ('authjs', 'firebase')
      AND typeof(scope_ref_digest) = 'text'
      AND length(scope_ref_digest) = 64
      AND scope_ref_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(actor_ref_digest) = 'text'
      AND length(actor_ref_digest) = 64
      AND actor_ref_digest NOT GLOB '*[^0-9a-f]*'
      AND typeof(action) = 'text'
      AND action IN ('global-revoke', 'refresh-replay')
      AND typeof(result) = 'text'
      AND result IN ('accepted', 'rejected')
      AND typeof(reason) = 'text'
      AND length(reason) BETWEEN 1 AND 256
      AND typeof(revoke_generation) = 'integer'
      AND revoke_generation > 0
      AND revoke_generation <= 9007199254740991
      AND typeof(correlation_id) = 'text'
      AND length(correlation_id) BETWEEN 1 AND 128
      AND typeof(occurred_at) = 'text'
      AND julianday(occurred_at) IS NOT NULL
    THEN 1
    ELSE 0
  END AS is_valid
FROM ocentra_account_browser_session_revoke_outcomes_legacy_0007;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT lower(hex(randomblob(16))), 'ocentra_account_browser_session_revoke_outcomes', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_revoke_outcomes_legacy_0007 AS outcome
JOIN ocentra_account_browser_session_revoke_outcomes_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = outcome.rowid
WHERE validity.is_valid = 0;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT NULL, 'ocentra_account_browser_session_revoke_outcomes', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_revoke_outcomes_legacy_0007 AS outcome
JOIN ocentra_account_browser_session_revoke_outcomes_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = outcome.rowid
WHERE validity.is_valid = 0;

CREATE TABLE ocentra_account_browser_session_revoke_outcomes (
  outcome_id TEXT NOT NULL PRIMARY KEY CHECK (length(outcome_id) BETWEEN 1 AND 256),
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  scope_ref_digest TEXT NOT NULL CHECK (
    length(scope_ref_digest) = 64 AND scope_ref_digest NOT GLOB '*[^0-9a-f]*'
  ),
  actor_ref_digest TEXT NOT NULL CHECK (
    length(actor_ref_digest) = 64 AND actor_ref_digest NOT GLOB '*[^0-9a-f]*'
  ),
  action TEXT NOT NULL CHECK (action IN ('global-revoke', 'refresh-replay')),
  result TEXT NOT NULL CHECK (result IN ('accepted', 'rejected')),
  reason TEXT NOT NULL CHECK (length(reason) BETWEEN 1 AND 256),
  revoke_generation INTEGER NOT NULL CHECK (
    revoke_generation > 0 AND revoke_generation <= 9007199254740991
  ),
  correlation_id TEXT NOT NULL CHECK (length(correlation_id) BETWEEN 1 AND 128),
  occurred_at TEXT NOT NULL CHECK (julianday(occurred_at) IS NOT NULL)
) STRICT;

INSERT INTO ocentra_account_browser_session_revoke_outcomes (
  outcome_id, provider, scope_ref_digest, actor_ref_digest, action, result, reason,
  revoke_generation, correlation_id, occurred_at
)
SELECT outcome_id, provider, scope_ref_digest, actor_ref_digest, action, result, reason,
       revoke_generation, correlation_id, occurred_at
FROM ocentra_account_browser_session_revoke_outcomes_legacy_0007 AS outcome
JOIN ocentra_account_browser_session_revoke_outcomes_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = outcome.rowid
WHERE validity.is_valid = 1;

DROP VIEW ocentra_account_browser_session_revoke_outcomes_legacy_validity_0007;
DROP TABLE ocentra_account_browser_session_revoke_outcomes_legacy_0007;

CREATE INDEX idx_ocentra_account_browser_session_revoke_outcomes_scope
  ON ocentra_account_browser_session_revoke_outcomes (scope_ref_digest, occurred_at);

DROP INDEX IF EXISTS idx_ocentra_account_browser_session_fences_subject;
ALTER TABLE ocentra_account_browser_session_fences RENAME TO ocentra_account_browser_session_fences_legacy_0007;

CREATE VIEW ocentra_account_browser_session_fences_legacy_validity_0007 AS
SELECT
  rowid AS legacy_rowid,
  CASE
    WHEN typeof(provider) = 'text'
      AND provider IN ('authjs', 'firebase')
      AND typeof(provider_subject) = 'text'
      AND length(provider_subject) BETWEEN 1 AND 256
      AND typeof(revoke_generation) = 'integer'
      AND revoke_generation > 0
      AND revoke_generation <= 9007199254740991
      AND typeof(updated_at) = 'text'
      AND julianday(updated_at) IS NOT NULL
    THEN 1
    ELSE 0
  END AS is_valid
FROM ocentra_account_browser_session_fences_legacy_0007;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT lower(hex(randomblob(16))), 'ocentra_account_browser_session_fences', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_fences_legacy_0007 AS fence
JOIN ocentra_account_browser_session_fences_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = fence.rowid
WHERE validity.is_valid = 0;

INSERT INTO ocentra_account_browser_session_schema_quarantine
  (quarantine_id, source_table, reason, quarantined_at)
SELECT NULL, 'ocentra_account_browser_session_fences', 'invalid-legacy-row',
       strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
FROM ocentra_account_browser_session_fences_legacy_0007 AS fence
JOIN ocentra_account_browser_session_fences_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = fence.rowid
WHERE validity.is_valid = 0;

CREATE TABLE ocentra_account_browser_session_fences (
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL CHECK (length(provider_subject) BETWEEN 1 AND 256),
  revoke_generation INTEGER NOT NULL CHECK (
    revoke_generation > 0 AND revoke_generation <= 9007199254740991
  ),
  updated_at TEXT NOT NULL CHECK (julianday(updated_at) IS NOT NULL),
  PRIMARY KEY (provider, provider_subject)
) STRICT;

INSERT INTO ocentra_account_browser_session_fences
  (provider, provider_subject, revoke_generation, updated_at)
SELECT fence.provider, fence.provider_subject, fence.revoke_generation, fence.updated_at
FROM ocentra_account_browser_session_fences_legacy_0007 AS fence
JOIN ocentra_account_browser_session_fences_legacy_validity_0007 AS validity
  ON validity.legacy_rowid = fence.rowid
WHERE validity.is_valid = 1;

DROP VIEW ocentra_account_browser_session_fences_legacy_validity_0007;
DROP TABLE ocentra_account_browser_session_fences_legacy_0007;

-- This sentinel is the runtime gate. BrowserSessionStore refuses every
-- authority-bearing read or mutation until the complete forward migration
-- has committed and this exact version is present.
CREATE TABLE IF NOT EXISTS ocentra_account_browser_session_schema (
  schema_name TEXT NOT NULL PRIMARY KEY CHECK (schema_name = 'browser-session-custody'),
  schema_version INTEGER NOT NULL CHECK (schema_version = 7),
  applied_at TEXT NOT NULL CHECK (julianday(applied_at) IS NOT NULL)
) STRICT;

INSERT INTO ocentra_account_browser_session_schema (schema_name, schema_version, applied_at)
VALUES ('browser-session-custody', 7, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
ON CONFLICT(schema_name) DO UPDATE SET schema_version = excluded.schema_version, applied_at = excluded.applied_at;
