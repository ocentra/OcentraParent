-- Refresh rotation custody keeps a digest-only record of every consumed
-- refresh credential. Reuse is therefore distinguishable from random input
-- without retaining bearer material or provider subjects.
CREATE TABLE IF NOT EXISTS ocentra_account_browser_session_consumed_refresh (
  refresh_token_digest TEXT PRIMARY KEY,
  session_id TEXT NOT NULL,
  refresh_generation INTEGER NOT NULL CHECK (
    refresh_generation > 0 AND refresh_generation <= 9007199254740991
  ),
  consumed_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_browser_session_consumed_refresh_session
  ON ocentra_account_browser_session_consumed_refresh (session_id, consumed_at);

-- Existing browser rows fail closed until these Account-currentness bindings
-- are present. New rows copy the complete current-authority identity and
-- support receipt so every later credential read can revalidate the owner.
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN household_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN member_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN device_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN child_profile_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN child_device_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_receipt_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_provider_subject TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_account_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_member_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_household_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_device_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_child_profile_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_child_device_id TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_scope TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_issuer TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_issued_at TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_expires_at TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_revocation_state TEXT;
ALTER TABLE ocentra_account_browser_sessions ADD COLUMN support_audit_identity TEXT;

-- Global revocation writes a durable scope outcome in the same atomic D1
-- mutation as the fence advance. The scope and actor are domain-separated
-- digests; raw provider subjects and session identifiers are never retained.
CREATE TABLE IF NOT EXISTS ocentra_account_browser_session_revoke_outcomes (
  outcome_id TEXT PRIMARY KEY,
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  scope_ref_digest TEXT NOT NULL,
  actor_ref_digest TEXT NOT NULL,
  action TEXT NOT NULL CHECK (action IN ('global-revoke', 'refresh-replay')),
  result TEXT NOT NULL CHECK (result IN ('accepted', 'rejected')),
  reason TEXT NOT NULL,
  revoke_generation INTEGER NOT NULL CHECK (
    revoke_generation > 0 AND revoke_generation <= 9007199254740991
  ),
  correlation_id TEXT NOT NULL,
  occurred_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_browser_session_revoke_outcomes_scope
  ON ocentra_account_browser_session_revoke_outcomes (scope_ref_digest, occurred_at);
