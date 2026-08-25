use super::super::super::{CustodyError, TransitionPhase};
use crate::platform::TransitionFailure;

pub(super) fn failure(error: TransitionFailure, phase: TransitionPhase) -> CustodyError {
    match error {
        TransitionFailure::DefinitelyNotApplied(error) => super::platform::platform_error(error),
        TransitionFailure::OutcomeUnknown => unknown_outcome(phase),
    }
}

fn unknown_outcome(phase: TransitionPhase) -> CustodyError {
    match phase {
        TransitionPhase::Prepare => CustodyError::PrepareAmbiguous,
        TransitionPhase::CommitIntent | TransitionPhase::CommitTerminal => {
            CustodyError::CommitAmbiguous
        }
        TransitionPhase::AbortIntent | TransitionPhase::AbortTerminal => {
            CustodyError::AbortAmbiguous
        }
    }
}
