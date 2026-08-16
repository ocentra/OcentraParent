use std::{fs, io::ErrorKind, path::Path};

use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::BrowserManagedProfileStoreError;

pub(crate) fn read_profile_store_entry(
    metadata_path: &Path,
) -> Result<Option<BrowserManagedProfileStoreEntry>, BrowserManagedProfileStoreError> {
    match fs::read_to_string(metadata_path) {
        Ok(contents) => serde_json::from_str(&contents)
            .map(Some)
            .map_err(|_error| BrowserManagedProfileStoreError::MetadataCorrupt),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}

pub(crate) fn write_profile_store_entry(
    metadata_path: &Path,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    let contents = serde_json::to_string_pretty(entry)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    fs::write(metadata_path, contents).map_err(|_error| BrowserManagedProfileStoreError::Io)
}
