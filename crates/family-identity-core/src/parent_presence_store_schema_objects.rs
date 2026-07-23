use rusqlite::Connection;

use crate::parent_presence_store::ParentPresenceStoreError;

const SQLITE_SEQUENCE_NAME: &str = "sqlite_sequence";
const SQLITE_SEQUENCE_SQL: &str = "CREATE TABLE sqlite_sequence(name,seq)";
const EXPECTED_INTERNAL_AUTOINDEXES: &[(&str, &str)] = &[
    (
        "sqlite_autoindex_parent_presence_challenges_1",
        "parent_presence_challenges",
    ),
    (
        "sqlite_autoindex_parent_presence_challenges_2",
        "parent_presence_challenges",
    ),
    (
        "sqlite_autoindex_parent_presence_decision_outbox_1",
        "parent_presence_decision_outbox",
    ),
    (
        "sqlite_autoindex_parent_presence_receipts_1",
        "parent_presence_receipts",
    ),
    (
        "sqlite_autoindex_parent_presence_receipts_2",
        "parent_presence_receipts",
    ),
];

struct SchemaObject {
    kind: String,
    name: String,
    table: String,
    sql: Option<String>,
}

pub(crate) fn validate_schema_objects(
    connection: &Connection,
    expected: &[(&str, &str, &str)],
) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare("SELECT type, name, tbl_name, sql FROM sqlite_schema ORDER BY type, name")
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([], |row| {
            Ok(SchemaObject {
                kind: row.get(0)?,
                name: row.get(1)?,
                table: row.get(2)?,
                sql: row.get(3)?,
            })
        })
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let actual = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;

    validate_internal_autoindexes(&actual)?;
    validate_sqlite_sequence(&actual)?;

    let actual = actual
        .iter()
        .filter(|object| !object.name.starts_with("sqlite_autoindex_"))
        .map(|object| {
            (
                object.kind.clone(),
                object.name.clone(),
                object.table.clone(),
                object.sql.is_some(),
            )
        })
        .collect::<Vec<_>>();
    let mut expected = expected
        .iter()
        .map(|(kind, name, table)| {
            (
                (*kind).to_owned(),
                (*name).to_owned(),
                (*table).to_owned(),
                true,
            )
        })
        .collect::<Vec<_>>();
    expected.push((
        "table".to_owned(),
        SQLITE_SEQUENCE_NAME.to_owned(),
        SQLITE_SEQUENCE_NAME.to_owned(),
        true,
    ));
    if actual == expected {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn validate_internal_autoindexes(objects: &[SchemaObject]) -> Result<(), ParentPresenceStoreError> {
    let actual = objects
        .iter()
        .filter(|object| object.name.starts_with("sqlite_autoindex_"))
        .collect::<Vec<_>>();
    if actual.len() != EXPECTED_INTERNAL_AUTOINDEXES.len() {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    let valid = actual
        .iter()
        .zip(EXPECTED_INTERNAL_AUTOINDEXES)
        .all(|(object, (name, table))| {
            object.kind == "index"
                && object.name == *name
                && object.table == *table
                && object.sql.is_none()
        });
    if valid {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}

fn validate_sqlite_sequence(objects: &[SchemaObject]) -> Result<(), ParentPresenceStoreError> {
    let valid = objects.iter().any(|object| {
        object.kind == "table"
            && object.name == SQLITE_SEQUENCE_NAME
            && object.table == SQLITE_SEQUENCE_NAME
            && object.sql.as_deref() == Some(SQLITE_SEQUENCE_SQL)
    });
    if valid {
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

pub(crate) fn validate_foreign_key_rows(
    connection: &Connection,
) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let violation = rows
        .next()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    if violation.is_none() {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}
