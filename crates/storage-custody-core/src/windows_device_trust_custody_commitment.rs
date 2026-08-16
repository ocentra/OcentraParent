#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use std::{fs, io, path::Path};
#[cfg(windows)]
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[cfg(windows)]
use super::record::hex;
use super::Error;

#[cfg(windows)]
const DEVICE_TRUST_RECORD_COMMITMENTS_REGISTRY_PATH: &str =
    "Software\\Ocentra\\DeviceTrust\\RecordCommitments";

#[cfg(windows)]
pub(super) fn write(binding: &[u8], record_path: &Path) -> Result<(), Error> {
    let encoded = fs::read(record_path).map_err(|_error| Error::Io)?;
    commitment_key()?
        .set_value(binding_key(binding), &encoded_digest(&encoded))
        .map_err(|_error| Error::Platform)
}

#[cfg(windows)]
pub(super) fn verify(binding: &[u8], encoded: &[u8]) -> Result<(), Error> {
    let expected: String = commitment_key()?
        .get_value(binding_key(binding))
        .map_err(|error| commitment_read_error(&error))?;
    (expected == encoded_digest(encoded))
        .then_some(())
        .ok_or(Error::Mismatch)
}

#[cfg(windows)]
fn commitment_key() -> Result<RegKey, Error> {
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(DEVICE_TRUST_RECORD_COMMITMENTS_REGISTRY_PATH)
        .map_err(|_error| Error::Platform)
        .map(|(key, _disposition)| key)
}

#[cfg(windows)]
fn binding_key(binding: &[u8]) -> String {
    hex(Sha256::digest(binding))
}

#[cfg(windows)]
fn encoded_digest(encoded: &[u8]) -> String {
    hex(Sha256::digest(encoded))
}

#[cfg(windows)]
fn commitment_read_error(error: &io::Error) -> Error {
    if error.kind() == io::ErrorKind::NotFound {
        Error::Mismatch
    } else {
        Error::Platform
    }
}

#[cfg(not(windows))]
pub(super) fn write(_: &[u8], _: &std::path::Path) -> Result<(), Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn verify(_: &[u8], _: &[u8]) -> Result<(), Error> {
    Err(Error::Platform)
}

#[cfg(all(test, windows))]
#[path = "windows_device_trust_custody_commitment_tests.rs"]
mod tests;
