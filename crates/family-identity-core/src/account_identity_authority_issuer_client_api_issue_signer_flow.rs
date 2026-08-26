use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use super::super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction, AccountIdentityIssuerCurrentness,
};

impl AccountIdentityAuthorityIssuerClient {
    /// Execute one complete Account-owned issue lifecycle while retaining the
    /// durable reservation inside this crate. The signer callback receives
    /// only the family-created request; preparation, finalization, and failure
    /// transitions cannot be abandoned by a caller between public calls.
    pub fn issue_current_authority_with_signer<F>(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        correlation_id: &str,
        idempotency_key: &str,
        signer: F,
    ) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError>
    where
        F: FnOnce(
            &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request,
        ) -> Result<
            [u8; ocentra_schema::account_identity_authority_producer_v2::
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
            AccountIdentityAuthorityIssuerClientError,
        >,
    {
        let currentness = self.resolve_current(provider, provider_subject)?;
        let transaction = self.begin_transaction()?;
        issue_with_transaction(
            transaction,
            &currentness,
            correlation_id,
            idempotency_key,
            signer,
        )
    }
}

fn issue_with_transaction<F>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    correlation_id: &str,
    idempotency_key: &str,
    signer: F,
) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError>
where
    F: FnOnce(
        &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request,
    ) -> Result<
        [u8; ocentra_schema::account_identity_authority_producer_v2::
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
        AccountIdentityAuthorityIssuerClientError,
    >,
{
    let preparation = match super::issue_signer_transitions::prepare_for_legacy(
        &transaction,
        currentness,
        correlation_id,
        idempotency_key,
    ) {
        Ok(preparation) => preparation,
        Err(error) => {
            return super::issue_signer_transitions::finish_for_legacy(transaction, error)
        }
    };
    let prepared = match preparation {
        super::super::AccountIdentityIssuerIssuePreparation::Replay(transport) => {
            transaction.commit()?;
            return Ok(AccountIdentityIssuerRecordedTransport {
                transport,
                replayed: true,
            });
        }
        super::super::AccountIdentityIssuerIssuePreparation::Prepared(prepared) => prepared,
    };
    let (request, reservation) = prepared.into_parts();
    let (transaction, signature) = super::issue_signer_failure::sign_or_record_failure(
        transaction,
        &request,
        &reservation,
        signer,
    )?;
    let (transaction, transport) = super::issue_signer_failure::finalize_or_record_failure(
        transaction,
        request,
        &reservation,
        signature,
    )?;
    super::issue_signer_transitions::finalize_for_legacy(
        transaction,
        currentness,
        reservation,
        &transport,
    )
}
