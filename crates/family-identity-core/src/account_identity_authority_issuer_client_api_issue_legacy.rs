use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use super::super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction, AccountIdentityIssuerCurrentness,
    AccountIdentityIssuerIssuePreparation, AccountIdentityIssuerPreparedIssue,
    AccountIdentityIssuerSignedIssue,
};

impl AccountIdentityAuthorityIssuerClient {
    pub(crate) fn prepare_issue_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError>
    {
        let currentness = self.resolve_current(provider, provider_subject)?;
        let transaction = self.begin_transaction()?;
        prepare_and_commit(transaction, &currentness, correlation_id, idempotency_key)
    }

    pub(crate) fn finalize_issued_transport(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        signed: AccountIdentityIssuerSignedIssue,
    ) -> Result<
        super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport,
        AccountIdentityAuthorityIssuerClientError,
    >{
        let currentness = self.resolve_current(provider, provider_subject)?;
        let (reservation, transport) = signed.into_parts();
        let transaction = self.begin_transaction()?;
        super::issue_signer_transitions::finalize_for_legacy(
            transaction,
            &currentness,
            reservation,
            &transport,
        )
    }

    pub(crate) fn record_signing_failure(
        &mut self,
        prepared: AccountIdentityIssuerPreparedIssue,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        let (request, reservation) = prepared.into_parts();
        let transaction = self.begin_transaction()?;
        match transaction.record_signing_failure(&request, &reservation) {
            Ok(()) => transaction.commit(),
            Err(error) => super::issue_signer_transitions::finish_for_legacy(transaction, error),
        }
    }
}

fn prepare_and_commit(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<AccountIdentityIssuerIssuePreparation, AccountIdentityAuthorityIssuerClientError> {
    match super::issue_signer_transitions::prepare_for_legacy(
        &transaction,
        currentness,
        correlation_id,
        idempotency_key,
    ) {
        Ok(preparation) => {
            transaction.commit()?;
            Ok(preparation)
        }
        Err(error) => super::issue_signer_transitions::finish_for_legacy(transaction, error),
    }
}
