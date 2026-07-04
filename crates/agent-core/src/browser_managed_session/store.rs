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
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
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

pub(crate) fn managed_profile_store_paths(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStorePaths, BrowserManagedProfileStoreError> {
    paths::managed_profile_store_paths(config)
}

pub(crate) fn profile_store_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    input: ProfileStoreRecordInput,
) -> BrowserManagedProfileStoreRecord {
    record::profile_store_record(config, paths, input)
}

pub(crate) fn read_profile_store_entry(
    metadata_path: &std::path::Path,
) -> Result<
    Option<ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry>,
    BrowserManagedProfileStoreError,
> {
    io::read_profile_store_entry(metadata_path)
}

pub(crate) fn write_profile_store_entry(
    metadata_path: &std::path::Path,
    entry: &ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    io::write_profile_store_entry(metadata_path, entry)
}

pub(crate) fn profile_id_contains_path_separator(profile_id: &str) -> bool {
    paths::profile_id_contains_path_separator(profile_id)
}

pub(crate) fn default_profile_path_rejected(path: &std::path::Path) -> bool {
    paths::default_profile_path_rejected(path)
}

pub(crate) fn managed_profile_path_owned(path: &std::path::Path) -> bool {
    paths::managed_profile_path_owned(path)
}
