//! Owned provider handle acquired from the fixed AccountIssuer key.

#![cfg(windows)]

use crate::{Error, Result};
use windows_sys::Win32::Security::Cryptography::{
    NCryptFreeObject, NCRYPT_KEY_HANDLE, NCRYPT_PROVIDER_HANDLE_PROPERTY, NCRYPT_PROV_HANDLE,
    NCRYPT_SILENT_FLAG,
};

/// `NCRYPT_PROVIDER_HANDLE_PROPERTY` returns an acquired provider handle. This
/// non-Clone owner releases that acquisition exactly once and never treats its
/// numeric value as provider identity.
pub(super) struct OwnedObservedProvider {
    handle: NCRYPT_PROV_HANDLE,
}

impl OwnedObservedProvider {
    pub(super) fn handle(&self) -> NCRYPT_PROV_HANDLE {
        self.handle
    }
}

pub(super) fn from_key(key: NCRYPT_KEY_HANDLE) -> Result<OwnedObservedProvider> {
    let value = super::cng_account_issuer_p256_security::get_property(
        key,
        NCRYPT_PROVIDER_HANDLE_PROPERTY,
        NCRYPT_SILENT_FLAG,
    )?;
    if value.len() != core::mem::size_of::<NCRYPT_PROV_HANDLE>() {
        return Err(Error::CryptoPropertyViolation);
    }
    let mut bytes = [0_u8; core::mem::size_of::<NCRYPT_PROV_HANDLE>()];
    bytes.copy_from_slice(&value);
    let handle = NCRYPT_PROV_HANDLE::from_le_bytes(bytes);
    if handle == 0 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(OwnedObservedProvider { handle })
}

impl Drop for OwnedObservedProvider {
    fn drop(&mut self) {
        let handle = core::mem::replace(&mut self.handle, 0);
        if handle != 0 {
            unsafe { NCryptFreeObject(handle) };
        }
    }
}
