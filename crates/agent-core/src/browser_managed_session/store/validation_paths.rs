use std::{fs, io::ErrorKind, path::Path};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths,
};

pub(super) fn validate_profile_store_paths(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
) -> Result<(), BrowserManagedProfileStoreError> {
    super::validation_path_metadata::validate_path_chain(&config.profile_root_dir)?;
    super::validation_path_metadata::validate_path_chain(&paths.profile_dir)?;
    super::validation_path_metadata::validate_path_chain(&paths.metadata_path)?;
    super::validation_path_metadata::validate_path_chain(&paths.deletion_path)?;
    super::validation_path_metadata::validate_path_chain(&paths.lock_path)?;

    validate_existing_kind(&config.profile_root_dir, ExistingKind::Directory)?;
    validate_existing_kind(&paths.profile_dir, ExistingKind::Directory)?;
    validate_existing_kind(&paths.deletion_path, ExistingKind::Directory)?;
    validate_existing_kind(&paths.metadata_path, ExistingKind::File)?;
    validate_existing_kind(&paths.lock_path, ExistingKind::File)?;

    if paths.profile_dir.exists() && paths.deletion_path.exists() {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    } else {
        Ok(())
    }
}

fn validate_existing_kind(
    path: &Path,
    expected: ExistingKind,
) -> Result<(), BrowserManagedProfileStoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if super::validation_path_metadata::metadata_is_indirection(&metadata) => {
            Err(BrowserManagedProfileStoreError::UnsafePath)
        }
        Ok(metadata) if expected.matches(&metadata) => Ok(()),
        Ok(_) => Err(BrowserManagedProfileStoreError::UnsafePath),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}

#[derive(Clone, Copy)]
enum ExistingKind {
    Directory,
    File,
}

impl ExistingKind {
    fn matches(self, metadata: &fs::Metadata) -> bool {
        match self {
            Self::Directory => metadata.is_dir(),
            Self::File => metadata.is_file(),
        }
    }
}
