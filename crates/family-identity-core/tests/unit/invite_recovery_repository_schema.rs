use std::collections::BTreeSet;

use ocentra_family_identity_core::account_identity_authority_repository::
    SqliteAccountIdentityAuthorityRepository;
use ocentra_family_identity_core::account_identity_authority_repository::invite_recovery_repository::
    INVITE_RECOVERY_SCHEMA_SQL;
use rusqlite::Connection;

#[test]
fn invite_recovery_schema_contains_only_the_expected_owned_objects() {
    let connection = Connection::open_in_memory().expect("open in-memory SQLite database");
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .expect("enable foreign-key enforcement");
    connection
        .execute_batch(INVITE_RECOVERY_SCHEMA_SQL)
        .expect("create invite/recovery schema");

    let mut statement = connection
        .prepare(
            "SELECT type, name FROM sqlite_master
             WHERE name LIKE 'account_identity_%' AND type IN ('table', 'index')
             ORDER BY type, name",
        )
        .expect("prepare owned object catalog query");
    let actual = statement
        .query_map([], |row| {
            Ok(format!(
                "{}:{}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?
            ))
        })
        .expect("query owned object catalog")
        .collect::<Result<BTreeSet<_>, _>>()
        .expect("collect owned object catalog");
    let expected = [
        "table:account_identity_invite_rate_limit",
        "table:account_identity_mutation_effect",
        "table:account_identity_pending_invite_membership",
        "table:account_identity_recovery",
        "table:account_identity_recovery_custody_handoff",
        "table:account_identity_recovery_rate_limit",
        "table:account_identity_runtime_clock",
        "table:account_identity_setup_invite",
        "index:account_identity_mutation_effect_retention",
        "index:account_identity_recovery_handoff_ready",
        "index:account_identity_recovery_household",
        "index:account_identity_setup_invite_household",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn repository_open_validates_and_reuses_the_same_schema_contract() {
    let path = std::env::temp_dir().join(format!(
        "ocentra-account-wp04-schema-{}-{}.sqlite",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is after the Unix epoch")
            .as_nanos()
    ));
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("repository should validate its fresh schema");
    }
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("repository should validate its schema after restart");
    }

    let connection = Connection::open(&path).expect("reopen repository schema");
    for table in [
        "account_identity_runtime_clock",
        "account_identity_mutation_effect",
        "account_identity_setup_invite",
        "account_identity_pending_invite_membership",
        "account_identity_recovery",
        "account_identity_recovery_rate_limit",
        "account_identity_invite_rate_limit",
        "account_identity_recovery_custody_handoff",
    ] {
        let definition: String = connection
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get(0),
            )
            .expect("read strict table definition");
        assert!(
            definition
                .trim_end_matches(';')
                .trim_end()
                .ends_with("STRICT"),
            "table {table} is not STRICT"
        );
    }

    drop(connection);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn schema_rejects_invalid_clock_rows_and_orphaned_data_custody_handoffs() {
    let connection = Connection::open_in_memory().expect("open in-memory SQLite database");
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .expect("enable foreign-key enforcement");
    connection
        .execute_batch(INVITE_RECOVERY_SCHEMA_SQL)
        .expect("create invite/recovery schema");

    assert!(connection
        .execute(
            "INSERT INTO account_identity_runtime_clock (clock_id, last_epoch_millis)
             VALUES (1, 0)",
            [],
        )
        .is_err());

    assert!(connection
        .execute(
            "INSERT INTO account_identity_recovery_custody_handoff (
                 handoff_id, correlation_id, recovery_id, household_id, account_id,
                 member_id, device_id, kind, requested_at_epoch_millis, state,
                 active_attempt_id, lease_expires_at_epoch_millis, attempt_count,
                 owner_transition_id, owner_receipt_digest
             ) VALUES (
                 'handoff-1', 'correlation-1', 'missing-recovery', 'household-1',
                 'account-1', 'member-1', 'device-1', 'forgot-login', 1, 'pending',
                 NULL, NULL, 0, NULL, NULL
             )",
            [],
        )
        .is_err());
}

#[test]
fn schema_stores_invite_digests_without_a_bearer_token_column() {
    let connection = Connection::open_in_memory().expect("open in-memory SQLite database");
    connection
        .execute_batch(INVITE_RECOVERY_SCHEMA_SQL)
        .expect("create invite/recovery schema");
    let mut statement = connection
        .prepare("PRAGMA table_info('account_identity_setup_invite')")
        .expect("inspect invite columns");
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .expect("query invite columns")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect invite columns");
    assert!(columns.iter().any(|column| column == "token_digest"));
    assert!(!columns.iter().any(|column| column == "token"));
}
