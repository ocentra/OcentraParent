use std::fs::{self, File};
use std::path::Path;

use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file_platform::{
    open_ancestor_guards, open_guard_file, validate_delete_sharing_capability,
    validate_private_store_metadata,
};

pub(crate) struct StoreFileGuard {
    identity: same_file::Handle,
    _ancestor_guards: Vec<File>,
}

pub(crate) fn open_store_file_guard(
    path: &Path,
) -> Result<StoreFileGuard, ParentPresenceStoreError> {
    let ancestor_guards = open_ancestor_guards(path)?;
    validate_delete_sharing_capability(path)?;
    let file = open_guard_file(path)?;
    let metadata = file
        .metadata()
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    validate_private_store_metadata(&metadata)?;
    let identity = same_file::Handle::from_file(file)
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    let guard = StoreFileGuard {
        identity,
        _ancestor_guards: ancestor_guards,
    };
    guard.validate_path_identity(path)?;
    Ok(guard)
}

impl StoreFileGuard {
    pub(crate) fn validate_path_identity(
        &self,
        path: &Path,
    ) -> Result<(), ParentPresenceStoreError> {
        let metadata =
            fs::symlink_metadata(path).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        validate_private_store_metadata(&metadata)?;
        let current = same_file::Handle::from_path(path)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if current == self.identity {
            Ok(())
        } else {
            Err(ParentPresenceStoreError::IntegrityRejected)
        }
    }
}
