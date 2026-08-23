mod prepared;
mod terminal;

use super::support::{ambiguous_state, finalize_outcome, lock_operation, terminal_state};
use super::{CustodyError, CustodyStore, Decision, FinalizeOutcome, PreparedCapability};
use crate::authority::CurrentBindingPort;
use crate::platform::{PlatformCustodyPort, SealedState};
use crate::storage::Record;

pub(super) fn run<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    capability: PreparedCapability,
    decision: Decision,
) -> Result<FinalizeOutcome, CustodyError> {
    let _operation = lock_operation(store)?;
    let mut record = prepared::load(store, &capability)?;
    if record.state == SealedState::Prepared {
        record = match terminal::advance(store, record, ambiguous_state(decision)) {
            Ok(record) => record,
            Err(CustodyError::CommitAmbiguous) => return Ok(FinalizeOutcome::CommitAmbiguous),
            Err(CustodyError::AbortAmbiguous) => return Ok(FinalizeOutcome::AbortAmbiguous),
            Err(error) => return Err(error),
        };
    }
    let expected_ambiguous = ambiguous_state(decision);
    if record.state != expected_ambiguous {
        return finalize_outcome(&record);
    }
    let terminal = terminal_state(record.state)?;
    match terminal::advance(store, record, terminal) {
        Ok(record) => finalize_outcome(&record),
        Err(CustodyError::CommitAmbiguous) => Ok(FinalizeOutcome::CommitAmbiguous),
        Err(CustodyError::AbortAmbiguous) => Ok(FinalizeOutcome::AbortAmbiguous),
        Err(error) => Err(error),
    }
}

pub(super) fn resolve_record<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    record: Record,
) -> Result<Record, CustodyError> {
    terminal::resolve_record(store, record)
}
