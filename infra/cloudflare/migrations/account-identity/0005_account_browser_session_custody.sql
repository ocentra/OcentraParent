-- Browser credentials are opaque bearer values.  Only SHA-256 digests are
-- retained; Account current-authority remains the source of identity and
-- household/device scope.
CREATE TABLE IF NOT EXISTS ocentra_account_browser_sessions (
  session_id TEXT PRIMARY KEY,
  session_token_digest TEXT NOT NULL UNIQUE,
  refresh_token_digest TEXT NOT NULL UNIQUE,
  csrf_token_digest TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL,
  account_id TEXT NOT NULL,
  authority_session_id TEXT NOT NULL,
  authority_session_generation INTEGER NOT NULL CHECK (
    authority_session_generation > 0 AND authority_session_generation <= 9007199254740991
  ),
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
  issued_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  refresh_generation INTEGER NOT NULL CHECK (
    refresh_generation > 0 AND refresh_generation <= 9007199254740991
  ),
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  last_seen_at TEXT NOT NULL,
  revoked_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_browser_sessions_subject
  ON ocentra_account_browser_sessions (provider, provider_subject, status);

CREATE TABLE IF NOT EXISTS ocentra_account_browser_session_audit (
  audit_id TEXT PRIMARY KEY,
  session_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL,
  action TEXT NOT NULL CHECK (action IN ('created', 'refreshed', 'logout', 'global-revoke', 'replay-rejected')),
  result TEXT NOT NULL CHECK (result IN ('accepted', 'rejected')),
  reason TEXT NOT NULL,
  correlation_id TEXT NOT NULL,
  occurred_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_browser_session_audit_session
  ON ocentra_account_browser_session_audit (session_id, occurred_at);
