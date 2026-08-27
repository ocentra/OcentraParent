#![forbid(unsafe_code)]

use rusqlite::Connection;

use super::BRIDGE_SCHEMA_VERSION;

pub(super) const TABLES: &[&str] = &[
    "account_identity_parent_local_bridge_schema",
    "account_identity_parent_local_bridge_revoke_epoch",
    "account_identity_parent_local_bridge_session",
    "account_identity_parent_local_bridge_audit_outbox",
];
pub(super) const INDEXES: &[&str] = &[
    "account_identity_parent_local_bridge_account",
    "account_identity_parent_local_bridge_audit_delivery",
    "account_identity_parent_local_bridge_audit_retention",
];

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    require_pragma_ok(connection, "PRAGMA integrity_check")?;
    require_no_foreign_key_violations(connection)?;
    super::validation_objects::validate(connection, false)?;
    validate_v3_columns(connection)?;
    validate_indexes(connection)?;
    validate_foreign_keys(connection)?;
    validate_rows(connection, Some(BRIDGE_SCHEMA_VERSION), true)
}

pub(super) fn validate_v2(connection: &Connection, require_version: bool) -> Result<(), ()> {
    require_pragma_ok(connection, "PRAGMA integrity_check")?;
    require_no_foreign_key_violations(connection)?;
    super::validation_objects::validate(connection, true)?;
    validate_legacy_v2_columns(connection)?;
    validate_indexes(connection)?;
    validate_foreign_keys(connection)?;
    validate_rows(connection, require_version.then_some(2), false)
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

fn validate_v3_columns(connection: &Connection) -> Result<(), ()> {
    validate_columns(
        connection,
        TABLES[0],
        &[
            ("schema_id", "INTEGER", 1, 1),
            ("schema_version", "INTEGER", 1, 0),
        ],
    )?;
    validate_columns(
        connection,
        TABLES[1],
        &[("account_id", "TEXT", 1, 1), ("epoch", "INTEGER", 1, 0)],
    )?;
    validate_columns(
        connection,
        TABLES[2],
        &[
            ("capability_digest", "TEXT", 1, 1),
            ("digest_algorithm", "TEXT", 1, 0),
            ("capability_digest_domain", "TEXT", 1, 0),
            ("audience", "TEXT", 1, 0),
            ("connection_nonce_digest", "TEXT", 1, 0),
            ("account_id", "TEXT", 1, 0),
            ("provider", "TEXT", 1, 0),
            ("provider_subject", "TEXT", 1, 0),
            ("household_id", "TEXT", 1, 0),
            ("member_id", "TEXT", 1, 0),
            ("device_id", "TEXT", 1, 0),
            ("authority_session_id", "TEXT", 1, 0),
            ("authority_session_generation", "INTEGER", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("authority_expires_at_epoch_millis", "INTEGER", 1, 0),
            ("issued_at_epoch_millis", "INTEGER", 1, 0),
            ("expires_at_epoch_millis", "INTEGER", 1, 0),
            ("bridge_revoke_epoch", "INTEGER", 1, 0),
            ("state", "TEXT", 1, 0),
            ("last_transition_at_epoch_millis", "INTEGER", 1, 0),
        ],
    )?;
    validate_columns(
        connection,
        TABLES[3],
        &[
            ("sequence", "INTEGER", 0, 1),
            ("event_id", "TEXT", 1, 0),
            ("account_id", "TEXT", 1, 0),
            ("provider", "TEXT", 1, 0),
            ("provider_subject_digest", "TEXT", 1, 0),
            ("household_id", "TEXT", 1, 0),
            ("member_id", "TEXT", 1, 0),
            ("device_id", "TEXT", 1, 0),
            ("authority_session_id", "TEXT", 1, 0),
            ("authority_session_generation", "INTEGER", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("audience", "TEXT", 1, 0),
            ("bridge_revoke_epoch", "INTEGER", 1, 0),
            ("action", "TEXT", 1, 0),
            ("occurred_at_epoch_millis", "INTEGER", 1, 0),
            ("retain_until_epoch_millis", "INTEGER", 1, 0),
            ("delivery_state", "TEXT", 1, 0),
            ("delivery_attempt_id", "TEXT", 0, 0),
            ("delivery_attempt_count", "INTEGER", 1, 0),
            ("delivery_claimed_at_epoch_millis", "INTEGER", 0, 0),
            ("delivery_lease_expires_at_epoch_millis", "INTEGER", 0, 0),
            ("next_delivery_at_epoch_millis", "INTEGER", 1, 0),
            ("delivered_at_epoch_millis", "INTEGER", 0, 0),
        ],
    )
}

fn validate_legacy_v2_columns(connection: &Connection) -> Result<(), ()> {
    validate_exact_columns(connection, TABLES[0], &["schema_id", "schema_version"])?;
    validate_exact_columns(connection, TABLES[1], &["account_id", "epoch"])?;
    validate_exact_columns(
        connection,
        TABLES[2],
        &[
            "capability_digest",
            "digest_algorithm",
            "capability_digest_domain",
            "audience",
            "connection_nonce_digest",
            "account_id",
            "provider",
            "provider_subject",
            "household_id",
            "member_id",
            "device_id",
            "authority_session_id",
            "authority_session_generation",
            "authority_generation",
            "authority_expires_at_epoch_millis",
            "issued_at_epoch_millis",
            "expires_at_epoch_millis",
            "bridge_revoke_epoch",
            "state",
            "last_transition_at_epoch_millis",
        ],
    )?;
    validate_exact_columns(
        connection,
        TABLES[3],
        &[
            "sequence",
            "event_id",
            "account_id",
            "provider",
            "provider_subject_digest",
            "household_id",
            "member_id",
            "device_id",
            "authority_session_id",
            "audience",
            "bridge_revoke_epoch",
            "action",
            "occurred_at_epoch_millis",
            "retain_until_epoch_millis",
            "delivery_state",
            "delivery_attempt_id",
            "delivery_attempt_count",
            "delivery_claimed_at_epoch_millis",
            "delivery_lease_expires_at_epoch_millis",
            "next_delivery_at_epoch_millis",
            "delivered_at_epoch_millis",
        ],
    )
}

fn validate_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info('{table}')"))
        .map_err(|_| ())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?.to_ascii_uppercase(),
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected = expected
        .iter()
        .map(|value| (value.0.to_owned(), value.1.to_owned(), value.2, value.3))
        .collect::<Vec<_>>();
    (rows == expected).then_some(()).ok_or(())
}

pub(super) fn validate_exact_columns(
    connection: &Connection,
    table: &str,
    expected: &[&str],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info('{table}')"))
        .map_err(|_| ())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    (rows == expected).then_some(()).ok_or(())
}

fn validate_indexes(connection: &Connection) -> Result<(), ()> {
    validate_index(connection, TABLES[2], INDEXES[0], &["account_id"])?;
    validate_index(
        connection,
        TABLES[3],
        INDEXES[1],
        &[
            "delivery_state",
            "next_delivery_at_epoch_millis",
            "sequence",
        ],
    )?;
    validate_index(
        connection,
        TABLES[3],
        INDEXES[2],
        &["retain_until_epoch_millis"],
    )
}

pub(super) fn validate_v1_indexes(connection: &Connection) -> Result<(), ()> {
    validate_index(connection, TABLES[2], INDEXES[0], &["account_id"])?;
    validate_index(
        connection,
        TABLES[3],
        INDEXES[1],
        &["delivery_state", "sequence"],
    )?;
    validate_index(
        connection,
        TABLES[3],
        INDEXES[2],
        &["retain_until_epoch_millis"],
    )
}

fn validate_index(
    connection: &Connection,
    table: &str,
    expected_name: &str,
    expected_columns: &[&str],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list('{table}')"))
        .map_err(|_| ())?;
    let names = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    if !names.iter().any(|name| name == expected_name)
        || names.iter().any(|name| {
            !name.starts_with("sqlite_autoindex_")
                && !INDEXES.iter().any(|expected| name == expected)
        })
    {
        return Err(());
    }
    let mut columns = connection
        .prepare(&format!("PRAGMA index_info('{expected_name}')"))
        .map_err(|_| ())?;
    let actual = columns
        .query_map([], |row| row.get::<_, String>(2))
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    (actual == expected_columns).then_some(()).ok_or(())
}

fn validate_foreign_keys(connection: &Connection) -> Result<(), ()> {
    validate_fk_none(connection, TABLES[0])?;
    validate_fk_none(connection, TABLES[1])?;
    validate_fk(connection, TABLES[2])?;
    validate_fk(connection, TABLES[3])
}

fn validate_fk_none(connection: &Connection, table: &str) -> Result<(), ()> {
    let count = connection
        .query_row(
            &format!("SELECT count(*) FROM pragma_foreign_key_list('{table}')"),
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (count == 0).then_some(()).ok_or(())
}

fn validate_fk(connection: &Connection, table: &str) -> Result<(), ()> {
    let row = connection
        .query_row(
            &format!("SELECT count(*), min(\"table\"), min(\"from\"), min(\"to\"), min(on_update), min(on_delete) FROM pragma_foreign_key_list('{table}')"),
            [],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?)),
        )
        .map_err(|_| ())?;
    (row == (
        1,
        TABLES[1].to_owned(),
        "account_id".to_owned(),
        "account_id".to_owned(),
        "RESTRICT".to_owned(),
        "RESTRICT".to_owned(),
    ))
        .then_some(())
        .ok_or(())
}

fn validate_rows(
    connection: &Connection,
    expected_version: Option<i64>,
    require_generations: bool,
) -> Result<(), ()> {
    let (version_rows, matching_version_rows) = connection
        .query_row(
            "SELECT count(*),
                    coalesce(sum(CASE WHEN schema_id = 1 AND schema_version = ?1
                                      THEN 1 ELSE 0 END), 0)
               FROM account_identity_parent_local_bridge_schema",
            [expected_version.unwrap_or(BRIDGE_SCHEMA_VERSION)],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .map_err(|_| ())?;
    match expected_version {
        Some(_) if version_rows == 1 && matching_version_rows == 1 => {}
        None if version_rows == 0 => {}
        _ => return Err(()),
    }
    require_zero(
        connection,
        "SELECT count(*) FROM account_identity_parent_local_bridge_revoke_epoch
          WHERE length(trim(account_id)) = 0 OR epoch <= 0",
    )?;
    require_zero(
        connection,
        "SELECT count(*) FROM account_identity_parent_local_bridge_session s
          LEFT JOIN account_identity_parent_local_bridge_revoke_epoch e USING(account_id)
         WHERE e.account_id IS NULL OR s.bridge_revoke_epoch <= 0
            OR s.issued_at_epoch_millis <= 0
            OR s.expires_at_epoch_millis <= s.issued_at_epoch_millis
            OR s.expires_at_epoch_millis > s.authority_expires_at_epoch_millis
            OR s.last_transition_at_epoch_millis < s.issued_at_epoch_millis",
    )?;
    require_zero(
        connection,
        "SELECT count(*) FROM account_identity_parent_local_bridge_audit_outbox a
          LEFT JOIN account_identity_parent_local_bridge_revoke_epoch e USING(account_id)
         WHERE e.account_id IS NULL OR a.occurred_at_epoch_millis <= 0
            OR a.retain_until_epoch_millis != a.occurred_at_epoch_millis + 2592000000
            OR a.next_delivery_at_epoch_millis < a.occurred_at_epoch_millis
            OR a.delivery_attempt_count < 0
            OR length(a.provider_subject_digest) != 64
            OR a.provider_subject_digest GLOB '*[^0-9a-f]*'",
    )?;
    if require_generations {
        require_zero(
            connection,
            "SELECT count(*) FROM account_identity_parent_local_bridge_audit_outbox
              WHERE authority_session_generation <= 0 OR authority_generation <= 0",
        )?;
    }
    Ok(())
}

fn require_zero(connection: &Connection, sql: &str) -> Result<(), ()> {
    let count = connection
        .query_row(sql, [], |row| row.get::<_, i64>(0))
        .map_err(|_| ())?;
    (count == 0).then_some(()).ok_or(())
}
