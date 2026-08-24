use rusqlite::TransactionBehavior;

use super::super::support::sqlite::{finish_step, lock_connection, map_error as map_storage_error};
use super::super::support::{validate_attestation, validate_current, verify_broker};
use super::super::{CustodyError, CustodyStore, TransitionPhase};
use crate::binding::Binding;
use crate::platform::{record::BrokerRecord, PlatformAttestation, SealedState};
use crate::storage::{self, Record};

pub(super) fn recover_initial(
    store: &CustodyStore,
    binding: &Binding,
    broker: BrokerRecord,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    let broker = verify_broker(store.platform.as_ref(), &broker, &store.secured_path)?;
    validate_current(&broker, binding)?;
    validate_attestation(&broker, attestation)?;
    if broker.state != SealedState::Prepared || broker.sequence != 1 {
        return Err(CustodyError::Conflict);
    }
    persist_initial(store, &broker).map_err(|error| {
        super::super::support::local_replica_failure(error, TransitionPhase::Prepare)
    })?;
    Ok(broker)
}

pub(super) fn advance(
    store: &CustodyStore,
    binding: &Binding,
    prior: &Record,
    next: Record,
) -> Result<Record, CustodyError> {
    validate_current(prior, binding)?;
    validate_current(&next, binding)?;
    let phase = match next.state {
        SealedState::Prepared => TransitionPhase::Prepare,
        SealedState::CommitAmbiguous => TransitionPhase::CommitIntent,
        SealedState::AbortAmbiguous => TransitionPhase::AbortIntent,
        SealedState::Committed => TransitionPhase::CommitTerminal,
        SealedState::Aborted => TransitionPhase::AbortTerminal,
    };
    store
        .secured_path
        .revalidate()
        .map_err(super::super::support::map_path_error)?;
    let mut connection = lock_connection(store)?;
    let result = (|| {
        storage::validate_all(&connection, store.secured_path.identity())
            .map_err(map_storage_error)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| CustodyError::Database)?;
        let changed =
            storage::compare_and_replace(&transaction, prior, &next).map_err(map_storage_error)?;
        if !changed {
            return Err(CustodyError::Conflict);
        }
        transaction
            .commit()
            .map_err(|_| CustodyError::Database)
            .map(|()| next)
    })();
    drop(connection);
    finish_step(store, result)
        .map_err(|error| super::super::support::local_replica_failure(error, phase))
}

fn persist_initial(store: &CustodyStore, broker: &Record) -> Result<(), CustodyError> {
    store
        .secured_path
        .revalidate()
        .map_err(super::super::support::map_path_error)?;
    let mut connection = lock_connection(store)?;
    let result = (|| {
        storage::validate_all(&connection, store.secured_path.identity())
            .map_err(map_storage_error)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| CustodyError::Database)?;
        if storage::load_by_lookup(&transaction, &broker.lookup_digest)
            .map_err(map_storage_error)?
            .is_some()
        {
            return Err(CustodyError::Conflict);
        }
        storage::insert(&transaction, broker).map_err(map_storage_error)?;
        transaction.commit().map_err(|_| CustodyError::Database)
    })();
    drop(connection);
    finish_step(store, result)
}
