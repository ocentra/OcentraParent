use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use rusqlite::Transaction;

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::AccountIdentityAuthorityIssuerClientError;

#[path = "account_identity_authority_issuer_client_transaction_recovery_reconcile.rs"]
mod reconcile;
#[path = "account_identity_authority_issuer_client_transaction_recovery_storage.rs"]
mod storage;
#[path = "account_identity_authority_issuer_client_transaction_recovery_transitions.rs"]
mod transitions;
#[path = "account_identity_authority_issuer_client_transaction_recovery_validation.rs"]
mod validation;

const RECOVERY_BATCH_SIZE: i64 = 256;

pub(super) struct RecoveryReservation {
    reservation_id: String,
    account_id: String,
    household_id: String,
    provider: String,
    provider_subject: String,
    service: String,
    service_binding_id: String,
    key_id: String,
    key_generation: i64,
    enrollment_generation: i64,
    authority_generation: i64,
    session_generation: i64,
    correlation_id: String,
    idempotency_key: String,
    request_digest: String,
    request_wire: Vec<u8>,
    reservation_state: String,
    signer_status: String,
    attempt_token: String,
    lease_expires_at: String,
}

pub(super) fn reconcile_issue_reservations(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    reconcile::reconcile_issue_reservations(transaction, now)
}

pub(super) fn mark_signing(
    transaction: &Transaction<'_>,
    reservation: &AccountIdentityIssuerReservation,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transitions::mark_signing(transaction, reservation, now)
}

pub(super) fn mark_signing_failure(
    transaction: &Transaction<'_>,
    request: &AccountIdentityAuthorityProducerV2Request,
    reservation: &AccountIdentityIssuerReservation,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transitions::mark_signing_failure(transaction, request, reservation, now)
}

pub(super) fn mark_issued(
    transaction: &Transaction<'_>,
    reservation: &AccountIdentityIssuerReservation,
    receipt_id: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transitions::mark_issued(transaction, reservation, receipt_id)
}

pub(super) fn mark_manual_required(
    transaction: &Transaction<'_>,
    reservation_id: &str,
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    transitions::mark_manual_required(transaction, reservation_id, now_text)
}
