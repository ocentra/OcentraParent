use rusqlite::{params, Transaction};

use super::{
    digest, storage, target, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceOutcome,
    DeviceTrustRuntimeFenceParticipant, DeviceTrustRuntimeFenceReservation,
};

impl DeviceTrustRuntimeFenceParticipant<'_> {
    /// Commit only after re-resolving the exact current signer binding. A
    /// stale/revoked binding cannot produce an outcome; the prepared row is
    /// left for the coordinator's explicit abort or recovery-uncertainty path.
    pub(crate) fn commit(
        &mut self,
        reservation: &DeviceTrustRuntimeFenceReservation,
    ) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
        let current = target::current_target(self.repository, &reservation.target)?;
        target::ensure_current(&reservation.target, &current)?;
        let transaction = self.repository.transaction()?;
        let stored = storage::read_reservation(&transaction, &reservation.operation_id)?
            .ok_or(DeviceTrustRuntimeFenceError::ReservationMissing)?;
        let stored_target = storage::target(&stored)?;
        if stored.reservation_ref != reservation.reservation_ref
            || stored_target != reservation.target
        {
            return Err(DeviceTrustRuntimeFenceError::TargetMismatch);
        }
        commit_stored(transaction, stored, reservation)
    }
}

fn commit_stored(
    transaction: Transaction<'_>,
    stored: super::StoredReservation,
    reservation: &DeviceTrustRuntimeFenceReservation,
) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
    match stored.state.as_str() {
        "prepared" => commit_prepared(transaction, reservation),
        "committed" => commit_existing(transaction, stored, reservation),
        "aborted" => Err(DeviceTrustRuntimeFenceError::ReservationAborted),
        _ => Err(DeviceTrustRuntimeFenceError::Unavailable),
    }
}

fn commit_prepared(
    transaction: Transaction<'_>,
    reservation: &DeviceTrustRuntimeFenceReservation,
) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
    let outcome_digest = digest::outcome_digest(
        &reservation.operation_id,
        &reservation.reservation_ref,
        &reservation.target,
    );
    let changed = transaction
        .execute(
            "UPDATE device_trust_runtime_fence_reservation
             SET reservation_state = 'committed', outcome_digest = ?1
             WHERE operation_id = ?2 AND reservation_ref = ?3
               AND reservation_state = 'prepared'",
            params![
                outcome_digest,
                reservation.operation_id,
                reservation.reservation_ref
            ],
        )
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    if changed != 1 {
        return Err(DeviceTrustRuntimeFenceError::RecoveryUncertain);
    }
    transaction
        .commit()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    Ok(DeviceTrustRuntimeFenceOutcome {
        operation_id: reservation.operation_id.clone(),
        reservation_ref: reservation.reservation_ref.clone(),
        outcome_digest,
        target: target::clone_target(&reservation.target),
    })
}

fn commit_existing(
    transaction: Transaction<'_>,
    stored: super::StoredReservation,
    reservation: &DeviceTrustRuntimeFenceReservation,
) -> Result<DeviceTrustRuntimeFenceOutcome, DeviceTrustRuntimeFenceError> {
    let digest = stored
        .outcome_digest
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)?;
    if digest
        != digest::outcome_digest(
            &reservation.operation_id,
            &reservation.reservation_ref,
            &reservation.target,
        )
    {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    transaction
        .commit()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    Ok(DeviceTrustRuntimeFenceOutcome {
        operation_id: reservation.operation_id.clone(),
        reservation_ref: reservation.reservation_ref.clone(),
        outcome_digest: digest,
        target: target::clone_target(&reservation.target),
    })
}
