use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};
use super::io::read_profile_store_entry;
use super::paths::managed_profile_store_paths;
use super::record::profile_store_record;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileLifecycleState;
use ocentra_parent_agent_protocol::constants;

pub(crate) fn load_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path)?;
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());

    if !paths.profile_dir.is_dir() {
        return Ok(profile_store_record(
            config,
            paths,
            ProfileStoreRecordInput {
                created_at,
                lifecycle_state: BrowserManagedProfileLifecycleState::Missing,
                missing_since: Some(config.now.clone()),
                repaired_at: None,
                deleted_at: None,
                repair_reason: Some(
                    constants::browser::PROFILE_STORE_REASON_PROFILE_DIR_MISSING.to_string(),
                ),
            },
        ));
    }

    if stored_entry.is_none() {
        return Ok(profile_store_record(
            config,
            paths,
            ProfileStoreRecordInput {
                created_at,
                lifecycle_state: BrowserManagedProfileLifecycleState::RepairRequired,
                missing_since: None,
                repaired_at: None,
                deleted_at: None,
                repair_reason: Some(
                    constants::browser::PROFILE_STORE_REASON_METADATA_MISSING.to_string(),
                ),
            },
        ));
    }

    Ok(profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at,
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: None,
        },
    ))
}
