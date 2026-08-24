use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};

pub(super) fn load_missing_profile_state(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
    now: String,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    match stored_entry {
        Some(entry) if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Deleted => {
            Ok(stored_record(paths, entry))
        }
        Some(entry) => Ok(missing_record(config, paths, entry.created_at, now)),
        None => Ok(missing_record(config, paths, now.clone(), now)),
    }
}

pub(super) fn load_existing_profile_state(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
    now: String,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    match stored_entry {
        Some(entry) if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready => {
            Ok(stored_record(paths, entry))
        }
        Some(_) => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
        None => Ok(repair_required_record(
            config,
            paths,
            now.clone(),
            now,
            constants::browser::PROFILE_STORE_REASON_METADATA_MISSING,
        )),
    }
}

pub(super) fn stored_record(
    paths: BrowserManagedProfileStorePaths,
    entry: BrowserManagedProfileStoreEntry,
) -> BrowserManagedProfileStoreRecord {
    BrowserManagedProfileStoreRecord {
        profile_dir: paths.profile_dir,
        metadata_path: paths.metadata_path,
        entry,
    }
}

pub(super) fn missing_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    created_at: String,
    updated_at: String,
) -> BrowserManagedProfileStoreRecord {
    super::record::profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at,
            updated_at: updated_at.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::Missing,
            missing_since: Some(updated_at),
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(
                constants::browser::PROFILE_STORE_REASON_PROFILE_DIR_MISSING.to_string(),
            ),
        },
    )
}

pub(super) fn repair_required_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    created_at: String,
    updated_at: String,
    reason: &'static str,
) -> BrowserManagedProfileStoreRecord {
    super::record::profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at,
            updated_at,
            lifecycle_state: BrowserManagedProfileLifecycleState::RepairRequired,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(reason.to_string()),
        },
    )
}
