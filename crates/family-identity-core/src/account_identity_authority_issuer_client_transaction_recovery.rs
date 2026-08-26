use rusqlite::{params, OptionalExtension, Transaction};

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::AccountIdentityAuthorityIssuerClientError;
use super::reservation::{
    lease_expires_at, timestamp, RESERVATION_ISSUED, RESERVATION_MANUAL_REQUIRED,
    RESERVATION_PREPARED, RESERVATION_SIGNING, SIGNER_IN_FLIGHT, SIGNER_NOT_STARTED,
    SIGNER_SUCCEEDED, SIGNER_UNCERTAIN,
};

const RECONCILE_SQL: &str = "UPDATE account_identity_issuer_v2_reservation
                SET reservation_state = ?1, signer_status = ?2,
                    uncertain_at = ?3, lease_expires_at = ?3
              WHERE reservation_state = ?4";
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

pub(super) fn reconcile_issue_reservations(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    transaction
        .execute(
            "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_state = ?1 AND lease_expires_at <= ?2",
            params![RESERVATION_PREPARED, now_text],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    transaction
        .execute(
            RECONCILE_SQL,
            params![
                RESERVATION_MANUAL_REQUIRED,
                SIGNER_UNCERTAIN,
                now_text,
                RESERVATION_SIGNING
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    Ok(())
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
