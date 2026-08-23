mod attestation;
mod mapping;
mod states;
mod validation;

use std::sync::MutexGuard;

use rusqlite::Connection;

use super::{CustodyError, CustodyStore, Decision, FinalizeOutcome, PreparedCapability};
use crate::authority::CurrentBindingPort;
use crate::binding::{Binding, BindingLocator, BINDING_VERSION};
use crate::path_security::SecuredPath;
use crate::platform::{
    record::BrokerRecord,
    request::{BrokerLookup, TransitionRequest},
    PlatformAttestation, PlatformCustodyPort, SealedState,
};
use crate::storage::Record;

pub(super) fn attest_path<P: PlatformCustodyPort>(
    platform: &P,
    path: &SecuredPath,
) -> Result<PlatformAttestation, CustodyError> {
    attestation::attest_path(platform, path)
}

pub(super) fn lock_connection<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
) -> Result<MutexGuard<'_, Connection>, CustodyError> {
    store.connection.lock().map_err(|_| CustodyError::Conflict)
}

pub(super) fn lock_operation<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
) -> Result<MutexGuard<'_, ()>, CustodyError> {
    store.operation.lock().map_err(|_| CustodyError::Conflict)
}

pub(super) fn resolve_current<A: CurrentBindingPort>(
    authority: &A,
    locator: &BindingLocator,
) -> Result<Binding, CustodyError> {
    validation::resolve_current(authority, locator)
}

pub(super) fn validate_current(record: &Record, binding: &Binding) -> Result<(), CustodyError> {
    validation::validate_current(record, binding)
}

pub(super) fn verify_broker<P: PlatformCustodyPort>(
    platform: &P,
    broker: &BrokerRecord,
    path: &SecuredPath,
) -> Result<Record, CustodyError> {
    validation::verify_broker(platform, broker, path)
}

pub(super) fn validate_transition<P: PlatformCustodyPort>(
    platform: &P,
    broker: &BrokerRecord,
    request: TransitionRequest<'_>,
    path: &SecuredPath,
) -> Result<Record, CustodyError> {
    validation::validate_transition(platform, broker, request, path)
}

pub(super) fn validate_attestation(
    record: &Record,
    attestation: PlatformAttestation,
) -> Result<(), CustodyError> {
    validation::validate_attestation(record, attestation)
}

pub(super) fn lookup<'a>(digest: &'a [u8; 32], path: &SecuredPath) -> BrokerLookup<'a> {
    BrokerLookup {
        record_namespace: crate::RECORD_NAMESPACE,
        schema_version: crate::STORAGE_SCHEMA_VERSION,
        binding_version: BINDING_VERSION,
        database_identity: path.identity(),
        lookup_digest: digest,
    }
}

pub(super) fn transition<'a>(
    binding: &'a Binding,
    record_id: &'a [u8; 32],
    lookup_digest: &'a [u8; 32],
    binding_digest: &'a [u8; 32],
    state: SealedState,
    sequence: u64,
    attestation: PlatformAttestation,
    minimum_watermark: u64,
) -> TransitionRequest<'a> {
    TransitionRequest {
        record_namespace: crate::RECORD_NAMESPACE,
        schema_version: crate::STORAGE_SCHEMA_VERSION,
        binding_version: BINDING_VERSION,
        database_identity: attestation.database_identity,
        record_id,
        lookup_digest,
        binding_digest,
        canonical_binding: binding.canonical_bytes(),
        state,
        sequence,
        key_epoch: attestation.key_epoch,
        writer_epoch: attestation.writer_epoch,
        minimum_watermark,
    }
}

pub(super) fn prepared(record: &Record) -> PreparedCapability {
    states::prepared(record)
}

pub(super) fn finalize_outcome(record: &Record) -> Result<FinalizeOutcome, CustodyError> {
    states::finalize(record)
}

pub(super) fn ambiguous_state(decision: Decision) -> SealedState {
    states::ambiguous(decision)
}

pub(super) fn terminal_state(state: SealedState) -> Result<SealedState, CustodyError> {
    states::terminal(state)
}

pub(super) fn map_path_error(error: crate::path_security::PathSecurityError) -> CustodyError {
    attestation::map_path_error(error)
}

pub(super) fn map_platform_error(error: crate::platform::PlatformError) -> CustodyError {
    mapping::map_platform_error(error)
}

pub(super) fn map_storage_error(error: crate::storage::StorageError) -> CustodyError {
    mapping::map_storage_error(error)
}
