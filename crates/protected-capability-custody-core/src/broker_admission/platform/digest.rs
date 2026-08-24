#[cfg(windows)]
use std::fs::{File, OpenOptions};
#[cfg(windows)]
use std::io::Read;
#[cfg(windows)]
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
};

#[cfg(windows)]
use super::acl;
#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn validate(executable: &File, path: &Path) -> Result<(), PlatformError> {
    let expected = read_deployment_digest(path)?;
    const MAX_EXECUTABLE_BYTES: u64 = 128 * 1024 * 1024;
    let metadata = executable
        .metadata()
        .map_err(|_| PlatformError::Unavailable)?;
    if metadata.len() == 0 || metadata.len() > MAX_EXECUTABLE_BYTES {
        return Err(PlatformError::Tampered);
    }
    let mut reader = executable
        .try_clone()
        .map_err(|_| PlatformError::Unavailable)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|_| PlatformError::Unavailable)?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(read as u64)
            .ok_or(PlatformError::Tampered)?;
        if total > MAX_EXECUTABLE_BYTES {
            return Err(PlatformError::Tampered);
        }
        hasher.update(&buffer[..read]);
    }
    if total != metadata.len() || hasher.finalize().as_slice() != expected.as_slice() {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

#[cfg(windows)]
fn read_deployment_digest(path: &Path) -> Result<[u8; 32], PlatformError> {
    let parent = path.parent().ok_or(PlatformError::InvalidAttestation)?;
    let manifest = parent.join(
        ocentra_protected_capability_custody_protocol::constants::BROKER_DIGEST_MANIFEST_NAME,
    );
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(&manifest)
        .map_err(|_| PlatformError::DeploymentRequired)?;
    let metadata = file
        .metadata()
        .map_err(|_| PlatformError::DeploymentRequired)?;
    if !metadata.is_file() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(PlatformError::Tampered);
    }
    // The manifest is installer-owned and is held open while read. ACL
    // validation is performed on that exact handle, so a path replacement
    // cannot change the authority after this point.
    acl::validate_file(&file)?;
    let mut bytes = Vec::with_capacity(33);
    file.take(33)
        .read_to_end(&mut bytes)
        .map_err(|_| PlatformError::Unavailable)?;
    if bytes.len() != 32 {
        return Err(PlatformError::Tampered);
    }
    let mut digest = [0_u8; 32];
    digest.copy_from_slice(&bytes);
    Ok(digest)
}
