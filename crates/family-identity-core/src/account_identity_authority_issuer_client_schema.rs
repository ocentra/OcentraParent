use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{Connection, OptionalExtension};

use super::AccountIdentityAuthorityIssuerClientError;

#[path = "account_identity_authority_issuer_client_schema_legacy.rs"]
mod schema_legacy;

#[path = "account_identity_authority_issuer_client_schema_validation.rs"]
mod schema_validation;

const SCHEMA_NAME: &str = "account_identity_issuer_v2";
const SCHEMA_VERSION: i64 = 2;

pub(super) fn initialize(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    connection
        .execute_batch("BEGIN IMMEDIATE;")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let result = migrate(connection);
    match result {
        Ok(()) => connection
            .execute_batch("COMMIT;")
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(error)
        }
    }
}

fn migrate(connection: &Connection) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    connection
        .execute_batch(SCHEMA_META_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let version = connection
        .query_row(
            "SELECT schema_version FROM account_identity_issuer_v2_schema
              WHERE schema_name = ?1",
            [SCHEMA_NAME],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    match version {
        Some(SCHEMA_VERSION) => schema_validation::validate_in_transaction(connection),
        Some(_) => Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
        None => {
            if has_any_issuer_table(connection)? {
                schema_validation::validate_legacy_shape(connection)?;
                schema_legacy::rebuild_legacy(connection)?;
            } else {
                connection
                    .execute_batch(CANONICAL_SCHEMA_SQL)
                    .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
            }
            connection
                .execute(
                    "INSERT INTO account_identity_issuer_v2_schema
                        (schema_name, schema_version, migration_state)
                     VALUES (?1, ?2, 'ready')",
                    rusqlite::params![SCHEMA_NAME, SCHEMA_VERSION],
                )
                .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
            schema_validation::validate_in_transaction(connection)
        }
    }
}

const SCHEMA_META_SQL: &str = "CREATE TABLE IF NOT EXISTS account_identity_issuer_v2_schema (
    schema_name TEXT PRIMARY KEY CHECK (schema_name = 'account_identity_issuer_v2'),
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    migration_state TEXT NOT NULL CHECK (migration_state = 'ready')
) STRICT;";

const CANONICAL_SCHEMA_SQL: &str = "CREATE TABLE account_identity_issuer_v2_key_registry (
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
    service TEXT NOT NULL CHECK (service = 'ocentra.account-authority-producer.cloudflare.v2'),
    service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
    key_id TEXT NOT NULL CHECK (key_id LIKE 'sha256:ecdsa-p256:%'),
    key_generation INTEGER NOT NULL CHECK (key_generation > 0),
    public_key BLOB NOT NULL CHECK (length(public_key) = 65),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    key_state TEXT NOT NULL CHECK (key_state IN ('active','revoked')),
    PRIMARY KEY (account_id, household_id, service, key_generation),
    UNIQUE (key_id)
) STRICT;
CREATE INDEX account_identity_issuer_v2_key_registry_current
    ON account_identity_issuer_v2_key_registry
       (account_id, household_id, service, key_state, key_generation);
CREATE TABLE account_identity_issuer_v2_receipt (
    receipt_id TEXT PRIMARY KEY CHECK (length(receipt_id) > 0),
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
    service TEXT NOT NULL CHECK (service = 'ocentra.account-authority-producer.cloudflare.v2'),
    service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
    key_id TEXT NOT NULL CHECK (length(key_id) > 0),
    key_generation INTEGER NOT NULL CHECK (key_generation > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    session_generation INTEGER NOT NULL CHECK (session_generation > 0),
    correlation_id TEXT NOT NULL CHECK (length(correlation_id) > 0),
    idempotency_key TEXT NOT NULL CHECK (length(idempotency_key) > 0),
    payload_digest TEXT NOT NULL CHECK (length(payload_digest) > 0),
    issued_at TEXT NOT NULL CHECK (length(issued_at) > 0),
    expires_at TEXT NOT NULL CHECK (length(expires_at) > 0),
    wire BLOB NOT NULL CHECK (length(wire) > 0),
    receipt_state TEXT NOT NULL CHECK (receipt_state IN ('issued','acknowledged')),
    UNIQUE (account_id, idempotency_key)
) STRICT;
CREATE INDEX account_identity_issuer_v2_receipt_lookup
    ON account_identity_issuer_v2_receipt
       (account_id, household_id, service, receipt_state);
CREATE TABLE account_identity_issuer_v2_outbox (
    receipt_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
    service TEXT NOT NULL CHECK (service = 'ocentra.account-authority-producer.cloudflare.v2'),
    service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
    key_id TEXT NOT NULL CHECK (length(key_id) > 0),
    key_generation INTEGER NOT NULL CHECK (key_generation > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    wire BLOB NOT NULL CHECK (length(wire) > 0),
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending','claimed','sent','failed','acknowledged')
    ),
    claim_id TEXT,
    claimed_at TEXT,
    attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
    last_error TEXT,
    last_result TEXT,
    next_attempt_at TEXT,
    FOREIGN KEY (receipt_id) REFERENCES account_identity_issuer_v2_receipt(receipt_id)
        ON DELETE RESTRICT,
    CHECK ((delivery_state = 'claimed' AND claim_id IS NOT NULL AND claimed_at IS NOT NULL)
        OR (delivery_state <> 'claimed' AND claim_id IS NULL AND claimed_at IS NULL)),
    CHECK (delivery_state <> 'acknowledged' OR last_result IS NOT NULL)
) STRICT;
CREATE INDEX account_identity_issuer_v2_outbox_delivery
    ON account_identity_issuer_v2_outbox
       (service, delivery_state, next_attempt_at, receipt_id);";

fn has_legacy_table(
    connection: &Connection,
    table: &str,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1
            )",
            [table],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn has_any_issuer_table(
    connection: &Connection,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                 WHERE type = 'table'
                   AND name IN (
                       'account_identity_issuer_v2_key_registry',
                       'account_identity_issuer_v2_receipt',
                       'account_identity_issuer_v2_outbox'
                   )
            )",
            [],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}
