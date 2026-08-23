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
        let stored = storage::read_reservation(&self.repository.connection, operation_id)?
            .ok_or(DeviceTrustRuntimeFenceError::ReservationMissing)?;
        let target = storage::target(&stored)?;
        match stored.state.as_str() {
            "prepared" => Err(DeviceTrustRuntimeFenceError::RecoveryUncertain),
            "aborted" => Err(DeviceTrustRuntimeFenceError::ReservationAborted),
            "committed" => recover_committed(self, stored, target),
            _ => Err(DeviceTrustRuntimeFenceError::Unavailable),
        }
    }
}

fn recover_committed(
    participant: &mut DeviceTrustRuntimeFenceParticipant<'_>,
    stored: super::StoredReservation,
    target: super::DeviceTrustRuntimeFenceTarget,
) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
    let digest = stored
        .outcome_digest
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)?;
    if digest != digest::outcome_digest(&stored.operation_id, &stored.reservation_ref, &target) {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    let current = target::current_target(participant.repository, &target)?;
    target::ensure_current(&target, &current)?;
    Ok(DeviceTrustRuntimeFenceOutcome {
        operation_id: stored.operation_id,
        reservation_ref: stored.reservation_ref,
        outcome_digest: digest,
        target,
    })
}
