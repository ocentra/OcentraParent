use super::super::account_identity_authority_issuer_client_owner_admission::AccountIdentityIssuerOwnerAdmission;
use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use super::super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction, AccountIdentityIssuerCurrentness,
};

impl AccountIdentityAuthorityIssuerClient {
    /// Execute one complete Account-owned issue lifecycle after consuming a
    /// non-mintable admission for this exact authority, key, and transport
    /// session. Admission is revalidated before any reservation lookup,
    /// capacity query, or write. The signer callback receives only the
    /// family-created request.
    pub fn issue_current_authority_with_account_owner_admission<F>(
        &mut self,
        admission: AccountIdentityIssuerOwnerAdmission,
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
        let currentness =
            self.resolve_current(admission.provider(), admission.provider_subject())?;
        admission.validate_currentness(&currentness, correlation_id, idempotency_key)?;
        let transaction = self.begin_transaction()?;
        transaction.validate_owner_admission(
            &currentness,
            &admission,
            correlation_id,
            idempotency_key,
        )?;
        drop(admission);
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
            let key = transaction.current_key(currentness)?;
            let recorded = AccountIdentityIssuerRecordedTransport::from_verified_currentness(
                currentness,
                &key,
                *transport,
                true,
            );
            transaction.commit()?;
            return Ok(recorded);
        }
        super::super::AccountIdentityIssuerIssuePreparation::Prepared(prepared) => prepared,
    };
    let (request, reservation) = (*prepared).into_parts();
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
