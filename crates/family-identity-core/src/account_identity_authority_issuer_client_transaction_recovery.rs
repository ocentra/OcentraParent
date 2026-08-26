use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{params, OptionalExtension, Transaction};

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::AccountIdentityAuthorityIssuerClientError;
use super::reservation::{
    lease_expires_at, request_digest, sql_generation, timestamp, RESERVATION_ISSUED,
    RESERVATION_MANUAL_REQUIRED, RESERVATION_PREPARED, RESERVATION_SIGNING, SIGNER_IN_FLIGHT,
    SIGNER_NOT_STARTED, SIGNER_SUCCEEDED, SIGNER_UNCERTAIN,
};

const RECONCILE_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                 SET reservation_state = ?1, signer_status = ?2,
                     uncertain_at = ?3, lease_expires_at = ?3
               WHERE reservation_id IN (
                   SELECT reservation_id
                     FROM account_identity_issuer_v2_reservation
                    WHERE reservation_state = ?4
                    ORDER BY reservation_id
                    LIMIT ?5
               )";
const DELETE_EXPIRED_PREPARED_SQL: &str = "DELETE FROM account_identity_issuer_v2_reservation
               WHERE reservation_id IN (
                   SELECT reservation_id
                     FROM account_identity_issuer_v2_reservation
                    WHERE reservation_state = ?1 AND lease_expires_at <= ?2
                    ORDER BY reservation_id
                    LIMIT ?3
               )";
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
const COMPACT_ISSUED_SQL: &str = "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_id IN (
                  SELECT reservation.reservation_id
                    FROM account_identity_issuer_v2_reservation AS reservation
                    JOIN account_identity_issuer_v2_receipt AS receipt
                      ON receipt.receipt_id = reservation.receipt_id
                    JOIN account_identity_issuer_v2_outbox AS outbox
                      ON outbox.receipt_id = receipt.receipt_id
                   WHERE reservation.reservation_state = ?1
                     AND reservation.signer_status = ?2
                     AND receipt.provenance_state = 'exact'
                     AND receipt.receipt_state IN ('issued','acknowledged')
                     AND receipt.account_id = reservation.account_id
                     AND receipt.household_id = reservation.household_id
                     AND receipt.provider = reservation.provider
                     AND receipt.provider_subject = reservation.provider_subject
                     AND receipt.service = reservation.service
                     AND receipt.service_binding_id = reservation.service_binding_id
                     AND receipt.key_id = reservation.key_id
                     AND receipt.key_generation = reservation.key_generation
                     AND receipt.enrollment_generation = reservation.enrollment_generation
                     AND receipt.authority_generation = reservation.authority_generation
                     AND receipt.session_generation = reservation.session_generation
                     AND receipt.correlation_id = reservation.correlation_id
                     AND receipt.idempotency_key = reservation.idempotency_key
                     AND outbox.account_id = reservation.account_id
                     AND outbox.household_id = reservation.household_id
                     AND outbox.service = reservation.service
                     AND outbox.service_binding_id = reservation.service_binding_id
                     AND outbox.key_id = reservation.key_id
                     AND outbox.key_generation = reservation.key_generation
                     AND outbox.enrollment_generation = reservation.enrollment_generation
                     AND outbox.authority_generation = reservation.authority_generation
                     AND outbox.wire = receipt.wire
                   ORDER BY reservation.reservation_id
                   LIMIT ?3
              )";
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
const RECOVERY_BATCH_SIZE: i64 = 256;

pub(super) fn reconcile_issue_reservations(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    transaction
        .execute(
            COMPACT_ISSUED_SQL,
            params![RESERVATION_ISSUED, SIGNER_SUCCEEDED, RECOVERY_BATCH_SIZE],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    transaction
        .execute(
            DELETE_EXPIRED_PREPARED_SQL,
            params![RESERVATION_PREPARED, now_text, RECOVERY_BATCH_SIZE],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    transaction
        .execute(
            RECONCILE_SQL,
            params![
                RESERVATION_MANUAL_REQUIRED,
                SIGNER_UNCERTAIN,
                now_text,
                RESERVATION_SIGNING,
                RECOVERY_BATCH_SIZE
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    let backlog: bool = transaction
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_reservation
                 WHERE reservation_state = ?1 AND lease_expires_at <= ?2
                 LIMIT 1
             ) OR EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_reservation
                 WHERE reservation_state = ?3
                 LIMIT 1
             )",
            params![RESERVATION_PREPARED, now_text, RESERVATION_SIGNING],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    Ok(backlog)
}

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
                RESERVATION_SIGNING
            ],
        )
        .map(|_| ())
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}
