use ocentra_schema::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
};
use rusqlite::{params, Connection, Transaction};

use crate::authenticated_delivery_grant::{
    sqlite_contention::immediate_transaction_with_contention_retry,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantTrustedIssuer,
};

use super::{
    parse_legacy_replay_grant, CREATE_FINGERPRINT_REPLAY_TABLE, DROP_LEGACY_REPLAY_TABLE,
    INSERT_FINGERPRINT_REPLAY_ROW, RENAME_FINGERPRINT_REPLAY_TABLE,
};

const SELECT_LEGACY_REPLAY_ROW_METADATA: &str = "SELECT rowid, length(CAST(issuer_key_id AS BLOB)), length(CAST(nonce AS BLOB)), length(CAST(grant_json AS BLOB)) FROM authenticated_delivery_grant_consumes_v2 WHERE rowid > ?1 ORDER BY rowid LIMIT ?2";
const SELECT_LEGACY_REPLAY_ROW: &str =
    "SELECT issuer_key_id, nonce, grant_json FROM authenticated_delivery_grant_consumes_v2 WHERE rowid = ?1";
const SELECT_LEGACY_REPLAY_AUDIT_JSON_BYTES: &str =
    "SELECT length(CAST(audit_json AS BLOB)) FROM authenticated_delivery_grant_consumes_v2 WHERE rowid = ?1";
const SELECT_LEGACY_REPLAY_AUDIT_JSON: &str =
    "SELECT audit_json FROM authenticated_delivery_grant_consumes_v2 WHERE rowid = ?1";
const MAX_LEGACY_REPLAY_ROWS_PER_MIGRATION_BATCH: i64 = 128;
const MAX_LEGACY_REPLAY_GRANT_JSON_BYTES: usize = AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES
    * 6
    + AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES * 8;
const MAX_LEGACY_REPLAY_AUDIT_JSON_BYTES: usize = AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES * 8;

pub(super) fn migrate_legacy_replay_records(
    connection: &Connection,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let transaction = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let has_legacy_json = transaction
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["grant_json"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    if !has_legacy_json {
        return transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable);
    }
    transaction
        .execute(CREATE_FINGERPRINT_REPLAY_TABLE, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let mut last_row_id = i64::MIN;
    loop {
        let legacy_rows = legacy_replay_row_metadata(&transaction, last_row_id)?;
        if legacy_rows.is_empty() {
            break;
        }
        for legacy_row in legacy_rows {
            legacy_row.validate_raw_payload_bounds()?;
            let (issuer_key_id, nonce, grant_json) =
                legacy_replay_row(&transaction, legacy_row.row_id)?;
            last_row_id = legacy_row.row_id;
            let migration = parse_legacy_replay_grant(&grant_json, trusted_issuer)?;
            if migration.issuer_key_id != issuer_key_id || migration.nonce != nonce {
                return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
            }
            let expires_at_nanos = migration
                .expires_at
                .parse::<chrono::DateTime<chrono::FixedOffset>>()
                .ok()
                .and_then(|instant| instant.timestamp_nanos_opt())
                .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
            let audit_json = legacy_replay_audit_json(&transaction, legacy_row.row_id)?;
            transaction
                .execute(
                    INSERT_FINGERPRINT_REPLAY_ROW,
                    params![
                        issuer_key_id,
                        nonce,
                        migration.replay_fingerprint,
                        audit_json,
                        expires_at_nanos
                    ],
                )
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        }
    }
    transaction
        .execute(DROP_LEGACY_REPLAY_TABLE, [])
        .and_then(|_| transaction.execute(RENAME_FINGERPRINT_REPLAY_TABLE, []))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
}

pub(super) fn legacy_replay_row_metadata(
    transaction: &Transaction<'_>,
    last_row_id: i64,
) -> Result<Vec<LegacyReplayRowMetadata>, AuthenticatedDeliveryGrantConsumeError> {
    let mut statement = transaction
        .prepare(SELECT_LEGACY_REPLAY_ROW_METADATA)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let rows = statement
        .query_map(
            params![last_row_id, MAX_LEGACY_REPLAY_ROWS_PER_MIGRATION_BATCH],
            |row| {
                Ok(LegacyReplayRowMetadata {
                    row_id: row.get(0)?,
                    issuer_key_id_bytes: row.get(1)?,
                    nonce_bytes: row.get(2)?,
                    grant_json_bytes: row.get(3)?,
                })
            },
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
}

pub(super) fn legacy_replay_row(
    transaction: &Transaction<'_>,
    row_id: i64,
) -> Result<(String, String, String), AuthenticatedDeliveryGrantConsumeError> {
    transaction
        .query_row(SELECT_LEGACY_REPLAY_ROW, [row_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
}

fn legacy_replay_audit_json(
    transaction: &Transaction<'_>,
    row_id: i64,
) -> Result<String, AuthenticatedDeliveryGrantConsumeError> {
    let bytes: i64 = transaction
        .query_row(SELECT_LEGACY_REPLAY_AUDIT_JSON_BYTES, [row_id], |row| {
            row.get(0)
        })
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    (0..=MAX_LEGACY_REPLAY_AUDIT_JSON_BYTES as i64)
        .contains(&bytes)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    transaction
        .query_row(SELECT_LEGACY_REPLAY_AUDIT_JSON, [row_id], |row| row.get(0))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
}

pub(super) struct LegacyReplayRowMetadata {
    pub(super) row_id: i64,
    issuer_key_id_bytes: i64,
    nonce_bytes: i64,
    grant_json_bytes: i64,
}

impl LegacyReplayRowMetadata {
    pub(super) fn validate_raw_payload_bounds(
        &self,
    ) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
        let payload_sizes = [
            self.issuer_key_id_bytes,
            self.nonce_bytes,
            self.grant_json_bytes,
        ];
        let payload_limits = [
            AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES as i64,
            AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES as i64,
            MAX_LEGACY_REPLAY_GRANT_JSON_BYTES as i64,
        ];
        payload_sizes
            .into_iter()
            .zip(payload_limits)
            .all(|(size, limit)| size <= limit)
            .then_some(())
            .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
    }
}
