use super::finalize;
use super::reconcile;
use super::support::{lock_operation, prepared};
use super::{CommittedCapability, CustodyError, CustodyStore, RecoveryOutcome};
use crate::authority::CurrentBindingPort;
use crate::binding::BindingLocator;
use crate::platform::PlatformCustodyPort;
use crate::platform::SealedState;
use crate::storage::Record;

pub(super) fn run<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    locator: &BindingLocator,
) -> Result<RecoveryOutcome, CustodyError> {
    let _operation = lock_operation(store)?;
    let record = reconcile::current(store, locator)?;
    outcome(record)
}

pub(super) fn resolve<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    locator: &BindingLocator,
) -> Result<RecoveryOutcome, CustodyError> {
    let _operation = lock_operation(store)?;
    let record = reconcile::current(store, locator)?;
    match finalize::resolve_record(store, record) {
        Ok(record) => outcome(record),
        Err(CustodyError::CommitAmbiguous) => Ok(RecoveryOutcome::CommitAmbiguous),
        Err(CustodyError::AbortAmbiguous) => Ok(RecoveryOutcome::AbortAmbiguous),
        Err(error) => Err(error),
    }
}

fn outcome(record: Record) -> Result<RecoveryOutcome, CustodyError> {
    match record.state {
        SealedState::Prepared => Ok(RecoveryOutcome::Prepared(prepared(&record))),
        SealedState::CommitAmbiguous => Ok(RecoveryOutcome::CommitAmbiguous),
        SealedState::AbortAmbiguous => Ok(RecoveryOutcome::AbortAmbiguous),
        SealedState::Committed => Ok(RecoveryOutcome::Committed(CommittedCapability {
            record_id: record.record_id,
            lookup_digest: record.lookup_digest,
            sequence: record.sequence,
        })),
        SealedState::Aborted => Ok(RecoveryOutcome::Aborted),
    }
}
