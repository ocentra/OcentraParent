#![forbid(unsafe_code)]

//! Exact legacy bridge schema accepted by the one-way v1 to v2 migration.

use rusqlite::Connection;

const V1_REVOKE_TABLE_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_revoke_epoch (
         account_id TEXT NOT NULL PRIMARY KEY CHECK (length(trim(account_id)) > 0),
         epoch INTEGER NOT NULL CHECK (epoch > 0)
     ) STRICT";

const V1_SESSION_TABLE_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_session (
         capability_digest TEXT NOT NULL PRIMARY KEY
           CHECK (length(capability_digest) = 64
             AND capability_digest NOT GLOB '*[^0-9a-f]*'),
         digest_algorithm TEXT NOT NULL CHECK (digest_algorithm = 'sha256'),
         capability_digest_domain TEXT NOT NULL CHECK (
             capability_digest_domain = 'ocentra-account-parent-local-bridge-capability-v1'
         ),
         audience TEXT NOT NULL CHECK (audience = 'parent-desktop-agent-service'),
         connection_nonce_digest TEXT NOT NULL
           CHECK (length(connection_nonce_digest) = 64
             AND connection_nonce_digest NOT GLOB '*[^0-9a-f]*'),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
         provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         authority_session_id TEXT NOT NULL CHECK (length(trim(authority_session_id)) > 0),
         authority_session_generation INTEGER NOT NULL CHECK (authority_session_generation > 0),
         authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
         authority_expires_at_epoch_millis INTEGER NOT NULL
           CHECK (authority_expires_at_epoch_millis > 0),
         issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
         expires_at_epoch_millis INTEGER NOT NULL
           CHECK (expires_at_epoch_millis > issued_at_epoch_millis
             AND expires_at_epoch_millis <= authority_expires_at_epoch_millis),
         bridge_revoke_epoch INTEGER NOT NULL CHECK (bridge_revoke_epoch > 0),
         state TEXT NOT NULL CHECK (state IN ('active','consumed','revoked')),
         last_transition_at_epoch_millis INTEGER NOT NULL,
         CHECK (last_transition_at_epoch_millis >= issued_at_epoch_millis),
         CHECK ((state = 'active' AND last_transition_at_epoch_millis = issued_at_epoch_millis)
             OR (state != 'active' AND last_transition_at_epoch_millis > issued_at_epoch_millis))
     ) STRICT";

const V1_AUDIT_TABLE_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_audit_outbox (
         sequence INTEGER PRIMARY KEY AUTOINCREMENT,
         event_id TEXT NOT NULL UNIQUE
           CHECK (length(event_id) = 78
             AND event_id GLOB 'session-audit-*'
             AND substr(event_id, 15) NOT GLOB '*[^0-9a-f]*'),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
         provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         authority_session_id TEXT NOT NULL CHECK (length(trim(authority_session_id)) > 0),
         audience TEXT NOT NULL CHECK (audience = 'parent-desktop-agent-service'),
         bridge_revoke_epoch INTEGER NOT NULL CHECK (bridge_revoke_epoch > 0),
         action TEXT NOT NULL CHECK (
             action IN ('issued','authenticated','revoked','globally-revoked')
         ),
         occurred_at_epoch_millis INTEGER NOT NULL CHECK (occurred_at_epoch_millis > 0),
         retain_until_epoch_millis INTEGER NOT NULL
           CHECK (retain_until_epoch_millis > occurred_at_epoch_millis),
         delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','in-flight','delivered')),
         delivery_attempt_id TEXT,
         delivery_claimed_at_epoch_millis INTEGER,
         delivered_at_epoch_millis INTEGER,
         CHECK ((delivery_state = 'pending' AND delivery_attempt_id IS NULL
                 AND delivery_claimed_at_epoch_millis IS NULL
                 AND delivered_at_epoch_millis IS NULL)
             OR (delivery_state = 'in-flight'
                AND length(delivery_attempt_id) = 81
                AND delivery_attempt_id GLOB 'delivery-attempt-*'
                AND substr(delivery_attempt_id, 18) NOT GLOB '*[^0-9a-f]*'
                AND delivery_claimed_at_epoch_millis >= occurred_at_epoch_millis
                AND delivered_at_epoch_millis IS NULL)
             OR (delivery_state = 'delivered' AND delivery_attempt_id IS NULL
                 AND delivery_claimed_at_epoch_millis IS NULL
                 AND delivered_at_epoch_millis >= occurred_at_epoch_millis))
     ) STRICT";

const OBJECTS: &[(&str, &str, &str)] = &[
    (
        "table",
        "account_identity_parent_local_bridge_revoke_epoch",
        V1_REVOKE_TABLE_SQL,
    ),
    (
        "table",
        "account_identity_parent_local_bridge_session",
        V1_SESSION_TABLE_SQL,
    ),
    (
        "table",
        "account_identity_parent_local_bridge_audit_outbox",
        V1_AUDIT_TABLE_SQL,
    ),
    (
        "index",
        "account_identity_parent_local_bridge_account",
        "CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_account
         ON account_identity_parent_local_bridge_session(account_id)",
    ),
    (
        "index",
        "account_identity_parent_local_bridge_audit_delivery",
        "CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_audit_delivery
         ON account_identity_parent_local_bridge_audit_outbox(delivery_state, sequence)",
    ),
    (
        "index",
        "account_identity_parent_local_bridge_audit_retention",
        "CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_audit_retention
         ON account_identity_parent_local_bridge_audit_outbox(retain_until_epoch_millis)",
    ),
];

pub(super) fn validate_objects(connection: &Connection) -> Result<(), ()> {
    require_integrity(connection)?;
    for (kind, name, expected) in OBJECTS {
        let actual = connection
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type = ?1 AND name = ?2 LIMIT 1",
                [kind, name],
                |row| row.get::<_, String>(0),
            )
            .map_err(|_| ())?;
        if normalize_sql(&actual) != normalize_sql(expected) {
            return Err(());
        }
    }
    reject_unknown_objects(connection)?;
    require_no_foreign_keys(connection)
}

fn require_integrity(connection: &Connection) -> Result<(), ()> {
    let value = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| ())?;
    (value == "ok").then_some(()).ok_or(())
}

fn reject_unknown_objects(connection: &Connection) -> Result<(), ()> {
    let unknown = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master
              WHERE type IN ('table','index','trigger','view')
                AND name LIKE 'account_identity_parent_local_bridge_%'
                AND name NOT IN (
                    'account_identity_parent_local_bridge_revoke_epoch',
                    'account_identity_parent_local_bridge_session',
                    'account_identity_parent_local_bridge_audit_outbox',
                    'account_identity_parent_local_bridge_account',
                    'account_identity_parent_local_bridge_audit_delivery',
                    'account_identity_parent_local_bridge_audit_retention'
                )",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (unknown == 0).then_some(()).ok_or(())
}

fn require_no_foreign_keys(connection: &Connection) -> Result<(), ()> {
    for table in [OBJECTS[0].1, OBJECTS[1].1, OBJECTS[2].1] {
        let count = connection
            .query_row(
                &format!("SELECT count(*) FROM pragma_foreign_key_list('{table}')"),
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| ())?;
        if count != 0 {
            return Err(());
        }
    }
    Ok(())
}

fn normalize_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect::<String>()
        .replace("IFNOTEXISTS", "")
}
