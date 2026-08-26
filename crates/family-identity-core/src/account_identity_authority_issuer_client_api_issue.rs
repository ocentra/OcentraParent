use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use super::super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction, AccountIdentityIssuerCurrentness,
    AccountIdentityIssuerIssuePreparation, AccountIdentityIssuerPreparedIssue,
    AccountIdentityIssuerSignedIssue,
};

impl AccountIdentityAuthorityIssuerClient {
    /// Prepare one exact Account-owned issue transition. The immediate
    /// transaction commits the prepared -> signing state before the caller's
    /// protected signer is invoked; only this opaque transition can finalize
    /// the resulting transport.
    pub fn prepare_issue_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError>
    {
        let currentness = self.resolve_current(provider, provider_subject)?;
        let transaction = self.begin_transaction()?;
        let preparation =
            prepare_and_commit(transaction, &currentness, correlation_id, idempotency_key)?;
        Ok(preparation)
    }

    /// Consume the family-owned prepared transition after the protected
    /// signer produced a transport. Receipt, outbox, and reservation
    /// finalization remain one SQLite transaction; no caller can supply raw
    /// reservation authority or drive an arbitrary state transition.
    pub fn finalize_issued_transport(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        signed: AccountIdentityIssuerSignedIssue,
    ) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError>
    {
        let currentness = self.resolve_current(provider, provider_subject)?;
        let (reservation, transport) = signed.into_parts();
        let transaction = self.begin_transaction()?;
        let recorded = finalize_and_commit(transaction, &currentness, reservation, &transport)?;
        Ok(recorded)
    }
}

fn prepare_and_commit(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError> {
    match prepare_issue_transition(&transaction, currentness, correlation_id, idempotency_key) {
        Ok(preparation) => {
            transaction.commit()?;
            Ok(preparation)
        }
        Err(error) => finish_error(transaction, error),
    }
}

fn finalize_and_commit(
    mut transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
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
        return Ok(AccountIdentityIssuerIssuePreparation::Replay(transport));
    }
    let (request, reservation) = transaction.prepare_issue_current_authority(
        currentness,
        correlation_id,
        idempotency_key,
    )?;
    Ok(AccountIdentityIssuerIssuePreparation::Prepared(
        AccountIdentityIssuerPreparedIssue {
            request,
            reservation,
        },
    ))
}
