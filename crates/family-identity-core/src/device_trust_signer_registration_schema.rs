use rusqlite::{Connection, OptionalExtension};

use crate::device_trust_lifecycle::DeviceTrustLifecycleError;

const SIGNER_TABLE: &str = "device_trust_signer_registration";
const SIGNER_TABLE_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_SIGNER_REGISTRATION(",
    "FAMILY_IDTEXTNOTNULL,",
    "TRUST_SUBJECTTEXTNOTNULL,",
    "PARENT_DEVICE_IDTEXTNOTNULL,",
    "CHILD_DEVICE_IDTEXTNOTNULL,",
    "INSTALLATION_IDTEXTNOTNULL,",
    "SIGNER_PUBLIC_KEYBLOBNOTNULLCHECK(LENGTH(SIGNER_PUBLIC_KEY)=32),",
    "SIGNER_KEY_IDTEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_ID)=32),",
    "SIGNER_KEY_SHA256TEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_SHA256)=64),",
    "REGISTRATION_RECEIPTTEXTNOTNULLUNIQUECHECK(LENGTH(REGISTRATION_RECEIPT)=64),",
    "PARENT_PRESENCE_RECEIPTTEXTNOTNULLCHECK(LENGTH(PARENT_PRESENCE_RECEIPT)=64),",
    "PARENT_INTENT_DIGESTTEXTNOTNULLCHECK(LENGTH(PARENT_INTENT_DIGEST)=64),",
    "PARENT_ROUTE_IDTEXTNOTNULLCHECK(LENGTH(PARENT_ROUTE_ID)BETWEEN1AND256),",
    "CREDENTIAL_IDTEXTNOTNULLCHECK(LENGTH(CREDENTIAL_ID)BETWEEN1AND512),",
    "CREDENTIAL_ALGORITHMINTEGERNOTNULLCHECK(CREDENTIAL_ALGORITHM=-8),",
    "CREDENTIAL_SIGN_COUNTINTEGERNOTNULLCHECK(CREDENTIAL_SIGN_COUNT>=0),",
    "LIFECYCLE_GENERATIONINTEGERNOTNULLCHECK(LIFECYCLE_GENERATION>0),",
    "INSTALLATION_BINDING_GENERATIONINTEGERNOTNULLCHECK(INSTALLATION_BINDING_GENERATION>0),",
    "AUTHORITY_GENERATIONINTEGERNOTNULLCHECK(AUTHORITY_GENERATION>0),",
    "REGISTRATION_STATETEXTNOTNULLCHECK(REGISTRATION_STATEIN('ACTIVE','REVOKED')),",
    "PRIMARYKEY(FAMILY_ID,TRUST_SUBJECT,PARENT_DEVICE_ID,CHILD_DEVICE_ID,INSTALLATION_ID,SIGNER_KEY_ID)",
    ")STRICT"
);
const ACTIVE_INDEX_SQL: &str = concat!(
    "CREATEUNIQUEINDEXDEVICE_TRUST_SIGNER_REGISTRATION_ACTIVE_KEY",
    "ONDEVICE_TRUST_SIGNER_REGISTRATION(",
    "FAMILY_ID,TRUST_SUBJECT,PARENT_DEVICE_ID,CHILD_DEVICE_ID)",
    "WHEREREGISTRATION_STATE='ACTIVE'"
);

pub(crate) fn validate(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    validate_table_sql(connection)?;
    validate_columns(connection)?;
    validate_indexes(connection)?;
    validate_owned_objects(connection)?;
    validate_integrity(connection)
}

fn validate_table_sql(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master
             WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [SIGNER_TABLE],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .ok_or(DeviceTrustLifecycleError::Unavailable)?;
    let compact = compact_sql(&sql);
    let compact = compact.trim_end_matches(';');
    let with_if_not_exists = SIGNER_TABLE_SQL.replacen("CREATETABLE", "CREATETABLEIFNOTEXISTS", 1);
    (compact == SIGNER_TABLE_SQL || compact == with_if_not_exists.as_str())
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn validate_columns(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare("PRAGMA table_info(device_trust_signer_registration)")
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let columns: Vec<(String, String, i64, i64)> = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let expected = [
        ("family_id", "TEXT", 1, 1),
        ("trust_subject", "TEXT", 1, 2),
        ("parent_device_id", "TEXT", 1, 3),
        ("child_device_id", "TEXT", 1, 4),
        ("installation_id", "TEXT", 1, 5),
        ("signer_public_key", "BLOB", 1, 0),
        ("signer_key_id", "TEXT", 1, 6),
        ("signer_key_sha256", "TEXT", 1, 0),
        ("registration_receipt", "TEXT", 1, 0),
        ("parent_presence_receipt", "TEXT", 1, 0),
        ("parent_intent_digest", "TEXT", 1, 0),
        ("parent_route_id", "TEXT", 1, 0),
        ("credential_id", "TEXT", 1, 0),
        ("credential_algorithm", "INTEGER", 1, 0),
        ("credential_sign_count", "INTEGER", 1, 0),
        ("lifecycle_generation", "INTEGER", 1, 0),
        ("installation_binding_generation", "INTEGER", 1, 0),
        ("authority_generation", "INTEGER", 1, 0),
        ("registration_state", "TEXT", 1, 0),
    ];
    (columns.len() == expected.len()
        && columns.iter().zip(expected).all(|(actual, expected)| {
            actual.0 == expected.0
                && actual.1 == expected.1
                && actual.2 == expected.2
                && actual.3 == expected.3
        }))
    .then_some(())
    .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn validate_indexes(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare("PRAGMA index_list(device_trust_signer_registration)")
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let indexes: Vec<(String, i64, String, i64)> = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let mut receipt_unique_count = 0;
    let mut primary_count = 0;
    let mut active_count = 0;
    for (name, unique, origin, partial) in indexes {
        match validate_index(connection, &name, unique, &origin, partial)? {
            IndexKind::ReceiptUnique => receipt_unique_count += 1,
            IndexKind::Primary => primary_count += 1,
            IndexKind::Active => active_count += 1,
        }
    }
    (receipt_unique_count == 1 && primary_count == 1 && active_count == 1)
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn validate_owned_objects(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name, COALESCE(sql, '')
             FROM sqlite_master
             WHERE type IN ('trigger', 'view')",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let has_unowned_object = statement
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
        .any(|(kind, name, table_name, sql)| {
            let object_text = format!(
                "{} {} {} {}",
                kind.to_ascii_lowercase(),
                name.to_ascii_lowercase(),
                table_name.to_ascii_lowercase(),
                sql.to_ascii_lowercase()
            );
            object_text.contains("device_trust_signer_registration")
        });
    (!has_unowned_object)
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn validate_integrity(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let result = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    (result == "ok")
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

#[derive(Clone, Copy)]
enum IndexKind {
    Active,
    ReceiptUnique,
    Primary,
}

fn validate_index(
    connection: &Connection,
    name: &str,
    unique: i64,
    origin: &str,
    partial: i64,
) -> Result<IndexKind, DeviceTrustLifecycleError> {
    let quoted_name = name.replace('"', "\"\"");
    let mut info = connection
        .prepare(&format!("PRAGMA index_info(\"{quoted_name}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let columns: Vec<String> = info
        .query_map([], |row| row.get::<_, String>(2))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let active_columns = [
        "family_id",
        "trust_subject",
        "parent_device_id",
        "child_device_id",
    ];
    let primary_columns = [
        "family_id",
        "trust_subject",
        "parent_device_id",
        "child_device_id",
        "installation_id",
        "signer_key_id",
    ];
    let active_columns_match = columns.iter().map(String::as_str).eq(active_columns);
    let primary_columns_match = columns.iter().map(String::as_str).eq(primary_columns);
    let kind = if name == "device_trust_signer_registration_active_key" {
        if unique != 1 || origin != "c" || partial != 1 || !active_columns_match {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        let sql = connection
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type = 'index' AND name = ?1",
                [name],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
            .ok_or(DeviceTrustLifecycleError::Unavailable)?;
        let compact = compact_sql(&sql);
        let compact = compact.trim_end_matches(';');
        let with_if_not_exists =
            ACTIVE_INDEX_SQL.replacen("CREATEUNIQUEINDEX", "CREATEUNIQUEINDEXIFNOTEXISTS", 1);
        if compact != ACTIVE_INDEX_SQL && compact != with_if_not_exists.as_str() {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        IndexKind::Active
    } else if origin == "u" && unique == 1 && partial == 0 {
        if columns.len() != 1 || columns.first().map(String::as_str) != Some("registration_receipt")
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        IndexKind::ReceiptUnique
    } else if origin == "pk" && unique == 1 && partial == 0 {
        if !primary_columns_match {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        IndexKind::Primary
    } else {
        return Err(DeviceTrustLifecycleError::Unavailable);
    };
    validate_binary_collation(connection, &quoted_name)?;
    Ok(kind)
}

fn compact_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

fn validate_binary_collation(
    connection: &Connection,
    quoted_name: &str,
) -> Result<(), DeviceTrustLifecycleError> {
    let mut xinfo = connection
        .prepare(&format!("PRAGMA index_xinfo(\"{quoted_name}\")"))
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let binary_collation = xinfo
        .query_map([], |row| {
            Ok((row.get::<_, Option<String>>(4)?, row.get::<_, i64>(5)?))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .into_iter()
        .filter(|(_, key)| *key == 1)
        .all(|(collation, _)| collation.as_deref() == Some("BINARY"));
    binary_collation
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}
