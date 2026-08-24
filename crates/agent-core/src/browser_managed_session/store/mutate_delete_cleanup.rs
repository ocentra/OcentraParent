use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::path_guards::ProfileStorePathGuards;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileLifecycleState;

pub(super) fn deleted_record_after_cleanup(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    if guards.directory_exists(&paths.profile_dir)?
        || guards.directory_exists(&paths.deletion_path)?
    {
        return Err(BrowserManagedProfileStoreError::Io);
    }
    let persisted = super::io::read_profile_store_entry(config, &paths, guards)?
        .ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    if persisted.lifecycle_state != BrowserManagedProfileLifecycleState::Deleted {
        return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
    }
    guards.validate()?;
    Ok(super::load_state::stored_record(paths, persisted))
}

pub(super) fn remove_deletion_staging(
    paths: &BrowserManagedProfileStorePaths,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    if guards.directory_exists(&paths.deletion_path)? {
        guards.remove_directory(&paths.deletion_path)?;
        super::atomic_write::sync_parent_directory(&paths.deletion_path)?;
    }
    Ok(())
}
