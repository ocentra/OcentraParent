use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;
use rusqlite::{params, Connection, TransactionBehavior};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

const ADD_EXPIRY_NANOS_COLUMN: &str =
    "ALTER TABLE authenticated_delivery_grant_consumes_v2 ADD COLUMN expires_at_nanos INTEGER";
const SELECT_UNINDEXED_GRANTS: &str = "SELECT issuer_key_id, nonce, grant_json FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos IS NULL";
const UPDATE_GRANT_EXPIRY: &str = "UPDATE authenticated_delivery_grant_consumes_v2 SET expires_at_nanos = ?3 WHERE issuer_key_id = ?1 AND nonce = ?2";
const CREATE_EXPIRY_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_consumes_v2_expiry_idx ON authenticated_delivery_grant_consumes_v2 (expires_at_nanos, issuer_key_id, nonce)";
const CREATE_AUDIT_GRANT_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_grant_idx ON authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce)";
const SELECT_EXPIRED_GRANTS: &str = "SELECT issuer_key_id, nonce FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos <= ?1 ORDER BY expires_at_nanos LIMIT ?2";
const DELETE_CONSUMED_GRANT: &str =
    "DELETE FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const DELETE_GRANT_AUDITS: &str =
    "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE: i64 = 128;

pub(super) fn ensure_retention_indexes(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let has_expiry_nanos_column = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["expires_at_nanos"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    if !has_expiry_nanos_column {
        connection
            .execute(ADD_EXPIRY_NANOS_COLUMN, [])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    let unindexed = {
        let mut statement = connection
            .prepare(SELECT_UNINDEXED_GRANTS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?
    };
    for (issuer_key_id, nonce, grant_json) in unindexed {
        let grant: AuthenticatedDeliveryGrant = serde_json::from_str(&grant_json)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let expires_at_nanos = grant
            .expires_at
            .parse::<chrono::DateTime<chrono::FixedOffset>>()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?
            .timestamp_nanos_opt()
            .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        connection
            .execute(
                UPDATE_GRANT_EXPIRY,
                params![issuer_key_id, nonce, expires_at_nanos],
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    connection
        .execute(CREATE_EXPIRY_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_AUDIT_GRANT_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

pub(super) fn purge_expired_replay_records(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    purge_expired_replay_record_batch(connection, trusted_now_nanos).map(|_count| ())
}

pub(super) fn drain_expired_replay_records_at_startup(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    while purge_expired_replay_record_batch(connection, trusted_now_nanos)?
        == MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE as usize
    {}
    Ok(())
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
            .execute(DELETE_GRANT_AUDITS, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(count)
}
