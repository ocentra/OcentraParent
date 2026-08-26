//! AccountIssuer owner command orchestration.

use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::ProtectedAccountIssuerSignerCapability;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_RPC_ERROR;

use crate::contract::IssueCurrentAuthorityCommand;
use crate::currentness::CurrentAuthority;
use crate::delivery::{AccountIssuerDeliveryError, DeliveryClaim, DeliveryFailure};
use crate::repository::{AccountIssuerRepository, AccountIssuerRepositoryError};
use crate::signing::{self, AccountIssuerSigningError};

pub struct AccountIssuerOwner {
    repository: AccountIssuerRepository,
}

pub struct IssuedAuthority {
    transport: AccountIdentityAuthorityProducerV2Transport,
    replayed: bool,
}

enum IssuePreparation {
    Replay(AccountIdentityAuthorityProducerV2Transport),
    Request(AccountIdentityAuthorityProducerV2Request),
}

impl IssuedAuthority {
    pub fn transport(&self) -> &AccountIdentityAuthorityProducerV2Transport {
        &self.transport
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
    pub fn wire(&self) -> &[u8] {
        self.inner.wire()
    }
}

impl DeliveryFailure {
    pub fn from_bytes(message: Vec<u8>) -> Result<Self, AccountIssuerDeliveryError> {
        let message =
            String::from_utf8(message).map_err(|_| AccountIssuerDeliveryError::Rejected)?;
        if message.trim().is_empty() || message.len() > 1_024 {
            return Err(AccountIssuerDeliveryError::Rejected);
        }
        Ok(Self { message })
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

    fn prepare_issue(
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

    fn replayed_authority(
        transport: AccountIdentityAuthorityProducerV2Transport,
    ) -> IssuedAuthority {
        IssuedAuthority {
            transport,
            replayed: true,
        }
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
            IssuePreparation::Replay(transport) => return Ok(Self::replayed_authority(transport)),
            IssuePreparation::Request(request) => request,
        };
        Err(AccountIssuerRpcError::Signing(signing::fail_closed()))
    }

    pub(crate) fn issue_current_authority_with_protected_capability(
        &mut self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        command: &IssueCurrentAuthorityCommand,
        capability: &ProtectedAccountIssuerSignerCapability,
    ) -> Result<IssuedAuthority, AccountIssuerRpcError> {
        let current = self
            .repository
            .resolve_current(provider, provider_subject)
            .map_err(AccountIssuerRpcError::Repository)?;
        let request = match self.prepare_issue(&current, command)? {
            IssuePreparation::Replay(transport) => return Ok(Self::replayed_authority(transport)),
            IssuePreparation::Request(request) => request,
        };
        let transport = signing::finalize_with_protected_capability(request, capability)
            .map_err(AccountIssuerRpcError::Signing)?;
        let mut issue_transaction = self
            .repository
            .begin()
            .map_err(AccountIssuerRpcError::Repository)?
            .into_issue_transaction();
        let recorded = issue_transaction
            .record_transport(&current, &transport)
            .map_err(AccountIssuerRpcError::Repository)?;
        let winner = recorded.transport().clone();
        let replayed = recorded.replayed();
        issue_transaction
            .commit()
            .map_err(AccountIssuerRpcError::Repository)?;
        Ok(IssuedAuthority {
            transport: winner,
            replayed,
        })
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
            .record_delivery_failure(&claim.inner, failure)
            .map_err(AccountIssuerRpcError::Repository)
    }
}
