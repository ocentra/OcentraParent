use std::{io, path::Path};

#[cfg(not(windows))]
use std::fs::File;
#[cfg(windows)]
use std::fs::OpenOptions;

use super::super::BrowserManagedProfileStoreError;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

#[cfg(windows)]
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
#[cfg(windows)]
const FILE_SHARE_READ_WRITE_DELETE: u32 = 0x0000_0007;

pub(crate) fn replace_and_sync(
    path: &Path,
    contents: &[u8],
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (path, contents);
    // `atomicwrites` replaces by path.  Its staging-file/rename sequence
    // cannot be tied to the retained profile-root handle under this crate's
    // `forbid(unsafe_code)` boundary, so even metadata replacement would let a
    // substituted name become authoritative.  Refuse all writes until an
    // owner-issued handle-relative replacement primitive is available.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}

pub(crate) fn sync_parent_directory(path: &Path) -> Result<(), BrowserManagedProfileStoreError> {
    let parent = path
        .parent()
        .ok_or(BrowserManagedProfileStoreError::UnsafePath)?;
    sync_directory(parent).map_err(|_error| BrowserManagedProfileStoreError::Io)
}

#[cfg(not(windows))]
fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(windows)]
fn sync_directory(path: &Path) -> io::Result<()> {
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ_WRITE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?
        .sync_all()
}
