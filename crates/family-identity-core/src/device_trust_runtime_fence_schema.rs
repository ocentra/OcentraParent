use rusqlite::{Connection, OptionalExtension};

use super::DeviceTrustRuntimeFenceError;

pub(super) const TABLE: &str = "device_trust_runtime_fence_reservation";
const TABLE_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_RUNTIME_FENCE_RESERVATION(",
    "OPERATION_IDTEXTNOTNULLPRIMARYKEY,",
    "RESERVATION_REFTEXTNOTNULLUNIQUECHECK(LENGTH(RESERVATION_REF)=64),",
    "ACTION_CODEINTEGERNOTNULLCHECK(ACTION_CODEBETWEEN0AND10),",
    "FAMILY_IDTEXTNOTNULL,",
    "TRUST_SUBJECTTEXTNOTNULL,",
    "PARENT_DEVICE_IDTEXTNOTNULL,",
    "CHILD_DEVICE_IDTEXTNOTNULL,",
    "INSTALLATION_IDTEXTNOTNULL,",
    "SIGNER_KEY_IDTEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_ID)=32),",
    "SIGNER_KEY_SHA256TEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_SHA256)=64),",
    "LIFECYCLE_GENERATIONINTEGERNOTNULLCHECK(LIFECYCLE_GENERATION>0),",
    "INSTALLATION_BINDING_GENERATIONINTEGERNOTNULLCHECK(INSTALLATION_BINDING_GENERATION>0),",
    "AUTHORITY_GENERATIONINTEGERNOTNULLCHECK(AUTHORITY_GENERATION>0),",
    "RESERVATION_STATETEXTNOTNULLCHECK(RESERVATION_STATEIN('PREPARED','COMMITTED','ABORTED')),",
    "OUTCOME_DIGESTTEXTCHECK(OUTCOME_DIGESTISNULLORLENGTH(OUTCOME_DIGEST)=64)",
    ")STRICT"
);

pub(super) fn create_schema(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    connection
        .execute_batch(
            "CREATE TABLE device_trust_runtime_fence_reservation (
                operation_id TEXT NOT NULL PRIMARY KEY,
                reservation_ref TEXT NOT NULL UNIQUE CHECK (length(reservation_ref) = 64),
                action_code INTEGER NOT NULL CHECK (action_code BETWEEN 0 AND 10),
                family_id TEXT NOT NULL,
                trust_subject TEXT NOT NULL,
                parent_device_id TEXT NOT NULL,
                child_device_id TEXT NOT NULL,
                installation_id TEXT NOT NULL,
                signer_key_id TEXT NOT NULL CHECK (length(signer_key_id) = 32),
                signer_key_sha256 TEXT NOT NULL CHECK (length(signer_key_sha256) = 64),
                lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
                installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
                authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
                reservation_state TEXT NOT NULL CHECK (reservation_state IN ('prepared', 'committed', 'aborted')),
                outcome_digest TEXT CHECK (outcome_digest IS NULL OR length(outcome_digest) = 64)
            ) STRICT;",
        )
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)
}

pub(super) fn validate_schema(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    validate_table_sql(connection)?;
    validate_columns(connection)?;
    validate_indexes(connection)?;
    reject_unowned_objects(connection)?;
    super::storage::validate_rows(connection)
}

fn validate_table_sql(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [TABLE],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)?;
    let compact_sql = compact_sql(&sql);
    let compact = compact_sql.trim_end_matches(';');
    (compact == TABLE_SQL)
        .then_some(())
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)
}

fn validate_columns(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    crate::device_trust_lifecycle_schema::validate_columns(
        connection,
        TABLE,
        &[
            ("operation_id", "TEXT", 1, 1),
            ("reservation_ref", "TEXT", 1, 0),
            ("action_code", "INTEGER", 1, 0),
            ("family_id", "TEXT", 1, 0),
            ("trust_subject", "TEXT", 1, 0),
            ("parent_device_id", "TEXT", 1, 0),
            ("child_device_id", "TEXT", 1, 0),
            ("installation_id", "TEXT", 1, 0),
            ("signer_key_id", "TEXT", 1, 0),
            ("signer_key_sha256", "TEXT", 1, 0),
            ("lifecycle_generation", "INTEGER", 1, 0),
            ("installation_binding_generation", "INTEGER", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("reservation_state", "TEXT", 1, 0),
            ("outcome_digest", "TEXT", 0, 0),
        ],
    )
    .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)
}

fn validate_indexes(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list(\"{TABLE}\")"))
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    let indexes: Vec<(String, i64, String, i64)> = statement
        .query_map([], |row| {
            Ok((row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    if indexes.len() != 2 {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    let mut found_primary = false;
    let mut found_reservation = false;
    for (name, unique, origin, partial) in indexes {
        if unique != 1 || partial != 0 {
            return Err(DeviceTrustRuntimeFenceError::Unavailable);
        }
        let quoted_name = name.replace('"', "\"\"");
        let mut info = connection
            .prepare(&format!("PRAGMA index_info(\"{quoted_name}\")"))
            .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
        let columns: Vec<String> = info
            .query_map([], |row| row.get(2))
            .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
            .collect::<Result<_, _>>()
            .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
        let expected_columns = match origin.as_str() {
            "pk" => {
                found_primary = true;
                ["operation_id"].as_slice()
            }
            "u" => {
                found_reservation = true;
                ["reservation_ref"].as_slice()
            }
            _ => return Err(DeviceTrustRuntimeFenceError::Unavailable),
        };
        if !columns
            .iter()
            .map(String::as_str)
            .eq(expected_columns.iter().copied())
        {
            return Err(DeviceTrustRuntimeFenceError::Unavailable);
        }
        validate_binary_collation(connection, &quoted_name)?;
    }
    (found_primary && found_reservation)
        .then_some(())
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)
}

fn validate_binary_collation(
    connection: &Connection,
    quoted_index: &str,
) -> Result<(), DeviceTrustRuntimeFenceError> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_xinfo(\"{quoted_index}\")"))
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    let is_binary = statement
        .query_map([], |row| {
            Ok((row.get::<_, Option<String>>(4)?, row.get::<_, i64>(5)?))
        })
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .into_iter()
        .filter(|(_, is_key)| *is_key == 1)
        .all(|(collation, _)| collation.as_deref() == Some("BINARY"));
    is_binary
        .then_some(())
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)
}

fn reject_unowned_objects(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name, COALESCE(sql, '')
             FROM sqlite_master WHERE type IN ('trigger', 'view')",
        )
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    let related = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .into_iter()
        .any(|(kind, name, table, sql)| {
            format!("{kind} {name} {table} {sql}")
                .to_ascii_lowercase()
                .contains(TABLE)
        });
    (!related)
        .then_some(())
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)
}

fn compact_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}
