use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport;
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Receipt, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES,
};

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::{AccountIdentityAuthorityIssuerClientError, AccountIdentityIssuerCurrentness};
use super::reservation::{
    request_digest, RESERVATION_ISSUED, RESERVATION_MANUAL_REQUIRED, RESERVATION_SIGNING,
    SIGNER_IN_FLIGHT,
};

type ReservationRow = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    i64,
    i64,
    i64,
    String,
    String,
    String,
    Vec<u8>,
    String,
    String,
    String,
    String,
);

pub(super) fn validate_signing_reservation(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    reservation: &AccountIdentityIssuerReservation,
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let (_, now_text) = super::super::clock::now(transaction)?;
    let row = load_reservation(transaction, reservation.reservation_id())?
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    validate_binding(&row, currentness, reservation)?;
    validate_wire(&row.14, transport)?;
    validate_receipt(&row, transport.receipt())?;
    validate_state(&row, &now_text, reservation.reservation_id(), transaction)
}

fn load_reservation(
    transaction: &Transaction<'_>,
    reservation_id: &str,
) -> Result<Option<ReservationRow>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            "SELECT account_id, household_id, provider, provider_subject, service,
                    service_binding_id, key_id, key_generation, enrollment_generation,
                    authority_generation, session_generation, correlation_id,
                    idempotency_key, request_digest, request_wire, reservation_state,
                    signer_status, attempt_token, lease_expires_at
               FROM account_identity_issuer_v2_reservation
              WHERE reservation_id = ?1",
            [reservation_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                    row.get(10)?,
                    row.get(11)?,
                    row.get(12)?,
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                    row.get(16)?,
                    row.get(17)?,
                    row.get(18)?,
                ))
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn validate_state(
    row: &ReservationRow,
    now_text: &str,
    reservation_id: &str,
    transaction: &Transaction<'_>,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if row.15 != RESERVATION_SIGNING || row.16 != SIGNER_IN_FLIGHT {
        return match row.15.as_str() {
            RESERVATION_MANUAL_REQUIRED => {
                Err(AccountIdentityAuthorityIssuerClientError::ManualRequired)
            }
            RESERVATION_ISSUED => Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected),
            _ => Err(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable),
        };
    }
    if row.18.as_str() <= now_text {
        mark_manual_required(transaction, reservation_id, now_text)?;
        return Err(AccountIdentityAuthorityIssuerClientError::ReservationExpired);
    }
    Ok(())
}

fn validate_binding(
    row: &ReservationRow,
    currentness: &AccountIdentityIssuerCurrentness,
    reservation: &AccountIdentityIssuerReservation,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let expected_provider = super::provider_label(currentness.authority().provider());
    let expected_subject = currentness.authority().provider_subject().as_str();
    let matches = [
        row.0 == currentness.account_id().as_str(),
        row.1 == currentness.household_id().as_str(),
        row.2 == expected_provider,
        row.3 == expected_subject,
        row.4 == ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
        row.5 == reservation.service_binding_id(),
        row.6 == reservation.key_id(),
        generation_matches(row.7, reservation.key_generation()),
        generation_matches(row.8, reservation.enrollment_generation()),
        generation_matches(row.9, reservation.authority_generation()),
        generation_matches(row.10, reservation.session_generation()),
        row.11 == reservation.correlation_id(),
        row.12 == reservation.idempotency_key(),
        row.13 == request_digest(row.14.as_slice()),
        row.17 == reservation.attempt_token(),
        row.14 == reservation.request_wire(),
        reservation.account_id() == row.0,
        reservation.household_id() == row.1,
        reservation.provider() == row.2,
        reservation.provider_subject() == row.3,
    ];
    matches
        .into_iter()
        .all(std::convert::identity)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReplayDetected)
}

fn validate_wire(
    request_wire: &[u8],
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let wire = transport.wire_bytes();
    (request_wire.len() + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES == wire.len()
        && wire[..request_wire.len()] == *request_wire)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReplayDetected)
}

fn validate_receipt(
    row: &ReservationRow,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let matches = [
        receipt.account_id == row.0,
        receipt.household_id == row.1,
        receipt.service_binding_id == row.5,
        receipt.key_id == row.6,
        generation_matches(row.7, receipt.key_generation),
        generation_matches(row.8, receipt.enrollment_generation),
        generation_matches(row.9, receipt.authority_generation),
        generation_matches(row.10, receipt.session_generation),
        receipt.correlation_id == row.11,
        receipt.idempotency_key == row.12,
    ];
    matches
        .into_iter()
        .all(std::convert::identity)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReplayDetected)
}

fn mark_manual_required(
    transaction: &Transaction<'_>,
    reservation_id: &str,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transaction
        .execute(
            "UPDATE account_identity_issuer_v2_reservation
                SET reservation_state = ?1, signer_status = ?2,
                    uncertain_at = ?3, lease_expires_at = ?3
              WHERE reservation_id = ?4 AND reservation_state = ?5",
            params![
                RESERVATION_MANUAL_REQUIRED,
                super::reservation::SIGNER_UNCERTAIN,
                now_text,
                reservation_id,
                RESERVATION_SIGNING
            ],
        )
        .map(|_| ())
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn generation_matches(stored: i64, expected: u64) -> bool {
    i64::try_from(expected).is_ok_and(|expected| stored == expected)
}
