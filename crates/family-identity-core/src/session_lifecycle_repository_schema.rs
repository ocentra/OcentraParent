#![forbid(unsafe_code)]

pub(crate) const SESSION_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_session (
         token_digest TEXT NOT NULL PRIMARY KEY CHECK (length(token_digest) = 64),
         session_id TEXT NOT NULL UNIQUE CHECK (length(session_id) > 0),
         account_user_id TEXT NOT NULL CHECK (length(account_user_id) > 0),
         refresh_family_id TEXT NOT NULL CHECK (length(refresh_family_id) > 0),
         refresh_generation INTEGER NOT NULL CHECK (refresh_generation > 0),
         issued_at TEXT NOT NULL CHECK (length(issued_at) > 0),
         expires_at TEXT NOT NULL CHECK (length(expires_at) > 0),
         activity_state TEXT NOT NULL CHECK (activity_state IN ('active','logged-out','revoked','globally-revoked')),
         freshness_state TEXT NOT NULL CHECK (freshness_state IN ('fresh','stale','expired')),
         global_revoke_epoch INTEGER NOT NULL CHECK (global_revoke_epoch > 0),
         last_transition_at TEXT NOT NULL CHECK (length(last_transition_at) > 0)
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_session_account
       ON account_identity_session(account_user_id);
     CREATE TABLE IF NOT EXISTS account_identity_session_revoke_epoch (
         account_user_id TEXT NOT NULL PRIMARY KEY CHECK (length(account_user_id) > 0),
         epoch INTEGER NOT NULL CHECK (epoch > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_session_audit_outbox (
         sequence INTEGER PRIMARY KEY AUTOINCREMENT,
         event_id TEXT NOT NULL UNIQUE CHECK (length(event_id) > 0),
         session_id TEXT NOT NULL CHECK (length(session_id) > 0),
         account_user_id TEXT NOT NULL CHECK (length(account_user_id) > 0),
         action TEXT NOT NULL CHECK (action IN ('created','rotated','logged-out','revoked','globally-revoked')),
         occurred_at TEXT NOT NULL CHECK (length(occurred_at) > 0),
         delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','delivered'))
     ) STRICT;";
