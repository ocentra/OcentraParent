use rusqlite::{params, Transaction};

use super::{
    storage, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceParticipant,
    DeviceTrustRuntimeFenceReservation,
};

impl DeviceTrustRuntimeFenceParticipant<'_> {
    /// Abort a prepared reservation without resolving a replacement authority.
    /// Abort is idempotent for the same opaque handle and never changes a
    /// committed outcome back to an uncommitted state.
    pub(crate) fn abort(
        &mut self,
        reservation: &DeviceTrustRuntimeFenceReservation,
    ) -> Result<(), DeviceTrustRuntimeFenceError> {
        let transaction = self.repository.transaction()?;
        let stored = storage::read_reservation(&transaction, &reservation.operation_id)?
            .ok_or(DeviceTrustRuntimeFenceError::ReservationMissing)?;
        if stored.reservation_ref != reservation.reservation_ref
            || storage::target(&stored)? != reservation.target
        {
            return Err(DeviceTrustRuntimeFenceError::TargetMismatch);
        }
        abort_stored(transaction, stored.state.as_str(), reservation)
    }
}

fn abort_stored(
    transaction: Transaction<'_>,
    state: &str,
    reservation: &DeviceTrustRuntimeFenceReservation,
) -> Result<(), DeviceTrustRuntimeFenceError> {
    match state {
        "prepared" => {
            let changed = transaction
                .execute(
                    "UPDATE device_trust_runtime_fence_reservation
                     SET reservation_state = 'aborted', outcome_digest = NULL
                     WHERE operation_id = ?1 AND reservation_ref = ?2
                       AND reservation_state = 'prepared'",
                    params![reservation.operation_id, reservation.reservation_ref],
                )
                .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
            if changed != 1 {
                return Err(DeviceTrustRuntimeFenceError::RecoveryUncertain);
            }
        }
        "aborted" => {}
        "committed" => return Err(DeviceTrustRuntimeFenceError::ReservationAlreadyCommitted),
        _ => return Err(DeviceTrustRuntimeFenceError::Unavailable),
    }
    transaction
        .commit()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)
}
