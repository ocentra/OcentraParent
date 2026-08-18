pub(super) const TABLE_SQL: &str = r#"CREATE TABLE account_identity_parent_storage_confirmation (
    receipt_id TEXT PRIMARY KEY NOT NULL CHECK (
        length(receipt_id) = 64 AND receipt_id NOT GLOB '*[^0-9a-f]*'
    ),
    nonce_id TEXT NOT NULL UNIQUE CHECK (
        length(nonce_id) = 64 AND nonce_id NOT GLOB '*[^0-9a-f]*'
    ),
    provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
    provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
    household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
    account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
    parent_device_id TEXT NOT NULL CHECK (length(trim(parent_device_id)) > 0),
    child_profile_id TEXT NOT NULL CHECK (length(trim(child_profile_id)) > 0),
    child_device_id TEXT NOT NULL CHECK (length(trim(child_device_id)) > 0),
    installation_id TEXT NOT NULL CHECK (length(trim(installation_id)) > 0),
    pairing_id TEXT NOT NULL CHECK (length(trim(pairing_id)) > 0),
    route_id TEXT NOT NULL CHECK (length(trim(route_id)) > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    session_generation INTEGER NOT NULL CHECK (session_generation > 0),
    device_trust_subject TEXT NOT NULL CHECK (length(trim(device_trust_subject)) > 0),
    device_lifecycle_generation INTEGER NOT NULL CHECK (device_lifecycle_generation > 0),
    device_installation_binding_generation INTEGER NOT NULL CHECK (
        device_installation_binding_generation > 0
    ),
    device_authority_generation INTEGER NOT NULL CHECK (device_authority_generation > 0),
    preview_id TEXT NOT NULL CHECK (length(trim(preview_id)) > 0),
    apply_intent_digest TEXT NOT NULL CHECK (
        length(apply_intent_digest) = 64 AND apply_intent_digest NOT GLOB '*[^0-9a-f]*'
    ),
    receipt_epoch INTEGER NOT NULL UNIQUE CHECK (receipt_epoch > 0),
    issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
    expires_at_epoch_millis INTEGER NOT NULL CHECK (
        expires_at_epoch_millis > issued_at_epoch_millis
        AND expires_at_epoch_millis <= issued_at_epoch_millis + 300000
    ),
    consumed_at_epoch_millis INTEGER,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('staged','consumed','expired')
    ),
    CHECK (
        (lifecycle_state = 'staged' AND consumed_at_epoch_millis IS NULL)
        OR (lifecycle_state = 'consumed'
            AND consumed_at_epoch_millis >= issued_at_epoch_millis)
        OR (lifecycle_state = 'expired' AND consumed_at_epoch_millis IS NULL)
    )
) STRICT"#;

pub(super) const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS account_identity_parent_storage_confirmation (
    receipt_id TEXT PRIMARY KEY NOT NULL CHECK (
        length(receipt_id) = 64 AND receipt_id NOT GLOB '*[^0-9a-f]*'
    ),
    nonce_id TEXT NOT NULL UNIQUE CHECK (
        length(nonce_id) = 64 AND nonce_id NOT GLOB '*[^0-9a-f]*'
    ),
    provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
    provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
    household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
    account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
    parent_device_id TEXT NOT NULL CHECK (length(trim(parent_device_id)) > 0),
    child_profile_id TEXT NOT NULL CHECK (length(trim(child_profile_id)) > 0),
    child_device_id TEXT NOT NULL CHECK (length(trim(child_device_id)) > 0),
    installation_id TEXT NOT NULL CHECK (length(trim(installation_id)) > 0),
    pairing_id TEXT NOT NULL CHECK (length(trim(pairing_id)) > 0),
    route_id TEXT NOT NULL CHECK (length(trim(route_id)) > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    session_generation INTEGER NOT NULL CHECK (session_generation > 0),
    device_trust_subject TEXT NOT NULL CHECK (length(trim(device_trust_subject)) > 0),
    device_lifecycle_generation INTEGER NOT NULL CHECK (device_lifecycle_generation > 0),
    device_installation_binding_generation INTEGER NOT NULL CHECK (
        device_installation_binding_generation > 0
    ),
    device_authority_generation INTEGER NOT NULL CHECK (device_authority_generation > 0),
    preview_id TEXT NOT NULL CHECK (length(trim(preview_id)) > 0),
    apply_intent_digest TEXT NOT NULL CHECK (
        length(apply_intent_digest) = 64 AND apply_intent_digest NOT GLOB '*[^0-9a-f]*'
    ),
    receipt_epoch INTEGER NOT NULL UNIQUE CHECK (receipt_epoch > 0),
    issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
    expires_at_epoch_millis INTEGER NOT NULL CHECK (
        expires_at_epoch_millis > issued_at_epoch_millis
        AND expires_at_epoch_millis <= issued_at_epoch_millis + 300000
    ),
    consumed_at_epoch_millis INTEGER,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('staged','consumed','expired')
    ),
    CHECK (
        (lifecycle_state = 'staged' AND consumed_at_epoch_millis IS NULL)
        OR (lifecycle_state = 'consumed'
            AND consumed_at_epoch_millis >= issued_at_epoch_millis)
        OR (lifecycle_state = 'expired' AND consumed_at_epoch_millis IS NULL)
    )
) STRICT;
CREATE INDEX IF NOT EXISTS account_identity_parent_storage_confirmation_state
    ON account_identity_parent_storage_confirmation(lifecycle_state, expires_at_epoch_millis);
CREATE UNIQUE INDEX IF NOT EXISTS account_identity_parent_storage_confirmation_intent_staged
    ON account_identity_parent_storage_confirmation(household_id, preview_id, apply_intent_digest)
    WHERE lifecycle_state = 'staged';
"#;

use rusqlite::{Connection, OptionalExtension};

use super::ParentStorageConfirmationStoreError;

pub(super) fn validate_table(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    validate_table_object(connection)?;
    validate_table_properties(connection)?;
    validate_table_columns(connection)?;
    validate_table_index(connection)
}

fn validate_table_object(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let object = connection
        .query_row(
            "SELECT type, sql FROM sqlite_master WHERE name = ?1",
            [super::TABLE],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)?;
    if object.0 != "table" || normalize_sql(&object.1) != normalize_sql(TABLE_SQL) {
        return Err(ParentStorageConfirmationStoreError::IntegrityRejected);
    }
    Ok(())
}

fn validate_table_properties(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let properties = connection
        .query_row(
            "SELECT wr, strict FROM pragma_table_list WHERE schema = 'main' AND name = ?1",
            [super::TABLE],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    if properties != (0, 1) {
        return Err(ParentStorageConfirmationStoreError::IntegrityRejected);
    }
    Ok(())
}

fn validate_table_columns(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let expected = [
        ("receipt_id", "TEXT", 1_i64, 1_i64),
        ("nonce_id", "TEXT", 1, 0),
        ("provider", "TEXT", 1, 0),
        ("provider_subject", "TEXT", 1, 0),
        ("household_id", "TEXT", 1, 0),
        ("account_id", "TEXT", 1, 0),
        ("parent_device_id", "TEXT", 1, 0),
        ("child_profile_id", "TEXT", 1, 0),
        ("child_device_id", "TEXT", 1, 0),
        ("installation_id", "TEXT", 1, 0),
        ("pairing_id", "TEXT", 1, 0),
        ("route_id", "TEXT", 1, 0),
        ("authority_generation", "INTEGER", 1, 0),
        ("session_generation", "INTEGER", 1, 0),
        ("device_trust_subject", "TEXT", 1, 0),
        ("device_lifecycle_generation", "INTEGER", 1, 0),
        ("device_installation_binding_generation", "INTEGER", 1, 0),
        ("device_authority_generation", "INTEGER", 1, 0),
        ("preview_id", "TEXT", 1, 0),
        ("apply_intent_digest", "TEXT", 1, 0),
        ("receipt_epoch", "INTEGER", 1, 0),
        ("issued_at_epoch_millis", "INTEGER", 1, 0),
        ("expires_at_epoch_millis", "INTEGER", 1, 0),
        ("consumed_at_epoch_millis", "INTEGER", 0, 0),
        ("lifecycle_state", "TEXT", 1, 0),
    ];
    let mut statement = connection
        .prepare("PRAGMA table_info('account_identity_parent_storage_confirmation')")
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let actual = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(|(actual, expected)| {
            actual.0 != expected.0
                || actual.1.to_ascii_uppercase() != expected.1
                || actual.2 != expected.2
                || actual.3 != expected.3
        })
    {
        return Err(ParentStorageConfirmationStoreError::IntegrityRejected);
    }
    Ok(())
}

fn validate_table_index(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    validate_index_sql(
        connection,
        "account_identity_parent_storage_confirmation_state",
        "CREATE INDEX account_identity_parent_storage_confirmation_state
         ON account_identity_parent_storage_confirmation(lifecycle_state, expires_at_epoch_millis)",
    )?;
    validate_index_sql(
        connection,
        "account_identity_parent_storage_confirmation_intent_staged",
        "CREATE UNIQUE INDEX account_identity_parent_storage_confirmation_intent_staged
         ON account_identity_parent_storage_confirmation(household_id, preview_id, apply_intent_digest)
         WHERE lifecycle_state = 'staged'",
    )
}

fn validate_index_sql(
    connection: &Connection,
    index_name: &str,
    expected_sql: &str,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let index_sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master
             WHERE type = 'index' AND name = ?1",
            [index_name],
            |row| row.get::<_, String>(0),
        )
        .map_err(|_| ParentStorageConfirmationStoreError::IntegrityRejected)?;
    (normalize_sql(&index_sql) == normalize_sql(expected_sql))
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
}

pub(super) fn validate_rows(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT receipt_id, nonce_id, provider, provider_subject, household_id, account_id,
                    parent_device_id, child_profile_id, child_device_id, installation_id,
                    pairing_id, route_id, authority_generation, session_generation,
                    device_trust_subject, device_lifecycle_generation,
                    device_installation_binding_generation, device_authority_generation,
                    preview_id, apply_intent_digest, receipt_epoch, issued_at_epoch_millis,
                    expires_at_epoch_millis, consumed_at_epoch_millis, lifecycle_state
             FROM account_identity_parent_storage_confirmation ORDER BY receipt_id",
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let rows = statement
        .query_map([], super::StoredRow::from_row)
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    for row in rows {
        row.map_err(|_| ParentStorageConfirmationStoreError::IntegrityRejected)?
            .validate()?;
    }
    Ok(())
}

pub(super) fn validate_provider(value: &str) -> Result<(), ParentStorageConfirmationStoreError> {
    matches!(value, "authjs" | "firebase")
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
}

pub(super) fn validate_identity(value: &str) -> Result<(), ParentStorageConfirmationStoreError> {
    (value.len() <= 256
        && !value.is_empty()
        && value.trim() == value
        && value.is_ascii()
        && value.bytes().all(|byte| (0x21..=0x7e).contains(&byte)))
    .then_some(())
    .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
}

pub(super) fn validate_hex_id(value: &str) -> Result<(), ParentStorageConfirmationStoreError> {
    validate_lower_hex(value, 64)
}

pub(super) fn validate_lower_hex(
    value: &str,
    length: usize,
) -> Result<(), ParentStorageConfirmationStoreError> {
    (value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    .then_some(())
    .ok_or(ParentStorageConfirmationStoreError::IntegrityRejected)
}

fn normalize_sql(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase()
}

pub(super) fn validate_related_objects(
    connection: &Connection,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name, COALESCE(sql, '')
             FROM sqlite_master
             WHERE type IN ('trigger', 'view')
               AND (tbl_name = ?1 OR lower(COALESCE(sql, '')) LIKE ?2)",
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let related_sql = format!("%{}%", super::TABLE.to_ascii_lowercase());
    let related = statement
        .query_map([super::TABLE, related_sql.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    if !related.is_empty() {
        return Err(ParentStorageConfirmationStoreError::IntegrityRejected);
    }

    let mut index_statement = connection
        .prepare("PRAGMA index_list('account_identity_parent_storage_confirmation')")
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let mut indexes = index_statement
        .query([])
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    while let Some(row) = indexes
        .next()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
    {
        let name = row
            .get::<_, String>(1)
            .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
        let origin = row
            .get::<_, String>(3)
            .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
        let partial = row
            .get::<_, i64>(4)
            .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
        let auto_index = name.starts_with("sqlite_autoindex_")
            && name.starts_with("sqlite_autoindex_account_identity_parent_storage_confirmation_");
        let owned_state_index = name == "account_identity_parent_storage_confirmation_state"
            && origin == "c"
            && partial == 0;
        let owned_intent_index = name
            == "account_identity_parent_storage_confirmation_intent_staged"
            && origin == "c"
            && partial == 1;
        if !auto_index && !owned_state_index && !owned_intent_index {
            return Err(ParentStorageConfirmationStoreError::IntegrityRejected);
        }
    }
    Ok(())
}
