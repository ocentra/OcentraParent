//! Non-Windows TBS methods fail closed.

#![cfg(not(windows))]

use crate::{Error, OwnedTbsContext, Result, TpmNvPublicObservation};

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

    pub fn observe_nv_public(&self, _index: u32) -> Result<TpmNvPublicObservation> {
        Err(Error::UnsupportedPlatform)
    }
}
