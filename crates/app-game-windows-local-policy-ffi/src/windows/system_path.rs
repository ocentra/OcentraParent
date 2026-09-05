use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;

use crate::{error::AppGameWindowsLocalPolicyError, Result};

const MAX_SYSTEM_PATH_CHARS: usize = 32 * 1024;
const POWERSHELL_DIRECTORY_NAME: &str = "WindowsPowerShell";
const POWERSHELL_VERSION_DIRECTORY_NAME: &str = "v1.0";
const POWERSHELL_MODULE_DIRECTORY_NAME: &str = "Modules";
const POWERSHELL_EXECUTABLE_NAME: &str = "powershell.exe";

pub(super) struct TrustedPaths {
    pub(super) system_directory: PathBuf,
    pub(super) powershell_directory: PathBuf,
    pub(super) version_directory: PathBuf,
    pub(super) executable: PathBuf,
    pub(super) system_root: PathBuf,
    pub(super) module_path: PathBuf,
}

pub(super) fn trusted_paths() -> Result<TrustedPaths> {
    let mut buffer = vec![0u16; MAX_SYSTEM_PATH_CHARS];
    let length = unsafe {
        GetSystemDirectoryW(
            buffer.as_mut_ptr(),
            u32::try_from(buffer.len()).map_err(|_size_error| {
                AppGameWindowsLocalPolicyError::SystemDirectoryUnavailable
            })?,
        )
    };
    if length == 0 {
        let error = unsafe { GetLastError() };
        return Err(if error == 0 {
            AppGameWindowsLocalPolicyError::SystemDirectoryUnavailable
        } else {
            AppGameWindowsLocalPolicyError::WindowsApi(error)
        });
    }
    let length = usize::try_from(length)
        .map_err(|_size_error| AppGameWindowsLocalPolicyError::SystemDirectoryUnavailable)?;
    if length >= buffer.len() {
        return Err(AppGameWindowsLocalPolicyError::SystemDirectoryUnavailable);
    }
    let system_directory = PathBuf::from(OsString::from_wide(&buffer[..length]));
    let system_root = system_directory
        .parent()
        .ok_or(AppGameWindowsLocalPolicyError::SystemDirectoryUnavailable)?
        .to_path_buf();
    let powershell_directory = system_directory.join(POWERSHELL_DIRECTORY_NAME);
    let version_directory = powershell_directory.join(POWERSHELL_VERSION_DIRECTORY_NAME);
    Ok(TrustedPaths {
        module_path: version_directory.join(POWERSHELL_MODULE_DIRECTORY_NAME),
        executable: version_directory.join(POWERSHELL_EXECUTABLE_NAME),
        system_directory,
        powershell_directory,
        version_directory,
        system_root,
    })
}
