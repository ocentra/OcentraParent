use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{params, OptionalExtension, Transaction};

use super::super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::super::AccountIdentityAuthorityIssuerClientError;
use super::super::reservation::{
    lease_expires_at, request_digest, sql_generation, timestamp, RESERVATION_ISSUED,
    RESERVATION_MANUAL_REQUIRED, RESERVATION_PREPARED, RESERVATION_SIGNING, SIGNER_IN_FLIGHT,
    SIGNER_NOT_STARTED, SIGNER_SUCCEEDED, SIGNER_UNCERTAIN,
};

const MARK_SIGNING_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                SET reservation_state = ?1, signer_status = ?2,
                    signing_started_at = ?3, lease_expires_at = ?4
              WHERE reservation_id = ?5 AND attempt_token = ?6
                AND reservation_state = ?7 AND signer_status = ?8
                AND lease_expires_at > ?3";
const MARK_ISSUED_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                SET reservation_state = ?1, signer_status = ?2, receipt_id = ?3
              WHERE reservation_id = ?4 AND attempt_token = ?5
                AND reservation_state = ?6 AND signer_status = ?7";
const MARK_MANUAL_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                 SET reservation_state = ?1, signer_status = ?2,
                     uncertain_at = ?3, lease_expires_at = ?3
                WHERE reservation_id = ?4 AND reservation_state = ?5";
const MARK_SIGNING_FAILURE_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                 SET reservation_state = ?1, signer_status = ?2,
                     uncertain_at = ?3, lease_expires_at = ?3
               WHERE reservation_id = ?4 AND attempt_token = ?5
                 AND account_id = ?6 AND household_id = ?7
                 AND provider = ?8 AND provider_subject = ?9
                 AND service = ?10 AND service_binding_id = ?11
                 AND key_id = ?12 AND key_generation = ?13
                 AND enrollment_generation = ?14 AND authority_generation = ?15
                 AND session_generation = ?16 AND correlation_id = ?17
                 AND idempotency_key = ?18 AND request_digest = ?19
                 AND request_wire = ?20
                 AND reservation_state = ?21 AND signer_status = ?22";

pub(super) fn mark_signing(
    transaction: &Transaction<'_>,
    reservation: &AccountIdentityIssuerReservation,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    let lease = lease_expires_at(now)?;
    let changed = transaction
        .execute(
            MARK_SIGNING_SQL,
            params![
                RESERVATION_SIGNING,
                SIGNER_IN_FLIGHT,
                now_text,
                lease,
                reservation.reservation_id(),
                reservation.attempt_token(),
                RESERVATION_PREPARED,
                SIGNER_NOT_STARTED,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    if changed == 1 {
        return Ok(());
    }
    reservation_state_error(transaction, reservation.reservation_id())
}

pub(super) fn mark_signing_failure(
    transaction: &Transaction<'_>,
    request: &AccountIdentityAuthorityProducerV2Request,
    reservation: &AccountIdentityIssuerReservation,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let request_wire = request.signing_bytes();
    if request_wire != reservation.request_wire() {
        return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
    }
    let binding = request.binding();
    let now_text = timestamp(now)?;
    let changed = transaction
        .execute(
            MARK_SIGNING_FAILURE_SQL,
            params![
                RESERVATION_MANUAL_REQUIRED,
                SIGNER_UNCERTAIN,
                now_text,
                reservation.reservation_id(),
                reservation.attempt_token(),
                binding.account_id,
                binding.household_id,
                reservation.provider(),
                reservation.provider_subject(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                binding.service_binding_id,
                binding.key_id,
                sql_generation(binding.key_generation)?,
                sql_generation(binding.enrollment_generation)?,
                sql_generation(binding.authority_generation)?,
                sql_generation(binding.session_generation)?,
                binding.correlation_id,
                binding.idempotency_key,
                request_digest(request_wire),
                request_wire,
                RESERVATION_SIGNING,
                SIGNER_IN_FLIGHT,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    if changed == 1 {
        return Ok(());
    }
    reservation_state_error(transaction, reservation.reservation_id())
}

pub(super) fn mark_issued(
    transaction: &Transaction<'_>,
    reservation: &AccountIdentityIssuerReservation,
    receipt_id: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            MARK_ISSUED_SQL,
            params![
                RESERVATION_ISSUED,
                SIGNER_SUCCEEDED,
                receipt_id,
                reservation.reservation_id(),
                reservation.attempt_token(),
                RESERVATION_SIGNING,
                SIGNER_IN_FLIGHT,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn reservation_state_error(
    transaction: &Transaction<'_>,
    reservation_id: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let state: Option<String> = transaction
        .query_row(
            "SELECT reservation_state FROM account_identity_issuer_v2_reservation
              WHERE reservation_id = ?1",
            [reservation_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    match state.as_deref() {
        Some(RESERVATION_MANUAL_REQUIRED) => {
            Err(AccountIdentityAuthorityIssuerClientError::ManualRequired)
        }
        Some(RESERVATION_ISSUED) => Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected),
        Some(_) | None => Err(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable),
    }
}

pub(super) fn mark_manual_required(
    transaction: &Transaction<'_>,
    reservation_id: &str,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transaction
        .execute(
            MARK_MANUAL_SQL,
            params![
                RESERVATION_MANUAL_REQUIRED,
                SIGNER_UNCERTAIN,
                now_text,
                reservation_id,
                RESERVATION_SIGNING,
            ],
        )
        .map(|_| ())
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}
