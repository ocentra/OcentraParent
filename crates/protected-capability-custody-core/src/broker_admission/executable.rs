use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
};

use super::{error_status, platform, BrokerExecutableGuard, BrokerRuntimeError};

impl BrokerExecutableGuard {
    pub fn open_client_sibling() -> Result<Self, BrokerRuntimeError> {
        open_exact(
            fixed_install_root().join(
                ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME,
            ),
            ExecutableValidation::Client,
        )
    }

    pub fn path(&self) -> &Path {
        &self.canonical_path
    }

    pub(super) fn open_current_broker() -> Result<Self, BrokerRuntimeError> {
        let current = std::env::current_exe().map_err(error_status::broker_executable)?;
        let expected_root = fixed_install_root()
            .join(ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME);
        if current != expected_root {
            return Err(BrokerRuntimeError::InvalidBrokerProcess);
        }
        open_exact(current, ExecutableValidation::Broker)
    }

    pub fn revalidate(&self) -> Result<(), BrokerRuntimeError> {
        platform::validate_broker_executable(&self._executable_handle, &self.canonical_path)
            .map_err(|error| error_status::broker_platform_admission(&error))
    }

    pub fn revalidate_client(&self) -> Result<(), BrokerRuntimeError> {
        platform::admission::validate_client_executable(
            &self._executable_handle,
            &self.canonical_path,
        )
        .map_err(|error| error_status::broker_platform_admission(&error))
    }
}

fn fixed_install_root() -> PathBuf {
    PathBuf::from(String::from_utf16_lossy(
        ocentra_protected_capability_custody_protocol::constants::BROKER_INSTALL_ROOT_UTF16,
    ))
}

#[derive(Clone, Copy)]
enum ExecutableValidation {
    Broker,
    Client,
}

fn open_exact(
    candidate: PathBuf,
    validation: ExecutableValidation,
) -> Result<BrokerExecutableGuard, BrokerRuntimeError> {
    reject_reparse_components(&candidate)?;
    let lexical_handle = open_pinned(&candidate)?;
    validate_pinned_file(&lexical_handle)?;
    let canonical = dunce::canonicalize(candidate).map_err(error_status::broker_executable)?;
    let expected = OsStr::new(
        ocentra_protected_capability_custody_protocol::constants::BROKER_EXECUTABLE_NAME,
    );
    if !canonical.is_absolute() || canonical.file_name() != Some(expected) {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    let executable_handle = open_pinned(&canonical)?;
    validate_pinned_file(&executable_handle)?;
    let lexical = same_file::Handle::from_file(
        lexical_handle
            .try_clone()
            .map_err(error_status::broker_executable)?,
    )
    .map_err(error_status::broker_executable)?;
    let canonical_handle = same_file::Handle::from_file(
        executable_handle
            .try_clone()
            .map_err(error_status::broker_executable)?,
    )
    .map_err(error_status::broker_executable)?;
    if lexical != canonical_handle {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    match validation {
        ExecutableValidation::Broker => {
            platform::validate_broker_executable(&executable_handle, &canonical)
        }
        ExecutableValidation::Client => {
            platform::admission::validate_client_executable(&executable_handle, &canonical)
        }
    }
    .map_err(|error| error_status::broker_platform_admission(&error))?;
    Ok(BrokerExecutableGuard {
        canonical_path: canonical,
        _executable_handle: executable_handle,
    })
}

fn validate_pinned_file(file: &File) -> Result<(), BrokerRuntimeError> {
    let metadata = file.metadata().map_err(error_status::broker_executable)?;
    if !metadata.is_file() {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    #[cfg(windows)]
    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(BrokerRuntimeError::InvalidBrokerProcess);
    }
    Ok(())
}

#[cfg(windows)]
fn reject_reparse_components(path: &Path) -> Result<(), BrokerRuntimeError> {
    use std::path::{Component, PathBuf};

    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        if matches!(component, Component::Prefix(_) | Component::RootDir) {
            continue;
        }
        let metadata =
            std::fs::symlink_metadata(&current).map_err(error_status::broker_executable)?;
        if metadata.file_type().is_symlink()
            || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        {
            return Err(BrokerRuntimeError::InvalidBrokerProcess);
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn reject_reparse_components(_path: &Path) -> Result<(), BrokerRuntimeError> {
    Err(BrokerRuntimeError::InvalidBrokerProcess)
}

#[cfg(windows)]
fn open_pinned(path: &Path) -> Result<File, BrokerRuntimeError> {
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(error_status::broker_executable)
}

#[cfg(not(windows))]
fn open_pinned(_path: &Path) -> Result<File, BrokerRuntimeError> {
    Err(BrokerRuntimeError::InvalidBrokerProcess)
}
