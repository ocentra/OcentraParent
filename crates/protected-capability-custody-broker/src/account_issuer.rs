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
use ocentra_account_issuer_owner::rpc::AccountIssuerOwner;
use ocentra_protected_capability_custody_core::account_issuer::AccountIssuerP256Signer;
use ocentra_protected_capability_custody_core::broker_admission::account_issuer_request::
    ProtectedAccountIssuerRequestAdmission;
use ocentra_protected_capability_custody_protocol::account_issuer::account_issuer_receipt_lineage::{
    AccountIssuerReceiptLineage, AccountIssuerReceiptLineageInput,
};
use ocentra_protected_capability_custody_protocol::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerReceiptInput,
    AccountIssuerRequestOperation,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::{
    AccountIssuerField, ACCOUNT_ISSUER_SERVICE,
};
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;

#[path = "account_issuer_error.rs"]
mod error;

pub(crate) struct BrokerAccountIssuer {
    owner: AccountIssuerOwner,
    signer: AccountIssuerP256Signer,
}

impl BrokerAccountIssuer {
    pub(crate) fn mount() -> Result<Self, BrokerError> {
        let owner = AccountIssuerOwner::mount_account_owned()
            .map_err(|owner_error| error::map_owner_error(&owner_error))?;
        let signer = AccountIssuerP256Signer::mount_account_owned()
            .map_err(|_error| BrokerError::DeploymentRequired)?;
        Ok(Self { owner, signer })
    }

    pub(crate) fn execute(
        &mut self,
        admission: ProtectedAccountIssuerRequestAdmission,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerReceipt, BrokerError> {
        match request.request().operation() {
            AccountIssuerRequestOperation::IssueCurrentAuthority { .. } => {
                self.issue_current_authority(admission, request)
            }
            AccountIssuerRequestOperation::AcknowledgeReceipt { .. } => {
                admission
                    .verify_and_consume(request)
                    .map_err(|_error| BrokerError::Request)?;
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
        admission: ProtectedAccountIssuerRequestAdmission,
        request: &AuthenticatedAccountIssuerRequest,
    ) -> Result<AccountIssuerReceipt, BrokerError> {
        let authorization = self
            .owner
            .authorize_protected_request(admission, request)
            .map_err(|owner_error| error::map_owner_error(&owner_error))?;
        let correlation_id = request
            .request()
            .correlation_id()
            .parse_correlation_id()
            .map_err(|_error| BrokerError::Request)?;
        let idempotency_key = request
            .request()
            .idempotency_key()
            .parse_idempotency_key()
            .map_err(|_error| BrokerError::Request)?;
        let command = IssueCurrentAuthorityCommand::new(correlation_id, idempotency_key);
        let issued = self
            .owner
            .issue_current_authority_with_protected_signer(authorization, &command, &self.signer)
            .map_err(|owner_error| error::map_owner_error(&owner_error))?;
        let receipt = issued.receipt();
        let (provider, provider_subject) = match request.request().operation() {
            AccountIssuerRequestOperation::IssueCurrentAuthority {
                provider,
                provider_subject,
            } => (provider, provider_subject),
            AccountIssuerRequestOperation::AcknowledgeReceipt { .. } => {
                return Err(BrokerError::Request)
            }
        };
        let lineage = receipt.lineage();
        if receipt.key_id() != request.request().key_id()
            || receipt.correlation_id() != request.request().correlation_id()
            || receipt.idempotency_key() != request.request().idempotency_key()
            || lineage.provider() != provider
            || lineage.provider_subject() != provider_subject
            || lineage.service() != ACCOUNT_ISSUER_SERVICE
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
    let verified = receipt.lineage();
    let lineage = AccountIssuerReceiptLineage::new(AccountIssuerReceiptLineageInput {
        provider: verified.provider().clone(),
        provider_subject: verified.provider_subject().clone(),
        account_id: field(verified.account_id().as_bytes())?,
        household_id: field(verified.household_id().as_bytes())?,
        member_id: field(verified.member_id().as_bytes())?,
        device_id: field(verified.device_id().as_bytes())?,
        session_id: field(verified.session_id().as_bytes())?,
        service_binding_id: receipt.service_binding_id().clone(),
        key_generation: verified.key_generation(),
        enrollment_generation: verified.enrollment_generation(),
        authority_generation: verified.authority_generation(),
        session_generation: verified.session_generation(),
        issued_at: field(verified.issued_at().as_bytes())?,
        expires_at: field(verified.expires_at().as_bytes())?,
    })
    .map_err(|_error| BrokerError::Request)?;
    AccountIssuerReceipt::new(AccountIssuerReceiptInput {
        kind,
        receipt_id: receipt.receipt_id().clone(),
        correlation_id: receipt.correlation_id().clone(),
        idempotency_key: receipt.idempotency_key().clone(),
        key_id: receipt.key_id().clone(),
        lineage,
        result_digest: receipt.payload_digest().clone(),
        signed_transport_digest: receipt.signed_transport_digest().clone(),
    })
    .map_err(|_error| BrokerError::Protocol(ProtocolError::InvalidFrameLength))
}

fn field(value: &[u8]) -> Result<AccountIssuerField, BrokerError> {
    AccountIssuerField::from_wire(value.to_vec()).map_err(|_error| BrokerError::Request)
}
