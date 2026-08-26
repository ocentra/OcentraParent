use rusqlite::Connection;

use super::super::AccountIdentityAuthorityIssuerClientError;
use super::{SCHEMA_META_SQL, SCHEMA_NAME, SCHEMA_VERSION};

const V3_RECEIPT_COLUMNS: &[&str] = &[
    "receipt_id",
    "account_id",
    "household_id",
    "service",
    "service_binding_id",
    "key_id",
    "key_generation",
    "enrollment_generation",
    "authority_generation",
    "session_generation",
    "correlation_id",
    "idempotency_key",
    "payload_digest",
    "issued_at",
    "expires_at",
    "wire",
    "ack_wire",
    "receipt_state",
];

const RESERVATION_SCHEMA_SQL: &str = "CREATE TABLE account_identity_issuer_v2_reservation (
    reservation_id TEXT PRIMARY KEY CHECK (length(reservation_id) > 0),
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
    provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
    provider_subject TEXT NOT NULL CHECK (length(provider_subject) > 0),
    service TEXT NOT NULL CHECK (service = 'ocentra.account-authority-producer.cloudflare.v2'),
    service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
    key_id TEXT NOT NULL CHECK (length(key_id) > 0),
    key_generation INTEGER NOT NULL CHECK (
        key_generation > 0 AND key_generation <= 9007199254740991
    ),
    enrollment_generation INTEGER NOT NULL CHECK (
        enrollment_generation > 0 AND enrollment_generation <= 9007199254740991
    ),
    authority_generation INTEGER NOT NULL CHECK (
        authority_generation > 0 AND authority_generation <= 9007199254740991
    ),
    session_generation INTEGER NOT NULL CHECK (
        session_generation > 0 AND session_generation <= 9007199254740991
    ),
    correlation_id TEXT NOT NULL CHECK (length(correlation_id) > 0),
    idempotency_key TEXT NOT NULL CHECK (length(idempotency_key) > 0),
    request_digest TEXT NOT NULL CHECK (request_digest LIKE 'sha256:request:%'),
    request_wire BLOB NOT NULL CHECK (length(request_wire) > 0),
    reservation_state TEXT NOT NULL CHECK (
        reservation_state IN ('prepared','signing','manual-required','issued')
    ),
    signer_status TEXT NOT NULL CHECK (
        signer_status IN ('not-started','in-flight','uncertain','succeeded')
    ),
    attempt_token TEXT NOT NULL UNIQUE CHECK (length(attempt_token) > 0),
    lease_expires_at TEXT NOT NULL CHECK (length(lease_expires_at) > 0),
    reserved_at TEXT NOT NULL CHECK (length(reserved_at) > 0),
    signing_started_at TEXT,
    uncertain_at TEXT,
    receipt_id TEXT,
    UNIQUE (account_id, service, idempotency_key),
    CHECK ((reservation_state = 'prepared' AND signer_status = 'not-started'
            AND signing_started_at IS NULL AND uncertain_at IS NULL AND receipt_id IS NULL)
        OR (reservation_state = 'signing' AND signer_status = 'in-flight'
            AND signing_started_at IS NOT NULL AND uncertain_at IS NULL AND receipt_id IS NULL)
        OR (reservation_state = 'manual-required' AND signer_status = 'uncertain'
            AND signing_started_at IS NOT NULL AND uncertain_at IS NOT NULL AND receipt_id IS NULL)
        OR (reservation_state = 'issued' AND signer_status = 'succeeded'
            AND signing_started_at IS NOT NULL AND uncertain_at IS NULL AND receipt_id IS NOT NULL))
 ) STRICT;
 CREATE INDEX account_identity_issuer_v2_reservation_lookup
     ON account_identity_issuer_v2_reservation
        (account_id, service, idempotency_key, reservation_state, lease_expires_at);";

/// Migrate the shipped v3 issuer schema in place.  Receipt and key rows are
/// never rebuilt or rewritten: the new provenance columns remain unbound for
/// legacy rows, so exact replay can fail closed rather than inventing an
/// authority source.
pub(super) fn migrate_from_v3(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_v3_receipt_shape(connection)?;
    connection
        .execute_batch(
            "ALTER TABLE account_identity_issuer_v2_receipt ADD COLUMN provider TEXT;
             ALTER TABLE account_identity_issuer_v2_receipt ADD COLUMN provider_subject TEXT;
             ALTER TABLE account_identity_issuer_v2_receipt
                 ADD COLUMN provenance_state TEXT NOT NULL DEFAULT 'legacy-unbound'
                 CHECK (provenance_state IN ('exact','legacy-unbound'));
             UPDATE account_identity_issuer_v2_receipt
                SET provenance_state = 'legacy-unbound'
              WHERE provenance_state IS NULL;",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    connection
        .execute_batch(RESERVATION_SCHEMA_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    connection
        .execute_batch("DROP TABLE account_identity_issuer_v2_schema;")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    connection
        .execute_batch(SCHEMA_META_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    connection
        .execute(
            "INSERT INTO account_identity_issuer_v2_schema
                (schema_name, schema_version, migration_state)
             VALUES (?1, ?2, 'ready')",
            rusqlite::params![SCHEMA_NAME, SCHEMA_VERSION],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    Ok(())
}

fn validate_v3_receipt_shape(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let exists: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                 WHERE type = 'table' AND name = 'account_identity_issuer_v2_receipt'
            )",
            [],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if !exists {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let actual = connection
        .prepare("PRAGMA table_info(account_identity_issuer_v2_receipt)")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if actual
        .iter()
        .map(String::as_str)
        .ne(V3_RECEIPT_COLUMNS.iter().copied())
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    Ok(())
}
