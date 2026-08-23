mod prepared;
mod terminal;

use rusqlite::TransactionBehavior;

use super::{CustodyError, CustodyStore, Decision, FinalizeOutcome, PreparedCapability};
use crate::binding::Binding;
use crate::platform::{PlatformCustodyPort, SealedState};
use crate::storage::{self, Record};

use super::support::{
    attest, context, lock_connection, map_platform_error, map_storage_error, state_code, to_i64,
};

pub(super) fn run<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    prepared: PreparedCapability,
    current_binding: &Binding,
    decision: Decision,
) -> Result<FinalizeOutcome, CustodyError> {
    if prepared.binding_digest != current_binding.digest() {
        return Err(CustodyError::WrongBinding);
    }
    let attestation = attest(store.platform.as_ref())?;
    let record = match prepared::load(store, &prepared, current_binding, attestation) {
        Ok(record) => record,
        Err(CustodyError::CommitAmbiguous) => return Ok(FinalizeOutcome::CommitAmbiguous),
        Err(CustodyError::AbortAmbiguous) => return Ok(FinalizeOutcome::AbortAmbiguous),
        Err(error) => return Err(error),
    };
    let next_sequence = record
        .sequence
        .checked_add(1)
        .ok_or(CustodyError::Conflict)?;
    let ambiguous_state = match decision {
        Decision::Commit => SealedState::CommitAmbiguous,
        Decision::Abort => SealedState::AbortAmbiguous,
    };
    let ambiguous = mark_ambiguous(store, &record, ambiguous_state, attestation, next_sequence)?;
    terminal::finish(store, &record, ambiguous, decision, attestation)
}

fn mark_ambiguous<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    prior: &Record,
    state: SealedState,
    attestation: crate::platform::PlatformAttestation,
    sequence: i64,
) -> Result<Record, CustodyError> {
    let sealed = store
        .platform
        .seal(context(
            &prior.canonical_binding,
            state,
            u64::try_from(sequence).map_err(|_| CustodyError::Conflict)?,
            attestation,
        ))
        .map_err(map_platform_error)?;
    if sealed.is_empty() {
        return Err(CustodyError::Unavailable);
    }
    let mut connection = lock_connection(store)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    let next = Record {
        record_id: prior.record_id.clone(),
        binding_digest: prior.binding_digest.clone(),
        canonical_binding: prior.canonical_binding.clone(),
        state: state_code(state),
        sequence,
        key_epoch: to_i64(attestation.key_epoch)?,
        writer_epoch: to_i64(attestation.writer_epoch)?,
        anti_rollback_watermark: to_i64(attestation.anti_rollback_watermark)?,
        sealed,
        schema_version: prior.schema_version,
    };
    let changed =
        storage::compare_and_replace(&transaction, prior, &next).map_err(map_storage_error)?;
    if !changed {
        return Err(CustodyError::Conflict);
    }
    transaction.commit().map_err(|_| CustodyError::Database)?;
    Ok(next)
}
