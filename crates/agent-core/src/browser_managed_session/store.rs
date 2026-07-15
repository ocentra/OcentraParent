#[path = "store/io.rs"]
mod io;
#[path = "store/load.rs"]
mod load;
#[path = "store/mutate.rs"]
mod mutate;
#[path = "store/paths.rs"]
mod paths;
#[path = "store/record.rs"]
mod record;

use super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStoreRecord,
};

pub(crate) fn profile_store_error_reason(error: &BrowserManagedProfileStoreError) -> &'static str {
    record::profile_store_error_reason(error)
}

pub(crate) fn load_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    load::load_managed_browser_profile_store(config)
}

pub(crate) fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    mutate::create_or_repair_managed_browser_profile_store(config)
}

pub(crate) fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    mutate::delete_managed_browser_profile_store(config)
}
