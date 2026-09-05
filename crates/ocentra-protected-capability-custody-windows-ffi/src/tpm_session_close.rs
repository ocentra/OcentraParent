//! Owned TPM handle closure and terminal-state transitions.

use super::super::codec_types::handles::{SessionHandle, TransientHandle};
use super::flush::{flush_handle, FlushOutcome};
use super::lifetimes::{OwnedTpmSession, OwnedTransientObject};
use crate::Result;

pub(super) enum SessionHandleState {
    Active(SessionHandle),
    Terminated,
    Abandoned,
}

pub(super) enum TransientHandleState {
    Active(TransientHandle),
    Terminated,
    Abandoned,
}

impl OwnedTpmSession<'_> {
    pub(super) fn close_in_place(&mut self) -> Result<()> {
        let SessionHandleState::Active(handle) = &self.handle else {
            return Ok(());
        };
        match flush_handle(self.context, handle.raw()) {
            FlushOutcome::Flushed => self.handle = SessionHandleState::Terminated,
            FlushOutcome::Rejected(error) => return Err(error),
            FlushOutcome::Uncertain(error) => {
                self.handle = SessionHandleState::Abandoned;
                return Err(error);
            }
        }
        Ok(())
    }
}

impl Drop for OwnedTpmSession<'_> {
    fn drop(&mut self) {
        let _ = self.close_in_place();
    }
}

impl OwnedTransientObject<'_> {
    pub(super) fn close_in_place(&mut self) -> Result<()> {
        let TransientHandleState::Active(handle) = &self.handle else {
            return Ok(());
        };
        match flush_handle(self.context, handle.raw()) {
            FlushOutcome::Flushed => self.handle = TransientHandleState::Terminated,
            FlushOutcome::Rejected(error) => return Err(error),
            FlushOutcome::Uncertain(error) => {
                self.handle = TransientHandleState::Abandoned;
                return Err(error);
            }
        }
        Ok(())
    }
}

impl Drop for OwnedTransientObject<'_> {
    fn drop(&mut self) {
        let _ = self.close_in_place();
    }
}
