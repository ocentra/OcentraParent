use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;

pub(super) fn sign_or_record_failure<'a, F>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'a>,
    request: &AccountIdentityAuthorityProducerV2Request,
    reservation: &AccountIdentityIssuerReservation,
    signer: F,
) -> Result<
    (
        AccountIdentityAuthorityIssuerTransaction<'a>,
        [u8; ocentra_schema::account_identity_authority_producer_v2::
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
    ),
    AccountIdentityAuthorityIssuerClientError,
>
where
    F: FnOnce(
        &AccountIdentityAuthorityProducerV2Request,
    ) -> Result<
        [u8; ocentra_schema::account_identity_authority_producer_v2::
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
        AccountIdentityAuthorityIssuerClientError,
    >,
{
    match signer(request) {
        Ok(signature) => Ok((transaction, signature)),
        Err(error) => {
            let failure = transaction.record_signing_failure(request, reservation);
            super::issue_signer_transitions::finish_signing_error(transaction, failure, error)
        }
    }
}

pub(super) fn finalize_or_record_failure<'a>(
    transaction: AccountIdentityAuthorityIssuerTransaction<'a>,
    request: AccountIdentityAuthorityProducerV2Request,
    reservation: &AccountIdentityIssuerReservation,
    signature: [u8; ocentra_schema::account_identity_authority_producer_v2::
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
) -> Result<
    (
        AccountIdentityAuthorityIssuerTransaction<'a>,
        crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport,
    ),
    AccountIdentityAuthorityIssuerClientError,
> {
    match request.finalize_preserving(signature) {
        Ok(transport) => Ok((transaction, transport)),
        Err((request, error)) => {
            let failure = transaction.record_signing_failure(&request, reservation);
            super::issue_signer_transitions::finish_signing_error(
                transaction,
                failure,
                AccountIdentityAuthorityIssuerClientError::Producer(error),
            )
        }
    }
}
