use std::fmt;

use super::ProtocolError;

mod frame;
mod request;
mod state;
mod wire;

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(result) = frame::write(self, formatter) {
            return result;
        }
        if let Some(result) = wire::write(self, formatter) {
            return result;
        }
        if let Some(result) = request::write(self, formatter) {
            return result;
        }
        match state::write(self, formatter) {
            Some(result) => result,
            None => Err(fmt::Error),
        }
    }
}

impl std::error::Error for ProtocolError {}
