use super::super::support::{
    context_from_record, ensure_current_epoch, lock_connection, map_platform_error,
    map_storage_error, sealed_state, validate_binding,
};
use super::super::{CustodyError, CustodyStore, PreparedCapability};
use crate::binding::Binding;
use crate::platform::{PlatformAttestation, PlatformCustodyPort};
use crate::storage;

pub(super) fn load<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
    prepared: &PreparedCapability,
    binding: &Binding,
    attestation: PlatformAttestation,
) -> Result<storage::Record, CustodyError> {
    let connection = lock_connection(store)?;
    storage::validate_all(&connection).map_err(map_storage_error)?;
    let record = storage::load_by_id(&connection, &prepared.record_id)
        .map_err(map_storage_error)?
        .ok_or(CustodyError::Missing)?;
    validate_binding(&record, binding, prepared.sequence)?;
    ensure_current_epoch(&record, attestation)?;
    let state = sealed_state(record.state)?;
    store
        .platform
        .open(context_from_record(&record, state)?, &record.sealed)
        .map_err(map_platform_error)?;
    match record.state {
        1 => Ok(record),
        2 => Err(CustodyError::CommitAmbiguous),
        3 => Err(CustodyError::AbortAmbiguous),
        4 => Err(CustodyError::AlreadyCommitted),
        5 => Err(CustodyError::Aborted),
        _ => Err(CustodyError::Tampered),
    }
}
