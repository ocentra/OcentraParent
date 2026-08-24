use std::{fs, io::ErrorKind};

use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths,
};

pub(crate) fn read_profile_store_entry(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
) -> Result<Option<BrowserManagedProfileStoreEntry>, BrowserManagedProfileStoreError> {
    match fs::read_to_string(&paths.metadata_path) {
        Ok(contents) => {
            let entry = serde_json::from_str(&contents)
                .map_err(|_error| BrowserManagedProfileStoreError::MetadataCorrupt)?;
            super::validation::validate_stored_entry(config, &entry)?;
            Ok(Some(entry))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}

pub(crate) fn write_profile_store_entry(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation::validate_stored_entry(config, entry)?;
    let contents = serde_json::to_string_pretty(entry)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    super::atomic_write::replace_and_sync(&paths.metadata_path, contents.as_bytes())?;
    let persisted = read_profile_store_entry(config, paths)?
        .ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    if persisted == *entry {
        Ok(())
    } else {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    }
}
