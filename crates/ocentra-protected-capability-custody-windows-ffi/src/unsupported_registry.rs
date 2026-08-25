//! Non-Windows registry methods fail closed.

#![cfg(not(windows))]

use crate::{
    Error, OwnedRegistryChain, RegistryAncestorObservation, RegistryValue, Result,
    SecurityDescriptorObservation,
};

impl OwnedRegistryChain {
    pub fn open_hklm(_path: &str) -> Result<OwnedRegistryChain> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn read_value(&self, _name: &str) -> Result<RegistryValue> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn security(&self) -> Result<SecurityDescriptorObservation> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn observations(&self) -> Result<&[RegistryAncestorObservation]> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn revalidate(&self) -> Result<()> {
        Err(Error::UnsupportedPlatform)
    }
}
