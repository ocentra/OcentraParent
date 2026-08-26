//! AccountIssuer owner command orchestration.

use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
};
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_RPC_ERROR;

use crate::contract::AccountIssuerReceiptView;
use crate::contract::{IssueCurrentAuthorityCommand, PreparedAccountIssuerV2Request};
use crate::currentness::CurrentAuthority;
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
    receipt: AccountIssuerReceiptView,
    replayed: bool,
}

pub(crate) enum IssuePreparation {
    Replay(AccountIdentityAuthorityProducerV2Transport),
    Request(AccountIdentityAuthorityProducerV2Request),
}

/// Result of the owner-only preparation phase. A replay is already durable;
/// a fresh request must cross the typed protected signer before finalization.
pub enum AccountIssuerPreparation {
    Replay(IssuedAuthority),
    Prepared(PreparedAccountIssuerV2Request),
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

    pub(crate) fn resolve_current_for_signing(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<CurrentAuthority, AccountIssuerRpcError> {
        self.repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)
    }

    pub(crate) fn prepare_issue(
        &mut self,
        current: &CurrentAuthority,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<IssuePreparation, AccountIssuerRpcError> {
        let mut transaction = self
            .repository
            .begin()
            .map_err(AccountIssuerRpcError::Repository)?;
        let preparation = match transaction
            .existing_issued_transport(current, command)
            .map_err(AccountIssuerRpcError::Repository)?
        {
            Some(transport) => IssuePreparation::Replay(transport),
            None => IssuePreparation::Request(
                transaction
                    .prepare_issue(current, command)
                    .map_err(AccountIssuerRpcError::Repository)?,
            ),
        };
        transaction
            .commit()
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(preparation)
    }

    pub(crate) fn replayed_authority(
        transport: AccountIdentityAuthorityProducerV2Transport,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let receipt = AccountIssuerReceiptView::from_receipt(transport.receipt()).ok_or(
            AccountIssuerRpcError::Repository(AccountIssuerRepositoryError::InvalidSchema),
        )?;
        Ok(IssuedAuthority {
            receipt,
            replayed: true,
        })
    }

    pub fn issue_current_authority(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let current = self
            .repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)?;
        let _request = match self.prepare_issue(&current, command)? {
            IssuePreparation::Replay(transport) => return Self::replayed_authority(transport),
            IssuePreparation::Request(request) => request,
        };
        Err(AccountIssuerRpcError::Signing(signing::fail_closed()))
    }

    pub(crate) fn record_issued_transport(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        transport: AccountIdentityAuthorityProducerV2Transport,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let current = self
            .repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)?;
        let mut issue_transaction = self
            .repository
            .begin()
            .map_err(AccountIssuerRpcError::Repository)?
            .into_issue_transaction();
        let recorded = issue_transaction
            .record_transport(&current, &transport)
            .map_err(AccountIssuerRpcError::Repository)?;
        let receipt = AccountIssuerReceiptView::from_receipt(recorded.transport().receipt())
            .ok_or(AccountIssuerRpcError::Repository(
                AccountIssuerRepositoryError::InvalidSchema,
            ))?;
        let replayed = recorded.replayed();
        issue_transaction
            .commit()
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(IssuedAuthority { receipt, replayed })
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
        let current = self
            .repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)?;
        let transaction = self
            .repository
            .begin()
            .map_err(AccountIssuerRpcError::Repository)?;
        let request = transaction
            .prepare_acknowledge_receipt(&current, claim)
            .map_err(AccountIssuerRpcError::Repository)?;
        transaction
            .commit()
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
        let current = self
            .repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)?;
        let mut transaction = self
            .repository
            .begin()
            .map_err(AccountIssuerRpcError::Repository)?;
        let receipt = transaction
            .acknowledge_receipt(&current, &claim, &protected_receipt)
            .map_err(AccountIssuerRpcError::Repository)?;
        transaction
            .commit()
            .map_err(AccountIssuerRpcError::Repository)?;
        AccountIssuerReceiptView::from_receipt(&receipt).ok_or(AccountIssuerRpcError::Repository(
            AccountIssuerRepositoryError::InvalidSchema,
        ))
    }
}
