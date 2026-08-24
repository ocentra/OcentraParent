use rusqlite::{params, Transaction};

use crate::{
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
    device_trust_signer_registration_validation::random_receipt,
    household_authority::HouseholdAuthorityAction,
};

use super::{
    storage, target, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceParticipant,
    DeviceTrustRuntimeFenceReservation,
};

impl DeviceTrustRuntimeFenceParticipant<'_> {
    /// Prepare one exact action and current signer binding. The supplied
    /// binding is only a lookup hint: the participant immediately re-resolves
    /// the same identity and generation from its durable Device Trust owner.
    pub(crate) fn prepare(
        &mut self,
        operation_id: &str,
        action_value: HouseholdAuthorityAction,
        binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<DeviceTrustRuntimeFenceReservation, DeviceTrustRuntimeFenceError> {
        storage::validate_schema(&self.repository.connection)?;
        storage::validate_operation(operation_id)?;
        let expected = target::from_binding(action_value, binding)?;
        let fence = self.repository.external_authority.read_fence()?;
        let transaction = self.repository.transaction()?;
        let _current = target::current_target_in_transaction(&transaction, &expected, &fence)?;
        let existing = storage::read_reservation(&transaction, operation_id)?;
        if let Some(existing) = existing {
            return prepare_existing(transaction, existing, expected);
        }

        let reservation_ref = random_receipt().map_err(DeviceTrustRuntimeFenceError::from)?;
        transaction
            .execute(
                "INSERT INTO device_trust_runtime_fence_reservation
                 (operation_id, reservation_ref, action_code, family_id, trust_subject,
                  parent_device_id, child_device_id, installation_id, signer_key_id,
                  signer_key_sha256, lifecycle_generation, installation_binding_generation,
                  authority_generation, reservation_state, outcome_digest)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'prepared', NULL)",
                params![
                    operation_id,
                    reservation_ref,
                    expected.action_code,
                    expected.family_id,
                    expected.trust_subject,
                    expected.parent_device_id,
                    expected.child_device_id,
                    expected.installation_id,
                    expected.signer_key_id,
                    expected.signer_key_sha256,
                    storage::to_sql_generation(expected.lifecycle_generation)?,
                    storage::to_sql_generation(expected.installation_binding_generation)?,
                    storage::to_sql_generation(expected.authority_generation)?,
                ],
            )
            .map_err(|error| storage::classify_insert_error(&transaction, operation_id, error))?;
        transaction
            .commit()
            .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
        Ok(DeviceTrustRuntimeFenceReservation {
            operation_id: operation_id.to_owned(),
            reservation_ref,
            target: expected,
        })
    }
}

fn prepare_existing(
    transaction: Transaction<'_>,
    existing: super::StoredReservation,
    expected: super::DeviceTrustRuntimeFenceTarget,
) -> Result<DeviceTrustRuntimeFenceReservation, DeviceTrustRuntimeFenceError> {
    let existing_target = storage::target(&existing)?;
    if existing_target != expected {
        return Err(DeviceTrustRuntimeFenceError::OperationConflict);
    }
    let result = match existing.state.as_str() {
        "prepared" => Ok(DeviceTrustRuntimeFenceReservation {
            operation_id: existing.operation_id,
            reservation_ref: existing.reservation_ref,
            target: existing_target,
        }),
        "committed" => Err(DeviceTrustRuntimeFenceError::ReservationAlreadyCommitted),
        "aborted" => Err(DeviceTrustRuntimeFenceError::ReservationAborted),
        _ => Err(DeviceTrustRuntimeFenceError::Unavailable),
    };
    transaction
        .commit()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    result
}
