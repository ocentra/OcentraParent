use rusqlite::TransactionBehavior;

use super::super::scope::OperationScope;
use super::super::support::sqlite::{finish_step, lock_connection, map_error as map_storage_error};
use super::super::support::{
    attest_path, local_replica_failure, map_transition_failure, terminal_state, transition,
    validate_attestation, validate_current, validate_transition,
};
use super::super::{CustodyError, CustodyStore, TransitionPhase};
use crate::platform::SealedState;
use crate::storage::{self, Record};

pub(super) fn resolve_record(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
    record: Record,
) -> Result<Record, CustodyError> {
    let phase = match record.state {
        SealedState::CommitAmbiguous => TransitionPhase::CommitTerminal,
        SealedState::AbortAmbiguous => TransitionPhase::AbortTerminal,
        _ => return Ok(record),
    };
    let terminal = terminal_state(record.state)?;
    advance(store, scope, record, terminal, phase)
}

pub(super) fn advance(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
    prior: Record,
    next_state: SealedState,
    phase: TransitionPhase,
) -> Result<Record, CustodyError> {
    let binding = scope.binding().clone();
    validate_current(&prior, &binding)?;
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    validate_attestation(&prior, attestation)?;
    let lookup_digest = binding.locator().lookup_digest();
    let binding_digest = binding.digest();
    let request = transition(
        &binding,
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
        .map_err(|error| map_transition_failure(error, phase))?;
    let next = validate_transition(
        store.platform.as_ref(),
        &broker,
        request,
        &store.secured_path,
    )?;
    persist(store, &binding, &prior, next).map_err(|error| local_replica_failure(error, phase))
}

fn persist(
    store: &CustodyStore,
    binding: &crate::binding::Binding,
    prior: &Record,
    next: Record,
) -> Result<Record, CustodyError> {
    store
        .secured_path
        .revalidate()
        .map_err(super::super::support::map_path_error)?;
    validate_current(prior, binding)?;
    validate_current(&next, binding)?;
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
}
