mod local;
mod phase;
mod platform;
mod storage;
mod transition;

use super::super::{CustodyError, Decision, TransitionPhase};
use crate::authority::AuthorityError;
use crate::platform::{PlatformError, TransitionFailure};

pub(super) fn map_platform_error(error: PlatformError) -> CustodyError {
    platform::platform_error(error)
}

pub(super) fn map_authority_error(error: AuthorityError) -> CustodyError {
    platform::authority_error(error)
}

pub(super) fn map_transition_failure(
    error: TransitionFailure,
    phase: TransitionPhase,
) -> CustodyError {
    transition::failure(error, phase)
}

pub(super) fn local_replica_failure(error: CustodyError, phase: TransitionPhase) -> CustodyError {
    local::replica_failure(error, phase)
}

pub(super) fn intent_phase(decision: Decision) -> TransitionPhase {
    phase::intent(decision)
}

pub(super) fn terminal_phase(decision: Decision) -> TransitionPhase {
    phase::terminal(decision)
}

pub(super) fn map_storage_error(error: crate::storage::StorageError) -> CustodyError {
    storage::error(error)
}
