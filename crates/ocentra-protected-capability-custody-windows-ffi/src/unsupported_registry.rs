//! Non-Windows registry methods fail closed.

#![cfg(not(windows))]

use crate::{
    Error, OwnedRegistryChain, RegistryAncestorObservation, RegistryPath, RegistryValueName,
    RegistryValueObservation, Result, SecurityDescriptorObservation,
};

impl OwnedRegistryChain {
    pub fn open_hklm(_path: &RegistryPath) -> Result<OwnedRegistryChain> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn observe_value(&self, _name: &RegistryValueName) -> Result<RegistryValueObservation> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn reobserve_value(
        &self,
        _previous: &RegistryValueObservation,
    ) -> Result<RegistryValueObservation> {
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
