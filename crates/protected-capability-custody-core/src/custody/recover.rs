use super::{CommittedCapability, CustodyError, CustodyStore, PreparedCapability, RecoveryOutcome};
use crate::binding::Binding;
use crate::platform::PlatformCustodyPort;
use crate::storage;

use super::support::{
    attest, context_from_record, ensure_current_epoch, lock_connection, map_platform_error,
    map_storage_error, sealed_state, to_u64, validate_binding,
};

pub(super) fn run<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    binding: &Binding,
) -> Result<RecoveryOutcome, CustodyError> {
    let attestation = attest(store.platform.as_ref())?;
    let digest = binding.digest();
    let connection = lock_connection(store)?;
    storage::validate_all(&connection).map_err(map_storage_error)?;
    let record = storage::load_by_digest(&connection, &digest)
        .map_err(map_storage_error)?
        .ok_or(CustodyError::Missing)?;
    validate_binding(&record, binding, to_u64(record.sequence)?)?;
    ensure_current_epoch(&record, attestation)?;
    let state = sealed_state(record.state)?;
    store
        .platform
        .open(context_from_record(&record, state)?, &record.sealed)
        .map_err(map_platform_error)?;
    match record.state {
        1 => Ok(RecoveryOutcome::Prepared(PreparedCapability {
            record_id: record.record_id,
            binding_digest: digest,
            sequence: to_u64(record.sequence)?,
        })),
        2 => Ok(RecoveryOutcome::CommitAmbiguous),
        3 => Ok(RecoveryOutcome::AbortAmbiguous),
        4 => Ok(RecoveryOutcome::Committed(CommittedCapability {
            record_id: record.record_id,
            binding_digest: digest,
            sequence: to_u64(record.sequence)?,
        })),
        5 => Ok(RecoveryOutcome::Aborted),
        _ => Err(CustodyError::Tampered),
    }
}
