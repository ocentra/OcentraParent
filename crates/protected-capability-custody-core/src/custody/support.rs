mod attestation;
mod mapping;
mod states;
mod validation;

use std::sync::MutexGuard;

use getrandom::fill;
use rusqlite::Connection;

use super::{CustodyError, CustodyStore};
use crate::binding::Binding;
use crate::platform::{
    PlatformAttestation, PlatformCustodyPort, PlatformError, SealContext, SealedState,
    RECORD_NAMESPACE,
};
use crate::storage::{Record, StorageError};

pub(super) fn attest<P: PlatformCustodyPort>(
    platform: &P,
) -> Result<PlatformAttestation, CustodyError> {
    attestation::attest(platform)
}

pub(super) fn lock_connection<P: PlatformCustodyPort>(
    store: &CustodyStore<P>,
) -> Result<MutexGuard<'_, Connection>, CustodyError> {
    store.connection.lock().map_err(|_| CustodyError::Conflict)
}

pub(super) fn validate_binding(
    record: &Record,
    binding: &Binding,
    expected_sequence: u64,
) -> Result<(), CustodyError> {
    validation::validate_binding(record, binding, expected_sequence)
}

pub(super) fn ensure_current_epoch(
    record: &Record,
    attestation: PlatformAttestation,
) -> Result<(), CustodyError> {
    validation::ensure_current_epoch(record, attestation)
}

pub(super) fn context<'a>(
    canonical_binding: &'a [u8],
    state: SealedState,
    sequence: u64,
    attestation: PlatformAttestation,
) -> SealContext<'a> {
    SealContext {
        record_namespace: RECORD_NAMESPACE,
        canonical_binding,
        state,
        sequence,
        key_epoch: attestation.key_epoch,
        writer_epoch: attestation.writer_epoch,
        anti_rollback_watermark: attestation.anti_rollback_watermark,
    }
}

pub(super) fn context_from_record<'a>(
    record: &'a Record,
    state: SealedState,
) -> Result<SealContext<'a>, CustodyError> {
    Ok(SealContext {
        record_namespace: RECORD_NAMESPACE,
        canonical_binding: &record.canonical_binding,
        state,
        sequence: to_u64(record.sequence)?,
        key_epoch: to_u64(record.key_epoch)?,
        writer_epoch: to_u64(record.writer_epoch)?,
        anti_rollback_watermark: to_u64(record.anti_rollback_watermark)?,
    })
}

pub(super) fn state_code(state: SealedState) -> i64 {
    state as i64
}

pub(super) fn random_record_id() -> Result<Vec<u8>, CustodyError> {
    let mut record_id = [0_u8; 32];
    fill(&mut record_id).map_err(|_| CustodyError::Unavailable)?;
    Ok(record_id.to_vec())
}

pub(super) fn to_i64(value: u64) -> Result<i64, CustodyError> {
    i64::try_from(value).map_err(|_| CustodyError::Conflict)
}

pub(super) fn to_u64(value: i64) -> Result<u64, CustodyError> {
    u64::try_from(value).map_err(|_| CustodyError::Tampered)
}

pub(super) fn map_platform_error(error: PlatformError) -> CustodyError {
    mapping::map_platform_error(error)
}

pub(super) fn map_storage_error(error: StorageError) -> CustodyError {
    mapping::map_storage_error(error)
}

pub(super) fn sealed_state(state: i64) -> Result<SealedState, CustodyError> {
    states::sealed_state(state)
}
