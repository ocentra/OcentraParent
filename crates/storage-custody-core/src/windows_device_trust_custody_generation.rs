use getrandom::fill;
use sha2::{Digest, Sha256};
use std::{io, path::Path};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use super::super::{active_record, record::hex, Error};

const DEVICE_TRUST_INSTALL_GENERATIONS_REGISTRY_PATH: &str =
    "Software\\Ocentra\\DeviceTrust\\InstallGenerations";
const INSTALL_GENERATION_EMPTY: &str = "empty";
const INSTALL_GENERATION_SEALED: &str = "sealed";

pub(super) fn load_or_rotate(root: &Path, root_was_absent: bool) -> Result<String, Error> {
    let key = install_generation_key()?;
    let root_key = hex(Sha256::digest(root.to_string_lossy().as_bytes()));
    let identity = root_identity(root)?;
    if root_was_absent {
        return rotate(&key, &root_key, &identity);
    }
    match key.get_value::<String, _>(&root_key) {
        Ok(anchor) => anchor_generation(&anchor, &identity, root)?
            .map(ToOwned::to_owned)
            .map(Ok)
            .unwrap_or_else(|| rotate(&key, &root_key, &identity)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => rotate(&key, &root_key, &identity),
        Err(_error) => Err(Error::Platform),
    }
}

pub(super) fn mark_sealed(root: &Path, generation: &str, binding: &[u8]) -> Result<(), Error> {
    let key = install_generation_key()?;
    let root_key = hex(Sha256::digest(root.to_string_lossy().as_bytes()));
    let identity = root_identity(root)?;
    key.set_value(
        &root_key,
        &format!(
            "{identity}|{generation}|{INSTALL_GENERATION_SEALED}|{}",
            hex(binding)
        ),
    )
    .map_err(|_error| Error::Platform)
}

fn install_generation_key() -> Result<RegKey, Error> {
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(DEVICE_TRUST_INSTALL_GENERATIONS_REGISTRY_PATH)
        .map_err(|_error| Error::Platform)
        .map(|(key, _disposition)| key)
}

fn anchor_generation<'a>(
    anchor: &'a str,
    identity: &str,
    root: &Path,
) -> Result<Option<&'a str>, Error> {
    let mut parts = anchor.split('|');
    let (stored_identity, generation, state, binding_hex, absent_tail) = (
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
    );
    let valid_generation = stored_identity == Some(identity)
        && generation.is_some_and(|generation| generation.len() == 64)
        && generation
            .is_some_and(|generation| generation.bytes().all(|byte| byte.is_ascii_hexdigit()))
        && absent_tail.is_none();
    if !valid_generation {
        return Ok(None);
    }
    match (state, binding_hex, generation) {
        (Some(INSTALL_GENERATION_EMPTY), None, Some(generation)) => Ok(Some(generation)),
        (Some(INSTALL_GENERATION_SEALED), Some(binding_hex), Some(generation)) => {
            active_record::is_present(root, binding_hex)
                .map(|present| present.then_some(generation))
        }
        _ => Ok(None),
    }
}

fn rotate(key: &RegKey, root_key: &str, identity: &str) -> Result<String, Error> {
    let generation = fresh()?;
    key.set_value(
        root_key,
        &format!("{identity}|{generation}|{INSTALL_GENERATION_EMPTY}"),
    )
    .map_err(|_error| Error::Platform)?;
    Ok(generation)
}

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

fn fresh() -> Result<String, Error> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| Error::Platform)?;
    Ok(hex(bytes))
}
