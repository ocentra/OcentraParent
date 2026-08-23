use rusqlite::TransactionBehavior;

use super::{CustodyError, CustodyStore, PreparedCapability};
use crate::binding::Binding;
use crate::platform::{PlatformCustodyPort, SealedState};
use crate::storage::{self, Record, SCHEMA_VERSION};

use super::support::{
    attest, context, lock_connection, map_platform_error, map_storage_error, random_record_id,
    to_i64,
};

pub(super) fn run<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    binding: &Binding,
) -> Result<PreparedCapability, CustodyError> {
    let attestation = attest(store.platform.as_ref())?;
    let digest = binding.digest();
    let canonical = binding.canonical_bytes();
    let record_id = random_record_id()?;
    let sealed = store
        .platform
        .seal(context(canonical, SealedState::Prepared, 1, attestation))
        .map_err(map_platform_error)?;
    if sealed.is_empty() {
        return Err(CustodyError::Unavailable);
    }
    let mut connection = lock_connection(store)?;
    storage::validate_all(&connection).map_err(map_storage_error)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| CustodyError::Database)?;
    if let Some(existing) =
        storage::load_by_digest(&transaction, &digest).map_err(map_storage_error)?
    {
        return existing_state_error(existing.state);
    }
    let record = Record {
        record_id: record_id.clone(),
        binding_digest: digest.to_vec(),
        canonical_binding: canonical.to_vec(),
        state: 1,
        sequence: 1,
        key_epoch: to_i64(attestation.key_epoch)?,
        writer_epoch: to_i64(attestation.writer_epoch)?,
        anti_rollback_watermark: to_i64(attestation.anti_rollback_watermark)?,
        sealed,
        schema_version: SCHEMA_VERSION,
    };
    storage::insert(&transaction, &record).map_err(map_storage_error)?;
    transaction.commit().map_err(|_| CustodyError::Database)?;
    Ok(PreparedCapability {
        record_id,
        binding_digest: digest,
        sequence: 1,
    })
}

fn existing_state_error(state: i64) -> Result<PreparedCapability, CustodyError> {
    match state {
        4 => Err(CustodyError::AlreadyCommitted),
        5 => Err(CustodyError::Aborted),
        1..=3 => Err(CustodyError::Conflict),
        _ => Err(CustodyError::Tampered),
    }
}
