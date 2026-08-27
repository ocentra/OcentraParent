//! Typed client boundary for the AccountIssuer v2 session.
//!
//! AccountIssuer operations carry only their canonical identity selector. The
//! client never accepts a generic operation buffer or authority/signer payload.

use std::fmt;

use ocentra_protected_capability_custody_protocol::account_issuer::account_issuer_receipt_lineage::AccountIssuerReceiptLineage as ProtocolAccountIssuerReceiptLineage;
use ocentra_protected_capability_custody_protocol::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt as ProtocolAccountIssuerReceipt,
    AccountIssuerRequest as ProtocolAccountIssuerRequest, ProtectedAccountIssuerReceiptWire,
    ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::AccountIssuerField;
use ocentra_protected_capability_custody_protocol::constants;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use crate::account_issuer_rpc::AccountIssuerClientError;

/// Protected receipt bytes for the AcknowledgeReceipt operation only.
///
/// The wrapper is bounded and move-only so callers cannot reuse or reinterpret
/// a protected receipt as an authority or signer payload.
pub struct AcknowledgeReceiptWire {
    wire: Vec<u8>,
}

impl TryFrom<Vec<u8>> for AcknowledgeReceiptWire {
    type Error = AccountIssuerClientError;

    fn try_from(wire: Vec<u8>) -> Result<Self, Self::Error> {
        if wire.is_empty() {
            return Err(AccountIssuerClientError::Protocol(
                ProtocolError::EmptyField,
            ));
        }
        if wire.len() > ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES {
            return Err(AccountIssuerClientError::Protocol(
                ProtocolError::FieldTooLarge,
            ));
        }
        Ok(Self { wire })
    }
}

impl AcknowledgeReceiptWire {
    fn into_wire(self) -> Vec<u8> {
        self.wire
    }
}

impl fmt::Debug for AcknowledgeReceiptWire {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple(constants::DEBUG_REDACTED)
            .field(&self.wire.len())
            .finish()
    }
}

impl PartialEq for AcknowledgeReceiptWire {
    fn eq(&self, other: &Self) -> bool {
        self.wire == other.wire
    }
}

impl Eq for AcknowledgeReceiptWire {}

/// A typed IssueCurrentAuthority command. Its operation body is only the
/// provider/provider-subject selector; key_id remains the expected binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCurrentAuthorityRequest {
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
}

impl IssueCurrentAuthorityRequest {
    pub fn new(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
            key_id,
            provider,
            provider_subject,
        }
    }

    pub(crate) fn into_protocol(
        self,
    ) -> Result<ProtocolAccountIssuerRequest, AccountIssuerClientError> {
        ProtocolAccountIssuerRequest::issue_current_authority(
            self.correlation_id,
            self.idempotency_key,
            self.key_id,
            self.provider,
            self.provider_subject,
        )
        .map_err(AccountIssuerClientError::Protocol)
    }
}

/// A typed AcknowledgeReceipt command. Receipt verification remains an
/// Account-owned operation; only its protected receipt wire crosses transport.
#[derive(Debug, Eq, PartialEq)]
pub struct AcknowledgeReceiptRequest {
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    protected_receipt: AcknowledgeReceiptWire,
}

impl AcknowledgeReceiptRequest {
    pub fn new(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
        protected_receipt: AcknowledgeReceiptWire,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
            key_id,
            provider,
            provider_subject,
            protected_receipt,
        }
    }

    pub(crate) fn into_protocol(
        self,
    ) -> Result<ProtocolAccountIssuerRequest, AccountIssuerClientError> {
        let protected_receipt =
            ProtectedAccountIssuerReceiptWire::try_from(self.protected_receipt.into_wire())
                .map_err(AccountIssuerClientError::Protocol)?;
        ProtocolAccountIssuerRequest::acknowledge_receipt(
            self.correlation_id,
            self.idempotency_key,
            self.key_id,
            self.provider,
            self.provider_subject,
            protected_receipt,
        )
        .map_err(AccountIssuerClientError::Protocol)
    }
}

/// The verified AccountIssuer receipt returned by the authenticated broker.
/// Signer and authority verification remain outside this client crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerReceipt {
    inner: ProtocolAccountIssuerReceipt,
}

impl AccountIssuerReceipt {
    pub(crate) fn from_protocol(inner: ProtocolAccountIssuerReceipt) -> Self {
        Self { inner }
    }

    pub fn kind(&self) -> AccountIssuerMessageKind {
        self.inner.kind()
    }

    pub fn receipt_id(&self) -> &AccountIssuerField {
        self.inner.receipt_id()
    }

    pub fn correlation_id(&self) -> &AccountIssuerField {
        self.inner.correlation_id()
    }

    pub fn idempotency_key(&self) -> &AccountIssuerField {
        self.inner.idempotency_key()
    }

    pub fn key_id(&self) -> &AccountIssuerField {
        self.inner.key_id()
    }

    pub fn result_digest(&self) -> &AccountIssuerField {
        self.inner.result_digest()
    }

    pub fn lineage(&self) -> &ProtocolAccountIssuerReceiptLineage {
        self.inner.lineage()
    }

    pub fn signed_transport_digest(&self) -> &AccountIssuerField {
        self.inner.signed_transport_digest()
    }
}
