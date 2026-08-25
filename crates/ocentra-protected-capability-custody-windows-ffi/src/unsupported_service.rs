//! Non-Windows SCM methods fail closed.

#![cfg(not(windows))]

use crate::{Error, OwnedScManager, OwnedService, Result, ServiceObservation};

impl OwnedScManager {
    pub fn open() -> Result<OwnedScManager> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn open_service(&self, _name: &str) -> Result<OwnedService> {
        Err(Error::UnsupportedPlatform)
    }
}

impl OwnedService {
    pub fn observation(&self) -> Result<ServiceObservation> {
        Err(Error::UnsupportedPlatform)
    }
}
