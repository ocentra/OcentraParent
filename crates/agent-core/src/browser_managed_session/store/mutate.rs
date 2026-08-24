use std::fs;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::{lock::with_profile_store_lock, paths::managed_profile_store_paths};

pub(crate) fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    ensure_profile_root(config, &paths)?;
    with_profile_store_lock(&paths, || {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_create::create_or_repair_locked(config, paths.clone())
    })
}

pub(crate) fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    ensure_profile_root(config, &paths)?;
    with_profile_store_lock(&paths, || {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_delete::delete_locked(config, paths.clone())
    })
}

fn ensure_profile_root(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation::validate_profile_store_paths(config, paths)?;
    if !config.profile_root_dir.exists() {
        fs::create_dir_all(&config.profile_root_dir)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        super::atomic_write::sync_parent_directory(&config.profile_root_dir)?;
    }
    super::validation::validate_profile_store_paths(config, paths)
}
