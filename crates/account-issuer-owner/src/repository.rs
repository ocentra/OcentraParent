//! Account-owned repository facade.

use ocentra_family_identity_core::account_identity_authority_issuer_client::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerIssuePreparation,
    AccountIdentityIssuerSignedIssue,
};
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::{
        AccountIdentityIssuerOutboxClaim, AccountIdentityIssuerRecordedTransport,
        ProtectedAccountIssuerKeyRegistration,
    };
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Receipt, ACCOUNT_ISSUER_DELIVERY_FAILURE_CODE,
    ACCOUNT_ISSUER_REPOSITORY_ERROR,
};

use crate::contract::IssueCurrentAuthorityCommand;
use crate::currentness::CurrentAuthority;
use crate::delivery::{
    DeliveryClaim, DeliveryFailure, PreparedAcknowledgeReceipt, ProtectedAccountIssuerReceipt,
};
use crate::key_registry::KeyRecord;

#[path = "repository_error.rs"]
mod error;

pub struct AccountIssuerRepository {
    pub(crate) client: AccountIdentityAuthorityIssuerClient,
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

    pub(crate) fn prepare_issue(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<AccountIdentityIssuerIssuePreparation, AccountIssuerRepositoryError> {
        self.client
            .prepare_issue_current_authority(
                provider,
                provider_subject,
                command.correlation_id().as_str(),
                command.idempotency_key().as_str(),
            )
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub(crate) fn finalize_issued_transport(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        signed: AccountIdentityIssuerSignedIssue,
    ) -> Result<AccountIdentityIssuerRecordedTransport, AccountIssuerRepositoryError> {
        self.client
            .finalize_issued_transport(provider, provider_subject, signed)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn current_key(
        &mut self,
        current: &CurrentAuthority,
    ) -> Result<KeyRecord, AccountIssuerRepositoryError> {
        self.client
            .current_key(&current.inner)
            .map(KeyRecord::from)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn register_protected_key(
        &mut self,
        current: &CurrentAuthority,
        registration: &ProtectedAccountIssuerKeyRegistration,
    ) -> Result<KeyRecord, AccountIssuerRepositoryError> {
        self.client
            .register_protected_key(&current.inner, registration)
            .map(KeyRecord::from)
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub fn prepare_acknowledge_receipt(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        claim: &DeliveryClaim,
    ) -> Result<PreparedAcknowledgeReceipt, AccountIssuerRepositoryError> {
        let current = self
            .resolve_current(provider, provider_subject)
            .map_err(|error| error)?;
        self.client
            .prepare_acknowledge_receipt(&current.inner, &claim.inner)
            .map(|request| PreparedAcknowledgeReceipt { request })
            .map_err(AccountIssuerRepositoryError::from)
    }

    pub(crate) fn acknowledge_receipt(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        claim: &DeliveryClaim,
        protected_receipt: &ProtectedAccountIssuerReceipt,
    ) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIssuerRepositoryError> {
        let current = self
            .resolve_current(provider, provider_subject)
            .map_err(|error| error)?;
        self.client
            .acknowledge_receipt(&current.inner, &claim.inner, protected_receipt.wire())
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
    ReservationUnavailable,
    ReservationExpired,
    ManualRequired,
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
