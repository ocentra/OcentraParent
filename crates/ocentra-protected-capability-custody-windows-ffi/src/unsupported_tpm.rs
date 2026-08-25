//! Non-Windows TBS methods fail closed.

#![cfg(not(windows))]

use crate::{Error, NvPublic, OwnedTbsContext, OwnedTpmNvIndex, Result, TpmNvIndex};

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

    pub fn open_nv_index(self, _index: TpmNvIndex) -> Result<OwnedTpmNvIndex> {
        Err(Error::UnsupportedPlatform)
    }
}

impl OwnedTpmNvIndex {
    pub fn read_public(&self) -> Result<NvPublic> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn read(&self, _authorization: &[u8], _size: u16, _offset: u16) -> Result<Vec<u8>> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn increment(&self, _authorization: &[u8]) -> Result<()> {
        Err(Error::UnsupportedPlatform)
    }
}
