mod prepared;
mod terminal;

use super::scope::OperationScope;
use super::support::{
    ambiguous_state, finalize_outcome, intent_phase, terminal_phase, terminal_state,
};
use super::{CustodyError, CustodyStore, Decision, FinalizeOutcome, PreparedCapability};
use crate::platform::SealedState;
use crate::storage::Record;

pub(super) fn run(
    store: &CustodyStore,
    capability: PreparedCapability,
    decision: Decision,
) -> Result<FinalizeOutcome, CustodyError> {
    let scope = OperationScope::acquire(store, &capability.locator)?;
    let mut record = prepared::load(store, &scope, capability)?;
    if record.state == SealedState::Prepared {
        record = match terminal::advance(
            store,
            &scope,
            &record,
            ambiguous_state(decision),
            intent_phase(decision),
        ) {
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
    match terminal::advance(store, &scope, &record, terminal, terminal_phase(decision)) {
        Ok(record) => finalize_outcome(&record),
        Err(CustodyError::CommitAmbiguous) => Ok(FinalizeOutcome::CommitAmbiguous),
        Err(CustodyError::AbortAmbiguous) => Ok(FinalizeOutcome::AbortAmbiguous),
        Err(error) => Err(error),
    }
}

pub(super) fn resolve_record(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
    record: Record,
) -> Result<Record, CustodyError> {
    terminal::resolve_record(store, scope, record)
}
