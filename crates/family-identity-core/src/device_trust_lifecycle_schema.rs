use rusqlite::{Connection, OptionalExtension};

use crate::{
    device_trust_lifecycle::{
        DeviceTrustLifecycleError, DeviceTrustLifecycleEvent, DeviceTrustLifecycleEventKind,
    },
    device_trust_signer_registration_validation::validate_canonical_identity,
};

const LIFECYCLE_TABLE: &str = "device_trust_lifecycle";
const OUTBOX_TABLE: &str = "device_trust_lifecycle_outbox";
const LIFECYCLE_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_LIFECYCLE(",
    "FAMILY_IDTEXTNOTNULL,",
    "TRUST_SUBJECTTEXTNOTNULL,",
    "DEVICE_REFTEXTNOTNULL,",
    "INSTALLATION_IDTEXTNOTNULL,",
    "LIFECYCLE_STATETEXTNOTNULLCHECK(LIFECYCLE_STATEIN('PENDING','TRUSTED','REVOKED','RESET-REQUIRED')),",
    "LIFECYCLE_GENERATIONINTEGERNOTNULLCHECK(LIFECYCLE_GENERATION>0),",
    "INSTALLATION_BINDING_GENERATIONINTEGERNOTNULLCHECK(INSTALLATION_BINDING_GENERATION>0),",
    "AUTHORITY_GENERATIONINTEGERNOTNULLCHECK(AUTHORITY_GENERATION>0),",
    "PRIMARYKEY(FAMILY_ID,TRUST_SUBJECT,DEVICE_REF)",
    ")STRICT"
);
const OUTBOX_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_LIFECYCLE_OUTBOX(",
    "SEQUENCEINTEGERPRIMARYKEYAUTOINCREMENT,",
    "EVENT_IDTEXTNOTNULLUNIQUE,",
    "CORRELATION_IDTEXTNOTNULL,",
    "EVENT_JSONTEXTNOTNULL,",
    "DELIVERY_STATETEXTNOTNULLCHECK(DELIVERY_STATEIN('PENDING','DELIVERED'))",
    ")STRICT"
);
pub(crate) fn validate(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    validate_table_sql(connection, LIFECYCLE_TABLE, LIFECYCLE_SQL)?;
    validate_table_sql(connection, OUTBOX_TABLE, OUTBOX_SQL)?;
    crate::device_trust_lifecycle_schema_transition::validate(connection)?;
    validate_columns(connection, LIFECYCLE_TABLE, lifecycle_columns())?;
    validate_columns(connection, OUTBOX_TABLE, outbox_columns())?;
    validate_indexes(
        connection,
        LIFECYCLE_TABLE,
        &["family_id", "trust_subject", "device_ref"],
        "pk",
    )?;
    validate_indexes(connection, OUTBOX_TABLE, &["event_id"], "u")?;
    reject_unowned_objects(connection)?;
    validate_rows(connection)?;
    validate_integrity(connection)
}

fn lifecycle_columns() -> &'static [(&'static str, &'static str, i64, i64)] {
    &[
        ("family_id", "TEXT", 1, 1),
        ("trust_subject", "TEXT", 1, 2),
        ("device_ref", "TEXT", 1, 3),
        ("installation_id", "TEXT", 1, 0),
        ("lifecycle_state", "TEXT", 1, 0),
        ("lifecycle_generation", "INTEGER", 1, 0),
        ("installation_binding_generation", "INTEGER", 1, 0),
        ("authority_generation", "INTEGER", 1, 0),
    ]
}

fn outbox_columns() -> &'static [(&'static str, &'static str, i64, i64)] {
    &[
        ("sequence", "INTEGER", 0, 1),
        ("event_id", "TEXT", 1, 0),
        ("correlation_id", "TEXT", 1, 0),
        ("event_json", "TEXT", 1, 0),
        ("delivery_state", "TEXT", 1, 0),
    ]
}

pub(crate) fn validate_table_sql(
    connection: &Connection,
    table: &str,
    expected: &str,
) -> Result<(), DeviceTrustLifecycleError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [table],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .ok_or(DeviceTrustLifecycleError::Unavailable)?;
    let compact_sql = compact_sql(&sql);
    let compact_sql = compact_sql.trim_end_matches(';');
    let with_if_not_exists = expected.replacen("CREATETABLE", "CREATETABLEIFNOTEXISTS", 1);
    (compact_sql == expected || compact_sql == with_if_not_exists.as_str())
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

pub(crate) fn validate_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), DeviceTrustLifecycleError> {
    let quoted_table = table.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info(\"{quoted_table}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let actual: Vec<ColumnShape> = statement
        .query_map([], |row| {
            Ok(ColumnShape {
                name: row.get(1)?,
                storage_type: row.get(2)?,
                not_null: row.get(3)?,
                primary_position: row.get(5)?,
            })
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    (actual.len() == expected.len()
        && actual.iter().zip(expected).all(|(actual, expected)| {
            actual.name.as_str() == expected.0
                && actual.storage_type.as_str() == expected.1
                && actual.not_null == expected.2
                && actual.primary_position == expected.3
        }))
    .then_some(())
    .ok_or(DeviceTrustLifecycleError::Unavailable)
}

pub(crate) fn validate_indexes(
    connection: &Connection,
    table: &str,
    expected_columns: &[&str],
    expected_origin: &str,
) -> Result<(), DeviceTrustLifecycleError> {
    let quoted_table = table.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list(\"{quoted_table}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let indexes: Vec<(String, i64, String, i64)> = statement
        .query_map([], |row| {
            Ok((row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if indexes.len() != 1 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let (name, unique, origin, partial) = &indexes[0];
    if *unique != 1 || origin != expected_origin || *partial != 0 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let quoted_name = name.replace('"', "\"\"");
    let mut info = connection
        .prepare(&format!("PRAGMA index_info(\"{quoted_name}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let columns: Vec<String> = info
        .query_map([], |row| row.get(2))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if !columns
        .iter()
        .map(String::as_str)
        .eq(expected_columns.iter().copied())
    {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    validate_binary_collation(connection, &quoted_name)
}

fn validate_binary_collation(
    connection: &Connection,
    quoted_index: &str,
) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_xinfo(\"{quoted_index}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let is_binary = statement
        .query_map([], |row| {
            Ok((row.get::<_, Option<String>>(4)?, row.get::<_, i64>(5)?))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .into_iter()
        .filter(|(_, is_key)| *is_key == 1)
        .all(|(collation, _)| collation.as_deref() == Some("BINARY"));
    is_binary
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn reject_unowned_objects(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name, COALESCE(sql, '')
             FROM sqlite_master
             WHERE type IN ('trigger', 'view')",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let has_related_object = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .into_iter()
        .any(|(kind, name, table, sql)| {
            let text = format!("{kind} {name} {table} {sql}").to_ascii_lowercase();
            text.contains(LIFECYCLE_TABLE)
                || text.contains(OUTBOX_TABLE)
                || text.contains(crate::device_trust_lifecycle_schema_transition::TABLE)
        });
    (!has_related_object)
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn validate_rows(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    validate_lifecycle_rows(connection)?;
    validate_outbox_rows(connection)
}

fn validate_lifecycle_rows(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT family_id, trust_subject, device_ref, installation_id, lifecycle_state,
                    lifecycle_generation, installation_binding_generation, authority_generation
             FROM device_trust_lifecycle
             ORDER BY family_id, trust_subject, device_ref",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, i64>(7)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    for row in rows {
        let (family, subject, device, installation, state, lifecycle, binding, authority) =
            row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        for identity in [&family, &subject, &device, &installation] {
            validate_canonical_identity(identity)
                .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        }
        if !matches!(
            state.as_str(),
            "pending" | "trusted" | "revoked" | "reset-required"
        ) || lifecycle <= 0
            || binding <= 0
            || authority <= 0
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
    }
    Ok(())
}

fn validate_outbox_rows(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT sequence, event_id, correlation_id, event_json, delivery_state
             FROM device_trust_lifecycle_outbox
             ORDER BY sequence",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    for row in rows {
        let (sequence, event_id, correlation_id, json, delivery_state) =
            row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let event: DeviceTrustLifecycleEvent =
            serde_json::from_str(&json).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        validate_canonical_identity(&correlation_id)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        if sequence <= 0
            || event_id != event.event_id
            || correlation_id != event.correlation_id
            || !matches!(delivery_state.as_str(), "pending" | "delivered")
            || event.lifecycle_generation == 0
            || event.installation_binding_generation == 0
            || !is_lower_hex(&event.household_binding, 64)
            || !is_lower_hex(&event.device_binding, 64)
            || !valid_event_state(&event)
            || !valid_event_id(&event)
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
    }
    Ok(())
}

fn valid_event_state(event: &DeviceTrustLifecycleEvent) -> bool {
    use crate::device_trust_lifecycle::DeviceTrustLifecycleState;

    matches!(
        (event.kind, event.state),
        (
            DeviceTrustLifecycleEventKind::Registered,
            DeviceTrustLifecycleState::Pending
        ) | (
            DeviceTrustLifecycleEventKind::Activated
                | DeviceTrustLifecycleEventKind::Repaired
                | DeviceTrustLifecycleEventKind::SignerRegistered,
            DeviceTrustLifecycleState::Trusted
        ) | (
            DeviceTrustLifecycleEventKind::Revoked,
            DeviceTrustLifecycleState::Revoked
        ) | (
            DeviceTrustLifecycleEventKind::ResetRequired,
            DeviceTrustLifecycleState::ResetRequired
        ) | (
            DeviceTrustLifecycleEventKind::SignerRevoked,
            DeviceTrustLifecycleState::Revoked | DeviceTrustLifecycleState::ResetRequired
        )
    )
}

fn valid_event_id(event: &DeviceTrustLifecycleEvent) -> bool {
    let prefix = format!("{}:", event.device_binding);
    if matches!(
        event.kind,
        DeviceTrustLifecycleEventKind::SignerRegistered
            | DeviceTrustLifecycleEventKind::SignerRevoked
    ) {
        let suffix = format!(":{:?}:{}", event.kind, event.lifecycle_generation);
        let Some(middle) = event
            .event_id
            .strip_prefix(&prefix)
            .and_then(|value| value.strip_suffix(&suffix))
        else {
            return false;
        };
        let Some((signer_binding, correlation)) = middle.split_once(':') else {
            return false;
        };
        is_lower_hex(signer_binding, 64) && correlation == event.correlation_id
    } else {
        let suffix = format!(":{}", event.lifecycle_generation);
        event
            .event_id
            .strip_prefix(&prefix)
            .and_then(|value| value.strip_suffix(&suffix))
            == Some(event.correlation_id.as_str())
    }
}

fn validate_integrity(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let result = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if result != "ok" {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let foreign_key_violation = connection
        .query_row("PRAGMA foreign_key_check", [], |_row| Ok(()))
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    foreign_key_violation
        .is_none()
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn compact_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

pub(crate) fn is_lower_hex(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

struct ColumnShape {
    name: String,
    storage_type: String,
    not_null: i64,
    primary_position: i64,
}
