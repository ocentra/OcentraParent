use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileLifecycleState;
use ocentra_parent_agent_protocol::constants;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};
use super::io::{read_profile_store_entry, write_profile_store_entry};
use super::paths::managed_profile_store_paths;
use super::record::profile_store_record;

pub(crate) fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    let profile_existed = paths.profile_dir.is_dir();
    std::fs::create_dir_all(&config.profile_root_dir)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path).unwrap_or(None);
    std::fs::create_dir_all(&paths.profile_dir)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());
    let repaired_at = if (profile_existed && stored_entry.is_none())
        || (!profile_existed && stored_entry.is_some())
    {
        Some(config.now.clone())
    } else {
        stored_entry
            .as_ref()
            .and_then(|entry| match entry.lifecycle_state {
                BrowserManagedProfileLifecycleState::Ready => None,
                _ => Some(config.now.clone()),
            })
    };
    let repair_reason = repaired_at
        .as_ref()
        .map(|_| constants::browser::PROFILE_STORE_REASON_REPAIRED.to_string());
    let record = profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at,
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at,
            deleted_at: None,
            repair_reason,
        },
    );
    write_profile_store_entry(&record.metadata_path, &record.entry)?;
    Ok(record)
}

pub(crate) fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    std::fs::create_dir_all(&config.profile_root_dir)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path).unwrap_or(None);
    if paths.profile_dir.exists() {
        std::fs::remove_dir_all(&paths.profile_dir)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    }
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());
    let record = profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at,
            lifecycle_state: BrowserManagedProfileLifecycleState::Deleted,
            missing_since: None,
            repaired_at: None,
            deleted_at: Some(config.now.clone()),
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_DELETED.to_string()),
        },
    );
    write_profile_store_entry(&record.metadata_path, &record.entry)?;
    Ok(record)
}
