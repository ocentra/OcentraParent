use rusqlite::{Connection, OptionalExtension};

use super::super::{
    AccountIdentityIssuerError, CLOCK_SCHEMA_SQL, KEY_REGISTRY_SCHEMA_SQL,
    TRANSPORT_RECEIPT_SCHEMA_SQL,
};

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_key_registry(connection)?;
    validate_receipt(connection)?;
    validate_clock(connection)
}

fn validate_key_registry(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_key_registry",
        KEY_REGISTRY_SCHEMA_SQL,
    )?;
    validate_columns(
        connection,
        "account_identity_issuer_key_registry",
        &[
            ("account_id", "TEXT", 1, 1),
            ("household_id", "TEXT", 1, 2),
            ("service_binding_id", "TEXT", 1, 3),
            ("key_id", "TEXT", 1, 0),
            ("key_version", "INTEGER", 1, 4),
            ("public_key", "BLOB", 1, 0),
            ("key_state", "TEXT", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("revoked_generation", "INTEGER", 0, 0),
            ("service_label", "TEXT", 1, 0),
        ],
    )?;
    validate_index_sql(
        connection,
        "account_identity_issuer_key_registry_current",
        KEY_REGISTRY_SCHEMA_SQL,
    )?;
    validate_index(
        connection,
        "account_identity_issuer_key_registry",
        "account_identity_issuer_key_registry_current",
        &[
            "account_id",
            "household_id",
            "service_label",
            "key_state",
            "key_version",
        ],
        &[
            (
                "sqlite_autoindex_account_identity_issuer_key_registry_1",
                "pk",
                &[
                    "account_id",
                    "household_id",
                    "service_binding_id",
                    "key_version",
                ],
            ),
            (
                "sqlite_autoindex_account_identity_issuer_key_registry_2",
                "u",
                &["key_id"],
            ),
        ],
    )
}

fn validate_receipt(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_transport_receipt",
        TRANSPORT_RECEIPT_SCHEMA_SQL,
    )?;
    validate_columns(
        connection,
        "account_identity_issuer_transport_receipt",
        &[
            ("receipt_id", "TEXT", 0, 1),
            ("account_id", "TEXT", 1, 0),
            ("household_id", "TEXT", 1, 0),
            ("service_binding_id", "TEXT", 1, 0),
            ("service_label", "TEXT", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("key_id", "TEXT", 1, 0),
            ("key_version", "INTEGER", 1, 0),
            ("issued_at_millis", "INTEGER", 1, 0),
            ("expires_at_millis", "INTEGER", 1, 0),
            ("receipt_state", "TEXT", 1, 0),
            ("consumed_at_millis", "INTEGER", 0, 0),
        ],
    )?;
    validate_index_sql(
        connection,
        "account_identity_issuer_transport_receipt_lookup",
        TRANSPORT_RECEIPT_SCHEMA_SQL,
    )?;
    validate_index(
        connection,
        "account_identity_issuer_transport_receipt",
        "account_identity_issuer_transport_receipt_lookup",
        &[
            "account_id",
            "household_id",
            "service_binding_id",
            "key_id",
            "key_version",
            "receipt_state",
        ],
        &[(
            "sqlite_autoindex_account_identity_issuer_transport_receipt_1",
            "pk",
            &["receipt_id"],
        )],
    )
}

fn validate_clock(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_clock",
        CLOCK_SCHEMA_SQL,
    )?;
    validate_columns(
        connection,
        "account_identity_issuer_clock",
        &[
            ("clock_id", "INTEGER", 0, 1),
            ("last_unix_millis", "INTEGER", 1, 0),
        ],
    )?;
    validate_index_catalog(connection, "account_identity_issuer_clock", &[])
}

pub(super) fn validate_table_sql(
    connection: &Connection,
    table: &str,
    schema: &str,
) -> Result<(), AccountIdentityIssuerError> {
    let actual = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [table],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
    let marker = format!("CREATETABLEIFNOTEXISTS{}", compact_sql(table));
    let expected = schema
        .split(';')
        .map(compact_sql)
        .find(|statement| statement.starts_with(&marker))
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
    let actual = compact_sql(&actual);
    (actual == expected || actual == expected.replace("IFNOTEXISTS", ""))
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)
}

pub(super) fn validate_index_sql(
    connection: &Connection,
    index: &str,
    schema: &str,
) -> Result<(), AccountIdentityIssuerError> {
    let actual = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'index' AND name = ?1",
            [index],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
    let marker = format!("CREATEINDEXIFNOTEXISTS{}", compact_sql(index));
    let expected = schema
        .split(';')
        .map(compact_sql)
        .find(|statement| statement.starts_with(&marker))
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
    let actual = compact_sql(&actual);
    (actual == expected || actual == expected.replace("IFNOTEXISTS", ""))
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)
}

fn compact_sql(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

pub(super) fn validate_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), AccountIdentityIssuerError> {
    let quoted_table = table.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info(\"{quoted_table}\")"))
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    let actual = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(
            |(
                (name, kind, not_null, primary_position),
                (expected_name, expected_kind, expected_not_null, expected_primary_position),
            )| {
                name != expected_name
                    || kind.to_ascii_uppercase() != *expected_kind
                    || not_null != expected_not_null
                    || primary_position != expected_primary_position
            },
        )
    {
        return Err(AccountIdentityIssuerError::InvalidDurableSchema);
    }
    Ok(())
}

pub(super) fn validate_index(
    connection: &Connection,
    table: &str,
    expected_name: &str,
    expected_columns: &[&str],
    expected_auto_indexes: &[(&str, &str, &[&str])],
) -> Result<(), AccountIdentityIssuerError> {
    let actual = index_list(connection, table)?;
    if actual.len() != expected_auto_indexes.len() + 1 {
        return Err(AccountIdentityIssuerError::InvalidDurableSchema);
    }
    let custom = actual
        .iter()
        .find(|(name, _, _, _)| name == expected_name)
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
    if custom.1 != 0 || custom.2 != "c" || custom.3 != 0 {
        return Err(AccountIdentityIssuerError::InvalidDurableSchema);
    }
    validate_index_columns(connection, expected_name, expected_columns)?;
    for (expected_name, expected_origin, expected_columns) in expected_auto_indexes {
        let actual = actual
            .iter()
            .find(|(name, _, _, _)| name == expected_name)
            .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
        if actual.1 != 1 || actual.2 != *expected_origin || actual.3 != 0 {
            return Err(AccountIdentityIssuerError::InvalidDurableSchema);
        }
        validate_index_columns(connection, expected_name, expected_columns)?;
    }
    Ok(())
}

fn validate_index_catalog(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, &[&str])],
) -> Result<(), AccountIdentityIssuerError> {
    let actual = index_list(connection, table)?;
    if actual.len() != expected.len() {
        return Err(AccountIdentityIssuerError::InvalidDurableSchema);
    }
    for (expected_name, expected_origin, expected_columns) in expected {
        let actual = actual
            .iter()
            .find(|(name, _, _, _)| name == expected_name)
            .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)?;
        if actual.1 != 1 || actual.2 != *expected_origin || actual.3 != 0 {
            return Err(AccountIdentityIssuerError::InvalidDurableSchema);
        }
        validate_index_columns(connection, expected_name, expected_columns)?;
    }
    Ok(())
}

fn index_list(
    connection: &Connection,
    table: &str,
) -> Result<Vec<(String, i64, String, i64)>, AccountIdentityIssuerError> {
    let quoted_table = table.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list(\"{quoted_table}\")"))
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    let actual = statement
        .query_map([], |row| {
            Ok((row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
        .collect::<Result<_, _>>()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    Ok(actual)
}

fn validate_index_columns(
    connection: &Connection,
    index: &str,
    expected: &[&str],
) -> Result<(), AccountIdentityIssuerError> {
    let quoted_index = index.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA index_info(\"{quoted_index}\")"))
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    let actual: Vec<String> = statement
        .query_map([], |row| row.get(2))
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
        .collect::<Result<_, _>>()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    (actual
        .iter()
        .map(String::as_str)
        .eq(expected.iter().copied()))
    .then_some(())
    .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)
}
