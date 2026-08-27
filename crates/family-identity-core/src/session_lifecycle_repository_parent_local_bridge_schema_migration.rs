#![forbid(unsafe_code)]

use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior};

use super::{validation, BRIDGE_SCHEMA_SQL, BRIDGE_SCHEMA_VERSION};

const SCHEMA_TABLE: &str = "account_identity_parent_local_bridge_schema";
const REVOKE_TABLE: &str = "account_identity_parent_local_bridge_revoke_epoch";
const SESSION_TABLE: &str = "account_identity_parent_local_bridge_session";
const AUDIT_TABLE: &str = "account_identity_parent_local_bridge_audit_outbox";

pub(super) fn initialize_or_migrate(connection: &mut Connection) -> Result<(), ()> {
    // Serialize the complete inspect/validate/replace decision. In particular,
    // legacy audit preservation must be checked while the same write lock that
    // protects the subsequent DROP/CREATE is held; a check followed by a later
    // transaction can lose a row inserted in between those operations.
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ())?;
    initialize_or_migrate_in_transaction(&transaction)?;
    transaction.commit().map_err(|_| ())
}

fn initialize_or_migrate_in_transaction(transaction: &Transaction<'_>) -> Result<(), ()> {
    reject_bridge_triggers_and_views(transaction)?;
    let schema_exists = table_exists(transaction, SCHEMA_TABLE)?;
    let existing = [
        table_exists(transaction, REVOKE_TABLE)?,
        table_exists(transaction, SESSION_TABLE)?,
        table_exists(transaction, AUDIT_TABLE)?,
    ];
    if existing.iter().all(|exists| !exists) {
        if schema_exists {
            return Err(());
        }
        return create_fresh(transaction);
    }
    if existing.iter().any(|exists| !exists) {
        return Err(());
    }
    if schema_exists {
        match read_schema_version(transaction)? {
            Some(version) if version == BRIDGE_SCHEMA_VERSION => {
                return validation::validate(transaction);
            }
            Some(2) => {
                return migrate_v2(transaction, true);
            }
            Some(_) => return Err(()),
            None => {
                return migrate_v2(transaction, false);
            }
        }
    }
    (!schema_exists).then_some(()).ok_or(())?;
    migrate_v1(transaction)
}

fn create_fresh(transaction: &Transaction<'_>) -> Result<(), ()> {
    transaction
        .execute_batch(BRIDGE_SCHEMA_SQL)
        .map_err(|_| ())?;
    insert_version(transaction)?;
    Ok(())
}

fn migrate_v2(transaction: &Transaction<'_>, require_version: bool) -> Result<(), ()> {
    // Validation and the legacy-audit emptiness check happen under the same
    // immediate transaction as replacement, so no legacy writer can slip in.
    validation::validate_v2(transaction, require_version)?;
    require_empty_legacy_audit(transaction)?;
    transaction
        .execute_batch(
            "DROP TABLE account_identity_parent_local_bridge_audit_outbox;
             DROP TABLE account_identity_parent_local_bridge_schema;",
        )
        .map_err(|_| ())?;
    transaction
        .execute_batch(BRIDGE_SCHEMA_SQL)
        .map_err(|_| ())?;
    insert_version(transaction)?;
    validation::validate(transaction)
}

fn migrate_v1(transaction: &Transaction<'_>) -> Result<(), ()> {
    // Keep v1 shape/row validation, ambiguous-audit preservation, copying,
    // and destructive replacement in one serialized transaction.
    validate_v1_shape(transaction)?;
    require_empty_legacy_audit(transaction)?;
    transaction
        .execute_batch(
            "CREATE TEMP TABLE account_identity_parent_local_bridge_session_copy AS
                 SELECT capability_digest, digest_algorithm, capability_digest_domain,
                        audience, connection_nonce_digest, account_id, provider,
                        provider_subject, household_id, member_id, device_id,
                        authority_session_id, authority_session_generation,
                        authority_generation, authority_expires_at_epoch_millis,
                        issued_at_epoch_millis, expires_at_epoch_millis,
                        bridge_revoke_epoch, state, last_transition_at_epoch_millis
                   FROM account_identity_parent_local_bridge_session;
             DROP TABLE account_identity_parent_local_bridge_audit_outbox;
             DROP TABLE account_identity_parent_local_bridge_session;",
        )
        .map_err(|_| ())?;
    transaction
        .execute_batch(BRIDGE_SCHEMA_SQL)
        .map_err(|_| ())?;
    transaction
        .execute(
            "INSERT INTO account_identity_parent_local_bridge_session (
                 capability_digest, digest_algorithm, capability_digest_domain,
                 audience, connection_nonce_digest, account_id, provider,
                 provider_subject, household_id, member_id, device_id,
                 authority_session_id, authority_session_generation,
                 authority_generation, authority_expires_at_epoch_millis,
                 issued_at_epoch_millis, expires_at_epoch_millis,
                 bridge_revoke_epoch, state, last_transition_at_epoch_millis
             ) SELECT capability_digest, digest_algorithm, capability_digest_domain,
                      audience, connection_nonce_digest, account_id, provider,
                      provider_subject, household_id, member_id, device_id,
                      authority_session_id, authority_session_generation,
                      authority_generation, authority_expires_at_epoch_millis,
                      issued_at_epoch_millis, expires_at_epoch_millis,
                      bridge_revoke_epoch, state, last_transition_at_epoch_millis
                 FROM account_identity_parent_local_bridge_session_copy",
            [],
        )
        .map_err(|_| ())?;
    transaction
        .execute_batch("DROP TABLE account_identity_parent_local_bridge_session_copy;")
        .map_err(|_| ())?;
    insert_version(transaction)?;
    validation::validate(transaction)
}

/// Legacy audit rows do not carry either Account generation. Inferring those
/// fields from a current authority or a surviving bridge session would rewrite
/// historical custody, so a non-empty legacy outbox is preserved untouched and
/// startup fails closed for explicit owner recovery.
fn require_empty_legacy_audit(connection: &Connection) -> Result<(), ()> {
    let count = connection
        .query_row(
            "SELECT count(*) FROM account_identity_parent_local_bridge_audit_outbox",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (count == 0).then_some(()).ok_or(())
}

fn validate_v1_shape(connection: &Connection) -> Result<(), ()> {
    super::v1::validate_objects(connection)?;
    validation::validate_exact_columns(connection, REVOKE_TABLE, &["account_id", "epoch"])?;
    validation::validate_exact_columns(
        connection,
        SESSION_TABLE,
        &[
            "capability_digest",
            "digest_algorithm",
            "capability_digest_domain",
            "audience",
            "connection_nonce_digest",
            "account_id",
            "provider",
            "provider_subject",
            "household_id",
            "member_id",
            "device_id",
            "authority_session_id",
            "authority_session_generation",
            "authority_generation",
            "authority_expires_at_epoch_millis",
            "issued_at_epoch_millis",
            "expires_at_epoch_millis",
            "bridge_revoke_epoch",
            "state",
            "last_transition_at_epoch_millis",
        ],
    )?;
    validation::validate_exact_columns(
        connection,
        AUDIT_TABLE,
        &[
            "sequence",
            "event_id",
            "account_id",
            "provider",
            "provider_subject",
            "household_id",
            "member_id",
            "device_id",
            "authority_session_id",
            "audience",
            "bridge_revoke_epoch",
            "action",
            "occurred_at_epoch_millis",
            "retain_until_epoch_millis",
            "delivery_state",
            "delivery_attempt_id",
            "delivery_claimed_at_epoch_millis",
            "delivered_at_epoch_millis",
        ],
    )?;
    validation::validate_v1_indexes(connection)
}

fn insert_version(transaction: &rusqlite::Transaction<'_>) -> Result<(), ()> {
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_parent_local_bridge_schema
                 (schema_id, schema_version) VALUES (1, ?1)",
            [BRIDGE_SCHEMA_VERSION],
        )
        .map_err(|_| ())?;
    (changed == 1).then_some(()).ok_or(())
}

fn read_schema_version(connection: &Connection) -> Result<Option<i64>, ()> {
    connection
        .query_row(
            "SELECT schema_version
               FROM account_identity_parent_local_bridge_schema
              WHERE schema_id = 1 LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| ())
}

fn table_exists(connection: &Connection, table: &str) -> Result<bool, ()> {
    connection
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
            [table],
            |_| Ok(()),
        )
        .optional()
        .map(|value| value.is_some())
        .map_err(|_| ())
}

fn reject_bridge_triggers_and_views(connection: &Connection) -> Result<(), ()> {
    let count = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master
              WHERE type IN ('trigger','view')
                AND (name LIKE 'account_identity_parent_local_bridge_%'
                  OR lower(coalesce(sql,'')) LIKE '%account_identity_parent_local_bridge_%')",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (count == 0).then_some(()).ok_or(())
}
