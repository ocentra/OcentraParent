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
use super::anti_rollback;
#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
#[path = "registry/io.rs"]
mod registry_io;
#[cfg(windows)]
const REGISTRY_ROOT: &str = "Software\\Ocentra\\ProtectedCapabilityCustody";
#[cfg(windows)]
const ENROLLMENT_SUBKEY: &str = "Enrollment";
#[cfg(windows)]
const RUNTIME_SUBKEY: &str = "Runtime";
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
pub(super) fn read_enrollment(
    registry_id: &str,
    name: &str,
) -> Result<Option<Vec<u8>>, PlatformError> {
    registry_io::read_enrollment(registry_id, name)
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
pub(super) struct RuntimeMutation<'a> {
    pub(super) name: &'a str,
    pub(super) value: Option<&'a [u8]>,
}

#[cfg(windows)]
pub(super) enum RuntimeBatchFailure {
    DefinitelyNotApplied(PlatformError),
    OutcomeUnknown,
}

#[cfg(windows)]
impl RuntimeBatchFailure {
    fn into_platform_error(self) -> PlatformError {
        match self {
            Self::DefinitelyNotApplied(error) => error,
            Self::OutcomeUnknown => PlatformError::Unavailable,
        }
    }
}

#[cfg(windows)]
pub(super) fn write_batch(
    registry_id: &str,
    mutations: &[RuntimeMutation<'_>],
) -> Result<(), RuntimeBatchFailure> {
    registry_io::write_batch(registry_id, mutations)
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
pub(super) fn open_runtime_read_key(registry_id: &str) -> Result<RegKey, PlatformError> {
    open_runtime_key(registry_id, KEY_READ)
}

#[cfg(windows)]
pub(super) fn open_runtime_write_key(registry_id: &str) -> Result<RegKey, PlatformError> {
    open_runtime_key(registry_id, KEY_READ | KEY_WRITE)
}

#[cfg(windows)]
fn open_runtime_key(registry_id: &str, access: u32) -> Result<RegKey, PlatformError> {
    // Mutable runtime state is separate from immutable installer enrollment.
    // Reads never request mutation rights; only the guarded batch writer opens
    // this child with KEY_WRITE. Enrollment remains a distinct read-only key.
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("{REGISTRY_ROOT}\\{registry_id}\\{RUNTIME_SUBKEY}");
    let key = root
        .open_subkey_with_flags(path, access)
        .map_err(map_io_error)?;
    acl::validate_secret_store(&key)?;
    Ok(key)
}

#[cfg(windows)]
pub(super) fn open_enrollment_key(registry_id: &str) -> Result<RegKey, PlatformError> {
    // Installer/SCM provisioning owns this key. The broker may read it, but
    // runtime state code has no write handle to the enrollment child.
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("{REGISTRY_ROOT}\\{registry_id}\\{ENROLLMENT_SUBKEY}");
    let key = root
        .open_subkey_with_flags(path, KEY_READ)
        .map_err(map_io_error)?;
    acl::validate_enrollment_store(&key)?;
    Ok(key)
}

#[cfg(windows)]
pub(super) fn verify_runtime_snapshot(
    registry_id: &str,
    key: &RegKey,
) -> Result<(), PlatformError> {
    anti_rollback::verify_runtime_snapshot(registry_id, key)
}

#[cfg(windows)]
pub(super) fn authorize_runtime_batch<'a>(
    registry_id: &str,
    key: &RegKey,
    mutations: &[RuntimeMutation<'a>],
) -> Result<anti_rollback::MutationPermit, RuntimeBatchFailure> {
    anti_rollback::authorize_runtime_batch(registry_id, key, mutations)
}

#[cfg(windows)]
pub(super) fn confirm_runtime_batch(
    registry_id: &str,
    key: &RegKey,
    permit: anti_rollback::MutationPermit,
) -> Result<(), PlatformError> {
    anti_rollback::confirm_runtime_batch(registry_id, key, permit)
}

#[cfg(windows)]
pub(super) fn map_io_error(_error: std::io::Error) -> PlatformError {
    PlatformError::Unavailable
}
