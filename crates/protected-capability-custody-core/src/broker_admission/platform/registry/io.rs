#[cfg(windows)]
use std::borrow::Cow;
#[cfg(windows)]
use std::io;

#[cfg(windows)]
use winreg::enums::REG_BINARY;
#[cfg(windows)]
use winreg::RegValue;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn read(registry_id: &str, name: &str) -> Result<Option<Vec<u8>>, PlatformError> {
    let key = super::open_key(registry_id)?;
    match key.get_raw_value(name) {
        Ok(value) if value.vtype == REG_BINARY => {
            if value.bytes.len()
                > ocentra_protected_capability_custody_protocol::constants::MAX_REGISTRY_VALUE_BYTES
            {
                return Err(PlatformError::Tampered);
            }
            Ok(Some(value.bytes.into_owned()))
        }
        Ok(_) => Err(PlatformError::Tampered),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(super::map_io_error(error)),
    }
}

#[cfg(windows)]
pub(super) fn write(registry_id: &str, name: &str, value: &[u8]) -> Result<(), PlatformError> {
    if value.len()
        > ocentra_protected_capability_custody_protocol::constants::MAX_REGISTRY_VALUE_BYTES
    {
        return Err(PlatformError::InvalidAttestation);
    }
    let key = super::open_key(registry_id)?;
    key.set_raw_value(
        name,
        &RegValue {
            bytes: Cow::Borrowed(value),
            vtype: REG_BINARY,
        },
    )
    .map_err(super::map_io_error)
}

#[cfg(windows)]
pub(super) fn delete(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    let key = super::open_key(registry_id)?;
    match key.delete_value(name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(super::map_io_error(error)),
    }
}
