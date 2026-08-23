use rusqlite::TransactionBehavior;

use super::super::support::{
    lock_connection, map_storage_error, resolve_current, validate_attestation, validate_current,
    verify_broker,
};
use super::super::{CustodyError, CustodyStore};
use crate::authority::CurrentBindingPort;
use crate::binding::Binding;
use crate::platform::{
    record::BrokerRecord, PlatformAttestation, PlatformCustodyPort, SealedState,
};
use crate::storage::{self, Record};

pub(super) fn recover_initial<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
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
    let current = resolve_current(store.authority.as_ref(), binding.locator())?;
    validate_current(&broker, &current)?;
    let mut connection = lock_connection(store)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    if storage::load_by_lookup(&transaction, &broker.lookup_digest)
        .map_err(map_storage_error)?
        .is_some()
    {
        return Err(CustodyError::Conflict);
    }
    storage::insert(&transaction, &broker).map_err(map_storage_error)?;
    transaction.commit().map_err(|_| CustodyError::Database)?;
    Ok(broker)
}

pub(super) fn advance<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    binding: &Binding,
    prior: &Record,
    next: Record,
) -> Result<Record, CustodyError> {
    let current = resolve_current(store.authority.as_ref(), binding.locator())?;
    validate_current(prior, &current)?;
    validate_current(&next, &current)?;
    let mut connection = lock_connection(store)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    let changed =
        storage::compare_and_replace(&transaction, prior, &next).map_err(map_storage_error)?;
    if !changed {
        return Err(CustodyError::Conflict);
    }
    transaction.commit().map_err(|_| CustodyError::Database)?;
    Ok(next)
}
