//! Protected signing-capability boundary.
//!
//! The owner never accepts a caller-implemented signer. A later broker or
//! Windows custody adapter supplies an opaque capability carrying a signed
//! request proof; this module binds that proof to the exact request bytes and
//! lets the family producer perform the fixed P-256 verification.

use ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityIssuerSignedIssue;
use ocentra_protected_capability_custody_core::account_issuer::{
    AccountIssuerP256Signer, AccountIssuerP256SignerError,
};
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_SIGNING_ERROR;

use crate::contract::{
    IssueCurrentAuthorityCommand, PreparedAccountIssuerV2Request, SignedAccountIssuerV2Envelope,
};
use crate::rpc::{
    AccountIssuerOwner, AccountIssuerPreparation, AccountIssuerRpcError, IssuePreparation,
    IssuedAuthority,
};

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
    /// Execute the complete owner-controlled issue lifecycle. The prepared
    /// request never crosses this crate's public boundary; protected signer
    /// failure is consumed into the exact durable manual-required reservation
    /// instead of leaving a live signing lease behind.
    pub fn issue_current_authority_with_protected_signer(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
        signer: &AccountIssuerP256Signer,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        match self.prepare_current_authority(provider, provider_subject, command)? {
            AccountIssuerPreparation::Replay(authority) => Ok(authority),
            AccountIssuerPreparation::Prepared(prepared) => {
                let signed = self.sign_prepared(prepared, signer)?;
                self.finalize_prepared_current_authority(provider, provider_subject, signed)
            }
        }
    }

    /// Prepare one request for the owner-controlled consuming lifecycle.
    pub(crate) fn prepare_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<AccountIssuerPreparation, AccountIssuerRpcError> {
        let prepared = match self.prepare_issue(provider, provider_subject, command)? {
            IssuePreparation::Replay(transport) => {
                return Ok(AccountIssuerPreparation::Replay(Self::replayed_authority(
                    transport,
                )?))
            }
            IssuePreparation::Request(prepared) => prepared,
        };
        Ok(AccountIssuerPreparation::Prepared(
            PreparedAccountIssuerV2Request::from_inner(prepared),
        ))
    }

    /// Finalize an owner-prepared request after the protected signer has
    /// produced its exact capability, then re-resolve currentness and
    /// atomically record the resulting transport in the same repository.
    pub(crate) fn finalize_prepared_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        signed: SignedAccountIssuerV2Envelope,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let signed = finalize_signed_envelope(signed).map_err(AccountIssuerRpcError::Signing)?;
        self.record_issued_transport(provider, provider_subject, signed)
    }
}

fn finalize_signed_envelope(
    signed: SignedAccountIssuerV2Envelope,
) -> Result<AccountIdentityIssuerSignedIssue, AccountIssuerSigningError> {
    let (prepared, capability) = signed.into_parts();
    let signature = capability
        .into_signature_for(prepared.request())
        .map_err(|_| AccountIssuerSigningError::Rejected)?;
    prepared
        .finalize_with_signature(signature)
        .map_err(|_| AccountIssuerSigningError::Rejected)
}

pub(crate) fn map_signer_error(error: AccountIssuerP256SignerError) -> AccountIssuerSigningError {
    match error {
        AccountIssuerP256SignerError::DeploymentRequired => {
            AccountIssuerSigningError::OwnerUnavailable
        }
        AccountIssuerP256SignerError::Rejected => AccountIssuerSigningError::Rejected,
    }
}

pub(crate) fn fail_closed() -> AccountIssuerSigningError {
    AccountIssuerSigningError::OwnerUnavailable
}
