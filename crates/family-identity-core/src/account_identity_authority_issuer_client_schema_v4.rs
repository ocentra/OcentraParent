use rusqlite::{Connection, OptionalExtension};

use super::super::AccountIdentityAuthorityIssuerClientError;
use super::{CANONICAL_SCHEMA_SQL, SCHEMA_META_SQL, SCHEMA_NAME, SCHEMA_VERSION};

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

const V3_KEY_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("account_id", "TEXT", 1, 0),
    ("household_id", "TEXT", 1, 0),
    ("service", "TEXT", 1, 0),
    ("service_binding_id", "TEXT", 1, 0),
    ("key_id", "TEXT", 1, 0),
    ("key_generation", "INTEGER", 1, 4),
    ("enrollment_generation", "INTEGER", 1, 0),
    ("public_key", "BLOB", 1, 0),
    ("authority_generation", "INTEGER", 1, 0),
    ("key_state", "TEXT", 1, 0),
];

const V3_OUTBOX_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("receipt_id", "TEXT", 1, 1),
    ("account_id", "TEXT", 1, 0),
    ("household_id", "TEXT", 1, 0),
    ("service", "TEXT", 1, 0),
    ("service_binding_id", "TEXT", 1, 0),
    ("key_id", "TEXT", 1, 0),
    ("key_generation", "INTEGER", 1, 0),
    ("enrollment_generation", "INTEGER", 1, 0),
    ("authority_generation", "INTEGER", 1, 0),
    ("wire", "BLOB", 1, 0),
    ("delivery_state", "TEXT", 1, 0),
    ("claim_id", "TEXT", 0, 0),
    ("claimed_at", "TEXT", 0, 0),
    ("claim_expires_at", "TEXT", 0, 0),
    ("attempt_count", "INTEGER", 1, 0),
    ("last_error_code", "TEXT", 0, 0),
    ("last_error_digest", "TEXT", 0, 0),
    ("last_result", "TEXT", 0, 0),
    ("ack_wire", "BLOB", 0, 0),
    ("next_attempt_at", "TEXT", 0, 0),
];

const V3_SCHEMA_META_SQL: &str = "CREATE TABLE IF NOT EXISTS account_identity_issuer_v2_schema (
    schema_name TEXT PRIMARY KEY CHECK (schema_name = 'account_identity_issuer_v2'),
    schema_version INTEGER NOT NULL CHECK (schema_version = 3),
    migration_state TEXT NOT NULL CHECK (migration_state = 'ready')
) STRICT";

const V3_RECEIPT_SQL: &str = "CREATE TABLE account_identity_issuer_v2_receipt (
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
) STRICT";

const V3_OUTBOX_SQL: &str = "CREATE TABLE account_identity_issuer_v2_outbox (
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
) STRICT";

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
    validate_v3_shape(connection)?;
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
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_receipt",
        V3_RECEIPT_SQL,
    )
}

fn validate_v3_shape(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_v3_metadata(connection)?;
    validate_v3_receipt_shape(connection)?;
    validate_v3_table(
        connection,
        "account_identity_issuer_v2_key_registry",
        V3_KEY_COLUMNS,
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_v3_table(
        connection,
        "account_identity_issuer_v2_outbox",
        V3_OUTBOX_COLUMNS,
        V3_OUTBOX_SQL,
    )?;
    validate_v3_index(
        connection,
        "account_identity_issuer_v2_key_registry_current",
        &[
            "account_id",
            "household_id",
            "service",
            "service_binding_id",
            "key_state",
            "key_generation",
        ],
    )?;
    validate_v3_index(
        connection,
        "account_identity_issuer_v2_receipt_lookup",
        &["account_id", "household_id", "service", "receipt_state"],
    )?;
    validate_v3_index(
        connection,
        "account_identity_issuer_v2_outbox_delivery",
        &[
            "service",
            "delivery_state",
            "claim_expires_at",
            "next_attempt_at",
            "receipt_id",
        ],
    )
}

fn validate_v3_metadata(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_schema",
        V3_SCHEMA_META_SQL,
    )?;
    let valid: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_schema
                 WHERE schema_name = ?1 AND schema_version = 3
                   AND migration_state = 'ready'
            ) AND (SELECT COUNT(*) FROM account_identity_issuer_v2_schema) = 1",
            [SCHEMA_NAME],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    valid
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_v3_table(
    connection: &Connection,
    table: &str,
    expected_columns: &[(&str, &str, i64, i64)],
    expected_schema: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_columns(connection, table, expected_columns)?;
    validate_table_sql(connection, table, expected_schema)
}

fn validate_v3_index(
    connection: &Connection,
    index: &str,
    expected_columns: &[&str],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let columns = connection
        .prepare(&format!("PRAGMA index_info(\"{index}\")"))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .query_map([], |row| row.get::<_, String>(2))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if columns
        .iter()
        .map(String::as_str)
        .ne(expected_columns.iter().copied())
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    validate_index_sql(connection, index, CANONICAL_SCHEMA_SQL)
}

fn validate_table_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let quoted_table = table.replace('"', "\"\"");
    let actual = connection
        .prepare(&format!("PRAGMA table_info(\"{quoted_table}\")"))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(|(actual, expected)| {
            actual.0 != expected.0
                || actual.1.to_ascii_uppercase() != expected.1
                || actual.2 != expected.2
                || actual.3 != expected.3
        })
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    Ok(())
}

fn validate_table_sql(
    connection: &Connection,
    table: &str,
    expected_schema: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let actual = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [table],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let marker = format!("CREATETABLE{}", compact_sql(table));
    let expected = expected_schema
        .split(';')
        .map(compact_sql)
        .find(|statement| statement.starts_with(&marker))
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    (compact_sql(&actual) == expected)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_index_sql(
    connection: &Connection,
    index: &str,
    expected_schema: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let actual = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'index' AND name = ?1",
            [index],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let marker = format!("CREATEINDEX{}", compact_sql(index));
    let expected = expected_schema
        .split(';')
        .map(compact_sql)
        .find(|statement| statement.starts_with(&marker))
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    (compact_sql(&actual) == expected)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn compact_sql(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}
