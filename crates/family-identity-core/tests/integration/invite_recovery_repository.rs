use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_family_identity_core::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityService,
    AccountIdentityMutationAuthorityServiceError, SqliteAccountIdentityAuthorityRepository,
};
use ocentra_family_identity_core::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use rusqlite::Connection;

struct TestFailure {
    context: &'static str,
    detail: String,
}

impl TestFailure {
    fn new(context: &'static str, error: impl std::fmt::Debug) -> Self {
        Self {
            context,
            detail: format!("{error:?}"),
        }
    }
}

impl std::fmt::Debug for TestFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.context, self.detail)
    }
}

type TestResult<T = ()> = Result<T, TestFailure>;

fn temporary_database_path() -> TestResult<PathBuf> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| TestFailure::new("system clock is after the Unix epoch", error))?;
    let nonce = elapsed.as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "ocentra-account-wp04-invite-recovery-integration-{}-{nonce}.sqlite",
        std::process::id()
    )))
}

#[test]
fn repository_restart_keeps_the_account_owned_sqlite_locking_contract() -> TestResult {
    let path = temporary_database_path()?;
    {
        SqliteAccountIdentityAuthorityRepository::open(&path).map_err(|error| {
            TestFailure::new(
                "open durable repository for the first process lifetime",
                error,
            )
        })?;
    }
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .map_err(|error| TestFailure::new("open durable repository after a restart", error))?;
    }

    let connection = Connection::open(&path)
        .map_err(|error| TestFailure::new("inspect restarted repository", error))?;
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .map_err(|error| TestFailure::new("read SQLite journal mode", error))?;
    assert_eq!(journal_mode, "delete");
    let clock_rows: i64 = connection
        .query_row(
            "SELECT count(*) FROM account_identity_runtime_clock",
            [],
            |row| row.get(0),
        )
        .map_err(|error| TestFailure::new("query persisted runtime clock rows", error))?;
    assert_eq!(clock_rows, 0);

    drop(connection);
    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[test]
fn public_account_service_does_not_mount_without_installer_owned_custody() {
    assert!(matches!(
        AccountIdentityAuthorityService::mount_account_owned(),
        Err(AccountIdentityAuthorityRepositoryError::Unavailable)
    ));
}

#[test]
fn public_mutation_boundary_reports_missing_owner_key_custody_without_parsing_wire_data(
) -> TestResult {
    let path = temporary_database_path()?;
    let mut service = AccountIdentityAuthorityService::open(&path)
        .map_err(|error| TestFailure::new("open account authority service", error))?;

    let result = service.consume_and_apply_mutation_authority(b"not-an-authority");

    assert!(matches!(
        result,
        Err(AccountIdentityMutationAuthorityServiceError::Mutation(
            AccountIdentityMutationAuthorityError::VerificationKeyUnavailable
        ))
    ));
    drop(service);
    let _ = std::fs::remove_file(&path);
    Ok(())
}
