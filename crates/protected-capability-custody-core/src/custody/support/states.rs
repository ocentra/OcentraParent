use super::super::CustodyError;
use crate::platform::SealedState;

pub(super) fn sealed_state(state: i64) -> Result<SealedState, CustodyError> {
    match state {
        1 => Ok(SealedState::Prepared),
        2 => Ok(SealedState::CommitAmbiguous),
        3 => Ok(SealedState::AbortAmbiguous),
        4 => Ok(SealedState::Committed),
        5 => Ok(SealedState::Aborted),
        _ => Err(CustodyError::Tampered),
    }
}
