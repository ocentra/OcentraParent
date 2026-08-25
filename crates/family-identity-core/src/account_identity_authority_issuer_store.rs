use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags};

use crate::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, SqliteAccountIdentityAuthorityRepository,
};
use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file::{open_store_file_guard, StoreFileGuard};
use crate::parent_presence_store_file_creation::publish_initialized_store_if_absent;
use crate::parent_presence_store_path::validate_caller_custody_path;
use crate::session_lifecycle_custody::SessionLifecyclePolicy;

use super::{startup, AccountIdentityIssuerError};

/// Protected file custody shared with the repository's established parent
/// presence store. The retained file/ancestor handles reject path replacement
/// and reparse/symlink traversal for the lifetime of the issuer.
pub(crate) struct AccountIdentityIssuerStore {
    path: PathBuf,
    repository: SqliteAccountIdentityAuthorityRepository,
    file_guard: StoreFileGuard,
}

/// Dependency-owned protected-custody seam. The success type cannot be
/// constructed outside this Account-owned module; the future adjacent adapter
/// must translate a real broker guard that proves owner ACL and exclusive
/// writer custody. Returning a path or a boolean attestation is insufficient.
pub(crate) trait AccountIdentityIssuerProtectedStoreOwner: Send + Sync {
    fn open_protected_store(
        &self,
        path: &Path,
    ) -> Result<AccountIdentityIssuerStore, AccountIdentityIssuerError>;
}

impl AccountIdentityIssuerStore {
    pub(crate) fn open(path: &Path) -> Result<Self, AccountIdentityIssuerError> {
        open_for_current_platform(path)
    }

    fn open_locally_guarded(path: &Path) -> Result<Self, AccountIdentityIssuerError> {
        validate_path(path)?;
        validate_caller_custody_path(path).map_err(map_store_error)?;
        publish_initialized_store_if_absent(path, initialize_temporary_store)
            .map_err(map_store_error)?;
        let file_guard = open_store_file_guard(path).map_err(map_store_error)?;
        let connection = open_connection(path)?;
        let repository = SqliteAccountIdentityAuthorityRepository::from_owned_connection(
            connection,
            SessionLifecyclePolicy::production_default(),
        )
        .map_err(map_repository_error)?;
        startup::initialize(repository.account_issuer_connection())?;
        file_guard
            .validate_path_identity(path)
            .map_err(map_store_error)?;
        Ok(Self {
            path: path.to_path_buf(),
            repository,
            file_guard,
        })
    }

    pub(crate) fn repository(&self) -> &SqliteAccountIdentityAuthorityRepository {
        &self.repository
    }

    pub(crate) fn repository_mut(&mut self) -> &mut SqliteAccountIdentityAuthorityRepository {
        &mut self.repository
    }

    pub(crate) fn validate_identity(&self) -> Result<(), AccountIdentityIssuerError> {
        self.file_guard
            .validate_path_identity(&self.path)
            .map_err(map_store_error)
    }
}

#[cfg(unix)]
fn open_for_current_platform(
    path: &Path,
) -> Result<AccountIdentityIssuerStore, AccountIdentityIssuerError> {
    AccountIdentityIssuerStore::open_locally_guarded(path)
}

#[cfg(windows)]
fn open_for_current_platform(
    _path: &Path,
) -> Result<AccountIdentityIssuerStore, AccountIdentityIssuerError> {
    // The existing local guard rejects reparse/path replacement, but Windows
    // owner ACL and isolated-writer custody require the dependency-owned
    // broker. Fail closed until its Account adapter is installed.
    Err(AccountIdentityIssuerError::ProtectedStoreUnavailable)
}

#[cfg(not(any(unix, windows)))]
fn open_for_current_platform(
    _path: &Path,
) -> Result<AccountIdentityIssuerStore, AccountIdentityIssuerError> {
    Err(AccountIdentityIssuerError::ProtectedStoreUnavailable)
}

fn initialize_temporary_store(path: &Path) -> Result<(), ParentPresenceStoreError> {
    let connection = open_connection(path).map_err(|_| ParentPresenceStoreError::Unavailable)?;
    let repository = SqliteAccountIdentityAuthorityRepository::from_owned_connection(
        connection,
        SessionLifecyclePolicy::production_default(),
    )
    .map_err(|_| ParentPresenceStoreError::Unavailable)?;
    startup::initialize(repository.account_issuer_connection())
        .map_err(|_| ParentPresenceStoreError::Unavailable)?;
    repository
        .account_issuer_connection()
        .execute_batch("PRAGMA optimize;")
        .map_err(|_| ParentPresenceStoreError::Unavailable)?;
    drop(repository);
    Ok(())
}

fn open_connection(path: &Path) -> Result<Connection, AccountIdentityIssuerError> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NOFOLLOW,
    )
    .map_err(|_| AccountIdentityIssuerError::NonDurableStorage)
}

fn validate_path(path: &Path) -> Result<(), AccountIdentityIssuerError> {
    if !path.is_absolute() || path.as_os_str().is_empty() {
        return Err(AccountIdentityIssuerError::NonDurableStorage);
    }
    let path_text = path.to_string_lossy().to_ascii_lowercase();
    if path_text == ":memory:"
        || path_text.starts_with("file:")
        || path_text.contains("mode=memory")
        || path_text.contains("cache=shared")
        || path.exists() && path.is_dir()
    {
        return Err(AccountIdentityIssuerError::NonDurableStorage);
    }
    Ok(())
}

fn map_repository_error(
    _error: AccountIdentityAuthorityRepositoryError,
) -> AccountIdentityIssuerError {
    AccountIdentityIssuerError::InvalidDurableSchema
}

fn map_store_error(error: ParentPresenceStoreError) -> AccountIdentityIssuerError {
    match error {
        ParentPresenceStoreError::Unavailable => AccountIdentityIssuerError::NonDurableStorage,
        ParentPresenceStoreError::IntegrityRejected => {
            AccountIdentityIssuerError::DurableIntegrityFailure
        }
    }
}
