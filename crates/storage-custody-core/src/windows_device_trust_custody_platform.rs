use sha2::{Digest, Sha256};

use super::{hex, Error};

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
        .create_subkey("Software\\Ocentra\\DeviceTrust\\Epochs")
        .map_err(|_error| Error::Platform)?
        .0;
    key.set_value(hex(Sha256::digest(binding)), &hex(protect(epoch, binding)?))
        .map_err(|_error| Error::Platform)
}

#[cfg(windows)]
pub(super) fn current(binding: &[u8]) -> Result<Vec<u8>, Error> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Ocentra\\DeviceTrust\\Epochs")
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

    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags("Software\\Ocentra\\DeviceTrust\\Epochs", KEY_WRITE)
        .map_err(|_error| Error::Missing)?
        .delete_value(hex(Sha256::digest(binding)))
        .map_err(|_error| Error::Missing)
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

fn decode_hex(value: &str) -> Result<Vec<u8>, Error> {
    if !value.len().is_multiple_of(2) {
        return Err(Error::Missing);
    }
    (0..value.len())
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(&value[index..index + 2], 16).map_err(|_error| Error::Missing)
        })
        .collect()
}
