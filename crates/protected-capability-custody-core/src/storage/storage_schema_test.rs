use rusqlite::Connection;

use super::{schema, StorageError};

#[test]
fn schema_initialization_is_repeatable_and_integrity_checked() -> Result<(), StorageError> {
    let mut connection = Connection::open_in_memory().map_err(StorageError::from)?;
    let instance = schema::initialize_or_validate(&mut connection, true)?;

    assert_ne!(instance, [0_u8; 32]);
    assert_eq!(
        schema::initialize_or_validate(&mut connection, false)?,
        instance
    );
    schema::validate_integrity(&connection)?;
    Ok(())
}

#[test]
fn schema_validation_rejects_object_pragma_and_column_drift() -> Result<(), StorageError> {
    let mut extra_object = Connection::open_in_memory().map_err(StorageError::from)?;
    schema::initialize_or_validate(&mut extra_object, true)?;
    extra_object
        .execute_batch("CREATE TABLE unexpected (value BLOB NOT NULL)")
        .map_err(StorageError::from)?;
    assert!(matches!(
        schema::validate(&extra_object),
        Err(StorageError::Tampered)
    ));

    let mut wrong_version = Connection::open_in_memory().map_err(StorageError::from)?;
    schema::initialize_or_validate(&mut wrong_version, true)?;
    wrong_version
        .pragma_update(None, "user_version", 99_i64)
        .map_err(StorageError::from)?;
    assert!(matches!(
        schema::validate(&wrong_version),
        Err(StorageError::Tampered)
    ));

    let mut wrong_column = Connection::open_in_memory().map_err(StorageError::from)?;
    schema::initialize_or_validate(&mut wrong_column, true)?;
    wrong_column
        .execute_batch(
            "ALTER TABLE protected_capability_custody_records
             RENAME COLUMN cas_digest TO changed_digest",
        )
        .map_err(StorageError::from)?;
    assert!(matches!(
        schema::validate(&wrong_column),
        Err(StorageError::Tampered)
    ));

    let mut wrong_index = Connection::open_in_memory().map_err(StorageError::from)?;
    schema::initialize_or_validate(&mut wrong_index, true)?;
    wrong_index
        .execute_batch(
            "CREATE INDEX unexpected_index
             ON protected_capability_custody_records (state)",
        )
        .map_err(StorageError::from)?;
    assert!(matches!(
        schema::validate(&wrong_index),
        Err(StorageError::Tampered)
    ));
    Ok(())
}

#[test]
fn nonempty_unrecognized_database_is_not_initialized_as_custody_schema() -> Result<(), StorageError>
{
    let mut connection = Connection::open_in_memory().map_err(StorageError::from)?;
    connection
        .execute_batch("CREATE TABLE unrelated (value BLOB NOT NULL)")
        .map_err(StorageError::from)?;

    assert!(matches!(
        schema::initialize_or_validate(&mut connection, true),
        Err(StorageError::Tampered)
    ));
    Ok(())
}
