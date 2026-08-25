use std::path::Path;

use rusqlite::{params, Connection, OpenFlags, OptionalExtension, Transaction};
use thiserror::Error;

use crate::platform::{identity::DatabaseIdentity, SealedState};

mod configuration;
mod record;
mod schema;

pub(crate) const TABLE_NAME: &str = "protected_capability_custody_records";
pub(crate) const METADATA_TABLE_NAME: &str = "protected_capability_custody_metadata";

pub(crate) struct Record {
    pub(crate) record_id: [u8; 32],
    pub(crate) lookup_digest: [u8; 32],
    pub(crate) binding_digest: [u8; 32],
    pub(crate) canonical_binding: Vec<u8>,
    pub(crate) state: SealedState,
    pub(crate) sequence: u64,
    pub(crate) key_epoch: u64,
    pub(crate) writer_epoch: u64,
    pub(crate) anti_rollback_watermark: u64,
    pub(crate) sealed: Vec<u8>,
    pub(crate) schema_version: u32,
    pub(crate) binding_version: u16,
    pub(crate) database_identity: DatabaseIdentity,
    pub(crate) cas_digest: [u8; 32],
}

#[derive(Debug, Error)]
pub(crate) enum StorageError {
    #[error("custody database support is unavailable")]
    Unavailable,
    #[error("sqlite database operation failed")]
    Sql(#[source] rusqlite::Error),
    #[error("custody database schema or row is tampered")]
    Tampered,
    #[error("custody database transition is illegal")]
    IllegalTransition,
}

impl From<rusqlite::Error> for StorageError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sql(error)
    }
}

pub(crate) fn open_connection(path: &Path) -> Result<(Connection, bool), StorageError> {
    let was_empty = std::fs::metadata(path)
        .map_err(|_| StorageError::Unavailable)?
        .len()
        == 0;
    let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_PRIVATE_CACHE
        | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    Connection::open_with_flags(path, flags)
        .map(|connection| (connection, was_empty))
        .map_err(StorageError::from)
}

pub(crate) fn configure(connection: &mut Connection) -> Result<(), StorageError> {
    configuration::configure(connection)
}

pub(crate) fn initialize_or_validate(
    connection: &mut Connection,
    was_empty: bool,
) -> Result<[u8; 32], StorageError> {
    schema::initialize_or_validate(connection, was_empty)
}

pub(crate) fn validate_all(
    connection: &Connection,
    identity: DatabaseIdentity,
) -> Result<(), StorageError> {
    configuration::validate(connection)?;
    schema::validate(connection)?;
    schema::validate_integrity(connection)?;
    let mut statement = connection.prepare(&select_sql("ORDER BY record_id"))?;
    let rows = statement.query_map([], record::read_raw)?;
    let mut count = 0_i64;
    for row in rows {
        count = count.checked_add(1).ok_or(StorageError::Tampered)?;
        if count > ocentra_protected_capability_custody_protocol::constants::MAX_CUSTODY_RECORDS {
            return Err(StorageError::Tampered);
        }
        let value = record::from_raw(row?)?;
        if value.database_identity != identity {
            return Err(StorageError::Tampered);
        }
    }
    Ok(())
}

pub(crate) fn load_by_lookup(
    connection: &Connection,
    digest: &[u8; 32],
) -> Result<Option<Record>, StorageError> {
    load_one(connection, "WHERE lookup_digest = ?1", digest)
}

pub(crate) fn insert(transaction: &Transaction<'_>, value: &Record) -> Result<(), StorageError> {
    record::validate(value)?;
    let count: i64 = transaction.query_row(
        "SELECT COUNT(*) FROM protected_capability_custody_records",
        [],
        |row| row.get(0),
    )?;
    if count >= ocentra_protected_capability_custody_protocol::constants::MAX_CUSTODY_RECORDS {
        return Err(StorageError::Unavailable);
    }
    transaction.execute(
        "INSERT INTO protected_capability_custody_records \
         (record_id, lookup_digest, binding_digest, canonical_binding, state, sequence, key_epoch, \
          writer_epoch, anti_rollback_watermark, sealed, schema_version, binding_version, \
          database_identity, cas_digest) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            value.record_id.as_slice(),
            value.lookup_digest.as_slice(),
            value.binding_digest.as_slice(),
            &value.canonical_binding,
            value.state as u8,
            value.sequence as i64,
            value.key_epoch as i64,
            value.writer_epoch as i64,
            value.anti_rollback_watermark as i64,
            &value.sealed,
            value.schema_version,
            value.binding_version,
            value.database_identity.as_bytes().as_slice(),
            value.cas_digest.as_slice(),
        ],
    )?;
    Ok(())
}

pub(crate) fn compare_and_replace(
    transaction: &Transaction<'_>,
    prior: &Record,
    next: &Record,
) -> Result<bool, StorageError> {
    record::validate_transition(prior, next)?;
    let changed = transaction.execute(
        "UPDATE protected_capability_custody_records SET lookup_digest = ?1, binding_digest = ?2, \
         canonical_binding = ?3, state = ?4, sequence = ?5, key_epoch = ?6, writer_epoch = ?7, \
         anti_rollback_watermark = ?8, sealed = ?9, schema_version = ?10, binding_version = ?11, \
         database_identity = ?12, cas_digest = ?13 \
         WHERE record_id = ?14 AND lookup_digest = ?15 AND binding_digest = ?16 \
         AND canonical_binding = ?17 AND state = ?18 AND sequence = ?19 AND key_epoch = ?20 \
         AND writer_epoch = ?21 AND anti_rollback_watermark = ?22 AND sealed = ?23 \
         AND schema_version = ?24 AND binding_version = ?25 AND database_identity = ?26 \
         AND cas_digest = ?27",
        params![
            next.lookup_digest.as_slice(),
            next.binding_digest.as_slice(),
            &next.canonical_binding,
            next.state as u8,
            next.sequence as i64,
            next.key_epoch as i64,
            next.writer_epoch as i64,
            next.anti_rollback_watermark as i64,
            &next.sealed,
            next.schema_version,
            next.binding_version,
            next.database_identity.as_bytes().as_slice(),
            next.cas_digest.as_slice(),
            prior.record_id.as_slice(),
            prior.lookup_digest.as_slice(),
            prior.binding_digest.as_slice(),
            &prior.canonical_binding,
            prior.state as u8,
            prior.sequence as i64,
            prior.key_epoch as i64,
            prior.writer_epoch as i64,
            prior.anti_rollback_watermark as i64,
            &prior.sealed,
            prior.schema_version,
            prior.binding_version,
            prior.database_identity.as_bytes().as_slice(),
            prior.cas_digest.as_slice(),
        ],
    )?;
    Ok(changed == 1)
}

pub(crate) fn from_broker(
    broker: &crate::platform::record::BrokerRecord,
) -> Result<Record, StorageError> {
    record::from_broker(broker)
}

pub(crate) fn to_broker(value: &Record) -> crate::platform::record::BrokerRecord {
    record::to_broker(value)
}

fn load_one<P: rusqlite::ToSql + ?Sized>(
    connection: &Connection,
    suffix: &str,
    parameter: &P,
) -> Result<Option<Record>, StorageError> {
    let raw = connection
        .query_row(&select_sql(suffix), [parameter], record::read_raw)
        .optional()?;
    raw.map(record::from_raw).transpose()
}

fn select_sql(suffix: &str) -> String {
    format!(
        "SELECT record_id, lookup_digest, binding_digest, canonical_binding, state, sequence, \
         key_epoch, writer_epoch, anti_rollback_watermark, sealed, schema_version, binding_version, \
         database_identity, cas_digest FROM {TABLE_NAME} {suffix}"
    )
}
