use rusqlite::Connection;

use super::super::INVITE_RECOVERY_SCHEMA_SQL;

pub(super) const OWNED_TABLES: [&str; 7] = [
    "account_identity_runtime_clock",
    "account_identity_setup_invite",
    "account_identity_pending_invite_membership",
    "account_identity_recovery",
    "account_identity_recovery_rate_limit",
    "account_identity_invite_rate_limit",
    "account_identity_recovery_custody_handoff",
];

pub(super) fn validate_objects(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, sql FROM sqlite_master
             WHERE name LIKE 'account_identity_%' OR type IN ('trigger', 'view')
             ORDER BY type, name",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let object_type = row.get::<_, String>(0).map_err(|_| ())?;
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        let sql = row.get::<_, Option<String>>(2).map_err(|_| ())?;
        if is_owned_trigger_or_view(&object_type, &name, sql.as_deref())
            || (object_type == "table" && !table_is_allowed(&name))
            || (object_type == "index" && !index_is_allowed(&name))
            || ((object_type == "table" || object_type == "index")
                && OWNED_TABLES.contains(&name.as_str())
                && !canonical_definition_matches(&object_type, &name, sql.as_deref().ok_or(())?))
        {
            return Err(());
        }
    }
    Ok(())
}

fn table_is_allowed(name: &str) -> bool {
    OWNED_TABLES.contains(&name) || owner_table(name)
}

fn index_is_allowed(name: &str) -> bool {
    name.starts_with("sqlite_autoindex_") || expected_index(name) || owner_index(name)
}

fn is_owned_trigger_or_view(object_type: &str, name: &str, sql: Option<&str>) -> bool {
    if !matches!(object_type, "trigger" | "view") {
        return false;
    }
    if name.starts_with("account_identity_") {
        return true;
    }
    let object_text = sql.unwrap_or_default().to_ascii_lowercase();
    OWNED_TABLES
        .iter()
        .chain(OWNER_TABLES.iter())
        .any(|table| object_text.contains(table))
}

const OWNER_TABLES: [&str; 5] = [
    "account_identity_current_authority",
    "account_identity_session",
    "account_identity_session_revoke_epoch",
    "account_identity_session_refresh_replay",
    "account_identity_session_audit_outbox",
];

fn canonical_definition_matches(object_type: &str, name: &str, actual: &str) -> bool {
    let Some(expected) = canonical_definition(object_type, name) else {
        return false;
    };
    let actual = compact_sql(actual);
    let actual = actual.trim_end_matches(';');
    actual == expected || without_if_not_exists(actual) == without_if_not_exists(&expected)
}

fn canonical_definition(object_type: &str, name: &str) -> Option<String> {
    let marker = match object_type {
        "table" => format!("CREATETABLEIFNOTEXISTS{}", compact_sql(name)),
        "index" => format!("CREATEINDEXIFNOTEXISTS{}", compact_sql(name)),
        _ => return None,
    };
    INVITE_RECOVERY_SCHEMA_SQL
        .split(';')
        .map(compact_sql)
        .find(|statement| statement.starts_with(&marker))
}

fn without_if_not_exists(sql: &str) -> String {
    sql.replace("IFNOTEXISTS", "")
}

fn compact_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

fn owner_table(name: &str) -> bool {
    OWNER_TABLES.contains(&name)
}

fn expected_index(name: &str) -> bool {
    matches!(
        name,
        "account_identity_setup_invite_household"
            | "account_identity_recovery_household"
            | "account_identity_recovery_handoff_ready"
    )
}

fn owner_index(name: &str) -> bool {
    name == "account_identity_session_account"
}
