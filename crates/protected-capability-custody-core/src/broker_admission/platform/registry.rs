#[cfg(windows)]
use std::os::windows::prelude::OsStrExt;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
use super::acl;
#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
#[path = "registry/io.rs"]
mod registry_io;
#[cfg(windows)]
#[path = "registry/limits.rs"]
mod registry_limits;

#[cfg(windows)]
const REGISTRY_ROOT: &str = "Software\\Ocentra\\ProtectedCapabilityCustody";
#[cfg(windows)]
const REGISTRY_PATH_DOMAIN: &[u8] = b"ocentra.pcc.registry-path.v1";

#[cfg(windows)]
pub(super) fn registry_id(path: &Path) -> Result<String, PlatformError> {
    if !path.is_absolute() {
        return Err(PlatformError::InvalidAttestation);
    }
    let mut canonical = Vec::new();
    for value in path.as_os_str().encode_wide() {
        canonical.extend_from_slice(&value.to_be_bytes());
    }
    let mut digest = Sha256::new();
    digest.update(REGISTRY_PATH_DOMAIN);
    digest.update((canonical.len() as u32).to_be_bytes());
    digest.update(&canonical);
    Ok(hex(&digest.finalize()))
}

#[cfg(windows)]
pub(super) fn read(registry_id: &str, name: &str) -> Result<Option<Vec<u8>>, PlatformError> {
    registry_io::read(registry_id, name)
}

#[cfg(windows)]
pub(super) fn write(registry_id: &str, name: &str, value: &[u8]) -> Result<(), PlatformError> {
    registry_io::write(registry_id, name, value)
}

#[cfg(windows)]
pub(super) fn delete(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    registry_io::delete(registry_id, name)
}

#[cfg(windows)]
pub(super) fn count_values_with_prefix(
    registry_id: &str,
    prefix: &str,
    limit: usize,
) -> Result<usize, PlatformError> {
    registry_limits::count_values_with_prefix(registry_id, prefix, limit)
}

#[cfg(windows)]
pub(super) fn hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
        encoded.push(char::from_digit(u32::from(byte & 0x0f), 16).unwrap_or('0'));
    }
    encoded
}

#[cfg(windows)]
pub(super) fn open_key(registry_id: &str) -> Result<RegKey, PlatformError> {
    // This key is provisioned by the dedicated broker/service installer.  A
    // client or ordinary user must never be able to create the authority
    // store on demand, so opening a missing key fails closed.
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("{REGISTRY_ROOT}\\{registry_id}");
    let key = root
        .open_subkey_with_flags(path, KEY_READ | KEY_WRITE)
        .map_err(map_io_error)?;
    acl::validate_secret_store(&key)?;
    Ok(key)
}

#[cfg(windows)]
pub(super) fn map_io_error(_error: std::io::Error) -> PlatformError {
    PlatformError::Unavailable
}
