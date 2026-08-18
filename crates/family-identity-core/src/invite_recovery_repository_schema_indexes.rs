use rusqlite::Connection;

pub(super) fn validate_indexes(connection: &Connection) -> Result<(), ()> {
    validate_index(
        connection,
        "account_identity_setup_invite",
        "account_identity_setup_invite_household",
        false,
        &["household_id", "state"],
    )?;
    validate_index(
        connection,
        "account_identity_recovery",
        "account_identity_recovery_household",
        false,
        &["household_id", "state"],
    )?;
    validate_index(
        connection,
        "account_identity_recovery_custody_handoff",
        "account_identity_recovery_handoff_ready",
        false,
        &["household_id", "state", "lease_expires_at_epoch_millis"],
    )
}

fn validate_index(
    connection: &Connection,
    table: &str,
    index: &str,
    unique: bool,
    columns: &[&str],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list('{table}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut found = false;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        if name == index {
            found = true;
            validate_index_shape(
                connection,
                &name,
                row.get::<_, i64>(2).map_err(|_| ())?,
                unique,
                columns,
            )?;
        } else if !name.starts_with("sqlite_autoindex_") {
            return Err(());
        }
    }
    found.then_some(()).ok_or(())
}

fn validate_index_shape(
    connection: &Connection,
    index: &str,
    actual_unique: i64,
    expected_unique: bool,
    columns: &[&str],
) -> Result<(), ()> {
    if actual_unique != i64::from(expected_unique) {
        return Err(());
    }
    let mut statement = connection
        .prepare(&format!("PRAGMA index_info('{index}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut actual = Vec::new();
    while let Some(row) = rows.next().map_err(|_| ())? {
        actual.push(row.get::<_, String>(2).map_err(|_| ())?);
    }
    (actual == columns).then_some(()).ok_or(())
}

pub(super) fn validate_foreign_keys(connection: &Connection) -> Result<(), ()> {
    validate_foreign_key(connection, "account_identity_runtime_clock", None)?;
    validate_foreign_key(connection, "account_identity_setup_invite", None)?;
    validate_foreign_key(
        connection,
        "account_identity_pending_invite_membership",
        Some(("account_identity_setup_invite", "invite_id", "invite_id")),
    )?;
    validate_foreign_key(
        connection,
        "account_identity_recovery_custody_handoff",
        Some(("account_identity_recovery", "recovery_id", "recovery_id")),
    )?;
    validate_foreign_key(connection, "account_identity_recovery", None)?;
    validate_foreign_key(connection, "account_identity_recovery_rate_limit", None)?;
    validate_foreign_key(connection, "account_identity_invite_rate_limit", None)
}

fn validate_foreign_key(
    connection: &Connection,
    table: &str,
    expected: Option<(&str, &str, &str)>,
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA foreign_key_list('{table}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut actual = Vec::new();
    while let Some(row) = rows.next().map_err(|_| ())? {
        actual.push((
            row.get::<_, String>(2).map_err(|_| ())?,
            row.get::<_, String>(3).map_err(|_| ())?,
            row.get::<_, String>(4).map_err(|_| ())?,
        ));
    }
    match expected {
        Some(expected) => (actual
            == vec![(
                expected.0.to_owned(),
                expected.1.to_owned(),
                expected.2.to_owned(),
            )])
        .then_some(())
        .ok_or(()),
        None => actual.is_empty().then_some(()).ok_or(()),
    }
}
