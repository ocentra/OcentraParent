//! Owner-facing AccountIssuer v2 command and result contracts.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2CorrelationId,
    AccountIdentityAuthorityProducerV2IdempotencyKey,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_owner_admission::
        AccountIdentityIssuerOwnerAdmission;
use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::AccountIdentityIssuerReceiptLineage;
use ocentra_protected_capability_custody_protocol::account_issuer_contract::AccountIssuerField;

pub const PRODUCER: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION;
pub const AUDIENCE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE;
pub const SERVICE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;

/// Account-owned authorization for one protected issuer request.
///
/// The provider and subject are intentionally private.  The broker cannot
/// construct this value from the transport operation; only the Account owner
/// may return it after binding the protected peer to its current authority.
pub struct AccountIssuerRequestAuthorization {
    admission: AccountIdentityIssuerOwnerAdmission,
}

impl AccountIssuerRequestAuthorization {
    pub(crate) fn into_admission(self) -> AccountIdentityIssuerOwnerAdmission {
        self.admission
    }
}

/// Owner-derived fields from a verified, durable issuer receipt.
///
/// This view deliberately contains no family transport, wire bytes, signer,
/// key material, path, or storage handle. Its fields are private and the only
/// constructor is owner-local, so a caller cannot mint a result that merely
/// looks current or durable.
pub struct AccountIssuerReceiptView {
    lineage: AccountIdentityIssuerReceiptLineage,
    receipt_id: AccountIssuerField,
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    payload_digest: AccountIssuerField,
    signed_transport_digest: AccountIssuerField,
    service_binding_id: AccountIssuerField,
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

    pub fn signed_transport_digest(&self) -> &AccountIssuerField {
        &self.signed_transport_digest
    }

    pub fn service_binding_id(&self) -> &AccountIssuerField {
        &self.service_binding_id
    }

    pub fn key_generation(&self) -> u64 {
        self.lineage.key_generation()
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.lineage.enrollment_generation()
    }

    pub fn authority_generation(&self) -> u64 {
        self.lineage.authority_generation()
    }

    pub fn session_generation(&self) -> u64 {
        self.lineage.session_generation()
    }

    pub fn lineage(&self) -> &AccountIdentityIssuerReceiptLineage {
        &self.lineage
    }

    pub(crate) fn from_lineage(lineage: AccountIdentityIssuerReceiptLineage) -> Option<Self> {
        Some(Self {
            receipt_id: field(lineage.receipt_id().as_bytes())?,
            correlation_id: field(lineage.correlation_id().as_bytes())?,
            idempotency_key: field(lineage.idempotency_key().as_bytes())?,
            key_id: field(lineage.key_id().as_bytes())?,
            payload_digest: field(lineage.payload_digest().as_bytes())?,
            signed_transport_digest: field(lineage.signed_transport_digest().as_bytes())?,
            service_binding_id: field(lineage.service_binding_id().as_bytes())?,
            lineage,
        })
    }
}

fn field(value: &[u8]) -> Option<AccountIssuerField> {
    AccountIssuerField::from_wire(value.to_vec()).ok()
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
