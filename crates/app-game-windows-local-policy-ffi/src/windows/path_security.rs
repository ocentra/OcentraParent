use std::fs::{File, OpenOptions};
use std::mem::{size_of, zeroed};
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;
use std::path::{Path, PathBuf};

use windows_sys::Win32::Foundation::{GetLastError, GENERIC_READ, HANDLE};
use windows_sys::Win32::Storage::FileSystem::{
    FileAttributeTagInfo, FileIdInfo, GetFileInformationByHandleEx, FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_TAG_INFO, FILE_FLAG_BACKUP_SEMANTICS,
    FILE_FLAG_OPEN_REPARSE_POINT, FILE_ID_INFO, FILE_LIST_DIRECTORY, FILE_READ_ATTRIBUTES,
    FILE_SHARE_READ, FILE_SHARE_WRITE, READ_CONTROL,
};

use super::security;
use super::system_path::TrustedPaths;
use crate::{error::AppGameWindowsLocalPolicyError, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileIdentity {
    volume_serial_number: u64,
    file_id: [u8; 16],
}

struct HeldPath {
    file: File,
    identity: FileIdentity,
    is_directory: bool,
}

pub(super) struct TrustedPowerShell {
    executable: PathBuf,
    system_directory: PathBuf,
    system_root: PathBuf,
    module_path: PathBuf,
    held_paths: Vec<HeldPath>,
}

impl TrustedPowerShell {
    pub(super) fn open(paths: TrustedPaths) -> Result<Self> {
        let held_paths = vec![
            open_held(&paths.system_directory, true)?,
            open_held(&paths.powershell_directory, true)?,
            open_held(&paths.version_directory, true)?,
            open_held(&paths.executable, false)?,
        ];
        Ok(Self {
            executable: paths.executable,
            system_directory: paths.system_directory,
            system_root: paths.system_root,
            module_path: paths.module_path,
            held_paths,
        })
    }

    pub(super) fn executable(&self) -> &Path {
        &self.executable
    }

    pub(super) fn system_directory(&self) -> &Path {
        &self.system_directory
    }

    pub(super) fn system_root(&self) -> &Path {
        &self.system_root
    }

    pub(super) fn module_path(&self) -> &Path {
        &self.module_path
    }

    pub(super) fn verify_current(&self) -> Result<()> {
        for held in &self.held_paths {
            verify_handle(held)?;
            if identity(&held.file)? != held.identity {
                return Err(AppGameWindowsLocalPolicyError::TrustedExecutableChanged);
            }
            security::verify_owner_and_acl(&held.file)?;
        }
        Ok(())
    }
}

fn open_held(path: &Path, is_directory: bool) -> Result<HeldPath> {
    let mut options = OpenOptions::new();
    options
        .access_mode(if is_directory {
            FILE_LIST_DIRECTORY | FILE_READ_ATTRIBUTES | READ_CONTROL
        } else {
            GENERIC_READ | READ_CONTROL
        })
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
    let file = options.open(path).map_err(|error| {
        error
            .raw_os_error()
            .and_then(|value| u32::try_from(value).ok())
            .map_or(
                AppGameWindowsLocalPolicyError::TrustedExecutableUnavailable,
                AppGameWindowsLocalPolicyError::WindowsApi,
            )
    })?;
    let held = HeldPath {
        identity: identity(&file)?,
        file,
        is_directory,
    };
    verify_handle(&held)?;
    security::verify_owner_and_acl(&held.file)?;
    Ok(held)
}

fn verify_handle(held: &HeldPath) -> Result<()> {
    let mut tag: FILE_ATTRIBUTE_TAG_INFO = unsafe { zeroed() };
    let ok = unsafe {
        GetFileInformationByHandleEx(
            held.file.as_raw_handle() as HANDLE,
            FileAttributeTagInfo,
            (&mut tag as *mut FILE_ATTRIBUTE_TAG_INFO).cast(),
            u32::try_from(size_of::<FILE_ATTRIBUTE_TAG_INFO>()).map_err(|_size_error| {
                AppGameWindowsLocalPolicyError::TrustedExecutableUnavailable
            })?,
        )
    };
    if ok == 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(unsafe {
            GetLastError()
        }));
    }
    if tag.FileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || tag.ReparseTag != 0 {
        return Err(AppGameWindowsLocalPolicyError::ReparsePointRejected);
    }
    let observed_directory = tag.FileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0;
    if observed_directory != held.is_directory {
        return Err(AppGameWindowsLocalPolicyError::TrustedExecutableUnavailable);
    }
    Ok(())
}

fn identity(file: &File) -> Result<FileIdentity> {
    let mut info: FILE_ID_INFO = unsafe { zeroed() };
    let ok = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle() as HANDLE,
            FileIdInfo,
            (&mut info as *mut FILE_ID_INFO).cast(),
            u32::try_from(size_of::<FILE_ID_INFO>()).map_err(|_size_error| {
                AppGameWindowsLocalPolicyError::TrustedExecutableUnavailable
            })?,
        )
    };
    if ok == 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(unsafe {
            GetLastError()
        }));
    }
    Ok(FileIdentity {
        volume_serial_number: info.VolumeSerialNumber,
        file_id: info.FileId.Identifier,
    })
}
