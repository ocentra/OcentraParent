use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::path_guards::ProfileStorePathGuards;

pub(super) fn complete_pending_deletion(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let entry = stored_entry.ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    match entry.lifecycle_state {
        BrowserManagedProfileLifecycleState::Ready => {
            let record = super::mutate_delete::deleted_record(config, paths.clone(), Some(&entry));
            super::io::write_profile_store_entry(config, &paths, &record.entry, guards)?;
        }
        BrowserManagedProfileLifecycleState::RepairRequired
            if entry.repair_reason.as_deref()
                == Some(constants::browser::PROFILE_STORE_REASON_DELETE_PENDING) =>
        {
            let record = super::mutate_delete::deleted_record(config, paths.clone(), Some(&entry));
            super::io::write_profile_store_entry(config, &paths, &record.entry, guards)?;
        }
        BrowserManagedProfileLifecycleState::Deleted => {}
        _ => return Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
    super::mutate_delete_cleanup::remove_deletion_staging(&paths, guards)?;
    super::mutate_delete_cleanup::deleted_record_after_cleanup(config, paths, guards)
}

pub(super) fn stage_profile_for_deletion(
    paths: &BrowserManagedProfileStorePaths,
    stored_entry: Option<&BrowserManagedProfileStoreEntry>,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    if !guards.directory_exists(&paths.profile_dir)? {
        return Ok(());
    }
    match stored_entry {
        Some(entry) if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready => {}
        Some(entry)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::RepairRequired
                && entry.repair_reason.as_deref()
                    == Some(constants::browser::PROFILE_STORE_REASON_DELETE_PENDING) => {}
        _ => return Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
    guards.validate_path(
        &paths.profile_dir,
        super::path_guards::guarded_directory_path_kind(),
    )?;
    guards.rename_directory(&paths.profile_dir, &paths.deletion_path)?;
    super::atomic_write::sync_parent_directory(&paths.profile_dir)?;
    guards.validate()?;
    Ok(())
}
