#![forbid(unsafe_code)]

use rusqlite::{Connection, OptionalExtension, TransactionBehavior};

use super::{validation, BRIDGE_SCHEMA_SQL, BRIDGE_SCHEMA_VERSION};

const SCHEMA_TABLE: &str = "account_identity_parent_local_bridge_schema";
const REVOKE_TABLE: &str = "account_identity_parent_local_bridge_revoke_epoch";
const SESSION_TABLE: &str = "account_identity_parent_local_bridge_session";
const AUDIT_TABLE: &str = "account_identity_parent_local_bridge_audit_outbox";

pub(super) fn initialize_or_migrate(
    connection: &mut Connection,
    delivery_lease_millis: i64,
) -> Result<(), ()> {
    reject_bridge_triggers_and_views(connection)?;
    let schema_exists = table_exists(connection, SCHEMA_TABLE)?;
    let existing = [
        table_exists(connection, REVOKE_TABLE)?,
        table_exists(connection, SESSION_TABLE)?,
        table_exists(connection, AUDIT_TABLE)?,
    ];
    if existing.iter().all(|exists| !exists) {
        if schema_exists {
            return Err(());
        }
        return create_fresh(connection);
    }
    if existing.iter().any(|exists| !exists) {
        return Err(());
    }
    if schema_exists {
        if read_schema_version(connection)? == Some(BRIDGE_SCHEMA_VERSION) {
            return validation::validate(connection);
        }
        if read_schema_version(connection)?.is_some() {
            return Err(());
        }
    }
    if validation::v2_shape_without_version(connection).is_ok() {
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| ())?;
        transaction
            .execute_batch(BRIDGE_SCHEMA_SQL)
            .map_err(|_| ())?;
        insert_version(&transaction)?;
        transaction.commit().map_err(|_| ())?;
        return Ok(());
    }
    (!schema_exists).then_some(()).ok_or(())?;
    validate_v1_shape(connection)?;
    migrate_v1(connection, delivery_lease_millis)
}

fn create_fresh(connection: &mut Connection) -> Result<(), ()> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ())?;
    transaction
        .execute_batch(BRIDGE_SCHEMA_SQL)
        .map_err(|_| ())?;
    insert_version(&transaction)?;
    transaction.commit().map_err(|_| ())
}

fn migrate_v1(connection: &mut Connection, delivery_lease_millis: i64) -> Result<(), ()> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ())?;
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
             CREATE TEMP TABLE account_identity_parent_local_bridge_audit_copy AS
                 SELECT sequence, event_id, account_id, provider, provider_subject,
                        household_id, member_id, device_id, authority_session_id,
                        audience, bridge_revoke_epoch, action,
                        occurred_at_epoch_millis, retain_until_epoch_millis,
                        delivery_state, delivery_attempt_id,
                        delivery_claimed_at_epoch_millis, delivered_at_epoch_millis
                   FROM account_identity_parent_local_bridge_audit_outbox;
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
    super::v1_rows::migrate_audit_rows(&transaction, delivery_lease_millis)?;
    transaction
        .execute_batch(
            "DROP TABLE account_identity_parent_local_bridge_audit_copy;
             DROP TABLE account_identity_parent_local_bridge_session_copy;",
        )
        .map_err(|_| ())?;
    insert_version(&transaction)?;
    validation::validate(&transaction)?;
    transaction.commit().map_err(|_| ())
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
