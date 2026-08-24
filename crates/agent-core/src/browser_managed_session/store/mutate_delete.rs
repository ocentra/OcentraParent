use std::fs;

use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};

pub(super) fn delete_locked(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let stored_entry = super::io::read_profile_store_entry(config, &paths)?;
    if paths.deletion_path.is_dir() {
        return complete_pending_deletion(config, paths, stored_entry);
    }
    stage_profile_for_deletion(&paths, stored_entry.as_ref())?;
    super::validation::validate_profile_store_paths(config, &paths)?;
    let record = deleted_record(config, paths.clone(), stored_entry.as_ref());
    super::io::write_profile_store_entry(config, &paths, &record.entry)?;
    remove_deletion_staging(&paths)?;
    Ok(record)
}

pub(super) fn complete_pending_deletion(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let entry = stored_entry.ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    let record = match entry.lifecycle_state {
        BrowserManagedProfileLifecycleState::Ready => {
            let record = deleted_record(config, paths.clone(), Some(&entry));
            super::io::write_profile_store_entry(config, &paths, &record.entry)?;
            record
        }
        BrowserManagedProfileLifecycleState::Deleted => {
            super::load_state::stored_record(paths.clone(), entry)
        }
        _ => return Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    };
    remove_deletion_staging(&paths)?;
    Ok(record)
}

fn stage_profile_for_deletion(
    paths: &BrowserManagedProfileStorePaths,
    stored_entry: Option<&BrowserManagedProfileStoreEntry>,
) -> Result<(), BrowserManagedProfileStoreError> {
    if !paths.profile_dir.is_dir() {
        return Ok(());
    }
    match stored_entry {
        Some(entry) if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready => {}
        _ => return Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
    fs::rename(&paths.profile_dir, &paths.deletion_path)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    super::atomic_write::sync_parent_directory(&paths.profile_dir)?;
    Ok(())
}

fn deleted_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<&BrowserManagedProfileStoreEntry>,
) -> BrowserManagedProfileStoreRecord {
    if let Some(entry) = stored_entry {
        if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Deleted {
            return super::load_state::stored_record(paths, entry.clone());
        }
    }
    let now = super::validation::timestamp_now();
    super::record::profile_store_record(
        config,
        paths,
        ProfileStoreRecordInput {
            created_at: stored_entry
                .map(|entry| entry.created_at.clone())
                .unwrap_or_else(|| now.clone()),
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::Deleted,
            missing_since: None,
            repaired_at: None,
            deleted_at: Some(now),
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_DELETED.to_string()),
        },
    )
}

fn remove_deletion_staging(
    paths: &BrowserManagedProfileStorePaths,
) -> Result<(), BrowserManagedProfileStoreError> {
    if paths.deletion_path.is_dir() {
        fs::remove_dir_all(&paths.deletion_path)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        super::atomic_write::sync_parent_directory(&paths.deletion_path)?;
    }
    Ok(())
}
