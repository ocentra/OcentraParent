#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
use super::registry::{RuntimeBatchFailure, RuntimeMutation};

/// Provider-owned proof that one exact protected registry snapshot was read at
/// a monotonic generation. Its fields remain private so registry or caller
/// state cannot manufacture an anti-rollback checkpoint.
#[cfg(windows)]
struct VerifiedCheckpoint {
    generation: u64,
    snapshot_digest: [u8; 32],
}

/// Provider-owned permit for one complete logical mutation batch. The future
/// adapter must advance exactly from `expected_generation` to
/// `committed_generation`, bind the complete post-write snapshot digest, and
/// make that binding non-restorable before any registry value is changed.
#[cfg(windows)]
pub(super) struct MutationPermit {
    expected_generation: u64,
    committed_generation: u64,
    committed_snapshot_digest: [u8; 32],
}

#[cfg(windows)]
trait MonotonicRegistryCustodyProvider {
    fn preflight_available(&self) -> Result<(), PlatformError>;

    fn read_checkpoint_and_verify_snapshot(
        &self,
        registry_id: &str,
        runtime_key: &RegKey,
    ) -> Result<VerifiedCheckpoint, PlatformError>;

    fn compare_and_advance_and_bind_batch(
        &self,
        registry_id: &str,
        runtime_key: &RegKey,
        current: &VerifiedCheckpoint,
        mutations: &[RuntimeMutation<'_>],
    ) -> Result<MutationPermit, RuntimeBatchFailure>;

    fn confirm_applied_snapshot(
        &self,
        registry_id: &str,
        runtime_key: &RegKey,
        permit: MutationPermit,
    ) -> Result<(), PlatformError>;
}

#[cfg(windows)]
struct MissingMonotonicRegistryCustodyProvider;

#[cfg(windows)]
impl MonotonicRegistryCustodyProvider for MissingMonotonicRegistryCustodyProvider {
    fn preflight_available(&self) -> Result<(), PlatformError> {
        Err(PlatformError::DeploymentRequired)
    }

    fn read_checkpoint_and_verify_snapshot(
        &self,
        _registry_id: &str,
        _runtime_key: &RegKey,
    ) -> Result<VerifiedCheckpoint, PlatformError> {
        Err(PlatformError::DeploymentRequired)
    }

    fn compare_and_advance_and_bind_batch(
        &self,
        _registry_id: &str,
        _runtime_key: &RegKey,
        _current: &VerifiedCheckpoint,
        _mutations: &[RuntimeMutation<'_>],
    ) -> Result<MutationPermit, RuntimeBatchFailure> {
        Err(RuntimeBatchFailure::DefinitelyNotApplied(
            PlatformError::DeploymentRequired,
        ))
    }

    fn confirm_applied_snapshot(
        &self,
        _registry_id: &str,
        _runtime_key: &RegKey,
        _permit: MutationPermit,
    ) -> Result<(), PlatformError> {
        Err(PlatformError::DeploymentRequired)
    }
}

#[cfg(windows)]
fn provider() -> MissingMonotonicRegistryCustodyProvider {
    MissingMonotonicRegistryCustodyProvider
}

#[cfg(windows)]
pub(super) fn provider_available() -> Result<(), PlatformError> {
    // Availability is checked without opening registry custody or advancing a
    // checkpoint. The concrete provider remains the only source of truth.
    provider().preflight_available()
}

#[cfg(windows)]
pub(super) fn verify_runtime_snapshot(
    registry_id: &str,
    runtime_key: &RegKey,
) -> Result<(), PlatformError> {
    let checkpoint = provider().read_checkpoint_and_verify_snapshot(registry_id, runtime_key)?;
    if checkpoint.generation == 0 || checkpoint.snapshot_digest == [0_u8; 32] {
        return Err(PlatformError::AntiRollback);
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn authorize_runtime_batch(
    registry_id: &str,
    runtime_key: &RegKey,
    mutations: &[RuntimeMutation<'_>],
) -> Result<MutationPermit, RuntimeBatchFailure> {
    let current = provider()
        .read_checkpoint_and_verify_snapshot(registry_id, runtime_key)
        .map_err(RuntimeBatchFailure::DefinitelyNotApplied)?;
    let permit = provider().compare_and_advance_and_bind_batch(
        registry_id,
        runtime_key,
        &current,
        mutations,
    )?;
    if permit.expected_generation != current.generation
        || permit.committed_generation
            != current
                .generation
                .checked_add(1)
                .ok_or(RuntimeBatchFailure::OutcomeUnknown)?
        || permit.committed_snapshot_digest == [0_u8; 32]
    {
        // The provider reported an advance, so a malformed permit can no
        // longer be described as definitely not applied.
        return Err(RuntimeBatchFailure::OutcomeUnknown);
    }
    Ok(permit)
}

#[cfg(windows)]
pub(super) fn confirm_runtime_batch(
    registry_id: &str,
    runtime_key: &RegKey,
    permit: MutationPermit,
) -> Result<(), PlatformError> {
    provider().confirm_applied_snapshot(registry_id, runtime_key, permit)
}
