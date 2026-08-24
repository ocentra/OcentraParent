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
    with_profile_store_lock(&paths, |guards| {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_create::create_or_repair_locked(config, paths.clone(), guards)
    })
}

pub(crate) fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    ensure_profile_root(config, &paths)?;
    with_profile_store_lock(&paths, |guards| {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_delete::delete_locked(config, paths.clone(), guards)
    })
}

fn ensure_profile_root(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation::validate_profile_store_paths(config, paths)?;
    let root_guard = super::path_guards_root::ensure_directory_chain(&config.profile_root_dir)?;
    if config.profile_root_dir.parent().is_some() {
        super::atomic_write::sync_parent_directory(&config.profile_root_dir)?;
    }
    root_guard.validate()?;
    super::validation::validate_profile_store_paths(config, paths)
}
