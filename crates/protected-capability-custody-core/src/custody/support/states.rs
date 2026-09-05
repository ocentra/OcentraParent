use super::super::{
    CommittedCapability, CustodyError, Decision, FinalizeOutcome, PreparedCapability,
};
use crate::binding::Binding;
use crate::platform::SealedState;
use crate::storage::Record;

pub(super) fn prepared(record: &Record, binding: &Binding) -> PreparedCapability {
    PreparedCapability {
        record_id: record.record_id,
        lookup_digest: record.lookup_digest,
        sequence: record.sequence,
        locator: binding.locator().clone(),
    }
}

pub(super) fn finalize(record: &Record) -> Result<FinalizeOutcome, CustodyError> {
    match record.state {
        SealedState::Committed => Ok(FinalizeOutcome::Committed(committed(record))),
        SealedState::Aborted => Ok(FinalizeOutcome::Aborted),
        SealedState::CommitAmbiguous => Ok(FinalizeOutcome::CommitAmbiguous),
        SealedState::AbortAmbiguous => Ok(FinalizeOutcome::AbortAmbiguous),
        SealedState::Prepared => Err(CustodyError::Conflict),
    }
}

pub(super) fn ambiguous(decision: Decision) -> SealedState {
    if decision == Decision::Commit {
        SealedState::CommitAmbiguous
    } else {
        SealedState::AbortAmbiguous
    }
}

pub(super) fn terminal(state: SealedState) -> Result<SealedState, CustodyError> {
    match state {
        SealedState::CommitAmbiguous => Ok(SealedState::Committed),
        SealedState::AbortAmbiguous => Ok(SealedState::Aborted),
        _ => Err(CustodyError::Conflict),
    }
}

fn committed(record: &Record) -> CommittedCapability {
    CommittedCapability {
        record_id: record.record_id,
        lookup_digest: record.lookup_digest,
        sequence: record.sequence,
    }
}
