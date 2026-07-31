use ed25519_dalek::Signature;
use ocentra_schema::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_PAYLOAD_DIGEST_HEX_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Deserialize;

use crate::authenticated_delivery_grant::storage_keys::storage_key_digest;
use crate::authenticated_delivery_grant::{
    digest, sqlite_contention::immediate_transaction_with_contention_retry,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantTrustedIssuer,
};

mod clock;
mod legacy_replay_rows;

const CREATE_FINGERPRINT_REPLAY_TABLE: &str = "CREATE TABLE authenticated_delivery_grant_consumes_v3 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_fingerprint TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
const INSERT_FINGERPRINT_REPLAY_ROW: &str = "INSERT INTO authenticated_delivery_grant_consumes_v3 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)";
const DROP_LEGACY_REPLAY_TABLE: &str = "DROP TABLE authenticated_delivery_grant_consumes_v2";
const RENAME_FINGERPRINT_REPLAY_TABLE: &str = "ALTER TABLE authenticated_delivery_grant_consumes_v3 RENAME TO authenticated_delivery_grant_consumes_v2";
const CREATE_EXPIRY_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_consumes_v2_expiry_idx ON authenticated_delivery_grant_consumes_v2 (expires_at_nanos, issuer_key_id, nonce)";
const CREATE_AUDIT_GRANT_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_grant_idx ON authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce)";
const CREATE_VALIDATION_REJECTION_RETENTION_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_validation_rejection_retention_idx ON authenticated_delivery_grant_audits_v2 (audit_scope, recorded_at_nanos DESC)";
const CREATE_REPLAY_RETENTION_CLOCK: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_replay_retention_v1 (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), highest_trusted_now_nanos INTEGER NOT NULL)";
const CREATE_CONFIRMED_REPLAY_RETENTION_CLOCK: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_replay_retention_v2 (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), highest_trusted_now_nanos INTEGER NOT NULL, confirmed INTEGER NOT NULL CHECK (confirmed IN (0, 1)))";
const MIGRATE_REPLAY_RETENTION_CLOCK: &str = "INSERT OR IGNORE INTO authenticated_delivery_grant_replay_retention_v2 (singleton, highest_trusted_now_nanos, confirmed) SELECT singleton, highest_trusted_now_nanos, 1 FROM authenticated_delivery_grant_replay_retention_v1";
const CREATE_PROVISIONAL_REPLAY_RETENTION_CLOCK: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_replay_retention_v3 (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), highest_trusted_now_nanos INTEGER NOT NULL, confirmed INTEGER NOT NULL CHECK (confirmed IN (0, 1)), provisional_observed_at_nanos INTEGER)";
const MIGRATE_PROVISIONAL_REPLAY_RETENTION_CLOCK: &str = "INSERT OR IGNORE INTO authenticated_delivery_grant_replay_retention_v3 (singleton, highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos) SELECT singleton, highest_trusted_now_nanos, confirmed, CASE confirmed WHEN 0 THEN highest_trusted_now_nanos ELSE NULL END FROM authenticated_delivery_grant_replay_retention_v2";
const SELECT_EXPIRED_GRANTS: &str = "SELECT issuer_key_id, nonce FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos <= ?1 ORDER BY expires_at_nanos LIMIT ?2";
const DELETE_CONSUMED_GRANT: &str =
    "DELETE FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const DELETE_REPLAY_AUDITS: &str =
    "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'replay'";
const MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE: i64 = 128;
const CREATE_STORAGE_PRIVACY_MARKER: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_storage_privacy_v1 (singleton INTEGER PRIMARY KEY CHECK (singleton = 1))";
const STORAGE_PRIVACY_MARKER_EXISTS: &str =
    "SELECT 1 FROM authenticated_delivery_grant_storage_privacy_v1 WHERE singleton = 1";
const INSERT_STORAGE_PRIVACY_MARKER: &str =
    "INSERT INTO authenticated_delivery_grant_storage_privacy_v1 (singleton) VALUES (1)";
const CREATE_PRIVACY_CONSUMES_TABLE: &str = "CREATE TABLE authenticated_delivery_grant_consumes_privacy_v3 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_fingerprint TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
const CREATE_PRIVACY_AUDITS_TABLE: &str = "CREATE TABLE authenticated_delivery_grant_audits_privacy_v3 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL, recorded_at_nanos INTEGER, audit_scope TEXT NOT NULL DEFAULT 'replay')";

pub(super) fn ensure_retention_indexes(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let has_fingerprint_column = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_consumes_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists(["grant_fingerprint"]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    has_fingerprint_column
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    migrate_raw_storage_keys(connection)?;
    connection
        .execute(CREATE_EXPIRY_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_AUDIT_GRANT_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_VALIDATION_REJECTION_RETENTION_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_REPLAY_RETENTION_CLOCK, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_CONFIRMED_REPLAY_RETENTION_CLOCK, [])
        .and_then(|_| connection.execute(MIGRATE_REPLAY_RETENTION_CLOCK, []))
        .and_then(|_| connection.execute(CREATE_PROVISIONAL_REPLAY_RETENTION_CLOCK, []))
        .and_then(|_| connection.execute(MIGRATE_PROVISIONAL_REPLAY_RETENTION_CLOCK, []))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn migrate_raw_storage_keys(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    connection
        .execute(CREATE_STORAGE_PRIVACY_MARKER, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let transaction = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let migrated = transaction
        .query_row(STORAGE_PRIVACY_MARKER_EXISTS, [], |_row| Ok(()))
        .optional()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?
        .is_some();
    if migrated {
        return transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable);
    }
    transaction
        .execute(CREATE_PRIVACY_CONSUMES_TABLE, [])
        .and_then(|_| transaction.execute(CREATE_PRIVACY_AUDITS_TABLE, []))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    copy_consumes_with_private_keys(&transaction)?;
    copy_audits_with_private_keys(&transaction)?;
    transaction
        .execute("DROP TABLE authenticated_delivery_grant_consumes_v2", [])
        .and_then(|_| transaction.execute("DROP TABLE authenticated_delivery_grant_audits_v2", []))
        .and_then(|_| transaction.execute("ALTER TABLE authenticated_delivery_grant_consumes_privacy_v3 RENAME TO authenticated_delivery_grant_consumes_v2", []))
        .and_then(|_| transaction.execute("ALTER TABLE authenticated_delivery_grant_audits_privacy_v3 RENAME TO authenticated_delivery_grant_audits_v2", []))
        .and_then(|_| transaction.execute(INSERT_STORAGE_PRIVACY_MARKER, []))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
}

fn copy_consumes_with_private_keys(
    transaction: &Transaction<'_>,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let mut statement = transaction
        .prepare("SELECT issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos FROM authenticated_delivery_grant_consumes_v2 ORDER BY rowid")
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    for row in rows {
        let (issuer_key_id, nonce, fingerprint, audit_json, expires_at_nanos) =
            row.map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        if issuer_key_id.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
            || nonce.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        {
            return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
        }
        transaction.execute(
            "INSERT INTO authenticated_delivery_grant_consumes_privacy_v3 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![storage_key_digest(&issuer_key_id), storage_key_digest(&nonce), fingerprint, audit_json, expires_at_nanos],
        ).map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    Ok(())
}

fn copy_audits_with_private_keys(
    transaction: &Transaction<'_>,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let mut statement = transaction
        .prepare("SELECT issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope FROM authenticated_delivery_grant_audits_v2 ORDER BY rowid")
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<i64>>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    for row in rows {
        let (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) =
            row.map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        if issuer_key_id.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
            || nonce.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        {
            return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
        }
        transaction.execute(
            "INSERT INTO authenticated_delivery_grant_audits_privacy_v3 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![storage_key_digest(&issuer_key_id), storage_key_digest(&nonce), audit_json, recorded_at_nanos, audit_scope],
        ).map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    Ok(())
}

pub(super) fn advance_replay_retention_clock(
    connection: &Connection,
    observed_now_nanos: i64,
    independently_confirmed: bool,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let transaction = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let effective_now_nanos = advance_replay_retention_clock_transaction(
        &transaction,
        observed_now_nanos,
        independently_confirmed,
    )?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(effective_now_nanos)
}

pub(super) fn advance_replay_retention_clock_transaction(
    transaction: &Transaction<'_>,
    observed_now_nanos: i64,
    independently_confirmed: bool,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    clock::advance(transaction, observed_now_nanos, independently_confirmed)
}

pub(super) fn migrate_legacy_replay_records(
    connection: &Connection,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    legacy_replay_rows::migrate_legacy_replay_records(connection, trusted_issuer)
}

fn parse_legacy_replay_grant(
    grant_json: &str,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<LegacyReplayGrantMigration, AuthenticatedDeliveryGrantConsumeError> {
    let grant: LegacyAuthenticatedDeliveryGrant = serde_json::from_str(grant_json)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    grant.validate_shape()?;
    let replay_fingerprint = if grant.issuer_key_id == trusted_issuer.key_id {
        let signature = Signature::from_slice(&grant.signature)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let signing_bytes = grant.signing_bytes();
        trusted_issuer
            .verifying_key
            .verify_strict(&signing_bytes, &signature)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let mut replay_material = signing_bytes;
        replay_material.extend_from_slice(&grant.signature);
        digest(replay_material)
    } else {
        // A rotated issuer cannot authenticate a retired key's historical signature.
        // Keep this row as a fail-closed tombstone rather than deleting it or
        // requiring the new issuer to have signed old grant material. Current
        // issuer rows still require full signature verification above.
        digest(grant_json)
    };
    Ok(LegacyReplayGrantMigration {
        issuer_key_id: grant.issuer_key_id,
        nonce: grant.nonce,
        expires_at: grant.expires_at,
        replay_fingerprint,
    })
}

struct LegacyReplayGrantMigration {
    issuer_key_id: String,
    nonce: String,
    expires_at: String,
    replay_fingerprint: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyAuthenticatedDeliveryGrant {
    schema_version: u16,
    issuer_key_id: String,
    issuer_actor_id: String,
    household_id: String,
    parent_device_id: String,
    child_profile_id: String,
    target_device_id: String,
    policy_decision_id: String,
    policy_version: String,
    action_id: String,
    capability_id: String,
    evidence_digest: String,
    payload_digest: String,
    dry_run: bool,
    nonce: String,
    issued_at: String,
    expires_at: String,
    revocation_version: String,
    signature: Vec<u8>,
}

impl LegacyAuthenticatedDeliveryGrant {
    fn validate_shape(&self) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
        if self.schema_version != 1
            || self.signature.len() != AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES
            || self.payload_digest.len() != AUTHENTICATED_DELIVERY_GRANT_PAYLOAD_DIGEST_HEX_BYTES
            || !self
                .payload_digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
        }
        let bindings = self.binding_values();
        let valid_time_window = self
            .issued_at
            .parse::<chrono::DateTime<chrono::FixedOffset>>()
            .ok()
            .zip(
                self.expires_at
                    .parse::<chrono::DateTime<chrono::FixedOffset>>()
                    .ok(),
            )
            .is_some_and(|(issued_at, expires_at)| issued_at < expires_at);
        if bindings.iter().any(|value| {
            value.trim().is_empty() || value.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        }) || self.signing_wire_len() > AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES
            || !valid_time_window
        {
            return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
        }
        Ok(())
    }

    fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.signing_wire_len());
        let schema_version = self.schema_version.to_string();
        let dry_run = self.dry_run.to_string();
        for value in [
            schema_version.as_str(),
            self.issuer_key_id.as_str(),
            self.issuer_actor_id.as_str(),
            self.household_id.as_str(),
            self.parent_device_id.as_str(),
            self.child_profile_id.as_str(),
            self.target_device_id.as_str(),
            self.policy_decision_id.as_str(),
            self.policy_version.as_str(),
            self.action_id.as_str(),
            self.capability_id.as_str(),
            self.evidence_digest.as_str(),
            self.payload_digest.as_str(),
            dry_run.as_str(),
            self.nonce.as_str(),
            self.issued_at.as_str(),
            self.expires_at.as_str(),
            self.revocation_version.as_str(),
        ] {
            bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes
    }

    fn signing_wire_len(&self) -> usize {
        let schema_version = self.schema_version.to_string();
        let dry_run = self.dry_run.to_string();
        [
            schema_version.as_str(),
            self.issuer_key_id.as_str(),
            self.issuer_actor_id.as_str(),
            self.household_id.as_str(),
            self.parent_device_id.as_str(),
            self.child_profile_id.as_str(),
            self.target_device_id.as_str(),
            self.policy_decision_id.as_str(),
            self.policy_version.as_str(),
            self.action_id.as_str(),
            self.capability_id.as_str(),
            self.evidence_digest.as_str(),
            self.payload_digest.as_str(),
            dry_run.as_str(),
            self.nonce.as_str(),
            self.issued_at.as_str(),
            self.expires_at.as_str(),
            self.revocation_version.as_str(),
        ]
        .into_iter()
        .map(|value| value.len() + std::mem::size_of::<u64>())
        .sum()
    }

    fn binding_values(&self) -> [&str; 16] {
        [
            &self.issuer_key_id,
            &self.issuer_actor_id,
            &self.household_id,
            &self.parent_device_id,
            &self.child_profile_id,
            &self.target_device_id,
            &self.policy_decision_id,
            &self.policy_version,
            &self.action_id,
            &self.capability_id,
            &self.evidence_digest,
            &self.payload_digest,
            &self.nonce,
            &self.issued_at,
            &self.expires_at,
            &self.revocation_version,
        ]
    }
}

pub(super) fn purge_expired_replay_records(
    connection: &Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    purge_expired_replay_record_batch(connection, trusted_now_nanos).map(|_count| ())
}

pub(super) fn drain_expired_replay_records_at_startup(
    connection: &Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    while purge_expired_replay_record_batch(connection, trusted_now_nanos)?
        == MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE as usize
    {}
    Ok(())
}

fn purge_expired_replay_record_batch(
    connection: &Connection,
    trusted_now_nanos: i64,
) -> Result<usize, AuthenticatedDeliveryGrantConsumeError> {
    let transaction = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let Some(purge_cutoff_nanos) = clock::confirmed_purge_cutoff(&transaction, trusted_now_nanos)?
    else {
        transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        return Ok(0);
    };
    let expired = {
        let mut statement = transaction
            .prepare(SELECT_EXPIRED_GRANTS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map(
                params![purge_cutoff_nanos, MAX_EXPIRED_REPLAY_RECORDS_PER_PURGE],
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
