//! Account-owned repository facade.

use ocentra_family_identity_core::account_identity_authority_issuer_client::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction, AccountIdentityIssuerCurrentness,
};
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::{
        AccountIdentityIssuerOutboxClaim, ProtectedAccountIssuerKeyRegistration,
    };
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::{
    ACCOUNT_ISSUER_DELIVERY_FAILURE_CODE, ACCOUNT_ISSUER_REPOSITORY_ERROR,
};

use crate::contract::IssueCurrentAuthorityCommand;
use crate::currentness::CurrentAuthority;
use crate::delivery::{
    DeliveryClaim, DeliveryFailure, PreparedAcknowledgeReceipt, ProtectedAccountIssuerReceipt,
};
use crate::key_registry::KeyRecord;
use crate::outbox::IssueTransaction;

#[path = "repository_error.rs"]
mod error;

pub struct AccountIssuerRepository {
    pub(crate) client: AccountIdentityAuthorityIssuerClient,
}

pub struct AccountIssuerTransaction<'a> {
    pub(crate) inner: AccountIdentityAuthorityIssuerTransaction<'a>,
}

impl AccountIssuerRepository {
    pub fn mount_account_owned() -> Result<Self, AccountIssuerRepositoryError> {
        AccountIdentityAuthorityIssuerClient::mount_account_owned()
            .map(|client| Self { client })
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn resolve_current(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<CurrentAuthority, AccountIssuerRepositoryError> {
        self.client
            .resolve_current(provider, provider_subject)
            .map(|inner| CurrentAuthority { inner })
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn begin(&mut self) -> Result<AccountIssuerTransaction<'_>, AccountIssuerRepositoryError> {
        self.client
            .begin()
            .map(|inner| AccountIssuerTransaction { inner })
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn recover_startup(
        &self,
    ) -> Result<ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityAuthorityIssuerStartupState, AccountIssuerRepositoryError>
    {
        self.client
            .recover_startup()
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub(crate) fn claim_delivery(
        &mut self,
    ) -> Result<Option<AccountIdentityIssuerOutboxClaim>, AccountIssuerRepositoryError> {
        self.client
            .claim_pending_outbox()
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn record_delivery_failure(
        &mut self,
        claim: &DeliveryClaim,
        failure: &DeliveryFailure,
    ) -> Result<(), AccountIssuerRepositoryError> {
        self.client
            .record_outbox_failure(
                &claim.inner,
                match failure.code {
                    crate::delivery::DeliveryFailureCode::TransportRejected => {
                        ACCOUNT_ISSUER_DELIVERY_FAILURE_CODE
                    }
                },
                Some(failure.detail_digest.as_str()),
            )
            .map_err(AccountIssuerRepositoryError::from)
    }
}

impl<'a> AccountIssuerTransaction<'a> {
    pub(crate) fn prepare_issue(
        &mut self,
        current: &CurrentAuthority,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request, AccountIssuerRepositoryError>
    {
        self.inner
            .prepare_issue_current_authority(
                &current.inner,
                command.correlation_id().as_str(),
                command.idempotency_key().as_str(),
            )
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn existing_issued_transport(
        &self,
        current: &CurrentAuthority,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<
        Option<ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport>,
        AccountIssuerRepositoryError,
    >{
        self.inner
            .existing_issued_transport(
                &current.inner,
                command.correlation_id().as_str(),
                command.idempotency_key().as_str(),
            )
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn current_key(
        &self,
        current: &CurrentAuthority,
    ) -> Result<KeyRecord, AccountIssuerRepositoryError> {
        self.inner
            .current_key(&current.inner)
            .map(KeyRecord::from)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn register_protected_key(
        &mut self,
        current: &CurrentAuthority,
        registration: &ProtectedAccountIssuerKeyRegistration,
    ) -> Result<KeyRecord, AccountIssuerRepositoryError> {
        self.inner
            .register_protected_key(&current.inner, registration)
            .map(KeyRecord::from)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn prepare_acknowledge_receipt(
        &self,
        current: &CurrentAuthority,
        claim: &DeliveryClaim,
    ) -> Result<PreparedAcknowledgeReceipt, AccountIssuerRepositoryError> {
        self.inner
            .prepare_acknowledge_receipt(&current.inner, &claim.inner)
            .map(|request| PreparedAcknowledgeReceipt { request })
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn acknowledge_receipt(
        &mut self,
        current: &CurrentAuthority,
        claim: &DeliveryClaim,
        protected_receipt: &ProtectedAccountIssuerReceipt,
    ) -> Result<(), AccountIssuerRepositoryError> {
        self.inner
            .acknowledge_receipt(&current.inner, &claim.inner, protected_receipt.wire())
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub(crate) fn into_issue_transaction(self) -> IssueTransaction<'a> {
        IssueTransaction { inner: self.inner }
    }

    pub fn commit(self) -> Result<(), AccountIssuerRepositoryError> {
        self.inner
            .commit()
            .map_err(AccountIssuerRepositoryError::from)
    }
}

#[derive(Debug)]
pub enum AccountIssuerRepositoryError {
    Unavailable,
    InvalidSchema,
    InvalidPath,
    CurrentnessUnavailable,
    CurrentnessRejected,
    KeyUnavailable,
    InvalidKey,
    ReplayDetected,
    ReceiptUnavailable,
    Producer,
}

impl std::fmt::Display for AccountIssuerRepositoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_REPOSITORY_ERROR)
    }
}

impl std::error::Error for AccountIssuerRepositoryError {}

impl From<AccountIdentityIssuerCurrentness> for CurrentAuthority {
    fn from(inner: AccountIdentityIssuerCurrentness) -> Self {
        Self { inner }
    }
}
