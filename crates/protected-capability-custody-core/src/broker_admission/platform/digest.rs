#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::io::Read;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use crate::platform::PlatformError;
#[cfg(windows)]
use sha2::{Digest, Sha256};

#[cfg(windows)]
pub(super) fn validate(executable: &File, path: &Path) -> Result<(), PlatformError> {
    let expected = read_deployment_digest(path)?;
    if compute(executable)? != expected {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn compute(executable: &File) -> Result<[u8; 32], PlatformError> {
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
    if total != metadata.len() {
        return Err(PlatformError::Tampered);
    }
    Ok(hasher.finalize().into())
}

#[cfg(windows)]
fn read_deployment_digest(path: &Path) -> Result<[u8; 32], PlatformError> {
    // A sibling file is mutable deployment input and cannot be an authority
    // manifest. The installer must provision this digest in the SYSTEM-only
    // registry key derived from the fixed executable path.
    let registry_id = super::registry::registry_id(path)?;
    let bytes = super::registry::read_enrollment(
        &registry_id,
        ocentra_protected_capability_custody_protocol::constants::BROKER_DIGEST_VALUE_NAME,
    )?
    .ok_or(PlatformError::DeploymentRequired)?;
    if bytes.len() != 32 {
        return Err(PlatformError::Tampered);
    }
    let mut digest = [0_u8; 32];
    digest.copy_from_slice(&bytes);
    Ok(digest)
}
