use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerIssuePreparation,
    AccountIdentityIssuerPreparedIssue,
};

pub(super) fn finalize_for_legacy(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    reservation: super::super::account_identity_authority_issuer_client_reservation::
        AccountIdentityIssuerReservation,
    transport: &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport,
) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError> {
    finalize_and_commit(transaction, currentness, reservation, transport)
}

pub(super) fn finish_for_legacy<T>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    error: AccountIdentityAuthorityIssuerClientError,
) -> Result<T, AccountIdentityAuthorityIssuerClientError> {
    finish_error(transaction, error)
}

pub(super) fn finish_signing_error<T>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    failure: Result<(), AccountIdentityAuthorityIssuerClientError>,
    error: AccountIdentityAuthorityIssuerClientError,
) -> Result<T, AccountIdentityAuthorityIssuerClientError> {
    match failure {
        Ok(()) => {
            transaction.commit()?;
            Err(error)
        }
        Err(failure_error) => finish_error(transaction, failure_error),
    }
}

pub(super) fn prepare_for_legacy(
    transaction: &AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError> {
    prepare_issue_transition(transaction, currentness, correlation_id, idempotency_key)
}

fn finalize_and_commit(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    reservation: super::super::account_identity_authority_issuer_client_reservation::
        AccountIdentityIssuerReservation,
    transport: &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport,
) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError> {
    match transaction.record_issued_transport(currentness, reservation, transport) {
        Ok(recorded) => {
            transaction.commit()?;
            Ok(recorded)
        }
        Err(error) => finish_error(transaction, error),
    }
}

fn finish_error<T>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    error: AccountIdentityAuthorityIssuerClientError,
) -> Result<T, AccountIdentityAuthorityIssuerClientError> {
    if super::super::is_manual_transition(&error) {
        transaction.commit()?;
    }
    Err(error)
}

fn prepare_issue_transition(
    transaction: &AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError> {
    if let Some(transport) =
        transaction.existing_issued_transport(currentness, correlation_id, idempotency_key)?
    {
        return Ok(AccountIdentityIssuerIssuePreparation::Replay(Box::new(
            transport,
        )));
    }
    let (request, reservation) = transaction.prepare_issue_current_authority(
        currentness,
        correlation_id,
        idempotency_key,
    )?;
    Ok(AccountIdentityIssuerIssuePreparation::Prepared(Box::new(
        AccountIdentityIssuerPreparedIssue {
            request,
            reservation,
        },
    )))
}
