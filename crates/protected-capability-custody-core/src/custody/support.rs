mod attestation;
mod mapping;
pub(super) mod sqlite;
mod states;
mod validation;

use super::{CustodyError, Decision, FinalizeOutcome, PreparedCapability, TransitionPhase};
use crate::authority::AuthorityError;
use crate::binding::{Binding, BINDING_VERSION};
use crate::path_security::SecuredPath;
use crate::platform::{
    record::BrokerRecord,
    request::{BrokerLookup, TransitionRequest},
    PlatformAttestation, PlatformDatabaseGuard, SealedState, TransitionFailure,
};
use crate::storage::Record;

pub(super) struct TransitionStep {
    pub(super) state: SealedState,
    pub(super) sequence: u64,
    pub(super) minimum_watermark: u64,
}

pub(super) fn attest_path(
    platform: &dyn PlatformDatabaseGuard,
    path: &SecuredPath,
) -> Result<PlatformAttestation, CustodyError> {
    attestation::attest_path(platform, path)
}

pub(super) fn validate_current(record: &Record, binding: &Binding) -> Result<(), CustodyError> {
    validation::validate_current(record, binding)
}

pub(super) fn verify_broker(
    platform: &dyn PlatformDatabaseGuard,
    broker: &BrokerRecord,
    path: &SecuredPath,
) -> Result<Record, CustodyError> {
    validation::verify_broker(platform, broker, path)
}

pub(super) fn validate_transition(
    platform: &dyn PlatformDatabaseGuard,
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
    attestation: PlatformAttestation,
    step: &TransitionStep,
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
        state: step.state,
        sequence: step.sequence,
        key_epoch: attestation.key_epoch,
        writer_epoch: attestation.writer_epoch,
        minimum_watermark: step.minimum_watermark,
    }
}

pub(super) fn prepared(record: &Record, binding: &Binding) -> PreparedCapability {
    states::prepared(record, binding)
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

pub(super) fn map_path_error(error: &crate::path_security::PathSecurityError) -> CustodyError {
    attestation::map_path_error(error)
}

pub(super) fn map_platform_error(error: &crate::platform::PlatformError) -> CustodyError {
    mapping::map_platform_error(error)
}

pub(super) fn map_authority_error(error: &AuthorityError) -> CustodyError {
    mapping::map_authority_error(error)
}

pub(super) fn map_transition_failure(
    error: TransitionFailure,
    phase: TransitionPhase,
) -> CustodyError {
    mapping::map_transition_failure(error, phase)
}

pub(super) fn local_replica_failure(error: CustodyError, phase: TransitionPhase) -> CustodyError {
    mapping::local_replica_failure(error, phase)
}

pub(super) fn intent_phase(decision: Decision) -> TransitionPhase {
    mapping::intent_phase(decision)
}

pub(super) fn terminal_phase(decision: Decision) -> TransitionPhase {
    mapping::terminal_phase(decision)
}
