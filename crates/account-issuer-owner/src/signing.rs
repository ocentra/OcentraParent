//! Protected signing-capability boundary.
//!
//! The owner keeps the durable issue transition inside the family crate while
//! the protected Account signer is invoked for the exact request it receives.
//! There is no public prepare/finalize pair that a caller can abandon, and a
//! signer failure is durably converted to the family-owned manual-required
//! state before this method returns.

use ocentra_family_identity_core::account_identity_authority_issuer_client::{
    AccountIdentityAuthorityIssuerClientError,
};
use ocentra_family_identity_core::account_identity_authority_issuer_client::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use ocentra_protected_capability_custody_core::account_issuer::{
    AccountIssuerP256Signer, AccountIssuerP256SignerError,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_SIGNING_ERROR;

use crate::contract::{
    AccountIssuerReceiptView, AccountIssuerRequestAuthorization, IssueCurrentAuthorityCommand,
};
use crate::repository::AccountIssuerRepositoryError;
use crate::rpc::{AccountIssuerOwner, AccountIssuerRpcError, IssuedAuthority};

#[derive(Debug)]
pub enum AccountIssuerSigningError {
    OwnerUnavailable,
    Rejected,
}

impl std::fmt::Display for AccountIssuerSigningError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_SIGNING_ERROR)
    }
}

impl std::error::Error for AccountIssuerSigningError {}

impl AccountIssuerOwner {
    /// Execute the complete owner-controlled issue lifecycle. The family
    /// request is borrowed only by the protected signer callback and never
    /// crosses this crate's public boundary. Any signer failure is recorded by
    /// the family transaction before the error is returned.
    pub fn issue_current_authority_with_protected_signer(
        &mut self,
        authorization: &AccountIssuerRequestAuthorization,
        command: &IssueCurrentAuthorityCommand,
        signer: &AccountIssuerP256Signer,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let recorded = self
            .repository_mut()
            .issue_current_authority_with_signer(
                authorization.provider(),
                authorization.provider_subject(),
                command,
                |request| {
                    let capability = signer.sign_request(request).map_err(map_signer_error)?;
                    capability
                        .into_signature_for(request)
                        .map_err(map_signer_error)
                },
            )
            .map_err(map_repository_error)?;
        issued_authority(recorded)
    }
}

fn issued_authority(
    recorded: AccountIdentityIssuerRecordedTransport,
) -> Result<IssuedAuthority, AccountIssuerRpcError> {
    let receipt = AccountIssuerReceiptView::from_receipt(recorded.transport().receipt()).ok_or(
        AccountIssuerRpcError::Repository(AccountIssuerRepositoryError::InvalidSchema),
    )?;
    Ok(IssuedAuthority {
        receipt,
        replayed: recorded.replayed(),
    })
}

fn map_signer_error(
    error: AccountIssuerP256SignerError,
) -> AccountIdentityAuthorityIssuerClientError {
    match error {
        AccountIssuerP256SignerError::DeploymentRequired => {
            AccountIdentityAuthorityIssuerClientError::SigningUnavailable
        }
        AccountIssuerP256SignerError::Rejected => {
            AccountIdentityAuthorityIssuerClientError::SigningRejected
        }
    }
}

fn map_repository_error(error: AccountIssuerRepositoryError) -> AccountIssuerRpcError {
    match error {
        AccountIssuerRepositoryError::SigningUnavailable => {
            AccountIssuerRpcError::Signing(AccountIssuerSigningError::OwnerUnavailable)
        }
        AccountIssuerRepositoryError::SigningRejected | AccountIssuerRepositoryError::Producer => {
            AccountIssuerRpcError::Signing(AccountIssuerSigningError::Rejected)
        }
        error => AccountIssuerRpcError::Repository(error),
    }
}

pub(crate) fn fail_closed() -> AccountIssuerSigningError {
    AccountIssuerSigningError::OwnerUnavailable
}
