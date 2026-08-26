//! Protected signing-capability boundary.
//!
//! The owner never accepts a caller-implemented signer. A later broker or
//! Windows custody adapter supplies an opaque capability carrying a signed
//! request proof; this module binds that proof to the exact request bytes and
//! lets the family producer perform the fixed P-256 verification.

use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
};
use ocentra_protected_capability_custody_core::account_issuer::AccountIssuerSignerCapability;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_SIGNING_ERROR;

use crate::contract::{IssueCurrentAuthorityCommand, PreparedAccountIssuerV2Request};
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
    pub(crate) fn issue_current_authority_with_protected_capability(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
        capability: AccountIssuerSignerCapability,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        match self.prepare_current_authority(provider, provider_subject, command)? {
            AccountIssuerPreparation::Replay(authority) => Ok(authority),
            AccountIssuerPreparation::Prepared(prepared) => self
                .finalize_prepared_current_authority(
                    provider,
                    provider_subject,
                    prepared,
                    capability,
                ),
        }
    }

    /// Prepare one Account-owned request after resolving currentness and
    /// durable idempotency. A fresh request is represented by a non-Clone
    /// owner envelope; the family request never leaves this crate.
    pub fn prepare_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<AccountIssuerPreparation, AccountIssuerRpcError> {
        let current = self.resolve_current_for_signing(provider, provider_subject)?;
        let (request, reservation) = match self.prepare_issue(&current, command)? {
            IssuePreparation::Replay(transport) => {
                return Ok(AccountIssuerPreparation::Replay(Self::replayed_authority(
                    transport,
                )?))
            }
            IssuePreparation::Request((request, reservation)) => {
                self.mark_issue_signing(&reservation)?;
                (request, reservation)
            }
        };
        let prepared = PreparedAccountIssuerV2Request::from_parts(request, reservation);
        Ok(AccountIssuerPreparation::Prepared(prepared))
    }

    /// Finalize an owner-prepared request with the core-owned protected
    /// capability, then re-resolve currentness and atomically record the
    /// resulting transport in the same Account-owned repository.
    pub fn finalize_prepared_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        prepared: PreparedAccountIssuerV2Request,
        capability: AccountIssuerSignerCapability,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let (request, reservation) = prepared.into_parts();
        let transport = finalize_with_protected_capability(request, capability)
            .map_err(AccountIssuerRpcError::Signing)?;
        self.record_issued_transport(provider, provider_subject, reservation, transport)
    }
}

pub(crate) fn finalize_with_protected_capability(
    request: AccountIdentityAuthorityProducerV2Request,
    capability: AccountIssuerSignerCapability,
) -> Result<AccountIdentityAuthorityProducerV2Transport, AccountIssuerSigningError> {
    let signature = capability
        .into_signature_for(&request)
        .map_err(|_| AccountIssuerSigningError::Rejected)?;
    request
        .finalize(signature)
        .map_err(|_| AccountIssuerSigningError::Rejected)
}

pub(crate) fn fail_closed() -> AccountIssuerSigningError {
    AccountIssuerSigningError::OwnerUnavailable
}
