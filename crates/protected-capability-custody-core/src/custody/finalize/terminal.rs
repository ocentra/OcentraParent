use rusqlite::TransactionBehavior;

use super::super::support::{
    attest_path, lock_connection, map_storage_error, resolve_current, terminal_state, transition,
    validate_attestation, validate_current, validate_transition,
};
use super::super::{CustodyError, CustodyStore};
use crate::authority::CurrentBindingPort;
use crate::binding::Binding;
use crate::platform::{PlatformCustodyPort, SealedState};
use crate::storage::{self, Record};

pub(super) fn resolve_record<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    record: Record,
) -> Result<Record, CustodyError> {
    if matches!(
        record.state,
        SealedState::CommitAmbiguous | SealedState::AbortAmbiguous
    ) {
        let terminal = terminal_state(record.state)?;
        return advance(store, record, terminal);
    }
    Ok(record)
}

pub(super) fn advance<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    prior: Record,
    next_state: SealedState,
) -> Result<Record, CustodyError> {
    let binding = Binding::decode(&prior.canonical_binding).map_err(|_| CustodyError::Tampered)?;
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    let current = resolve_current(store.authority.as_ref(), binding.locator())?;
    validate_current(&prior, &current)?;
    validate_attestation(&prior, attestation)?;
    let lookup_digest = current.locator().lookup_digest();
    let binding_digest = current.digest();
    let request = transition(
        &current,
        &prior.record_id,
        &lookup_digest,
        &binding_digest,
        next_state,
        prior
            .sequence
            .checked_add(1)
            .ok_or(CustodyError::Conflict)?,
        attestation,
        attestation
            .watermark_floor
            .max(prior.anti_rollback_watermark),
    );
    let prior_broker = storage::to_broker(&prior);
    let broker = store
        .platform
        .advance(&prior_broker, request)
        .map_err(|_| ambiguity(next_state))?;
    let next = validate_transition(
        store.platform.as_ref(),
        &broker,
        request,
        &store.secured_path,
    )
    .map_err(|_| ambiguity(next_state))?;
    let persisted = persist(store, &prior, next).map_err(|_| ambiguity(next_state))?;
    let latest = resolve_current(store.authority.as_ref(), binding.locator())
        .map_err(|_| ambiguity(next_state))?;
    validate_current(&persisted, &latest).map_err(|_| ambiguity(next_state))?;
    Ok(persisted)
}

fn persist<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    prior: &Record,
    next: Record,
) -> Result<Record, CustodyError> {
    store
        .secured_path
        .revalidate()
        .map_err(super::super::support::map_path_error)?;
    let binding = Binding::decode(&next.canonical_binding).map_err(|_| CustodyError::Tampered)?;
    let current = resolve_current(store.authority.as_ref(), binding.locator())?;
    validate_current(prior, &current)?;
    validate_current(&next, &current)?;
    let mut connection = lock_connection(store)?;
    storage::validate_all(&connection, store.secured_path.identity()).map_err(map_storage_error)?;
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

fn ambiguity(state: SealedState) -> CustodyError {
    if matches!(state, SealedState::CommitAmbiguous | SealedState::Committed) {
        CustodyError::CommitAmbiguous
    } else {
        CustodyError::AbortAmbiguous
    }
}
