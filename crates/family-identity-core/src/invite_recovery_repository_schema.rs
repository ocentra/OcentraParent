use rusqlite::Connection;

#[path = "invite_recovery_repository_schema_columns.rs"]
mod columns;
#[path = "invite_recovery_repository_schema_handoff_rows.rs"]
mod handoff_rows;
#[path = "invite_recovery_repository_schema_indexes.rs"]
mod indexes;
#[path = "invite_recovery_repository_schema_invite_rows.rs"]
mod invite_rows;
#[path = "invite_recovery_repository_schema_membership_rows.rs"]
mod membership_rows;
#[path = "account_identity_mutation_effect_rows.rs"]
mod mutation_effect_rows;
#[path = "invite_recovery_repository_schema_objects.rs"]
mod objects;
#[path = "invite_recovery_repository_schema_rate_rows.rs"]
mod rate_rows;
#[path = "invite_recovery_repository_schema_recovery_rows.rs"]
mod recovery_rows;
#[path = "invite_recovery_repository_schema_recovery_state.rs"]
mod schema_recovery_state;
#[path = "invite_recovery_repository_schema_recovery_support.rs"]
mod schema_recovery_support;

pub(crate) fn validate(connection: &Connection) -> Result<(), ()> {
    require_pragma_ok(connection, "PRAGMA integrity_check")?;
    require_no_foreign_key_violations(connection)?;
    objects::validate_objects(connection)?;
    columns::validate_tables(connection)?;
    indexes::validate_foreign_keys(connection)?;
    indexes::validate_indexes(connection)?;
    validate_rows(connection)
}

fn require_pragma_ok(connection: &Connection, sql: &str) -> Result<(), ()> {
    let value = connection
        .query_row(sql, [], |row| row.get::<_, String>(0))
        .map_err(|_| ())?;
    (value == "ok").then_some(()).ok_or(())
}

fn require_no_foreign_key_violations(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    rows.next()
        .map_err(|_| ())?
        .is_none()
        .then_some(())
        .ok_or(())
}

fn validate_rows(connection: &Connection) -> Result<(), ()> {
    validate_clock_rows(connection)?;
    mutation_effect_rows::validate(connection)?;
    invite_rows::validate(connection)?;
    membership_rows::validate(connection)?;
    recovery_rows::validate(connection)?;
    rate_rows::validate(connection, "account_identity_recovery_rate_limit")?;
    rate_rows::validate(connection, "account_identity_invite_rate_limit")?;
    handoff_rows::validate(connection)
}

fn validate_clock_rows(connection: &Connection) -> Result<(), ()> {
    let count = connection
        .query_row(
            "SELECT count(*) FROM account_identity_runtime_clock",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    if count == 0 {
        return Ok(());
    }
    if count != 1 {
        return Err(());
    }
    let value = connection
        .query_row(
            "SELECT last_epoch_millis FROM account_identity_runtime_clock WHERE clock_id = 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (value > 0).then_some(()).ok_or(())
}

fn hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
