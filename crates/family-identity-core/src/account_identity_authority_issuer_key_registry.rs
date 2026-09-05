use rusqlite::Connection;

use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

#[path = "account_identity_authority_issuer_key_registry_lineage.rs"]
mod lineage;
#[path = "account_identity_authority_issuer_key_registry_receipts.rs"]
pub(crate) mod receipts;
#[path = "account_identity_authority_issuer_key_registry_rows.rs"]
pub(crate) mod rows;
#[path = "account_identity_authority_issuer_key_registry_schema.rs"]
pub(crate) mod schema;

pub(crate) const KEY_REGISTRY_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_key_registry (
        account_id TEXT NOT NULL CHECK (length(account_id) > 0),
        household_id TEXT NOT NULL CHECK (length(household_id) > 0),
        service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
        key_id TEXT NOT NULL CHECK (length(key_id) > 0),
        key_version INTEGER NOT NULL CHECK (key_version > 0),
        public_key BLOB NOT NULL CHECK (length(public_key) = 32),
        key_state TEXT NOT NULL CHECK (key_state IN ('active','revoked')),
        authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
        revoked_generation INTEGER,
        service_label TEXT NOT NULL CHECK (length(service_label) > 0),
        PRIMARY KEY (account_id, household_id, service_binding_id, key_version),
        UNIQUE (key_id)
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_key_registry_current
        ON account_identity_issuer_key_registry (
            account_id, household_id, service_label, key_state, key_version
        );";

pub(crate) const TRANSPORT_RECEIPT_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_transport_receipt (
        receipt_id TEXT PRIMARY KEY CHECK (length(receipt_id) > 0),
        account_id TEXT NOT NULL CHECK (length(account_id) > 0),
        household_id TEXT NOT NULL CHECK (length(household_id) > 0),
        service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
        service_label TEXT NOT NULL CHECK (length(service_label) > 0),
        authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
        key_id TEXT NOT NULL CHECK (length(key_id) > 0),
        key_version INTEGER NOT NULL CHECK (key_version > 0),
        issued_at_millis INTEGER NOT NULL CHECK (issued_at_millis >= 0),
        expires_at_millis INTEGER NOT NULL CHECK (expires_at_millis > issued_at_millis),
        receipt_state TEXT NOT NULL CHECK (receipt_state IN ('issued','consumed')),
        consumed_at_millis INTEGER,
        CHECK (
            (receipt_state = 'issued' AND consumed_at_millis IS NULL)
            OR (receipt_state = 'consumed' AND consumed_at_millis IS NOT NULL)
        )
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_transport_receipt_lookup
        ON account_identity_issuer_transport_receipt (
            account_id, household_id, service_binding_id, key_id, key_version, receipt_state
        );";

pub(crate) const CLOCK_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_clock (
        clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1),
        last_unix_millis INTEGER NOT NULL CHECK (last_unix_millis >= 0)
    ) STRICT;
    INSERT INTO account_identity_issuer_clock (clock_id, last_unix_millis)
        VALUES (1, 0)
        ON CONFLICT(clock_id) DO NOTHING;";

pub(crate) fn ensure_schema(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    connection
        .execute_batch(KEY_REGISTRY_SCHEMA_SQL)
        .and_then(|_| connection.execute_batch(TRANSPORT_RECEIPT_SCHEMA_SQL))
        .and_then(|_| connection.execute_batch(CLOCK_SCHEMA_SQL))
        .and_then(|_| connection.execute_batch(super::outbox::OUTBOX_SCHEMA_SQL))
        .map_err(|_error| AccountIdentityIssuerError::Unavailable)?;
    schema::validate(connection)
}

pub(crate) fn validate_durable_state(
    connection: &Connection,
) -> Result<u64, AccountIdentityIssuerError> {
    let active_count = rows::validate(connection)?;
    receipts::validate_transport_receipts(connection)?;
    receipts::validate_clock_state(connection)?;
    super::outbox::validate(connection)?;
    Ok(active_count)
}
