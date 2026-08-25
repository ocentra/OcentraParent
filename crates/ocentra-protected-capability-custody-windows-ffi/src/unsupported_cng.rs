//! CNG/PCP mechanics fail closed on non-Windows targets.

#![cfg(not(windows))]

use crate::{Error, OwnedPcpProvider, OwnedPcpSigningKey, PcpKeyObservation, Result};

impl OwnedPcpProvider {
    pub fn open_machine() -> Result<Self> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn open_fixed_signing_key(self) -> Result<OwnedPcpSigningKey> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn create_fixed_signing_key(self) -> Result<OwnedPcpSigningKey> {
        Err(Error::UnsupportedPlatform)
    }
}

impl OwnedPcpSigningKey {
    pub fn observation(&self) -> Result<PcpKeyObservation> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn sign_sha256_digest(&self, _digest: &[u8; 32]) -> Result<Vec<u8>> {
        Err(Error::UnsupportedPlatform)
    }
}
