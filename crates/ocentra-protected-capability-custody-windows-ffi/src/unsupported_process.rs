//! Non-Windows process and token methods fail closed.

#![cfg(not(windows))]

use crate::{Error, OwnedProcess, OwnedToken, ProcessObservation, Result, TokenObservation};

impl OwnedProcess {
    pub fn open_for_peer_observation(_process_id: u32) -> Result<OwnedProcess> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn observation(&self) -> Result<ProcessObservation> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn open_token(&self) -> Result<OwnedToken> {
        Err(Error::UnsupportedPlatform)
    }
}

impl OwnedToken {
    pub fn open_current_thread() -> Result<OwnedToken> {
        Err(Error::UnsupportedPlatform)
    }

    pub fn observation(&self) -> Result<TokenObservation> {
        Err(Error::UnsupportedPlatform)
    }
}
