use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Transaction};
use thiserror::Error;

use crate::binding::Binding;

pub(crate) const TABLE_NAME: &str = "protected_capability_custody_records";
pub(crate) const SCHEMA_VERSION: i64 = 1;
const MAX_CANONICAL_BYTES: usize = 16 * 1024;
const MAX_SEALED_BYTES: usize = 64 * 1024;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) record_id: Vec<u8>,
    pub(crate) binding_digest: Vec<u8>,
    pub(crate) canonical_binding: Vec<u8>,
    pub(crate) state: i64,
    pub(crate) sequence: i64,
    pub(crate) key_epoch: i64,
    pub(crate) writer_epoch: i64,
    pub(crate) anti_rollback_watermark: i64,
    pub(crate) sealed: Vec<u8>,
    pub(crate) schema_version: i64,
}

#[derive(Debug, Error)]
pub(crate) enum StorageError {
    #[error("sqlite database operation failed")]
    Sql(#[source] rusqlite::Error),
    #[error("custody database schema or row is tampered")]
    Tampered,
}

impl From<rusqlite::Error> for StorageError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sql(error)
    }
}

pub(crate) fn open(path: &Path) -> Result<Connection, StorageError> {
    let connection = Connection::open(path)?;
    connection.execute_batch("PRAGMA foreign_keys = ON; PRAGMA synchronous = FULL;")?;
    initialize_or_validate(&connection)?;
    Ok(connection)
}

pub(crate) fn validate_all(connection: &Connection) -> Result<(), StorageError> {
    validate_schema(connection)?;
    let integrity =
        connection.query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))?;
    if integrity != "ok" {
        return Err(StorageError::Tampered);
    }
    let mut statement = connection.prepare(
        "SELECT record_id, binding_digest, canonical_binding, state, sequence, key_epoch, \
         writer_epoch, anti_rollback_watermark, sealed, schema_version \
         FROM protected_capability_custody_records ORDER BY record_id",
    )?;
    let rows = statement.query_map([], read_record)?;
    for row in rows {
        validate_record(&row?)?;
    }
    Ok(())
}

pub(crate) fn load_by_digest(
    connection: &Connection,
    digest: &[u8],
) -> Result<Option<Record>, StorageError> {
    let record = connection
        .query_row(
            "SELECT record_id, binding_digest, canonical_binding, state, sequence, key_epoch, \
             writer_epoch, anti_rollback_watermark, sealed, schema_version \
             FROM protected_capability_custody_records WHERE binding_digest = ?1",
            params![digest],
            read_record,
        )
        .optional()?;
    record.map_or(Ok(None), |record| {
        validate_record(&record)?;
        Ok(Some(record))
    })
}

pub(crate) fn load_by_id(
    connection: &Connection,
    record_id: &[u8],
) -> Result<Option<Record>, StorageError> {
    let record = connection
        .query_row(
            "SELECT record_id, binding_digest, canonical_binding, state, sequence, key_epoch, \
             writer_epoch, anti_rollback_watermark, sealed, schema_version \
             FROM protected_capability_custody_records WHERE record_id = ?1",
            params![record_id],
            read_record,
        )
        .optional()?;
    record.map_or(Ok(None), |record| {
        validate_record(&record)?;
        Ok(Some(record))
    })
}

pub(crate) fn insert(transaction: &Transaction<'_>, record: &Record) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO protected_capability_custody_records \
         (record_id, binding_digest, canonical_binding, state, sequence, key_epoch, writer_epoch, \
          anti_rollback_watermark, sealed, schema_version) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            &record.record_id,
            &record.binding_digest,
            &record.canonical_binding,
            record.state,
            record.sequence,
            record.key_epoch,
            record.writer_epoch,
            record.anti_rollback_watermark,
            &record.sealed,
            record.schema_version,
        ],
    )?;
    Ok(())
}

pub(crate) fn compare_and_replace(
    transaction: &Transaction<'_>,
    prior: &Record,
    next: &Record,
) -> Result<bool, StorageError> {
    let changed = transaction.execute(
        "UPDATE protected_capability_custody_records SET binding_digest = ?1, canonical_binding = ?2, \
         state = ?3, sequence = ?4, key_epoch = ?5, writer_epoch = ?6, \
         anti_rollback_watermark = ?7, sealed = ?8, schema_version = ?9 \
         WHERE record_id = ?10 AND state = ?11 AND sequence = ?12",
        params![
            &next.binding_digest,
            &next.canonical_binding,
            next.state,
            next.sequence,
            next.key_epoch,
            next.writer_epoch,
            next.anti_rollback_watermark,
            &next.sealed,
            next.schema_version,
            &prior.record_id,
            prior.state,
            prior.sequence,
        ],
    )?;
    Ok(changed == 1)
}

fn initialize_or_validate(connection: &Connection) -> Result<(), StorageError> {
    let mut statement = connection.prepare(
        "SELECT type, name FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name",
    )?;
    let objects = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;
    if objects.is_empty() {
        connection.execute_batch(
            "CREATE TABLE protected_capability_custody_records (\
               record_id BLOB NOT NULL PRIMARY KEY,\
               binding_digest BLOB NOT NULL UNIQUE,\
               canonical_binding BLOB NOT NULL,\
               state INTEGER NOT NULL,\
               sequence INTEGER NOT NULL,\
               key_epoch INTEGER NOT NULL,\
               writer_epoch INTEGER NOT NULL,\
               anti_rollback_watermark INTEGER NOT NULL,\
               sealed BLOB NOT NULL,\
               schema_version INTEGER NOT NULL\
             ) WITHOUT ROWID;",
        )?;
    } else if objects.len() != 1
        || objects.first().map(|object| object.0.as_str()) != Some("table")
        || objects.first().map(|object| object.1.as_str()) != Some(TABLE_NAME)
    {
        return Err(StorageError::Tampered);
    }
    validate_schema(connection)
}

fn validate_schema(connection: &Connection) -> Result<(), StorageError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1",
            params![TABLE_NAME],
            |row| row.get::<_, String>(0),
        )
        .optional()?
        .ok_or(StorageError::Tampered)?;
    let normalized = sql
        .split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase();
    let expected = "create table protected_capability_custody_records ( record_id blob not null primary key, binding_digest blob not null unique, canonical_binding blob not null, state integer not null, sequence integer not null, key_epoch integer not null, writer_epoch integer not null, anti_rollback_watermark integer not null, sealed blob not null, schema_version integer not null ) without rowid"
        .split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase();
    if normalized != expected {
        return Err(StorageError::Tampered);
    }

    let mut statement =
        connection.prepare("PRAGMA table_info(protected_capability_custody_records)")?;
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
    let expected_columns = [
        ("record_id", "BLOB", 1_i64, 1_i64),
        ("binding_digest", "BLOB", 1_i64, 0_i64),
        ("canonical_binding", "BLOB", 1_i64, 0_i64),
        ("state", "INTEGER", 1_i64, 0_i64),
        ("sequence", "INTEGER", 1_i64, 0_i64),
        ("key_epoch", "INTEGER", 1_i64, 0_i64),
        ("writer_epoch", "INTEGER", 1_i64, 0_i64),
        ("anti_rollback_watermark", "INTEGER", 1_i64, 0_i64),
        ("sealed", "BLOB", 1_i64, 0_i64),
        ("schema_version", "INTEGER", 1_i64, 0_i64),
    ];
    let columns_match = columns.len() == expected_columns.len()
        && columns
            .iter()
            .zip(expected_columns)
            .all(|(actual, expected)| {
                actual.0 == expected.0
                    && actual.1 == expected.1
                    && actual.2 == expected.2
                    && actual.3 == expected.3
            });
    if !columns_match {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn read_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<Record> {
    Ok(Record {
        record_id: row.get(0)?,
        binding_digest: row.get(1)?,
        canonical_binding: row.get(2)?,
        state: row.get(3)?,
        sequence: row.get(4)?,
        key_epoch: row.get(5)?,
        writer_epoch: row.get(6)?,
        anti_rollback_watermark: row.get(7)?,
        sealed: row.get(8)?,
        schema_version: row.get(9)?,
    })
}

fn validate_record(record: &Record) -> Result<(), StorageError> {
    if record.record_id.len() != 32
        || record.binding_digest.len() != 32
        || record.canonical_binding.is_empty()
        || record.canonical_binding.len() > MAX_CANONICAL_BYTES
        || record.sealed.is_empty()
        || record.sealed.len() > MAX_SEALED_BYTES
        || record.schema_version != SCHEMA_VERSION
        || !(1..=5).contains(&record.state)
        || record.sequence <= 0
        || record.key_epoch <= 0
        || record.writer_epoch <= 0
        || record.anti_rollback_watermark <= 0
    {
        return Err(StorageError::Tampered);
    }
    let binding = Binding::decode(&record.canonical_binding).map_err(|_| StorageError::Tampered)?;
    if binding.digest().as_slice() != record.binding_digest.as_slice() {
        return Err(StorageError::Tampered);
    }
    Ok(())
}
