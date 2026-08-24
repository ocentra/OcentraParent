use std::path::Path;

use chrono::{SecondsFormat, Utc};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths,
};

pub(crate) fn timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub(crate) fn validate_profile_store_config(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<(), BrowserManagedProfileStoreError> {
    if !config.profile_root_dir.is_absolute()
        || config.profile_root_dir.as_os_str().is_empty()
        || !bounded_non_empty(
            &config.profile_id,
            ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_PROFILE_ID_BYTES,
        )
        || !bounded_non_empty(
            &config.profile_scope_id,
            ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_PROFILE_SCOPE_ID_BYTES,
        )
        || !bounded_non_empty(
            &config.device_id,
            ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_DEVICE_ID_BYTES,
        )
        || !bounded_non_empty(
            &config.policy_revision,
            ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_POLICY_REVISION_BYTES,
        )
    {
        return Err(BrowserManagedProfileStoreError::UnownedProfileRejected);
    }
    Ok(())
}

fn bounded_non_empty(value: &str, max_bytes: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max_bytes
}

pub(crate) fn validate_profile_store_paths(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation_paths::validate_profile_store_paths(config, paths)
}

pub(crate) fn validate_path_chain_for_lock(
    path: &Path,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation_path_metadata::validate_path_chain(path)
}

pub(crate) fn validate_stored_entry(
    config: &BrowserManagedProfileStoreConfig,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation_entry::validate_stored_entry(config, entry)
}
