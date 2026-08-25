//! Non-Windows TBS methods fail closed.

#![cfg(not(windows))]

use crate::{Error, OwnedTbsContext, OwnedTpmNvCapability, Result, TpmNvEnrollment};

impl OwnedTbsContext {
    pub fn open() -> Result<OwnedTbsContext> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn is_tpm_present() -> Result<bool> {
        Err(Error::UnsupportedPlatform)
    }

    pub(crate) fn submit(&self, _command: &[u8]) -> Result<Vec<u8>> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn bind_enrolled_nv(self, _enrollment: TpmNvEnrollment) -> Result<OwnedTpmNvCapability> {
        Err(Error::UnsupportedPlatform)
    }
}

impl OwnedTpmNvCapability {
    pub fn read(&self, _authorization: &[u8], _size: u16, _offset: u16) -> Result<Vec<u8>> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn increment(&self, _authorization: &[u8]) -> Result<()> {
        Err(Error::UnsupportedPlatform)
    }
}
