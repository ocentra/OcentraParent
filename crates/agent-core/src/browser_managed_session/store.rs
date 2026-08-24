#[path = "store/atomic_write.rs"]
mod atomic_write;
#[path = "store/io.rs"]
mod io;
#[path = "store/load.rs"]
mod load;
#[path = "store/load_deletion_state.rs"]
mod load_deletion_state;
#[path = "store/load_state.rs"]
mod load_state;
#[path = "store/lock.rs"]
mod lock;
#[path = "store/mutate.rs"]
mod mutate;
#[path = "store/mutate_create.rs"]
mod mutate_create;
#[path = "store/mutate_delete.rs"]
mod mutate_delete;
#[path = "store/paths.rs"]
mod paths;
#[path = "store/record.rs"]
mod record;
#[path = "store/validation.rs"]
mod validation;
#[path = "store/validation_entry.rs"]
mod validation_entry;
#[path = "store/validation_entry_state.rs"]
mod validation_entry_state;
#[path = "store/validation_path_metadata.rs"]
mod validation_path_metadata;
#[path = "store/validation_paths.rs"]
mod validation_paths;
#[path = "store/validation_timestamps.rs"]
mod validation_timestamps;

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
