//! Broker-owned AccountIssuer composition.
//!
//! The broker is the only process allowed to mount the Account issuer and its
//! protected P-256 signer. No request can select a database path, key, or
//! fallback signer. Until installer provisioning supplies both owner mounts,
//! construction returns `DeploymentRequired` and the broker never emits an
//! authority receipt.

use crate::BrokerError;
use ocentra_account_issuer_owner::contract::{
    AccountIssuerReceiptView, IssueCurrentAuthorityCommand,
};
use ocentra_account_issuer_owner::rpc::{AccountIssuerOwner, AccountIssuerRpcError};
use ocentra_account_issuer_owner::signing::AccountIssuerSigningError;
use ocentra_protected_capability_custody_core::account_issuer::AccountIssuerP256Signer;
use ocentra_protected_capability_custody_protocol::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerRequestOperation,
};
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;

pub(crate) struct BrokerAccountIssuer {
    owner: AccountIssuerOwner,
    signer: AccountIssuerP256Signer,
}

impl BrokerAccountIssuer {
    pub(crate) fn mount() -> Result<Self, BrokerError> {
        let owner = AccountIssuerOwner::mount_account_owned().map_err(map_owner_error)?;
        let signer = AccountIssuerP256Signer::mount_account_owned()
            .map_err(|_| BrokerError::DeploymentRequired)?;
        Ok(Self { owner, signer })
    }

    pub(crate) fn execute(
        &mut self,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerReceipt, BrokerError> {
        match request.request().operation() {
            AccountIssuerRequestOperation::IssueCurrentAuthority { .. } => {
                self.issue_current_authority(request)
            }
            AccountIssuerRequestOperation::AcknowledgeReceipt { .. } => {
                // Delivery acknowledgement requires an Account-owned delivery
                // adapter to bind the claimed outbox row and protected receipt.
                // There is no safe broker fallback while that adapter is not
                // provisioned, so fail closed before any state mutation.
                Err(BrokerError::DeploymentRequired)
            }
        }
    }

    fn issue_current_authority(
        &mut self,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerReceipt, BrokerError> {
        let authorization = self
            .owner
            .authorize_protected_request(request)
            .map_err(map_owner_error)?;
        let correlation_id = request
            .request()
            .correlation_id()
            .parse_correlation_id()
            .map_err(|_| BrokerError::Request)?;
        let idempotency_key = request
            .request()
            .idempotency_key()
            .parse_idempotency_key()
            .map_err(|_| BrokerError::Request)?;
        let command = IssueCurrentAuthorityCommand::new(correlation_id, idempotency_key);
        let issued = self
            .owner
            .issue_current_authority_with_protected_signer(&authorization, &command, &self.signer)
            .map_err(map_owner_error)?;
        let receipt = issued.receipt();
        if receipt.key_id() != request.request().key_id()
            || receipt.correlation_id() != request.request().correlation_id()
            || receipt.idempotency_key() != request.request().idempotency_key()
        {
            return Err(BrokerError::Request);
        }
        protocol_receipt(AccountIssuerMessageKind::IssueCurrentAuthority, receipt)
    }
}

fn protocol_receipt(
    kind: AccountIssuerMessageKind,
    receipt: &AccountIssuerReceiptView,
) -> Result<AccountIssuerReceipt, BrokerError> {
    AccountIssuerReceipt::new(
        kind,
        receipt.receipt_id().clone(),
        receipt.correlation_id().clone(),
        receipt.idempotency_key().clone(),
        receipt.key_id().clone(),
        receipt.payload_digest().clone(),
    )
    .map_err(|_| BrokerError::Protocol(ProtocolError::InvalidFrameLength))
}

fn map_owner_error(error: AccountIssuerRpcError) -> BrokerError {
    match error {
        AccountIssuerRpcError::Signing(AccountIssuerSigningError::OwnerUnavailable)
        | AccountIssuerRpcError::Repository(
            ocentra_account_issuer_owner::repository::AccountIssuerRepositoryError::Unavailable,
        ) => BrokerError::DeploymentRequired,
        AccountIssuerRpcError::Signing(AccountIssuerSigningError::Rejected)
        | AccountIssuerRpcError::Repository(
            ocentra_account_issuer_owner::repository::AccountIssuerRepositoryError::SigningRejected,
        ) => BrokerError::Request,
        AccountIssuerRpcError::Repository(_) | AccountIssuerRpcError::Delivery(_) => {
            BrokerError::Request
        }
    }
}
