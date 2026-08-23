//! Device Trust's private reservation participant for multi-owner effects.
//!
//! The Account effect coordinator owns operation ordering, but it must not copy
//! Device Trust currentness or revocation state. This participant keeps that
//! state in the Device Trust owner and records only an opaque, durable
//! reservation against the current signer binding. A prepared reservation is
//! never treated as an approval: only an exact committed row can be recovered.

use std::fmt;

use crate::device_trust_lifecycle::{DeviceTrustLifecycleRepository, DeviceTrustLifecycleState};

#[path = "device_trust_runtime_fence_abort.rs"]
mod abort;
#[path = "device_trust_runtime_fence_action.rs"]
mod action;
#[path = "device_trust_runtime_fence_commit.rs"]
mod commit;
#[path = "device_trust_runtime_fence_digest.rs"]
mod digest;
#[path = "device_trust_runtime_fence_error.rs"]
mod error;
#[path = "device_trust_runtime_fence_prepare.rs"]
mod prepare;
#[path = "device_trust_runtime_fence_recovery.rs"]
mod recovery;
#[path = "device_trust_runtime_fence_storage.rs"]
mod storage;
#[path = "device_trust_runtime_fence_target.rs"]
mod target;

/// Fail-closed participant failures. No variant exposes a stored identity,
/// generation, signer key, or reservation handle to a downstream caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeviceTrustRuntimeFenceError {
    Unavailable,
    InvalidOperation,
    InvalidTarget,
    DeviceTrustRevoked,
    DeviceTrustUnavailable,
    TargetMismatch,
    GenerationMismatch,
    ReservationMissing,
    ReservationAlreadyCommitted,
    ReservationAborted,
    OperationConflict,
    RecoveryUncertain,
}

/// The participant borrows the canonical Device Trust repository so every
/// reservation re-resolves the current durable signer authority from this
/// owner. Its ledger is an exact SQLite table in that same owner database;
/// there is no in-memory production custody or caller-selected backing store.
pub(crate) struct DeviceTrustRuntimeFenceParticipant<'a> {
    repository: &'a mut DeviceTrustLifecycleRepository,
}

/// Opaque prepared reservation returned only to the in-crate coordinator.
/// Fields are private, and the type is neither `Clone` nor serializable.
pub(crate) struct DeviceTrustRuntimeFenceReservation {
    operation_id: String,
    reservation_ref: String,
    target: DeviceTrustRuntimeFenceTarget,
}

/// Opaque committed outcome. A recovery call returns this only when the
/// persisted committed row and the current Device Trust binding still match.
pub(crate) struct DeviceTrustRuntimeFenceOutcome {
    operation_id: String,
    reservation_ref: String,
    outcome_digest: String,
    target: DeviceTrustRuntimeFenceTarget,
}

#[derive(PartialEq, Eq)]
struct DeviceTrustRuntimeFenceTarget {
    action_code: i64,
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_key_id: String,
    signer_key_sha256: String,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    state: DeviceTrustLifecycleState,
}

struct StoredReservation {
    operation_id: String,
    reservation_ref: String,
    action_code: i64,
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_key_id: String,
    signer_key_sha256: String,
    lifecycle_generation: i64,
    installation_binding_generation: i64,
    authority_generation: i64,
    state: String,
    outcome_digest: Option<String>,
}

impl fmt::Debug for DeviceTrustRuntimeFenceParticipant<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceTrustRuntimeFenceParticipant")
            .field("repository", &"device-trust-owner")
            .finish()
    }
}

impl fmt::Debug for DeviceTrustRuntimeFenceReservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceTrustRuntimeFenceReservation")
            .field("operation_id", &"opaque")
            .field("reservation_ref", &"opaque")
            .field("target", &"opaque")
            .finish()
    }
}

impl fmt::Debug for DeviceTrustRuntimeFenceOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceTrustRuntimeFenceOutcome")
            .field("operation_id", &"opaque")
            .field("reservation_ref", &"opaque")
            .field("outcome_digest", &"opaque")
            .field("target", &"opaque")
            .finish()
    }
}

impl DeviceTrustRuntimeFenceParticipant<'_> {
    /// Attach the participant to an already opened Device Trust repository and
    /// validate/create only its exact owner ledger. The lifecycle repository
    /// has already validated its own tables before this method is called.
    pub(crate) fn attach(
        repository: &mut DeviceTrustLifecycleRepository,
    ) -> Result<DeviceTrustRuntimeFenceParticipant<'_>, DeviceTrustRuntimeFenceError> {
        storage::ensure_schema(&repository.connection)?;
        Ok(DeviceTrustRuntimeFenceParticipant { repository })
    }
}

impl DeviceTrustRuntimeFenceOutcome {
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }
}
