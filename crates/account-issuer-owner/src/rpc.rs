//! AccountIssuer owner command orchestration.

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

impl DeliveryClaim {
    pub(crate) fn wire(&self) -> &[u8] {
        self.inner.wire()
    }
}

impl AccountIssuerOwner {
    /// Mount the fixed Account-owned store. The path and signer/key source are
    /// not caller-selected; a missing protected adapter leaves issuance
    /// fail-closed until a later broker/Windows wave is composed.
    pub fn mount_account_owned() -> Result<Self, AccountIssuerRpcError> {
        let repository = AccountIssuerRepository::mount_account_owned()
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(Self { repository })
    }

    pub(crate) fn new(repository: AccountIssuerRepository) -> Self {
        Self { repository }
    }

    pub(crate) fn repository_mut(&mut self) -> &mut AccountIssuerRepository {
        &mut self.repository
    }

    /// Bind a protected transport request to Account-owned authority.
    ///
    /// Transport authentication proves the pipe session, not which Account
    /// the peer may issue for.  The OS enrollment/currentness adapter that can
    /// produce this opaque authorization is not mounted yet, so this boundary
    /// remains fail-closed and never promotes caller-selected provider fields.
    pub fn authorize_protected_request(
        &self,
        _request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerRequestAuthorization, AccountIssuerRpcError> {
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
    ) -> Result<AccountIssuerReceiptView, AccountIssuerRpcError> {
        let receipt = self
            .repository
            .acknowledge_receipt(provider, provider_subject, &claim, &protected_receipt)
            .map_err(AccountIssuerRpcError::Repository)?;
        AccountIssuerReceiptView::from_receipt(&receipt).ok_or(AccountIssuerRpcError::Repository(
            AccountIssuerRepositoryError::InvalidSchema,
        ))
    }
}
