#[cfg(windows)]
use std::io;

#[cfg(windows)]
use winreg::enums::REG_BINARY;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn runtime(registry_id: &str, name: &str) -> Result<Option<Vec<u8>>, PlatformError> {
    let key = super::super::open_runtime_read_key(registry_id)?;
    super::super::verify_runtime_snapshot(registry_id, &key)?;
    from_key(&key, name)
}

#[cfg(windows)]
fn from_key(key: &winreg::RegKey, name: &str) -> Result<Option<Vec<u8>>, PlatformError> {
    match key.get_raw_value(name) {
        Ok(value) if value.vtype == REG_BINARY => {
            validate_value(value.bytes.into_owned()).map(Some)
        }
        Ok(_) => Err(PlatformError::Tampered),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(super::super::map_io_error(error)),
    }
}

#[cfg(windows)]
fn validate_value(value: Vec<u8>) -> Result<Vec<u8>, PlatformError> {
    if value.len()
        > ocentra_protected_capability_custody_protocol::constants::MAX_REGISTRY_VALUE_BYTES
    {
        Err(PlatformError::Tampered)
    } else {
        Ok(value)
    }
}
