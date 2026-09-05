//! RAII ownership for CNG provider and key handles.

#![cfg(windows)]

use windows_sys::Win32::Security::Cryptography::{
    NCryptFreeObject, NCRYPT_KEY_HANDLE, NCRYPT_PROV_HANDLE,
};

pub(crate) struct PcpProviderInner {
    pub(crate) handle: NCRYPT_PROV_HANDLE,
}

pub(crate) struct PcpSigningKeyInner {
    pub(crate) handle: NCRYPT_KEY_HANDLE,
    // The provider is deliberately retained for the entire key lifetime.
    pub(crate) _provider: PcpProviderInner,
}

impl Drop for PcpProviderInner {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe { NCryptFreeObject(self.handle) };
        }
    }
}

impl Drop for PcpSigningKeyInner {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe { NCryptFreeObject(self.handle) };
        }
    }
}
