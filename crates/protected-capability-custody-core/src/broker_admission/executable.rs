use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;

use super::{error_status, platform, BrokerExecutableGuard, BrokerRuntimeError};

impl BrokerExecutableGuard {
    pub fn open_client_sibling() -> Result<Self, BrokerRuntimeError> {
        let current = std::env::current_exe().map_err(error_status::broker_executable)?;
        let parent = current
            .parent()
            .ok_or(BrokerRuntimeError::InvalidBrokerProcess)?;
        open_exact(
            parent.join(
                ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME,
            ),
        )
    }

    pub fn path(&self) -> &Path {
        &self.canonical_path
    }

    pub(super) fn open_current_broker() -> Result<Self, BrokerRuntimeError> {
        let current = std::env::current_exe().map_err(error_status::broker_executable)?;
        open_exact(current)
    }
}

fn open_exact(candidate: PathBuf) -> Result<BrokerExecutableGuard, BrokerRuntimeError> {
    let canonical = dunce::canonicalize(candidate).map_err(error_status::broker_executable)?;
    let expected = OsStr::new(
        ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME,
    );
    if !canonical.is_absolute() || canonical.file_name() != Some(expected) {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    let executable_handle = open_pinned(&canonical)?;
    if !executable_handle
        .metadata()
        .map_err(error_status::broker_executable)?
        .is_file()
    {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    platform::validate_broker_executable(&executable_handle, &canonical)
        .map_err(|error| error_status::broker_platform_admission(&error))?;
    Ok(BrokerExecutableGuard {
        canonical_path: canonical,
        _executable_handle: executable_handle,
    })
}

#[cfg(windows)]
fn open_pinned(path: &Path) -> Result<File, BrokerRuntimeError> {
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .open(path)
        .map_err(error_status::broker_executable)
}

#[cfg(not(windows))]
fn open_pinned(_path: &Path) -> Result<File, BrokerRuntimeError> {
    Err(BrokerRuntimeError::InvalidBrokerProcess)
}
