use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_family_identity_core::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityService,
    AccountIdentityMutationAuthorityServiceError, SqliteAccountIdentityAuthorityRepository,
};
use ocentra_family_identity_core::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use rusqlite::Connection;

type TestResult<T> = Result<T, String>;

fn with_context<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> TestResult<T> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

fn temporary_database_path() -> TestResult<PathBuf> {
    let nonce = with_context(
        SystemTime::now().duration_since(UNIX_EPOCH),
        "system clock is after the Unix epoch",
    )?
    .as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "ocentra-account-wp04-invite-recovery-{}-{nonce}.sqlite",
        std::process::id()
    )))
}

fn remove_database(path: &Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension("sqlite-journal"));
}

#[test]
fn opening_the_account_repository_creates_the_durable_invite_recovery_tables() -> TestResult<()> {
    let path = temporary_database_path()?;
    {
        with_context(
            SqliteAccountIdentityAuthorityRepository::open(&path),
            "account repository should initialize its owned schema",
        )?;
    }

    let connection = with_context(
        Connection::open(&path),
        "reopen initialized account repository",
    )?;
    for table in [
        "account_identity_runtime_clock",
        "account_identity_setup_invite",
        "account_identity_pending_invite_membership",
        "account_identity_recovery",
        "account_identity_recovery_rate_limit",
        "account_identity_invite_rate_limit",
        "account_identity_recovery_custody_handoff",
    ] {
        let present: i64 = with_context(
            connection.query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get(0),
            ),
            "query repository table catalog",
        )?;
        assert_eq!(present, 1, "missing durable table {table}");
    }

    drop(connection);
    remove_database(&path);
    Ok(())
}

#[test]
fn reopening_the_repository_preserves_empty_pending_invite_and_recovery_state() -> TestResult<()> {
    let path = temporary_database_path()?;
    {
        with_context(
            SqliteAccountIdentityAuthorityRepository::open(&path),
            "first repository open should succeed",
        )?;
    }
    {
        with_context(
            SqliteAccountIdentityAuthorityRepository::open(&path),
            "restart repository open should reuse the durable schema",
        )?;
    }

    let connection = with_context(
        Connection::open(&path),
        "reopen durable repository for inspection",
    )?;
    for table in [
        "account_identity_setup_invite",
        "account_identity_pending_invite_membership",
        "account_identity_recovery",
        "account_identity_recovery_custody_handoff",
    ] {
        let rows: i64 = with_context(
            connection.query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                row.get(0)
            }),
            "query durable lifecycle rows",
        )?;
        assert_eq!(
            rows, 0,
            "fresh repository unexpectedly contains {table} rows"
        );
    }

    drop(connection);
    remove_database(&path);
    Ok(())
}

#[test]
fn installer_owned_repository_mount_remains_fail_closed_without_an_owner_handle() {
    assert!(matches!(
        AccountIdentityAuthorityService::mount_account_owned(),
        Err(AccountIdentityAuthorityRepositoryError::Unavailable)
    ));
}

#[test]
fn mutation_consumption_fails_closed_before_untrusted_wire_input_is_interpreted() -> TestResult<()>
{
    let path = temporary_database_path()?;
    let mut service = with_context(
        AccountIdentityAuthorityService::open(&path),
        "open account authority service",
    )?;

    let result = service.consume_and_apply_mutation_authority(&[]);

    assert!(matches!(
        result,
        Err(AccountIdentityMutationAuthorityServiceError::Mutation(
            AccountIdentityMutationAuthorityError::VerificationKeyUnavailable
        ))
    ));
    drop(service);
    remove_database(&path);
    Ok(())
}
