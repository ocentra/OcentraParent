use getrandom::fill;
use rusqlite::TransactionBehavior;

use super::reconcile;
use super::support::{
    attest_path, lock_connection, lock_operation, map_platform_error, map_storage_error, prepared,
    resolve_current, transition, validate_transition,
};
use super::{CustodyError, CustodyStore, PreparedCapability};
use crate::authority::CurrentBindingPort;
use crate::binding::BindingLocator;
use crate::platform::{PlatformCustodyPort, SealedState};
use crate::storage;

pub(super) fn run<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    locator: &BindingLocator,
) -> Result<PreparedCapability, CustodyError> {
    let _operation = lock_operation(store)?;
    match reconcile::current(store, locator) {
        Ok(record) => return existing(record.state),
        Err(CustodyError::Missing) => {}
        Err(error) => return Err(error),
    }
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    let binding = resolve_current(store.authority.as_ref(), locator)?;
    let record_id = random_record_id()?;
    let lookup_digest = binding.locator().lookup_digest();
    let binding_digest = binding.digest();
    let request = transition(
        &binding,
        &record_id,
        &lookup_digest,
        &binding_digest,
        SealedState::Prepared,
        1,
        attestation,
        attestation.watermark_floor,
    );
    let broker = store
        .platform
        .reserve(request)
        .map_err(map_platform_error)?;
    let record = validate_transition(
        store.platform.as_ref(),
        &broker,
        request,
        &store.secured_path,
    )?;
    persist(store, &record)?;
    Ok(prepared(&record))
}

fn random_record_id() -> Result<[u8; 32], CustodyError> {
    let mut record_id = [0_u8; 32];
    fill(&mut record_id).map_err(|_| CustodyError::Unavailable)?;
    if record_id == [0_u8; 32] {
        return Err(CustodyError::Unavailable);
    }
    Ok(record_id)
}

fn persist<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    record: &storage::Record,
) -> Result<(), CustodyError> {
    let mut connection = lock_connection(store)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    if storage::load_by_lookup(&transaction, &record.lookup_digest)
        .map_err(map_storage_error)?
        .is_some()
    {
        return Err(CustodyError::Conflict);
    }
    storage::insert(&transaction, record).map_err(map_storage_error)?;
    transaction.commit().map_err(|_| CustodyError::Database)
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
