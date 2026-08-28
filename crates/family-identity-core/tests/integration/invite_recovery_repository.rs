use std::path::PathBuf;
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
        "ocentra-account-wp04-invite-recovery-integration-{}-{nonce}.sqlite",
        std::process::id()
    ))
}

#[test]
fn repository_restart_keeps_the_account_owned_sqlite_locking_contract() {
    let path = temporary_database_path();
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("open durable repository for the first process lifetime");
    }
    {
        SqliteAccountIdentityAuthorityRepository::open(&path)
            .expect("open durable repository after a restart");
    }

    let connection = Connection::open(&path).expect("inspect restarted repository");
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("read SQLite journal mode");
    assert_eq!(journal_mode, "delete");
    let clock_rows: i64 = connection
        .query_row(
            "SELECT count(*) FROM account_identity_runtime_clock",
            [],
            |row| row.get(0),
        )
        .expect("query persisted runtime clock rows");
    assert_eq!(clock_rows, 0);

    drop(connection);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn public_account_service_does_not_mount_without_installer_owned_custody() {
    assert!(matches!(
        AccountIdentityAuthorityService::mount_account_owned(),
        Err(AccountIdentityAuthorityRepositoryError::Unavailable)
    ));
}

#[test]
fn public_mutation_boundary_reports_missing_owner_key_custody_without_parsing_wire_data() {
    let path = temporary_database_path();
    let mut service =
        AccountIdentityAuthorityService::open(&path).expect("open account authority service");

    let result = service.consume_and_apply_mutation_authority(b"not-an-authority");

    assert!(matches!(
        result,
        Err(AccountIdentityMutationAuthorityServiceError::Mutation(
            AccountIdentityMutationAuthorityError::VerificationKeyUnavailable
        ))
    ));
    drop(service);
    let _ = std::fs::remove_file(&path);
}
