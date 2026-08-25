use super::super::{Record, StorageError};
use crate::platform::SealedState;

pub(super) fn validate(prior: &Record, next: &Record) -> Result<(), StorageError> {
    super::validate(prior)?;
    super::validate(next)?;
    let immutable = prior.record_id == next.record_id
        && prior.lookup_digest == next.lookup_digest
        && prior.binding_digest == next.binding_digest
        && prior.canonical_binding == next.canonical_binding
        && prior.schema_version == next.schema_version
        && prior.binding_version == next.binding_version
        && prior.database_identity == next.database_identity;
    let monotonic = prior.sequence.checked_add(1) == Some(next.sequence)
        && next.anti_rollback_watermark > prior.anti_rollback_watermark
        && next.key_epoch == prior.key_epoch
        && next.writer_epoch >= prior.writer_epoch
        && next.sealed != prior.sealed;
    if !immutable || !monotonic || !legal_edge(prior.state, next.state) {
        return Err(StorageError::IllegalTransition);
    }
    Ok(())
}

pub(super) fn validate_state_sequence(
    state: SealedState,
    sequence: u64,
) -> Result<(), StorageError> {
    let expected = match state {
        SealedState::Prepared => 1,
        SealedState::CommitAmbiguous | SealedState::AbortAmbiguous => 2,
        SealedState::Committed | SealedState::Aborted => 3,
    };
    if sequence != expected {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

fn legal_edge(prior: SealedState, next: SealedState) -> bool {
    matches!(
        (prior, next),
        (SealedState::Prepared, SealedState::CommitAmbiguous)
            | (SealedState::Prepared, SealedState::AbortAmbiguous)
            | (SealedState::CommitAmbiguous, SealedState::Committed)
            | (SealedState::AbortAmbiguous, SealedState::Aborted)
    )
}
