use rusqlite::{params, Transaction};

use super::super::super::AccountIdentityAuthorityIssuerClientError;
use super::RecoveryReservation;

const RESERVATION_MANUAL_REQUIRED: &str = "manual-required";
const SIGNER_UNCERTAIN: &str = "uncertain";

pub(super) fn delete_valid_prepared(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_id = ?1 AND account_id = ?2 AND household_id = ?3
                AND provider = ?4 AND provider_subject = ?5 AND service = ?6
                AND service_binding_id = ?7 AND key_id = ?8
                AND key_generation = ?9 AND enrollment_generation = ?10
                AND authority_generation = ?11 AND session_generation = ?12
                AND correlation_id = ?13 AND idempotency_key = ?14
                AND request_digest = ?15 AND request_wire = ?16
                AND reservation_state = ?17 AND signer_status = ?18
                AND attempt_token = ?19 AND lease_expires_at <= ?20",
            reservation_params(candidate, now_text),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

pub(super) fn mark_valid_signing_manual(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    mark_manual_with_exact_row(transaction, candidate, now_text)
}

pub(super) fn mark_invalid_manual(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    mark_manual_with_exact_row(transaction, candidate, now_text)
}

fn mark_manual_with_exact_row(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_reservation
                SET reservation_state = ?1, signer_status = ?2,
                    signing_started_at = COALESCE(signing_started_at, ?3),
                    uncertain_at = ?3, lease_expires_at = ?3
              WHERE reservation_id = ?4 AND account_id = ?5 AND household_id = ?6
                AND provider = ?7 AND provider_subject = ?8 AND service = ?9
                AND service_binding_id = ?10 AND key_id = ?11
                AND key_generation = ?12 AND enrollment_generation = ?13
                AND authority_generation = ?14 AND session_generation = ?15
                AND correlation_id = ?16 AND idempotency_key = ?17
                AND request_digest = ?18 AND request_wire = ?19
                AND attempt_token = ?20 AND reservation_state = ?21
                AND signer_status = ?22",
            params![
                RESERVATION_MANUAL_REQUIRED,
                SIGNER_UNCERTAIN,
                now_text,
                candidate.reservation_id,
                candidate.account_id,
                candidate.household_id,
                candidate.provider,
                candidate.provider_subject,
                candidate.service,
                candidate.service_binding_id,
                candidate.key_id,
                candidate.key_generation,
                candidate.enrollment_generation,
                candidate.authority_generation,
                candidate.session_generation,
                candidate.correlation_id,
                candidate.idempotency_key,
                candidate.request_digest,
                candidate.request_wire,
                candidate.attempt_token,
                candidate.reservation_state,
                candidate.signer_status,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn reservation_params<'a>(
    candidate: &'a RecoveryReservation,
    now_text: &'a str,
) -> rusqlite::ParamsFromIter<std::array::IntoIter<rusqlite::types::Value, 20>> {
    let values: [rusqlite::types::Value; 20] = [
        candidate.reservation_id.clone().into(),
        candidate.account_id.clone().into(),
        candidate.household_id.clone().into(),
        candidate.provider.clone().into(),
        candidate.provider_subject.clone().into(),
        candidate.service.clone().into(),
        candidate.service_binding_id.clone().into(),
        candidate.key_id.clone().into(),
        candidate.key_generation.into(),
        candidate.enrollment_generation.into(),
        candidate.authority_generation.into(),
        candidate.session_generation.into(),
        candidate.correlation_id.clone().into(),
        candidate.idempotency_key.clone().into(),
        candidate.request_digest.clone().into(),
        candidate.request_wire.clone().into(),
        candidate.reservation_state.clone().into(),
        candidate.signer_status.clone().into(),
        candidate.attempt_token.clone().into(),
        now_text.to_owned().into(),
    ];
    rusqlite::params_from_iter(values.into_iter())
}
