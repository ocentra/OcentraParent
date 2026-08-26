use std::collections::HashSet;

use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{Connection, OptionalExtension};

#[path = "account_identity_authority_issuer_client_schema_rows.rs"]
mod schema_rows;

use super::{
    has_legacy_table, AccountIdentityAuthorityIssuerClientError, CANONICAL_SCHEMA_SQL,
    SCHEMA_META_SQL, SCHEMA_NAME, SCHEMA_VERSION,
};

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
        let sql = connection
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get::<_, String>(0),
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        if !compact_sql(&sql).starts_with(&format!("CREATETABLE{}", compact_sql(table))) {
            return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
        }
    }
    Ok(())
}

pub(super) fn validate_in_transaction(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_metadata(connection)?;
    validate_key_registry(connection)?;
    validate_receipt(connection)?;
    validate_outbox(connection)?;
    validate_indexes(connection)?;
    validate_owned_objects(connection)?;
    schema_rows::validate_rows(connection)
}

fn validate_metadata(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_schema",
        SCHEMA_META_SQL,
    )?;
    validate_table(
        connection,
        "account_identity_issuer_v2_schema",
        &[
            ("schema_name", "TEXT", 1, 1),
            ("schema_version", "INTEGER", 1, 0),
            ("migration_state", "TEXT", 1, 0),
        ],
    )?;
    validate_metadata_row(connection)
}

fn validate_key_registry(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_key_registry",
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_table(
        connection,
        "account_identity_issuer_v2_key_registry",
        &[
            ("account_id", "TEXT", 1, 0),
            ("household_id", "TEXT", 1, 0),
            ("service", "TEXT", 1, 0),
            ("service_binding_id", "TEXT", 1, 0),
            ("key_id", "TEXT", 1, 0),
            ("key_generation", "INTEGER", 1, 4),
            ("public_key", "BLOB", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("key_state", "TEXT", 1, 0),
        ],
    )
}

fn validate_receipt(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_receipt",
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_table(
        connection,
        "account_identity_issuer_v2_receipt",
        &[
            ("receipt_id", "TEXT", 1, 1),
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
        ],
    )
}

fn validate_outbox(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_v2_outbox",
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_table(
        connection,
        "account_identity_issuer_v2_outbox",
        &[
            ("receipt_id", "TEXT", 1, 1),
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
        ],
    )
}

fn validate_indexes(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
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
    validate_index_sql(
        connection,
        "account_identity_issuer_v2_key_registry_current",
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_index(
        connection,
        "account_identity_issuer_v2_receipt_lookup",
        &["account_id", "household_id", "service", "receipt_state"],
    )?;
    validate_index_sql(
        connection,
        "account_identity_issuer_v2_receipt_lookup",
        CANONICAL_SCHEMA_SQL,
    )?;
    validate_index(
        connection,
        "account_identity_issuer_v2_outbox_delivery",
        &["service", "delivery_state", "next_attempt_at", "receipt_id"],
    )?;
    validate_index_sql(
        connection,
        "account_identity_issuer_v2_outbox_delivery",
        CANONICAL_SCHEMA_SQL,
    )
}

fn validate_metadata_row(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let valid: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_schema
                 WHERE schema_name = ?1 AND schema_version = ?2
                   AND migration_state = 'ready'
            ) AND (SELECT COUNT(*) FROM account_identity_issuer_v2_schema) = 1",
            rusqlite::params![SCHEMA_NAME, SCHEMA_VERSION],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    valid
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_table(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let actual = table_columns(connection, table)?;
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

fn validate_table_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let actual = table_columns(connection, table)?;
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

fn table_columns(
    connection: &Connection,
    table: &str,
) -> Result<Vec<(String, String, i64, i64)>, AccountIdentityAuthorityIssuerClientError> {
    let quoted_table = table.replace('"', "\"\"");
    connection
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
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_table_sql(
    connection: &Connection,
    table: &str,
    schema: &str,
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
    let marker = format!("CREATETABLEIFNOTEXISTS{}", compact_sql(table));
    let marker_without_if = format!("CREATETABLE{}", compact_sql(table));
    let expected = schema
        .split(';')
        .map(compact_sql)
        .find(|statement| {
            statement.starts_with(&marker) || statement.starts_with(&marker_without_if)
        })
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let actual = compact_sql(&actual);
    (actual == expected || actual == expected.replace("IFNOTEXISTS", ""))
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_index_sql(
    connection: &Connection,
    index: &str,
    schema: &str,
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
    let marker = format!("CREATEINDEXIFNOTEXISTS{}", compact_sql(index));
    let marker_without_if = format!("CREATEINDEX{}", compact_sql(index));
    let expected = schema
        .split(';')
        .map(compact_sql)
        .find(|statement| {
            statement.starts_with(&marker) || statement.starts_with(&marker_without_if)
        })
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

fn validate_owned_objects(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let allowed_tables = [
        "account_identity_issuer_v2_schema",
        "account_identity_issuer_v2_key_registry",
        "account_identity_issuer_v2_receipt",
        "account_identity_issuer_v2_outbox",
    ];
    let allowed_indexes = [
        "account_identity_issuer_v2_key_registry_current",
        "account_identity_issuer_v2_receipt_lookup",
        "account_identity_issuer_v2_outbox_delivery",
    ];
    let mut statement = connection
        .prepare(
            "SELECT type, name FROM sqlite_master
             WHERE name LIKE 'account_identity_issuer_v2_%'
                OR (type IN ('trigger', 'view')
                    AND lower(COALESCE(sql, '')) LIKE '%account_identity_issuer_v2_%')",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut tables = HashSet::new();
    let mut indexes = HashSet::new();
    while let Some(row) = rows
        .next()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    {
        let object_type = row
            .get::<_, String>(0)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        let name = row
            .get::<_, String>(1)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        match object_type.as_str() {
            "table" if allowed_tables.contains(&name.as_str()) => {
                tables.insert(name);
            }
            "index" if allowed_indexes.contains(&name.as_str()) => {
                indexes.insert(name);
            }
            _ => return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
        }
    }
    (tables.len() == allowed_tables.len() && indexes.len() == allowed_indexes.len())
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
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
