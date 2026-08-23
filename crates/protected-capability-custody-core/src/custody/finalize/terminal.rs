use rusqlite::TransactionBehavior;

use super::super::support::{
    context, ensure_current_epoch, lock_connection, map_platform_error, map_storage_error,
    state_code, to_i64, to_u64,
};
use super::super::{CustodyError, CustodyStore, Decision, FinalizeOutcome};
use crate::platform::{PlatformAttestation, PlatformCustodyPort, PlatformError, SealedState};
use crate::storage::{self, Record};

pub(super) fn finish<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    prior: &Record,
    ambiguous: Record,
    decision: Decision,
    attestation: PlatformAttestation,
) -> Result<FinalizeOutcome, CustodyError> {
    if prior.record_id != ambiguous.record_id
        || prior.sequence.checked_add(1) != Some(ambiguous.sequence)
    {
        return Err(CustodyError::Conflict);
    }
    ensure_current_epoch(&ambiguous, attestation)?;
    let terminal_state = if decision == Decision::Commit {
        SealedState::Committed
    } else {
        SealedState::Aborted
    };
    let sequence = to_u64(ambiguous.sequence)
        .and_then(|value| value.checked_add(1).ok_or(CustodyError::Conflict))?;
    let sealed = match store.platform.seal(context(
        &ambiguous.canonical_binding,
        terminal_state,
        sequence,
        attestation,
    )) {
        Ok(value) if !value.is_empty() => value,
        Ok(_) | Err(PlatformError::Unavailable | PlatformError::Rejected) => {
            return Ok(ambiguous_outcome(decision));
        }
        Err(error) => return Err(map_platform_error(error)),
    };
    let next = Record {
        record_id: ambiguous.record_id.clone(),
        binding_digest: ambiguous.binding_digest.clone(),
        canonical_binding: ambiguous.canonical_binding.clone(),
        state: state_code(terminal_state),
        sequence: i64::try_from(sequence).map_err(|_| CustodyError::Conflict)?,
        key_epoch: to_i64(attestation.key_epoch)?,
        writer_epoch: to_i64(attestation.writer_epoch)?,
        anti_rollback_watermark: to_i64(attestation.anti_rollback_watermark)?,
        sealed,
        schema_version: ambiguous.schema_version,
    };
    let mut connection = lock_connection(store)?;
    storage::validate_all(&connection).map_err(map_storage_error)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    let changed =
        storage::compare_and_replace(&transaction, &ambiguous, &next).map_err(map_storage_error)?;
    if !changed {
        return Err(CustodyError::Conflict);
    }
    transaction.commit().map_err(|_| CustodyError::Database)?;
    if decision == Decision::Commit {
        Ok(FinalizeOutcome::Committed(
            super::super::CommittedCapability {
                record_id: next.record_id,
                binding_digest: next
                    .binding_digest
                    .as_slice()
                    .try_into()
                    .map_err(|_| CustodyError::Tampered)?,
                sequence,
            },
        ))
    } else {
        Ok(FinalizeOutcome::Aborted)
    }
}

fn ambiguous_outcome(decision: Decision) -> FinalizeOutcome {
    if decision == Decision::Commit {
        FinalizeOutcome::CommitAmbiguous
    } else {
        FinalizeOutcome::AbortAmbiguous
    }
}
