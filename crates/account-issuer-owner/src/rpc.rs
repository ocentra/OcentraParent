//! AccountIssuer owner command orchestration.

use ocentra_protected_capability_custody_core::broker_admission::account_issuer_request::ProtectedAccountIssuerRequestAdmission;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_RPC_ERROR;

use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;

use crate::contract::{
    AccountIssuerReceiptView, AccountIssuerRequestAuthorization, IssueCurrentAuthorityCommand,
};
use crate::delivery::{
    AccountIssuerDeliveryError, DeliveryClaim, DeliveryFailure, PreparedAcknowledgeReceipt,
    ProtectedAccountIssuerReceipt,
};
use crate::repository::{AccountIssuerRepository, AccountIssuerRepositoryError};
use crate::signing::{self, AccountIssuerSigningError};

pub struct AccountIssuerOwner {
    repository: AccountIssuerRepository,
}

pub struct IssuedAuthority {
    pub(crate) receipt: AccountIssuerReceiptView,
    pub(crate) replayed: bool,
}

impl IssuedAuthority {
    pub fn receipt(&self) -> &AccountIssuerReceiptView {
        &self.receipt
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }
}

#[derive(Debug)]
pub enum AccountIssuerRpcError {
    ProtectedAdmissionRejected,
    Repository(AccountIssuerRepositoryError),
    Signing(AccountIssuerSigningError),
    Delivery(AccountIssuerDeliveryError),
}

impl std::fmt::Display for AccountIssuerRpcError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_RPC_ERROR)
    }
}

impl std::error::Error for AccountIssuerRpcError {}

impl AccountIssuerOwner {
    /// Mount the fixed Account-owned store. The path and signer/key source are
    /// not caller-selected; a missing protected adapter leaves issuance
    /// fail-closed until a later broker/Windows wave is composed.
    pub fn mount_account_owned() -> Result<Self, AccountIssuerRpcError> {
        let repository = AccountIssuerRepository::mount_account_owned()
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(Self { repository })
    }

    pub(crate) fn repository_mut(&mut self) -> &mut AccountIssuerRepository {
        &mut self.repository
    }

    /// Consume the exact Protected transport proof before Account admission.
    ///
    /// The Protected proof binds the retained OS peer/session/enrollment to the
    /// complete authenticated request. It still does not identify an Account.
    /// Until Account mounts a one-shot current authority session, this boundary
    /// remains fail-closed and never promotes caller-selected provider fields.
    pub fn authorize_protected_request(
        &self,
        admission: ProtectedAccountIssuerRequestAdmission,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerRequestAuthorization, AccountIssuerRpcError> {
        admission
            .verify_and_consume(request)
            .map_err(|_error| AccountIssuerRpcError::ProtectedAdmissionRejected)?;
        Err(AccountIssuerRpcError::Signing(signing::fail_closed()))
    }

    pub fn issue_current_authority(
        &mut self,
        _provider: &AccountIdentityProvider,
        _provider_subject: &AccountIdentityProviderSubject,
        _command: &IssueCurrentAuthorityCommand,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        // This legacy convenience path has no protected signer parameter. It
        // must fail before preparation so an unavailable signer cannot burn
        // an idempotency key or leave a durable signing reservation behind.
        Err(AccountIssuerRpcError::Signing(signing::fail_closed()))
    }

    pub fn claim_delivery(&mut self) -> Result<Option<DeliveryClaim>, AccountIssuerRpcError> {
        self.repository
            .claim_delivery()
            .map(|claim| claim.map(|inner| DeliveryClaim { inner }))
            .map_err(AccountIssuerRpcError::Repository)
    }

    pub fn record_delivery_failure(
        &mut self,
        claim: &DeliveryClaim,
        failure: &DeliveryFailure,
    ) -> Result<(), AccountIssuerRpcError> {
        self.repository
            .record_delivery_failure(claim, failure)
            .map_err(AccountIssuerRpcError::Repository)
    }

    pub fn prepare_acknowledge_receipt(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        claim: &DeliveryClaim,
    ) -> Result<PreparedAcknowledgeReceipt, AccountIssuerRpcError> {
        let request = self
            .repository
            .prepare_acknowledge_receipt(provider, provider_subject, claim)
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(request)
    }

    pub fn acknowledge_receipt(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        claim: DeliveryClaim,
        protected_receipt: ProtectedAccountIssuerReceipt,
    ) -> Result<(), AccountIssuerRpcError> {
        let DeliveryClaim { inner } = claim;
        let protected_receipt_wire = protected_receipt.into_wire();
        self.repository
            .acknowledge_receipt(
                provider,
                provider_subject,
                &inner,
                protected_receipt_wire.as_slice(),
            )
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(())
    }
}
