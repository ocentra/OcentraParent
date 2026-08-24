use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStoreRecord,
};
use super::{lock::with_profile_store_lock, paths::managed_profile_store_paths};

pub(crate) fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    with_profile_store_lock(&paths, |guards| {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_create::create_or_repair_locked(config, paths.clone(), guards)
    })
}

pub(crate) fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    with_profile_store_lock(&paths, |guards| {
        super::validation::validate_profile_store_paths(config, &paths)?;
        super::mutate_delete::delete_locked(config, paths.clone(), guards)
    })
}
