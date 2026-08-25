use ocentra_protected_capability_custody_protocol::constants;

use crate::path_security::PendingSecuredPath;

use super::{error_status, BrokerRuntimeError};

pub(super) fn open_fixed_database() -> Result<PendingSecuredPath, BrokerRuntimeError> {
    let directory =
        std::path::PathBuf::from(String::from_utf16_lossy(constants::BROKER_DATA_ROOT_UTF16))
            .join(constants::BROKER_STORAGE_DIRECTORY);
    validate_fixed_directory(&directory)?;
    let database = directory.join(constants::BROKER_DATABASE_FILE);
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
