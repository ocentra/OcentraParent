use std::{fs, path::Path};

use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::path_guards::ProfileStorePathGuards;

pub(super) fn is_pending_deletion(entry: Option<&BrowserManagedProfileStoreEntry>) -> bool {
    matches!(
        entry,
        Some(entry)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::RepairRequired
                && entry.repair_reason.as_deref()
                    == Some(constants::browser::PROFILE_STORE_REASON_DELETE_PENDING)
    )
}

pub(super) fn resume_pending_deletion(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    entry: Option<BrowserManagedProfileStoreEntry>,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let entry = entry.ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    super::mutate_delete_state::stage_profile_for_deletion(&paths, Some(&entry), guards)?;
    super::mutate_delete_state::complete_pending_deletion(config, paths, Some(entry), guards)
}

pub(super) fn persist_ready_or_remove(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    record: &BrowserManagedProfileStoreRecord,
    guards: &ProfileStorePathGuards,
    owns_profile_dir: bool,
) -> Result<(), BrowserManagedProfileStoreError> {
    if let Err(error) = super::io::write_profile_store_entry(config, paths, &record.entry, guards) {
        if owns_profile_dir {
            guards
                .remove_directory(&record.profile_dir)
                .map_err(|_cleanup_error| BrowserManagedProfileStoreError::CleanupFailed)?;
        }
        return Err(error);
    }
    Ok(())
}

pub(super) fn create_profile_dir_or_remove(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    profile_dir: &Path,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    fs::create_dir(profile_dir).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    if let Err(error) = super::atomic_write::sync_parent_directory(profile_dir)
        .and_then(|()| super::validation::validate_profile_store_paths(config, paths))
    {
        return guards
            .remove_directory(profile_dir)
            .map_err(|_cleanup_error| BrowserManagedProfileStoreError::CleanupFailed)
            .and(Err(error));
    }
    Ok(())
}
