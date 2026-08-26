//! Owner-facing AccountIssuer v2 command and result contracts.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2CorrelationId,
    AccountIdentityAuthorityProducerV2IdempotencyKey, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};

use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use ocentra_protected_capability_custody_core::account_issuer::{
    AccountIssuerP256Signer, AccountIssuerP256SignerError, AccountIssuerSignerCapability,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::AccountIssuerField;

pub const PRODUCER: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION;
pub const AUDIENCE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE;
pub const SERVICE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;

/// Owner-derived fields from a verified, durable issuer receipt.
///
/// This view deliberately contains no family transport, wire bytes, signer,
/// key material, path, or storage handle. Its fields are private and the only
/// constructor is owner-local, so a caller cannot mint a result that merely
/// looks current or durable.
#[derive(Debug, Eq, PartialEq)]
pub struct AccountIssuerReceiptView {
    receipt_id: AccountIssuerField,
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    payload_digest: AccountIssuerField,
    service_binding_id: AccountIssuerField,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
}

impl AccountIssuerReceiptView {
    pub fn receipt_id(&self) -> &AccountIssuerField {
        &self.receipt_id
    }

    pub fn correlation_id(&self) -> &AccountIssuerField {
        &self.correlation_id
    }

    pub fn idempotency_key(&self) -> &AccountIssuerField {
        &self.idempotency_key
    }

    pub fn key_id(&self) -> &AccountIssuerField {
        &self.key_id
    }

    pub fn payload_digest(&self) -> &AccountIssuerField {
        &self.payload_digest
    }

    pub fn service_binding_id(&self) -> &AccountIssuerField {
        &self.service_binding_id
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.enrollment_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub(crate) fn from_receipt(
        receipt: &AccountIdentityAuthorityProducerV2Receipt,
    ) -> Option<Self> {
        Some(Self {
            receipt_id: AccountIssuerField::from_wire(receipt.receipt_id.as_bytes().to_vec())
                .ok()?,
            correlation_id: AccountIssuerField::from_wire(
                receipt.correlation_id.as_bytes().to_vec(),
            )
            .ok()?,
            idempotency_key: AccountIssuerField::from_wire(
                receipt.idempotency_key.as_bytes().to_vec(),
            )
            .ok()?,
            key_id: AccountIssuerField::from_wire(receipt.key_id.as_bytes().to_vec()).ok()?,
            payload_digest: AccountIssuerField::from_wire(
                receipt.payload_digest.as_bytes().to_vec(),
            )
            .ok()?,
            service_binding_id: AccountIssuerField::from_wire(
                receipt.service_binding_id.as_bytes().to_vec(),
            )
            .ok()?,
            key_generation: receipt.key_generation,
            enrollment_generation: receipt.enrollment_generation,
            authority_generation: receipt.authority_generation,
            session_generation: receipt.session_generation,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCurrentAuthorityCommand {
    correlation_id: AccountIdentityAuthorityProducerV2CorrelationId,
    idempotency_key: AccountIdentityAuthorityProducerV2IdempotencyKey,
}

impl IssueCurrentAuthorityCommand {
    pub fn new(
        correlation_id: AccountIdentityAuthorityProducerV2CorrelationId,
        idempotency_key: AccountIdentityAuthorityProducerV2IdempotencyKey,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
        }
    }

    pub(crate) fn correlation_id(&self) -> &AccountIdentityAuthorityProducerV2CorrelationId {
        &self.correlation_id
    }

    pub(crate) fn idempotency_key(&self) -> &AccountIdentityAuthorityProducerV2IdempotencyKey {
        &self.idempotency_key
    }
}

/// Owner-created bridge state for the protected P-256 signer.
///
/// The family request remains private inside this value until the owner has
/// received the exact protected signing capability. Callers never receive the
/// family request, a raw signer, or a platform handle.
pub struct PreparedAccountIssuerV2Request {
    request: AccountIdentityAuthorityProducerV2Request,
    reservation: AccountIdentityIssuerReservation,
}

impl PreparedAccountIssuerV2Request {
    pub(crate) fn from_parts(
        request: AccountIdentityAuthorityProducerV2Request,
        reservation: AccountIdentityIssuerReservation,
    ) -> Self {
        Self {
            request,
            reservation,
        }
    }

    /// Return the exact binding selected by the Account-owned request.
    pub fn binding(&self) -> &ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Binding{
        self.request.binding()
    }

    /// Ask the Account-specific protected core adapter to sign this exact
    /// owner-created request. The family request remains private here and is
    /// retained for owner-side verification and durable finalization.
    pub fn sign_with(
        &self,
        signer: &AccountIssuerP256Signer,
    ) -> Result<AccountIssuerSignerCapability, AccountIssuerP256SignerError> {
        signer.sign_request(&self.request)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        AccountIdentityAuthorityProducerV2Request,
        AccountIdentityIssuerReservation,
    ) {
        (self.request, self.reservation)
    }
}
