use std::fs;

use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};

pub(super) fn create_or_repair_locked(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let stored_entry = super::io::read_profile_store_entry(config, &paths)?;
    if paths.deletion_path.is_dir() {
        return super::mutate_delete::complete_pending_deletion(config, paths, stored_entry);
    }

    match (stored_entry, paths.profile_dir.is_dir()) {
        (None, false) => create_new_profile(config, paths),
        (Some(entry), true)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready =>
        {
            Ok(super::load_state::stored_record(paths, entry))
        }
        (Some(entry), false)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready =>
        {
            repair_missing_profile(config, paths, entry)
        }
        (Some(entry), false)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Deleted =>
        {
            Ok(super::load_state::stored_record(paths, entry))
        }
        _ => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
}

fn create_new_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let now = super::validation::timestamp_now();
    let record = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: now.clone(),
            updated_at: now,
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_CREATED.to_string()),
        },
    );
    fs::create_dir(&record.profile_dir).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    super::atomic_write::sync_parent_directory(&record.profile_dir)?;
    super::validation::validate_profile_store_paths(config, &paths)?;
    persist_new_profile(config, &paths, &record)?;
    Ok(record)
}

fn persist_new_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    record: &BrowserManagedProfileStoreRecord,
) -> Result<(), BrowserManagedProfileStoreError> {
    if let Err(error) = super::io::write_profile_store_entry(config, paths, &record.entry) {
        let _ = fs::remove_dir(&record.profile_dir);
        return Err(error);
    }
    Ok(())
}

fn repair_missing_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    entry: BrowserManagedProfileStoreEntry,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    fs::create_dir(&paths.profile_dir).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    super::atomic_write::sync_parent_directory(&paths.profile_dir)?;
    super::validation::validate_profile_store_paths(config, &paths)?;
    let now = super::validation::timestamp_now();
    let record = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: entry.created_at,
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: Some(now),
            deleted_at: None,
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_REPAIRED.to_string()),
        },
    );
    super::io::write_profile_store_entry(config, &paths, &record.entry)?;
    Ok(record)
}
