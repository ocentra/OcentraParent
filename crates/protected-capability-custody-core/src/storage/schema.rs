use getrandom::fill;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

use super::{StorageError, METADATA_TABLE_NAME, TABLE_NAME};

const APPLICATION_ID: i64 = 1_329_811_523;
const CREATE_METADATA_SQL: &str = "CREATE TABLE protected_capability_custody_metadata (
    singleton INTEGER NOT NULL PRIMARY KEY CHECK (singleton = 1),
    record_namespace BLOB NOT NULL,
    schema_version INTEGER NOT NULL,
    database_instance_id BLOB NOT NULL
) WITHOUT ROWID";
const CREATE_RECORDS_SQL: &str = "CREATE TABLE protected_capability_custody_records (
    record_id BLOB NOT NULL PRIMARY KEY,
    lookup_digest BLOB NOT NULL UNIQUE,
    binding_digest BLOB NOT NULL,
    canonical_binding BLOB NOT NULL,
    state INTEGER NOT NULL,
    sequence INTEGER NOT NULL,
    key_epoch INTEGER NOT NULL,
    writer_epoch INTEGER NOT NULL,
    anti_rollback_watermark INTEGER NOT NULL,
    sealed BLOB NOT NULL,
    schema_version INTEGER NOT NULL,
    binding_version INTEGER NOT NULL,
    database_identity BLOB NOT NULL,
    cas_digest BLOB NOT NULL
) WITHOUT ROWID";

pub(super) fn initialize_or_validate(
    connection: &mut Connection,
    was_empty: bool,
) -> Result<[u8; 32], StorageError> {
    let objects = objects(connection)?;
    if objects.is_empty() {
        if !was_empty {
            return Err(StorageError::Tampered);
        }
        initialize(connection)?;
    }
    validate(connection)?;
    load_instance_id(connection)
}

pub(super) fn validate(connection: &Connection) -> Result<(), StorageError> {
    let objects = objects(connection)?;
    let expected = [
        ("table".to_owned(), METADATA_TABLE_NAME.to_owned()),
        ("table".to_owned(), TABLE_NAME.to_owned()),
    ];
    if objects.as_slice() != expected {
        return Err(StorageError::Tampered);
    }
    validate_pragmas(connection)?;
    validate_sql(connection, METADATA_TABLE_NAME, CREATE_METADATA_SQL)?;
    validate_sql(connection, TABLE_NAME, CREATE_RECORDS_SQL)?;
    validate_metadata_columns(connection)?;
    validate_record_columns(connection)?;
    load_instance_id(connection).map(|_| ())
}

pub(super) fn validate_integrity(connection: &Connection) -> Result<(), StorageError> {
    let integrity =
        connection.query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))?;
    if integrity != "ok" {
        return Err(StorageError::Tampered);
    }
    let foreign_key_violation = connection
        .query_row("PRAGMA foreign_key_check", [], |_| Ok(()))
        .optional()?;
    if foreign_key_violation.is_some() {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn initialize(connection: &mut Connection) -> Result<(), StorageError> {
    let mut database_instance_id = [0_u8; 32];
    fill(&mut database_instance_id).map_err(|_| StorageError::Unavailable)?;
    if database_instance_id == [0_u8; 32] {
        return Err(StorageError::Unavailable);
    }
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Exclusive)?;
    transaction.execute_batch(CREATE_METADATA_SQL)?;
    transaction.execute_batch(CREATE_RECORDS_SQL)?;
    transaction.pragma_update(None, "application_id", APPLICATION_ID)?;
    transaction.pragma_update(None, "user_version", crate::STORAGE_SCHEMA_VERSION)?;
    transaction.execute(
        "INSERT INTO protected_capability_custody_metadata \
         (singleton, record_namespace, schema_version, database_instance_id) \
         VALUES (1, ?1, ?2, ?3)",
        params![
            crate::RECORD_NAMESPACE,
            crate::STORAGE_SCHEMA_VERSION,
            database_instance_id.as_slice(),
        ],
    )?;
    transaction.commit()?;
    Ok(())
}

fn objects(connection: &Connection) -> Result<Vec<(String, String)>, StorageError> {
    let mut statement = connection.prepare(
        "SELECT type, name FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name",
    )?;
    let result = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<Vec<_>, rusqlite::Error>>()
        .map_err(StorageError::from)?;
    Ok(result)
}

fn validate_pragmas(connection: &Connection) -> Result<(), StorageError> {
    let application_id: i64 =
        connection.pragma_query_value(None, "application_id", |row| row.get(0))?;
    let user_version: i64 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if application_id != APPLICATION_ID || user_version != i64::from(crate::STORAGE_SCHEMA_VERSION)
    {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn validate_sql(connection: &Connection, table: &str, expected: &str) -> Result<(), StorageError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            params![table],
            |row| row.get::<_, String>(0),
        )
        .optional()?
        .ok_or(StorageError::Tampered)?;
    if normalize(&sql) != normalize(expected) {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn validate_metadata_columns(connection: &Connection) -> Result<(), StorageError> {
    let expected = [
        ("singleton", "INTEGER", 1_i64, 1_i64),
        ("record_namespace", "BLOB", 1, 0),
        ("schema_version", "INTEGER", 1, 0),
        ("database_instance_id", "BLOB", 1, 0),
    ];
    validate_columns(
        connection,
        "PRAGMA table_info(protected_capability_custody_metadata)",
        &expected,
    )
}

fn validate_record_columns(connection: &Connection) -> Result<(), StorageError> {
    let expected = [
        ("record_id", "BLOB", 1_i64, 1_i64),
        ("lookup_digest", "BLOB", 1, 0),
        ("binding_digest", "BLOB", 1, 0),
        ("canonical_binding", "BLOB", 1, 0),
        ("state", "INTEGER", 1, 0),
        ("sequence", "INTEGER", 1, 0),
        ("key_epoch", "INTEGER", 1, 0),
        ("writer_epoch", "INTEGER", 1, 0),
        ("anti_rollback_watermark", "INTEGER", 1, 0),
        ("sealed", "BLOB", 1, 0),
        ("schema_version", "INTEGER", 1, 0),
        ("binding_version", "INTEGER", 1, 0),
        ("database_identity", "BLOB", 1, 0),
        ("cas_digest", "BLOB", 1, 0),
    ];
    validate_columns(
        connection,
        "PRAGMA table_info(protected_capability_custody_records)",
        &expected,
    )
}

fn validate_columns(
    connection: &Connection,
    pragma: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), StorageError> {
    let mut statement = connection.prepare(pragma)?;
    let columns = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;
    let matches = columns.len() == expected.len()
        && columns.iter().zip(expected).all(|(actual, expected)| {
            actual.0 == expected.0
                && actual.1 == expected.1
                && actual.2 == expected.2
                && actual.3 == expected.3
        });
    if !matches {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn load_instance_id(connection: &Connection) -> Result<[u8; 32], StorageError> {
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM protected_capability_custody_metadata",
        [],
        |row| row.get(0),
    )?;
    if count != 1 {
        return Err(StorageError::Tampered);
    }
    let (namespace, schema_version, instance): (Vec<u8>, i64, Vec<u8>) = connection.query_row(
        "SELECT record_namespace, schema_version, database_instance_id \
         FROM protected_capability_custody_metadata WHERE singleton = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let instance: [u8; 32] = instance.try_into().map_err(|_| StorageError::Tampered)?;
    if namespace != crate::RECORD_NAMESPACE
        || schema_version != i64::from(crate::STORAGE_SCHEMA_VERSION)
        || instance == [0_u8; 32]
    {
        return Err(StorageError::Tampered);
    }
    Ok(instance)
}

fn normalize(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}
