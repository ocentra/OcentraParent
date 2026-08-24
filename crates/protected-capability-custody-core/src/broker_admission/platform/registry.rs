#[cfg(windows)]
use std::borrow::Cow;
#[cfg(windows)]
use std::io;
#[cfg(windows)]
use std::os::windows::prelude::OsStrExt;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, REG_BINARY};
#[cfg(windows)]
use winreg::{RegKey, RegValue};

#[cfg(windows)]
use super::acl;
#[cfg(windows)]
use crate::platform::PlatformError;

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
    let key = open_key(registry_id)?;
    match key.get_raw_value(name) {
        Ok(value) if value.vtype == REG_BINARY => Ok(Some(value.bytes.into_owned())),
        Ok(_) => Err(PlatformError::Tampered),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(map_io_error(error)),
    }
}

#[cfg(windows)]
pub(super) fn write(registry_id: &str, name: &str, value: &[u8]) -> Result<(), PlatformError> {
    let key = open_key(registry_id)?;
    key.set_raw_value(
        name,
        &RegValue {
            bytes: Cow::Borrowed(value),
            vtype: REG_BINARY,
        },
    )
    .map_err(map_io_error)
}

#[cfg(windows)]
pub(super) fn delete(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    let key = open_key(registry_id)?;
    match key.delete_value(name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(map_io_error(error)),
    }
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
fn open_key(registry_id: &str) -> Result<RegKey, PlatformError> {
    let root = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!("{REGISTRY_ROOT}\\{registry_id}");
    let key = root
        .create_subkey(path)
        .map(|(key, _disposition)| key)
        .map_err(map_io_error)?;
    acl::validate_registry(&key)?;
    Ok(key)
}

#[cfg(windows)]
fn map_io_error(_error: io::Error) -> PlatformError {
    PlatformError::Unavailable
}
