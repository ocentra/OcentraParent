use ocentra_protected_capability_custody_protocol::constants;

use crate::path_security::PendingSecuredPath;

use super::{error_status, BrokerRuntimeError};

pub(super) fn fixed_database_path() -> std::path::PathBuf {
    std::path::PathBuf::from(String::from_utf16_lossy(constants::BROKER_DATA_ROOT_UTF16))
        .join(constants::BROKER_STORAGE_DIRECTORY)
        .join(constants::BROKER_DATABASE_FILE)
}

pub(super) fn fixed_database_identity_path() -> Result<std::path::PathBuf, BrokerRuntimeError> {
    let database = fixed_database_path();
    let directory = database.parent().ok_or(BrokerRuntimeError::Unavailable)?;
    validate_fixed_directory(directory)?;
    let canonical_directory = dunce::canonicalize(directory).map_err(error_status::storage_io)?;
    Ok(canonical_directory.join(constants::BROKER_DATABASE_FILE))
}

pub(super) fn open_fixed_database() -> Result<PendingSecuredPath, BrokerRuntimeError> {
    let database = fixed_database_identity_path()?;
    let directory = database.parent().ok_or(BrokerRuntimeError::Unavailable)?;
    validate_fixed_directory(directory)?;
    match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&database)
    {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error_status::storage_io(error)),
    }
    PendingSecuredPath::open(&database).map_err(error_status::path_security)
}

fn validate_fixed_directory(path: &std::path::Path) -> Result<(), BrokerRuntimeError> {
    let metadata = std::fs::symlink_metadata(path).map_err(error_status::storage_io)?;
    if !metadata.is_dir() {
        return Err(BrokerRuntimeError::Unavailable);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(BrokerRuntimeError::Unavailable);
        }
    }
    Ok(())
}
