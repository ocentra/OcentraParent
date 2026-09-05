use super::finalize;
use super::reconcile;
use super::scope::OperationScope;
use super::support::prepared;
use super::{CommittedCapability, CustodyError, CustodyStore, RecoveryOutcome};
use crate::binding::BindingLocator;
use crate::platform::SealedState;
use crate::storage::Record;

pub(super) fn run(
    store: &CustodyStore,
    locator: &BindingLocator,
) -> Result<RecoveryOutcome, CustodyError> {
    let scope = OperationScope::acquire(store, locator)?;
    let record = reconcile::current(store, &scope)?;
    outcome(&record, scope.binding())
}

pub(super) fn resolve(
    store: &CustodyStore,
    locator: &BindingLocator,
) -> Result<RecoveryOutcome, CustodyError> {
    let scope = OperationScope::acquire(store, locator)?;
    let record = reconcile::current(store, &scope)?;
    match finalize::resolve_record(store, &scope, record) {
        Ok(record) => outcome(&record, scope.binding()),
        Err(CustodyError::CommitAmbiguous) => Ok(RecoveryOutcome::CommitAmbiguous),
        Err(CustodyError::AbortAmbiguous) => Ok(RecoveryOutcome::AbortAmbiguous),
        Err(error) => Err(error),
    }
}

fn outcome(
    record: &Record,
    binding: &crate::binding::Binding,
) -> Result<RecoveryOutcome, CustodyError> {
    match record.state {
        SealedState::Prepared => Ok(RecoveryOutcome::Prepared(prepared(record, binding))),
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
