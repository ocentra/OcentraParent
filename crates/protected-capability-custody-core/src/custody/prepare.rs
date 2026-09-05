use getrandom::fill;
use rusqlite::TransactionBehavior;

use super::reconcile;
use super::scope::OperationScope;
use super::support::sqlite::{finish_step, lock_connection, map_error as map_storage_error};
use super::support::{
    attest_path, local_replica_failure, map_transition_failure, prepared, transition,
    validate_transition, TransitionStep,
};
use super::{CustodyError, CustodyStore, PreparedCapability, TransitionPhase};
use crate::binding::BindingLocator;
use crate::platform::SealedState;
use crate::storage;

pub(super) fn run(
    store: &CustodyStore,
    locator: &BindingLocator,
) -> Result<PreparedCapability, CustodyError> {
    let scope = OperationScope::acquire(store, locator)?;
    match reconcile::current(store, &scope) {
        Ok(record) => return existing(record.state),
        Err(CustodyError::Missing) => {}
        Err(error) => return Err(error),
    }
    let binding = scope.binding().clone();
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    let record_id = random_record_id()?;
    let lookup_digest = binding.locator().lookup_digest();
    let binding_digest = binding.digest();
    let request = transition(
        &binding,
        &record_id,
        &lookup_digest,
        &binding_digest,
        attestation,
        &TransitionStep {
            state: SealedState::Prepared,
            sequence: 1,
            minimum_watermark: attestation.watermark_floor,
        },
    );
    let broker = store
        .platform
        .reserve(request)
        .map_err(|error| map_transition_failure(error, TransitionPhase::Prepare))?;
    let record = validate_transition(
        store.platform.as_ref(),
        &broker,
        request,
        &store.secured_path,
    )?;
    persist(store, &record)
        .map_err(|error| local_replica_failure(error, TransitionPhase::Prepare))?;
    Ok(prepared(&record, &binding))
}

fn random_record_id() -> Result<[u8; 32], CustodyError> {
    let mut record_id = [0_u8; 32];
    fill(&mut record_id).map_err(|_random_error| CustodyError::Unavailable)?;
    if record_id == [0_u8; 32] {
        return Err(CustodyError::Unavailable);
    }
    Ok(record_id)
}

fn persist(store: &CustodyStore, record: &storage::Record) -> Result<(), CustodyError> {
    store
        .secured_path
        .revalidate()
        .map_err(|error| super::support::map_path_error(&error))?;
    let mut connection = lock_connection(store)?;
    let result = (|| {
        storage::validate_all(&connection, store.secured_path.identity())
            .map_err(|error| map_storage_error(&error))?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_sqlite_error| CustodyError::Database)?;
        if storage::load_by_lookup(&transaction, &record.lookup_digest)
            .map_err(|error| map_storage_error(&error))?
            .is_some()
        {
            return Err(CustodyError::Conflict);
        }
        storage::insert(&transaction, record).map_err(|error| map_storage_error(&error))?;
        transaction
            .commit()
            .map_err(|_sqlite_error| CustodyError::Database)
    })();
    drop(connection);
    finish_step(store, result)
}

fn existing(state: SealedState) -> Result<PreparedCapability, CustodyError> {
    match state {
        SealedState::Committed => Err(CustodyError::AlreadyCommitted),
        SealedState::Aborted => Err(CustodyError::Aborted),
        SealedState::Prepared | SealedState::CommitAmbiguous | SealedState::AbortAmbiguous => {
            Err(CustodyError::Conflict)
        }
    }
}
