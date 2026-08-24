use std::{fs, io::ErrorKind, path::Path};

use super::super::BrowserManagedProfileStoreError;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[cfg(windows)]
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

pub(super) fn validate_path_chain(path: &Path) -> Result<(), BrowserManagedProfileStoreError> {
    for candidate in path.ancestors() {
        validate_path_metadata(candidate)?;
    }
    Ok(())
}

fn validate_path_metadata(path: &Path) -> Result<(), BrowserManagedProfileStoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_indirection(&metadata) => {
            Err(BrowserManagedProfileStoreError::UnsafePath)
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}

pub(super) fn metadata_is_indirection(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        return metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0;
    }
    #[cfg(not(windows))]
    {
        false
    }
}
