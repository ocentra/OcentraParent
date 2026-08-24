use crate::types::ProtocolError;

use super::ResponseStatus;

pub(super) mod validation;

impl ResponseStatus {
    pub(crate) fn decode(value: u8) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Prepared),
            2 => Ok(Self::Committed),
            3 => Ok(Self::Aborted),
            4 => Ok(Self::CommitAmbiguous),
            5 => Ok(Self::AbortAmbiguous),
            6 => Ok(Self::Rejected),
            7 => Ok(Self::Unavailable),
            8 => Ok(Self::UnsupportedPlatform),
            other => Err(ProtocolError::UnsupportedStatus(other)),
        }
    }
}
