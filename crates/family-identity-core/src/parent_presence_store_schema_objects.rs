use rusqlite::Connection;

use crate::parent_presence_store::ParentPresenceStoreError;

pub(crate) fn validate_schema_objects(
    connection: &Connection,
    expected: &[(&str, &str, &str)],
) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name",
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let actual = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let expected = expected
        .iter()
        .map(|(kind, name, table)| ((*kind).to_owned(), (*name).to_owned(), (*table).to_owned()))
        .collect::<Vec<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

pub(crate) fn validate_foreign_keys_enabled(
    connection: &Connection,
) -> Result<(), ParentPresenceStoreError> {
    let enabled = connection
        .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    if enabled == 1 {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}
