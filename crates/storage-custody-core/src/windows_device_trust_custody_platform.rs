#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use std::io;
use std::path::Path;

#[cfg(windows)]
use super::record::hex;
use super::Error;

#[cfg(windows)]
#[path = "windows_device_trust_custody_generation.rs"]
mod generation;

#[cfg(windows)]
const DEVICE_TRUST_EPOCHS_REGISTRY_PATH: &str = "Software\\Ocentra\\DeviceTrust\\Epochs";
#[cfg(windows)]
pub(super) fn load_or_rotate_install_generation(
    root: &Path,
    root_was_absent: bool,
    sealed_content_present: bool,
) -> Result<String, Error> {
    generation::load_or_rotate(root, root_was_absent, sealed_content_present)
}

#[cfg(windows)]
pub(super) fn mark_install_generation_sealed(root: &Path, generation: &str) -> Result<(), Error> {
    generation::mark_sealed(root, generation)
}

#[cfg(windows)]
pub(super) fn protect(value: &[u8], binding: &[u8]) -> Result<Vec<u8>, Error> {
    windows_dpapi::encrypt_data(value, windows_dpapi::Scope::User, Some(binding))
        .map_err(|_error| Error::Platform)
}

#[cfg(windows)]
pub(super) fn unprotect(value: &[u8], binding: &[u8]) -> Result<Vec<u8>, Error> {
    windows_dpapi::decrypt_data(value, windows_dpapi::Scope::User, Some(binding))
        .map_err(|_error| Error::Unseal)
}

#[cfg(windows)]
pub(super) fn activate(binding: &[u8], epoch: &[u8]) -> Result<(), Error> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(DEVICE_TRUST_EPOCHS_REGISTRY_PATH)
        .map_err(|_error| Error::Platform)?
        .0;
    key.set_value(hex(Sha256::digest(binding)), &hex(protect(epoch, binding)?))
        .map_err(|_error| Error::Platform)
}

#[cfg(windows)]
pub(super) fn current(binding: &[u8]) -> Result<Vec<u8>, Error> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(DEVICE_TRUST_EPOCHS_REGISTRY_PATH)
        .map_err(|_error| Error::Missing)?;
    let protected_epoch: String = key
        .get_value(hex(Sha256::digest(binding)))
        .map_err(|_error| Error::Missing)?;
    unprotect(&decode_hex(&protected_epoch)?, binding)
}

#[cfg(windows)]
pub(super) fn remove(binding: &[u8]) -> Result<(), Error> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, KEY_WRITE},
        RegKey,
    };

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(DEVICE_TRUST_EPOCHS_REGISTRY_PATH, KEY_WRITE)
        .map_err(|error| registry_open_error(&error))?;
    let result = key.delete_value(hex(Sha256::digest(binding)));
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Err(Error::Missing),
        Err(_error) => Err(Error::Platform),
    }
}

#[cfg(windows)]
fn registry_open_error(error: &io::Error) -> Error {
    if error.kind() == io::ErrorKind::NotFound {
        Error::Missing
    } else {
        Error::Platform
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::{registry_open_error, Error};
    use std::io;

    #[test]
    fn registry_key_open_only_swallows_not_found() {
        assert_eq!(
            registry_open_error(&io::Error::from(io::ErrorKind::NotFound)),
            Error::Missing
        );
        assert_eq!(
            registry_open_error(&io::Error::from(io::ErrorKind::PermissionDenied)),
            Error::Platform
        );
    }
}

#[cfg(not(windows))]
pub(super) fn protect(_: &[u8], _: &[u8]) -> Result<Vec<u8>, Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn unprotect(_: &[u8], _: &[u8]) -> Result<Vec<u8>, Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn activate(_: &[u8], _: &[u8]) -> Result<(), Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn current(_: &[u8]) -> Result<Vec<u8>, Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn remove(_: &[u8]) -> Result<(), Error> {
    Err(Error::Platform)
}

#[cfg(not(windows))]
pub(super) fn load_or_rotate_install_generation(
    _: &Path,
    _: bool,
    _: bool,
) -> Result<String, Error> {
    Err(Error::Platform)
}

#[cfg(windows)]
fn decode_hex(value: &str) -> Result<Vec<u8>, Error> {
    let bytes = value.as_bytes();
    bytes
        .len()
        .is_multiple_of(2)
        .then_some(())
        .ok_or(Error::Missing)?;
    bytes
        .chunks_exact(2)
        .map(|pair| {
            std::str::from_utf8(pair)
                .map_err(|_error| Error::Missing)
                .and_then(|pair| u8::from_str_radix(pair, 16).map_err(|_error| Error::Missing))
        })
        .collect()
}
