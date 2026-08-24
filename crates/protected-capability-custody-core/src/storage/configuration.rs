use rusqlite::config::DbConfig;
use rusqlite::{Connection, TransactionBehavior};

use super::StorageError;

const JOURNAL_MODE: &str = "persist";
const LOCKING_MODE: &str = "exclusive";
const SYNCHRONOUS_FULL: i64 = 2;
const TEMP_STORE_MEMORY: i64 = 2;
const JOURNAL_SIZE_LIMIT: i64 = 0;

pub(super) fn configure(connection: &mut Connection) -> Result<(), StorageError> {
    let existing_journal_mode = pragma_text(connection, "journal_mode")?;
    if !existing_journal_mode.eq_ignore_ascii_case("delete")
        && !existing_journal_mode.eq_ignore_ascii_case(JOURNAL_MODE)
    {
        return Err(StorageError::Tampered);
    }
    if !connection.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?
        || connection.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false)?
    {
        return Err(StorageError::Unavailable);
    }
    connection.execute_batch(
        "PRAGMA foreign_keys = ON;
         PRAGMA synchronous = FULL;
         PRAGMA trusted_schema = OFF;
         PRAGMA temp_store = MEMORY;",
    )?;
    let journal_mode = connection.query_row("PRAGMA journal_mode = PERSIST", [], |row| {
        row.get::<_, String>(0)
    })?;
    if !journal_mode.eq_ignore_ascii_case(JOURNAL_MODE) {
        return Err(StorageError::Unavailable);
    }
    let journal_size_limit = connection.query_row("PRAGMA journal_size_limit = 0", [], |row| {
        row.get::<_, i64>(0)
    })?;
    if journal_size_limit != JOURNAL_SIZE_LIMIT {
        return Err(StorageError::Unavailable);
    }
    let locking_mode = connection.query_row("PRAGMA locking_mode = EXCLUSIVE", [], |row| {
        row.get::<_, String>(0)
    })?;
    if !locking_mode.eq_ignore_ascii_case(LOCKING_MODE) {
        return Err(StorageError::Unavailable);
    }
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Exclusive)?;
    transaction.commit()?;
    validate(connection)
}

pub(super) fn validate(connection: &Connection) -> Result<(), StorageError> {
    let journal_mode = pragma_text(connection, "journal_mode")?;
    let locking_mode = pragma_text(connection, "locking_mode")?;
    let foreign_keys = pragma_integer(connection, "foreign_keys")?;
    let synchronous = pragma_integer(connection, "synchronous")?;
    let trusted_schema = pragma_integer(connection, "trusted_schema")?;
    let temp_store = pragma_integer(connection, "temp_store")?;
    let journal_size_limit = pragma_integer(connection, "journal_size_limit")?;
    if !journal_mode.eq_ignore_ascii_case(JOURNAL_MODE)
        || !locking_mode.eq_ignore_ascii_case(LOCKING_MODE)
        || foreign_keys != 1
        || synchronous != SYNCHRONOUS_FULL
        || trusted_schema != 0
        || temp_store != TEMP_STORE_MEMORY
        || journal_size_limit != JOURNAL_SIZE_LIMIT
        || !connection.db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE)?
        || connection.db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA)?
    {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn pragma_text(connection: &Connection, name: &str) -> Result<String, StorageError> {
    connection
        .query_row(&format!("PRAGMA {name}"), [], |row| row.get(0))
        .map_err(StorageError::from)
}

fn pragma_integer(connection: &Connection, name: &str) -> Result<i64, StorageError> {
    connection
        .query_row(&format!("PRAGMA {name}"), [], |row| row.get(0))
        .map_err(StorageError::from)
}
