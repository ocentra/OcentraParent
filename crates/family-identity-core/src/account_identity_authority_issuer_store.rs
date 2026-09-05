use std::path::{Path, PathBuf};

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file::StoreFileGuard;

use super::AccountIdentityIssuerError;

/// Protected file custody shared with the repository's established parent
/// presence store. The retained file/ancestor handles reject path replacement
/// and reparse/symlink traversal for the lifetime of the issuer.
pub(crate) struct AccountIdentityIssuerStore {
    path: PathBuf,
    repository: SqliteAccountIdentityAuthorityRepository,
    file_guard: StoreFileGuard,
}

impl AccountIdentityIssuerStore {
    pub(crate) fn open(path: &Path) -> Result<Self, AccountIdentityIssuerError> {
        open_for_current_platform(path)
    }

    pub(crate) fn repository(&self) -> &SqliteAccountIdentityAuthorityRepository {
        &self.repository
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

fn map_store_error(error: ParentPresenceStoreError) -> AccountIdentityIssuerError {
    match error {
        ParentPresenceStoreError::Unavailable => AccountIdentityIssuerError::NonDurableStorage,
        ParentPresenceStoreError::IntegrityRejected => {
            AccountIdentityIssuerError::DurableIntegrityFailure
        }
    }
}
