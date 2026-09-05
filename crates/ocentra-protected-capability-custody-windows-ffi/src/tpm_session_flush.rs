//! Strict FlushContext response classification.

use super::super::{command, response};
use crate::{Error, OwnedTbsContext};

pub(super) enum FlushOutcome {
    Flushed,
    Rejected(Error),
    Uncertain(Error),
}

pub(super) fn flush_handle(context: &OwnedTbsContext, handle: u32) -> FlushOutcome {
    let command = match command::policy::encode_flush_context(handle) {
        Ok(command) => command,
        Err(error) => return FlushOutcome::Rejected(error),
    };
    let response = match context.submit(&command) {
        Ok(response) => response,
        Err(error) => return FlushOutcome::Uncertain(error),
    };
    match response::sessions::decode_success_no_parameters(&response) {
        Ok(()) => FlushOutcome::Flushed,
        Err(error @ Error::Tpm(_)) => FlushOutcome::Rejected(error),
        Err(error) => FlushOutcome::Uncertain(error),
    }
}
