#[cfg(windows)]
use getrandom::fill;
#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use std::io;
use std::path::Path;

#[cfg(windows)]
use super::record::hex;
use super::Error;

#[cfg(windows)]
const DEVICE_TRUST_EPOCHS_REGISTRY_PATH: &str = "Software\\Ocentra\\DeviceTrust\\Epochs";
#[cfg(windows)]
const DEVICE_TRUST_INSTALL_GENERATIONS_REGISTRY_PATH: &str =
    "Software\\Ocentra\\DeviceTrust\\InstallGenerations";

#[cfg(windows)]
pub(super) fn load_or_rotate_install_generation(
    root: &Path,
    root_was_absent: bool,
) -> Result<String, Error> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(DEVICE_TRUST_INSTALL_GENERATIONS_REGISTRY_PATH)
        .map_err(|_error| Error::Platform)?
        .0;
    let root_key = hex(Sha256::digest(root.to_string_lossy().as_bytes()));
    let identity = root_identity(root)?;
    if root_was_absent {
        let generation = fresh_install_generation()?;
        key.set_value(&root_key, &format!("{identity}|{generation}"))
            .map_err(|_error| Error::Platform)?;
        return Ok(generation);
    }
    match key.get_value::<String, _>(&root_key) {
        Ok(anchor) => anchor
            .split_once('|')
            .filter(|(stored_identity, generation)| {
                stored_identity == &identity
                    && generation.len() == 64
                    && generation.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            .map(|(_identity, generation)| generation.to_owned())
            .map(Ok)
            .unwrap_or_else(|| rotate_install_generation(&key, &root_key, &identity)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            rotate_install_generation(&key, &root_key, &identity)
        }
        Err(_error) => Err(Error::Platform),
    }
}

#[cfg(windows)]
fn rotate_install_generation(
    key: &winreg::RegKey,
    root_key: &str,
    identity: &str,
) -> Result<String, Error> {
    let generation = fresh_install_generation()?;
    key.set_value(root_key, &format!("{identity}|{generation}"))
        .map_err(|_error| Error::Platform)?;
    Ok(generation)
}

#[cfg(windows)]
fn root_identity(root: &Path) -> Result<String, Error> {
    let metadata = root.metadata().map_err(|_error| Error::Io)?;
    let created = metadata
        .created()
        .map_err(|_error| Error::Io)?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_error| Error::Io)?;
    Ok(hex(Sha256::digest(format!(
        "{}:{}",
        root.to_string_lossy(),
        created.as_nanos()
    ))))
}

#[cfg(windows)]
fn fresh_install_generation() -> Result<String, Error> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| Error::Platform)?;
    Ok(hex(bytes))
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
pub(super) fn load_or_rotate_install_generation(_: &Path, _: bool) -> Result<String, Error> {
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
