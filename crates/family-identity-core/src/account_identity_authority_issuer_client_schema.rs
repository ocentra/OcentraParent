use rusqlite::{Connection, OptionalExtension};

use super::AccountIdentityAuthorityIssuerClientError;

#[path = "account_identity_authority_issuer_client_schema_legacy.rs"]
mod schema_legacy;

#[path = "account_identity_authority_issuer_client_schema_validation.rs"]
mod schema_validation;

#[path = "account_identity_authority_issuer_client_schema_previous.rs"]
mod schema_previous;

const SCHEMA_NAME: &str = "account_identity_issuer_v2";
const SCHEMA_VERSION: i64 = 3;
const PREVIOUS_SCHEMA_VERSION: i64 = 2;

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
        Some(PREVIOUS_SCHEMA_VERSION) => migrate_previous(connection),
        Some(_) => Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
        None => migrate_missing(connection),
    }
}

fn migrate_previous(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    schema_previous::validate_previous_shape(connection)?;
    schema_legacy::rebuild_legacy(connection)?;
    replace_metadata(connection)?;
    schema_validation::validate_in_transaction(connection)
}

fn migrate_missing(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if has_any_issuer_table(connection)? {
        validate_and_rebuild_existing(connection)?;
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

fn validate_and_rebuild_existing(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if schema_previous::is_previous_shape(connection) {
        schema_previous::validate_previous_shape(connection)?;
    } else {
        schema_previous::validate_legacy_shape(connection)?;
    }
    schema_legacy::rebuild_legacy(connection)
}

fn replace_metadata(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    connection
        .execute_batch("DROP TABLE account_identity_issuer_v2_schema;")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    connection
        .execute_batch(SCHEMA_META_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

const SCHEMA_META_SQL: &str = "CREATE TABLE IF NOT EXISTS account_identity_issuer_v2_schema (
    schema_name TEXT PRIMARY KEY CHECK (schema_name = 'account_identity_issuer_v2'),
    schema_version INTEGER NOT NULL CHECK (schema_version = 3),
    migration_state TEXT NOT NULL CHECK (migration_state = 'ready')
) STRICT;";

const CANONICAL_SCHEMA_SQL: &str = "CREATE TABLE account_identity_issuer_v2_key_registry (
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
    service TEXT NOT NULL CHECK (service = 'ocentra.account-authority-producer.cloudflare.v2'),
    service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
    key_id TEXT NOT NULL CHECK (key_id LIKE 'sha256:ecdsa-p256:%'),
    key_generation INTEGER NOT NULL CHECK (
        key_generation > 0 AND key_generation <= 9007199254740991
    ),
    enrollment_generation INTEGER NOT NULL CHECK (
        enrollment_generation > 0 AND enrollment_generation <= 9007199254740991
    ),
    public_key BLOB NOT NULL CHECK (length(public_key) = 65),
    authority_generation INTEGER NOT NULL CHECK (
        authority_generation > 0 AND authority_generation <= 9007199254740991
    ),
    key_state TEXT NOT NULL CHECK (key_state IN ('active','revoked')),
    PRIMARY KEY (account_id, household_id, service, key_generation),
    UNIQUE (key_id)
) STRICT;
CREATE INDEX account_identity_issuer_v2_key_registry_current
    ON account_identity_issuer_v2_key_registry
       (account_id, household_id, service, service_binding_id, key_state, key_generation);
CREATE TABLE account_identity_issuer_v2_receipt (
    receipt_id TEXT PRIMARY KEY CHECK (length(receipt_id) > 0),
    account_id TEXT NOT NULL CHECK (length(account_id) > 0),
    household_id TEXT NOT NULL CHECK (length(household_id) > 0),
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
    payload_digest TEXT NOT NULL CHECK (length(payload_digest) > 0),
    issued_at TEXT NOT NULL CHECK (length(issued_at) > 0),
    expires_at TEXT NOT NULL CHECK (length(expires_at) > 0),
    wire BLOB NOT NULL CHECK (length(wire) > 0),
    ack_wire BLOB,
    receipt_state TEXT NOT NULL CHECK (receipt_state IN ('issued','acknowledged')),
    UNIQUE (account_id, idempotency_key),
    CHECK ((receipt_state = 'acknowledged' AND ack_wire IS NOT NULL AND length(ack_wire) > 0)
        OR (receipt_state = 'issued' AND ack_wire IS NULL))
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
    key_generation INTEGER NOT NULL CHECK (
        key_generation > 0 AND key_generation <= 9007199254740991
    ),
    enrollment_generation INTEGER NOT NULL CHECK (
        enrollment_generation > 0 AND enrollment_generation <= 9007199254740991
    ),
    authority_generation INTEGER NOT NULL CHECK (
        authority_generation > 0 AND authority_generation <= 9007199254740991
    ),
    wire BLOB NOT NULL CHECK (length(wire) > 0),
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending','claimed','sent','failed','acknowledged')
    ),
    claim_id TEXT,
    claimed_at TEXT,
    claim_expires_at TEXT,
    attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
    last_error_code TEXT CHECK (last_error_code IN ('delivery_failed','lease_expired')),
    last_error_digest TEXT CHECK (
        last_error_digest IS NULL OR last_error_digest LIKE 'sha256:delivery-detail:%'
    ),
    CHECK ((last_error_code IS NULL AND last_error_digest IS NULL)
        OR (last_error_code = 'lease_expired' AND last_error_digest IS NULL)
        OR (last_error_code = 'delivery_failed' AND last_error_digest IS NOT NULL)),
    last_result TEXT,
    ack_wire BLOB,
    next_attempt_at TEXT,
    FOREIGN KEY (receipt_id) REFERENCES account_identity_issuer_v2_receipt(receipt_id)
        ON DELETE RESTRICT,
    CHECK ((delivery_state = 'claimed' AND claim_id IS NOT NULL AND claimed_at IS NOT NULL
            AND claim_expires_at IS NOT NULL)
        OR (delivery_state <> 'claimed' AND claim_id IS NULL AND claimed_at IS NULL
            AND claim_expires_at IS NULL)),
    CHECK ((delivery_state = 'acknowledged' AND last_result IS NOT NULL
            AND ack_wire IS NOT NULL AND length(ack_wire) > 0)
        OR (delivery_state <> 'acknowledged' AND ack_wire IS NULL))
) STRICT;
CREATE INDEX account_identity_issuer_v2_outbox_delivery
    ON account_identity_issuer_v2_outbox
       (service, delivery_state, claim_expires_at, next_attempt_at, receipt_id);";

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
