use ocentra_parent_agent_protocol::authenticated_delivery_grant::AuthenticatedDeliveryGrant;
use rusqlite::{params, Connection};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

const ADD_EXPIRY_COLUMN: &str =
    "ALTER TABLE authenticated_delivery_grant_consumes_v2 ADD COLUMN expires_at_micros INTEGER";
const SELECT_UNINDEXED_GRANTS: &str = "SELECT issuer_key_id, nonce, grant_json FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_micros IS NULL";
const UPDATE_GRANT_EXPIRY: &str = "UPDATE authenticated_delivery_grant_consumes_v2 SET expires_at_micros = ?3 WHERE issuer_key_id = ?1 AND nonce = ?2";
const CREATE_EXPIRY_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_consumes_v2_expiry_idx ON authenticated_delivery_grant_consumes_v2 (expires_at_micros, issuer_key_id, nonce)";
const SELECT_EXPIRED_GRANTS: &str = "SELECT issuer_key_id, nonce FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_micros <= ?1 ORDER BY expires_at_micros LIMIT ?2";
const DELETE_CONSUMED_GRANT: &str =
    "DELETE FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const DELETE_GRANT_AUDITS: &str =
    "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE: i64 = 128;

pub(super) fn ensure_expiry_index(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let has_expiry_column = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["expires_at_micros"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    if !has_expiry_column {
        connection
            .execute(ADD_EXPIRY_COLUMN, [])
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
        let expires_at_micros = grant
            .expires_at
            .parse::<chrono::DateTime<chrono::FixedOffset>>()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?
            .timestamp_micros();
        connection
            .execute(
                UPDATE_GRANT_EXPIRY,
                params![issuer_key_id, nonce, expires_at_micros],
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    connection
        .execute(CREATE_EXPIRY_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

pub(super) fn purge_expired_replay_records(
    connection: &Connection,
    trusted_now_micros: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let expired = {
        let mut statement = connection
            .prepare(SELECT_EXPIRED_GRANTS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map(
                params![trusted_now_micros, MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE],
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
    for (issuer_key_id, nonce) in expired {
        connection
            .execute(DELETE_CONSUMED_GRANT, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .execute(DELETE_GRANT_AUDITS, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    Ok(())
}
