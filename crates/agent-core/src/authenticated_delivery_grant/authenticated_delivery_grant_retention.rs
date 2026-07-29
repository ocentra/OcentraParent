use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;
use rusqlite::{params, Connection, TransactionBehavior};

use crate::authenticated_delivery_grant::{
    replay_fingerprint, AuthenticatedDeliveryGrantConsumeError,
};

const CREATE_FINGERPRINT_REPLAY_TABLE: &str = "CREATE TABLE authenticated_delivery_grant_consumes_v3 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_fingerprint TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
const SELECT_LEGACY_REPLAY_ROWS: &str = "SELECT issuer_key_id, nonce, grant_json, audit_json FROM authenticated_delivery_grant_consumes_v2";
const INSERT_FINGERPRINT_REPLAY_ROW: &str = "INSERT INTO authenticated_delivery_grant_consumes_v3 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)";
const DROP_LEGACY_REPLAY_TABLE: &str = "DROP TABLE authenticated_delivery_grant_consumes_v2";
const RENAME_FINGERPRINT_REPLAY_TABLE: &str = "ALTER TABLE authenticated_delivery_grant_consumes_v3 RENAME TO authenticated_delivery_grant_consumes_v2";
const CREATE_EXPIRY_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_consumes_v2_expiry_idx ON authenticated_delivery_grant_consumes_v2 (expires_at_nanos, issuer_key_id, nonce)";
const CREATE_AUDIT_GRANT_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_grant_idx ON authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce)";
const SELECT_EXPIRED_GRANTS: &str = "SELECT issuer_key_id, nonce FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos <= ?1 ORDER BY expires_at_nanos LIMIT ?2";
const DELETE_CONSUMED_GRANT: &str =
    "DELETE FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const DELETE_REPLAY_AUDITS: &str =
    "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'replay'";
const MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE: i64 = 128;
// Keep a consumed-grant tombstone beyond its grant validity window so a bounded
// backward wall-clock correction cannot make the same signed grant consumable again.
const REPLAY_TOMBSTONE_CLOCK_SKEW_RETENTION_NANOS: i64 = 86_400_000_000_000;

pub(super) fn ensure_retention_indexes(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let has_fingerprint_column = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["grant_fingerprint"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    if !has_fingerprint_column {
        return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
    }
    connection
        .execute(CREATE_EXPIRY_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_AUDIT_GRANT_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

pub(super) fn migrate_legacy_replay_records(
    connection: &mut Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let has_legacy_json = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["grant_json"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    if !has_legacy_json {
        return Ok(());
    }
    let legacy_rows = {
        let mut statement = connection
            .prepare(SELECT_LEGACY_REPLAY_ROWS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?
    };
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .execute(CREATE_FINGERPRINT_REPLAY_TABLE, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    for (issuer_key_id, nonce, grant_json, audit_json) in legacy_rows {
        let grant: AuthenticatedDeliveryGrant = serde_json::from_str(&grant_json)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let expires_at_nanos = grant
            .expires_at
            .parse::<chrono::DateTime<chrono::FixedOffset>>()
            .ok()
            .and_then(|instant| instant.timestamp_nanos_opt())
            .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        transaction
            .execute(
                INSERT_FINGERPRINT_REPLAY_ROW,
                params![
                    issuer_key_id,
                    nonce,
                    replay_fingerprint(&grant),
                    audit_json,
                    expires_at_nanos
                ],
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    transaction
        .execute(DROP_LEGACY_REPLAY_TABLE, [])
        .and_then(|_| transaction.execute(RENAME_FINGERPRINT_REPLAY_TABLE, []))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
}

pub(super) fn purge_expired_replay_records(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    purge_expired_replay_record_batch(connection, replay_tombstone_cutoff(trusted_now_nanos))
        .map(|_count| ())
}

pub(super) fn drain_expired_replay_records_at_startup(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    while purge_expired_replay_record_batch(connection, replay_tombstone_cutoff(trusted_now_nanos))?
        == MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE as usize
    {}
    Ok(())
}

fn replay_tombstone_cutoff(trusted_now_nanos: i64) -> i64 {
    trusted_now_nanos.saturating_sub(REPLAY_TOMBSTONE_CLOCK_SKEW_RETENTION_NANOS)
}

fn purge_expired_replay_record_batch(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<usize, AuthenticatedDeliveryGrantConsumeError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let expired = {
        let mut statement = transaction
            .prepare(SELECT_EXPIRED_GRANTS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map(
                params![trusted_now_nanos, MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let mut expired = Vec::new();
        for row in rows {
            let (issuer_key_id, nonce) =
                row.map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            expired.push((issuer_key_id, nonce));
        }
        expired
    };
    let count = expired.len();
    for (issuer_key_id, nonce) in expired {
        transaction
            .execute(DELETE_CONSUMED_GRANT, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        transaction
            .execute(DELETE_REPLAY_AUDITS, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(count)
}
