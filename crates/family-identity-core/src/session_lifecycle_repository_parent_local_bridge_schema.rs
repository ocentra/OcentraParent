#![forbid(unsafe_code)]

//! Versioned Account-owned schema for parent-local bridge custody.

use rusqlite::Connection;

#[path = "session_lifecycle_repository_parent_local_bridge_schema_migration.rs"]
mod migration;
#[path = "session_lifecycle_repository_parent_local_bridge_schema_v1.rs"]
mod v1;
#[path = "session_lifecycle_repository_parent_local_bridge_schema_v1_rows.rs"]
mod v1_rows;
#[path = "session_lifecycle_repository_parent_local_bridge_schema_validation.rs"]
mod validation;

pub(super) const BRIDGE_SCHEMA_VERSION: i64 = 2;

pub(super) const BRIDGE_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_schema (
         schema_id INTEGER NOT NULL PRIMARY KEY CHECK (schema_id = 1),
         schema_version INTEGER NOT NULL CHECK (schema_version = 2)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_revoke_epoch (
         account_id TEXT NOT NULL PRIMARY KEY CHECK (length(trim(account_id)) > 0),
         epoch INTEGER NOT NULL CHECK (epoch > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_session (
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
             OR (state != 'active' AND last_transition_at_epoch_millis > issued_at_epoch_millis)),
         FOREIGN KEY (account_id)
           REFERENCES account_identity_parent_local_bridge_revoke_epoch(account_id)
           ON UPDATE RESTRICT ON DELETE RESTRICT
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_account
       ON account_identity_parent_local_bridge_session(account_id);
     CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_audit_outbox (
         sequence INTEGER PRIMARY KEY AUTOINCREMENT,
         event_id TEXT NOT NULL UNIQUE
           CHECK (length(event_id) = 78
             AND event_id GLOB 'session-audit-*'
             AND substr(event_id, 15) NOT GLOB '*[^0-9a-f]*'),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
         provider_subject_digest TEXT NOT NULL
           CHECK (length(provider_subject_digest) = 64
             AND provider_subject_digest NOT GLOB '*[^0-9a-f]*'),
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
           CHECK (retain_until_epoch_millis = occurred_at_epoch_millis + 2592000000),
         delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','in-flight','delivered')),
         delivery_attempt_id TEXT,
         delivery_attempt_count INTEGER NOT NULL CHECK (delivery_attempt_count >= 0),
         delivery_claimed_at_epoch_millis INTEGER,
         delivery_lease_expires_at_epoch_millis INTEGER,
         next_delivery_at_epoch_millis INTEGER NOT NULL
           CHECK (next_delivery_at_epoch_millis >= occurred_at_epoch_millis),
         delivered_at_epoch_millis INTEGER,
         CHECK ((delivery_state = 'pending' AND delivery_attempt_id IS NULL
                 AND delivery_claimed_at_epoch_millis IS NULL
                 AND delivery_lease_expires_at_epoch_millis IS NULL
                 AND delivered_at_epoch_millis IS NULL)
             OR (delivery_state = 'in-flight'
                AND length(delivery_attempt_id) = 81
                AND delivery_attempt_id GLOB 'delivery-attempt-*'
                AND substr(delivery_attempt_id, 18) NOT GLOB '*[^0-9a-f]*'
                AND delivery_attempt_count > 0
                AND delivery_claimed_at_epoch_millis >= occurred_at_epoch_millis
                AND delivery_lease_expires_at_epoch_millis > delivery_claimed_at_epoch_millis
                AND delivered_at_epoch_millis IS NULL)
             OR (delivery_state = 'delivered' AND delivery_attempt_id IS NULL
                 AND delivery_claimed_at_epoch_millis IS NULL
                 AND delivery_lease_expires_at_epoch_millis IS NULL
                 AND delivered_at_epoch_millis >= occurred_at_epoch_millis)),
         FOREIGN KEY (account_id)
           REFERENCES account_identity_parent_local_bridge_revoke_epoch(account_id)
           ON UPDATE RESTRICT ON DELETE RESTRICT
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_audit_delivery
       ON account_identity_parent_local_bridge_audit_outbox(
           delivery_state, next_delivery_at_epoch_millis, sequence
       );
     CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_audit_retention
       ON account_identity_parent_local_bridge_audit_outbox(retain_until_epoch_millis);";

pub(crate) fn initialize(
    connection: &mut Connection,
    delivery_lease_millis: i64,
) -> Result<(), ()> {
    if delivery_lease_millis <= 0 {
        return Err(());
    }
    migration::initialize_or_migrate(connection, delivery_lease_millis)?;
    validation::validate(connection)
}
