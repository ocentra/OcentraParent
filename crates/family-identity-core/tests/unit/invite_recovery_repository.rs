use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_family_identity_core::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityService,
    AccountIdentityMutationAuthorityServiceError, SqliteAccountIdentityAuthorityRepository,
};
use ocentra_family_identity_core::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use rusqlite::Connection;

fn temporary_database_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ocentra-account-wp04-invite-recovery-{}-{nonce}.sqlite",
        std::process::id()
    ))
}

fn remove_database(path: &Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension("sqlite-journal"));
}

#[test]
fn opening_the_account_repository_creates_the_durable_invite_recovery_tables() {
    let path = temporary_database_path();
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("account repository should initialize its owned schema");
    }

    let connection = Connection::open(&path).expect("reopen initialized account repository");
    for table in [
        "account_identity_runtime_clock",
        "account_identity_setup_invite",
        "account_identity_pending_invite_membership",
        "account_identity_recovery",
        "account_identity_recovery_rate_limit",
        "account_identity_invite_rate_limit",
        "account_identity_recovery_custody_handoff",
    ] {
        let present: i64 = connection
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get(0),
            )
            .expect("query repository table catalog");
        assert_eq!(present, 1, "missing durable table {table}");
    }

    drop(connection);
    remove_database(&path);
}

#[test]
fn reopening_the_repository_preserves_empty_pending_invite_and_recovery_state() {
    let path = temporary_database_path();
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("first repository open should succeed");
    }
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("restart repository open should reuse the durable schema");
    }

    let connection = Connection::open(&path).expect("reopen durable repository for inspection");
    for table in [
        "account_identity_setup_invite",
        "account_identity_pending_invite_membership",
        "account_identity_recovery",
        "account_identity_recovery_custody_handoff",
    ] {
        let rows: i64 = connection
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .expect("query durable lifecycle rows");
        assert_eq!(
            rows, 0,
            "fresh repository unexpectedly contains {table} rows"
        );
    }

    drop(connection);
    remove_database(&path);
}

#[test]
fn installer_owned_repository_mount_remains_fail_closed_without_an_owner_handle() {
    assert!(matches!(
        AccountIdentityAuthorityService::mount_account_owned(),
        Err(AccountIdentityAuthorityRepositoryError::Unavailable)
    ));
}

#[test]
fn mutation_consumption_fails_closed_before_untrusted_wire_input_is_interpreted() {
    let path = temporary_database_path();
    let mut service =
        AccountIdentityAuthorityService::open(&path).expect("open account authority service");

    let result = service.consume_and_apply_mutation_authority(&[]);

    assert!(matches!(
        result,
        Err(AccountIdentityMutationAuthorityServiceError::Mutation(
            AccountIdentityMutationAuthorityError::VerificationKeyUnavailable
        ))
    ));
    drop(service);
    remove_database(&path);
}
