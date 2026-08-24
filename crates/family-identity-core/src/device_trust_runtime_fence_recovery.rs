use rusqlite::Transaction;

use crate::device_trust_lifecycle_authority_lock::AuthorityReadFence;

use super::{
    digest, storage, target, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceOutcome,
    DeviceTrustRuntimeFenceParticipant,
};

impl DeviceTrustRuntimeFenceParticipant<'_> {
    /// Recover exactly one previously committed operation. Prepared rows are
    /// intentionally uncertain after restart; recovery never promotes them or
    /// creates a new receipt from a persisted snapshot.
    pub(crate) fn recover(
        &mut self,
        operation_id: &str,
    ) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
        storage::validate_operation(operation_id)?;
        let fence = self.repository.external_authority.read_fence()?;
        let transaction = self.repository.transaction()?;
        storage::validate_operational_schema(&transaction)?;
        let stored = storage::read_reservation(&transaction, operation_id)?
            .ok_or(DeviceTrustRuntimeFenceError::ReservationMissing)?;
        let target = storage::target(&stored)?;
        let result = match stored.state.as_str() {
            "prepared" => Err(DeviceTrustRuntimeFenceError::RecoveryUncertain),
            "aborted" => Err(DeviceTrustRuntimeFenceError::ReservationAborted),
            "committed" => recover_committed(&transaction, &fence, stored, target),
            _ => Err(DeviceTrustRuntimeFenceError::Unavailable),
        };
        transaction
            .commit()
            .map_err(|_| DeviceTrustRuntimeFenceError::RecoveryUncertain)?;
        result
    }
}

fn recover_committed(
    transaction: &Transaction<'_>,
    fence: &AuthorityReadFence,
    stored: super::StoredReservation,
    target: super::DeviceTrustRuntimeFenceTarget,
) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
    let digest = stored
        .outcome_digest
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)?;
    if digest != digest::outcome_digest(&stored.operation_id, &stored.reservation_ref, &target) {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    let _current = target::current_target_in_transaction(transaction, &target, fence)?;
    Ok(DeviceTrustRuntimeFenceOutcome {
        operation_id: stored.operation_id,
        reservation_ref: stored.reservation_ref,
        outcome_digest: digest,
        target,
    })
}
