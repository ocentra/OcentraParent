use rusqlite::Connection;

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
             WHERE name LIKE 'account_identity_%'
             ORDER BY type, name",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let object_type = row.get::<_, String>(0).map_err(|_| ())?;
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        let sql = row.get::<_, Option<String>>(2).map_err(|_| ())?;
        if matches!(object_type.as_str(), "trigger" | "view")
            || (object_type == "table" && !table_is_allowed(&name))
            || (object_type == "index" && !index_is_allowed(&name))
            || (OWNED_TABLES.contains(&name.as_str()) && !is_strict(sql.as_deref()))
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

fn is_strict(sql: Option<&str>) -> bool {
    sql.is_some_and(|value| value.to_ascii_uppercase().contains("STRICT"))
}

fn owner_table(name: &str) -> bool {
    matches!(
        name,
        "account_identity_current_authority"
            | "account_identity_session"
            | "account_identity_session_revoke_epoch"
            | "account_identity_session_refresh_replay"
            | "account_identity_session_audit_outbox"
    )
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
