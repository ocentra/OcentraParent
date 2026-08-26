use rusqlite::{Connection, OptionalExtension};

use super::{has_legacy_table, AccountIdentityAuthorityIssuerClientError, SCHEMA_NAME};

#[path = "account_identity_authority_issuer_client_schema_objects.rs"]
mod schema_objects;

pub(super) fn validate_legacy_shape(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let expected = [
        (
            "account_identity_issuer_v2_key_registry",
            &[
                ("account_id", "TEXT", 1_i64, 0_i64),
                ("household_id", "TEXT", 1, 0),
                ("service_binding_id", "TEXT", 1, 0),
                ("key_id", "TEXT", 1, 0),
                ("key_generation", "INTEGER", 1, 4),
                ("public_key", "BLOB", 1, 0),
                ("authority_generation", "INTEGER", 1, 0),
                ("key_state", "TEXT", 1, 0),
            ][..],
        ),
        (
            "account_identity_issuer_v2_receipt",
            &[
                ("receipt_id", "TEXT", 1_i64, 1_i64),
                ("account_id", "TEXT", 1, 0),
                ("household_id", "TEXT", 1, 0),
                ("service_binding_id", "TEXT", 1, 0),
                ("key_id", "TEXT", 1, 0),
                ("key_generation", "INTEGER", 1, 0),
                ("authority_generation", "INTEGER", 1, 0),
                ("session_generation", "INTEGER", 1, 0),
                ("correlation_id", "TEXT", 1, 0),
                ("idempotency_key", "TEXT", 1, 0),
                ("payload_digest", "TEXT", 1, 0),
                ("issued_at", "TEXT", 1, 0),
                ("expires_at", "TEXT", 1, 0),
                ("wire", "BLOB", 1, 0),
                ("receipt_state", "TEXT", 1, 0),
            ][..],
        ),
        (
            "account_identity_issuer_v2_outbox",
            &[
                ("receipt_id", "TEXT", 1_i64, 1_i64),
                ("wire", "BLOB", 1, 0),
                ("delivery_state", "TEXT", 1, 0),
                ("attempt_count", "INTEGER", 1, 0),
                ("last_error", "TEXT", 0, 0),
            ][..],
        ),
    ];
    for (table, columns) in expected {
        if !has_legacy_table(connection, table)? {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
        }
        validate_table_columns(connection, table, columns)?;
        let sql = table_sql(connection, table)?;
        if !compact_sql(&sql).starts_with(&format!("CREATETABLE{}", compact_sql(table))) {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
        }
    }
    schema_objects::validate_owned_objects(connection, &LEGACY_TABLES, &[], false)
}

pub(super) fn is_previous_shape(connection: &Connection) -> bool {
    validate_previous_shape(connection).is_ok()
}

pub(super) fn validate_previous_shape(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_previous_columns(connection)?;
    validate_previous_table_sql(connection, "account_identity_issuer_v2_key_registry")?;
    validate_previous_table_sql(connection, "account_identity_issuer_v2_receipt")?;
    validate_previous_table_sql(connection, "account_identity_issuer_v2_outbox")?;
    validate_index(
        connection,
        "account_identity_issuer_v2_key_registry_current",
        &[
            "account_id",
            "household_id",
            "service",
            "key_state",
            "key_generation",
        ],
    )?;
    validate_index(
        connection,
        "account_identity_issuer_v2_receipt_lookup",
        &["account_id", "household_id", "service", "receipt_state"],
    )?;
    validate_index(
        connection,
        "account_identity_issuer_v2_outbox_delivery",
        &["service", "delivery_state", "next_attempt_at", "receipt_id"],
    )?;
    schema_objects::validate_previous_metadata_if_present(connection)?;
    schema_objects::validate_owned_objects(connection, &PREVIOUS_TABLES, &PREVIOUS_INDEXES, true)
}

fn validate_previous_columns(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let expected = [
        (
            "account_identity_issuer_v2_key_registry",
            &[
                ("account_id", "TEXT", 1_i64, 0_i64),
                ("household_id", "TEXT", 1, 0),
                ("service", "TEXT", 1, 0),
                ("service_binding_id", "TEXT", 1, 0),
                ("key_id", "TEXT", 1, 0),
                ("key_generation", "INTEGER", 1, 4),
                ("public_key", "BLOB", 1, 0),
                ("authority_generation", "INTEGER", 1, 0),
                ("key_state", "TEXT", 1, 0),
            ][..],
        ),
        (
            "account_identity_issuer_v2_receipt",
            &[
                ("receipt_id", "TEXT", 1_i64, 1_i64),
                ("account_id", "TEXT", 1, 0),
                ("household_id", "TEXT", 1, 0),
                ("service", "TEXT", 1, 0),
                ("service_binding_id", "TEXT", 1, 0),
                ("key_id", "TEXT", 1, 0),
                ("key_generation", "INTEGER", 1, 0),
                ("authority_generation", "INTEGER", 1, 0),
                ("session_generation", "INTEGER", 1, 0),
                ("correlation_id", "TEXT", 1, 0),
                ("idempotency_key", "TEXT", 1, 0),
                ("payload_digest", "TEXT", 1, 0),
                ("issued_at", "TEXT", 1, 0),
                ("expires_at", "TEXT", 1, 0),
                ("wire", "BLOB", 1, 0),
                ("receipt_state", "TEXT", 1, 0),
            ][..],
        ),
        (
            "account_identity_issuer_v2_outbox",
            &[
                ("receipt_id", "TEXT", 1_i64, 1_i64),
                ("account_id", "TEXT", 1, 0),
                ("household_id", "TEXT", 1, 0),
                ("service", "TEXT", 1, 0),
                ("service_binding_id", "TEXT", 1, 0),
                ("key_id", "TEXT", 1, 0),
                ("key_generation", "INTEGER", 1, 0),
                ("authority_generation", "INTEGER", 1, 0),
                ("wire", "BLOB", 1, 0),
                ("delivery_state", "TEXT", 1, 0),
                ("claim_id", "TEXT", 0, 0),
                ("claimed_at", "TEXT", 0, 0),
                ("attempt_count", "INTEGER", 1, 0),
                ("last_error", "TEXT", 0, 0),
                ("last_result", "TEXT", 0, 0),
                ("next_attempt_at", "TEXT", 0, 0),
            ][..],
        ),
    ];
    for (table, columns) in expected {
        if !has_legacy_table(connection, table)? {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
        }
        validate_table_columns(connection, table, columns)?;
    }
    Ok(())
}

fn validate_previous_metadata(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_schema",
        PREVIOUS_SCHEMA_META_SQL,
    )?;
    validate_table_columns(
        connection,
        "account_identity_issuer_v2_schema",
        &[
            ("schema_name", "TEXT", 1, 1),
            ("schema_version", "INTEGER", 1, 0),
            ("migration_state", "TEXT", 1, 0),
        ],
    )?;
    let valid: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_schema
                 WHERE schema_name = ?1 AND schema_version = 2
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

fn validate_table_sql(
    connection: &Connection,
    table: &str,
    schema: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let actual = table_sql(connection, table)?;
    let expected = schema
        .split(';')
        .map(compact_sql)
        .find(|statement| {
            statement.starts_with(&format!("CREATETABLEIFNOTEXISTS{}", compact_sql(table)))
                || statement.starts_with(&format!("CREATETABLE{}", compact_sql(table)))
        })
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    (compact_sql(&actual) == expected
        || compact_sql(&actual) == expected.replace("IFNOTEXISTS", ""))
    .then_some(())
    .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_previous_table_sql(
    connection: &Connection,
    table: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let expected = match table {
        "account_identity_issuer_v2_key_registry" => PREVIOUS_KEY_REGISTRY_SQL,
        "account_identity_issuer_v2_receipt" => PREVIOUS_RECEIPT_SQL,
        "account_identity_issuer_v2_outbox" => PREVIOUS_OUTBOX_SQL,
        _ => return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
    };
    let actual = table_sql(connection, table)?;
    (compact_sql(&actual) == compact_sql(expected))
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn table_sql(
    connection: &Connection,
    table: &str,
) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [table],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
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

fn validate_index(
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
    Ok(())
}

fn compact_sql(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

const LEGACY_TABLES: [&str; 3] = [
    "account_identity_issuer_v2_key_registry",
    "account_identity_issuer_v2_receipt",
    "account_identity_issuer_v2_outbox",
];

const PREVIOUS_TABLES: [&str; 4] = [
    "account_identity_issuer_v2_schema",
    "account_identity_issuer_v2_key_registry",
    "account_identity_issuer_v2_receipt",
    "account_identity_issuer_v2_outbox",
];

const PREVIOUS_INDEXES: [&str; 3] = [
    "account_identity_issuer_v2_key_registry_current",
    "account_identity_issuer_v2_receipt_lookup",
    "account_identity_issuer_v2_outbox_delivery",
];

const PREVIOUS_SCHEMA_META_SQL: &str = "CREATE TABLE account_identity_issuer_v2_schema (
    schema_name TEXT PRIMARY KEY CHECK (schema_name = 'account_identity_issuer_v2'),
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    migration_state TEXT NOT NULL CHECK (migration_state = 'ready')
) STRICT";

const PREVIOUS_KEY_REGISTRY_SQL: &str = "CREATE TABLE account_identity_issuer_v2_key_registry (
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
) STRICT";

const PREVIOUS_RECEIPT_SQL: &str = "CREATE TABLE account_identity_issuer_v2_receipt (
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
) STRICT";

const PREVIOUS_OUTBOX_SQL: &str = "CREATE TABLE account_identity_issuer_v2_outbox (
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
) STRICT";
