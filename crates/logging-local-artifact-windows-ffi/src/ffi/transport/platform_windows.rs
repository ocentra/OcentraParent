//! Windows file-handle operations for the local-artifact owner.
//!
//! This module is the only source unit that calls the Windows file ABI. The
//! parent platform module exposes it only within the native crate.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::{size_of, MaybeUninit};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;
use std::path::{Path, PathBuf};
use std::ptr;

use fs2::FileExt;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Storage::FileSystem::{
    FileAttributeTagInfo, FileBasicInfo, FileDispositionInfo, FileDispositionInfoEx, FileIdInfo,
    FileRenameInfo, FileStandardInfo, GetFileInformationByHandleEx, SetFileInformationByHandle,
    DELETE, FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_TAG_INFO, FILE_BASIC_INFO,
    FILE_DISPOSITION_FLAG_DELETE, FILE_DISPOSITION_FLAG_POSIX_SEMANTICS, FILE_DISPOSITION_INFO,
    FILE_DISPOSITION_INFO_EX, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_ID_INFO, FILE_RENAME_INFO, FILE_RENAME_INFO_0,
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_STANDARD_INFO,
};

use crate::constants::{
    CURRENT_COMPONENT, DESTINATION_NOT_COMPONENT, DESTINATION_UNSAFE,
    FILE_ATTRIBUTE_TAG_INFO_FAILURE, FILE_BASIC_INFO_FAILURE, FILE_DISPOSITION_INFO_FAILURE,
    FILE_ID_INFO_FAILURE, FILE_RENAME_INFO_FAILURE, FILE_STANDARD_INFO_FAILURE,
    NEGATIVE_FILE_LENGTH, NEGATIVE_LAST_WRITE_TIME, PARENT_COMPONENT, PATH_MUST_BE_DIRECTORY,
    PATH_MUST_BE_FILE, PROCESS_IMAGE_NOT_FILE,
};
use crate::error::{io_error, ArtifactError};

pub(crate) const MAX_COMPONENT_CHARS: usize = 255;
pub(crate) const MAX_RELATIVE_PATH_CHARS: usize = 32 * 1024;
pub(crate) const MAX_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Identity {
    pub(crate) volume_serial_number: u64,
    pub(crate) file_id: [u8; 16],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Metadata {
    pub(crate) identity: Identity,
    pub(crate) length: u64,
    pub(crate) links: u32,
    pub(crate) directory: bool,
    pub(crate) attributes: u32,
    pub(crate) reparse_tag: u32,
    pub(crate) modified_ms: i64,
}

#[derive(Debug)]
pub(crate) struct OwnedFile {
    file: File,
    path: PathBuf,
}

#[path = "platform_file_io.rs"]
mod file_io;
#[path = "platform_file_mutation.rs"]
mod file_mutation;
#[path = "platform_file_open.rs"]
mod file_open;

#[derive(Clone, Copy)]
enum OpenKind {
    File { consistent: bool },
    StagingFile,
    Directory,
    SyncDirectory,
    RenameDirectory,
    MutationDirectory,
}

pub(crate) fn verify_metadata(
    file: &OwnedFile,
    directory: bool,
) -> Result<Metadata, ArtifactError> {
    let metadata = file.metadata()?;
    if metadata.attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || metadata.reparse_tag != 0 {
        return Err(ArtifactError::LinkOrReparseDetected);
    }
    if metadata.directory != directory {
        return Err(ArtifactError::InvalidPath(if directory {
            PATH_MUST_BE_DIRECTORY
        } else {
            PATH_MUST_BE_FILE
        }));
    }
    if !directory && metadata.links != 1 {
        return Err(ArtifactError::HardlinkDetected);
    }
    Ok(metadata)
}

fn verify_process_image_metadata(file: &OwnedFile) -> Result<Metadata, ArtifactError> {
    let metadata = file.metadata()?;
    if metadata.attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || metadata.reparse_tag != 0 {
        return Err(ArtifactError::LinkOrReparseDetected);
    }
    if metadata.directory {
        return Err(ArtifactError::InvalidPath(PROCESS_IMAGE_NOT_FILE));
    }
    Ok(metadata)
}

#[path = "platform_metadata.rs"]
mod metadata;

pub(crate) fn query_metadata(handle: HANDLE) -> Result<Metadata, ArtifactError> {
    metadata::query_metadata(handle)
}

pub(crate) fn validate_leaf(name: &str) -> Result<(), ArtifactError> {
    if name.is_empty()
        || name == CURRENT_COMPONENT
        || name == PARENT_COMPONENT
        || name.contains(['\\', '/'])
    {
        return Err(ArtifactError::InvalidPath(DESTINATION_NOT_COMPONENT));
    }
    if name.encode_utf16().count() > MAX_COMPONENT_CHARS
        || name
            .chars()
            .any(|character| character == '\0' || character == ':')
    {
        return Err(ArtifactError::InvalidPath(DESTINATION_UNSAFE));
    }
    Ok(())
}

fn win32_error(prefix: &str) -> ArtifactError {
    ArtifactError::Io(format!("{prefix}: {}", std::io::Error::last_os_error()))
}
