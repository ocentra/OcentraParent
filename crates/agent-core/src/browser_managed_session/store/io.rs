use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths,
};
use super::path_guards::ProfileStorePathGuards;

pub(crate) fn read_profile_store_entry(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    guards: &ProfileStorePathGuards,
) -> Result<Option<BrowserManagedProfileStoreEntry>, BrowserManagedProfileStoreError> {
    match guards.read_text(&paths.metadata_path)? {
        Some(contents) => {
            let entry = serde_json::from_str(&contents)
                .map_err(|_error| BrowserManagedProfileStoreError::MetadataCorrupt)?;
            super::validation::validate_stored_entry(config, &entry)?;
            Ok(Some(entry))
        }
        None => Ok(None),
    }
}

pub(crate) fn write_profile_store_entry(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    entry: &BrowserManagedProfileStoreEntry,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation::validate_stored_entry(config, entry)?;
    guards.validate()?;
    let contents = serde_json::to_string_pretty(entry)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    if contents.len()
        > ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_METADATA_BYTES
    {
        return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
    }
    super::atomic_write::replace_and_sync(&paths.metadata_path, contents.as_bytes())?;
    guards.validate()?;
    let persisted = read_profile_store_entry(config, paths, guards)?
        .ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    if persisted == *entry {
        Ok(())
    } else {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    }
}
