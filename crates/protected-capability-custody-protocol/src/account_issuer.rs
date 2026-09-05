//! Protected AccountIssuer v2 messages.
//!
//! These values are untrusted transport envelopes. The operation body is
//! deliberately typed so callers cannot smuggle authority, signer, or
//! lifecycle fields through a generic byte buffer.

use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PAYLOAD_DIGEST_PREFIX,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNED_TRANSPORT_DIGEST_PREFIX,
};

use crate::account_issuer_contract::AccountIssuerField;
use crate::types::ProtocolError;

#[path = "account_issuer_receipt_lineage.rs"]
pub mod account_issuer_receipt_lineage;
#[path = "account_issuer_receipt_validation.rs"]
mod account_issuer_receipt_validation;

use account_issuer_receipt_lineage::AccountIssuerReceiptLineage;

pub const ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES: usize =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AccountIssuerMessageKind {
    IssueCurrentAuthority = 6,
    AcknowledgeReceipt = 7,
}

/// A bounded protected receipt produced by the Account-owned delivery path.
///
/// This is only valid for the AcknowledgeReceipt operation. It is intentionally
/// move-only and does not expose a generic payload constructor or raw bytes.
pub struct ProtectedAccountIssuerReceiptWire {
    pub(crate) wire: Vec<u8>,
}

/// AccountIssuer request operation bodies. Every variant names the only
/// identity selector and operation-specific data that may cross this boundary.
#[derive(Debug, Eq, PartialEq)]
pub enum AccountIssuerRequestOperation {
    IssueCurrentAuthority {
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
    },
    AcknowledgeReceipt {
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
        protected_receipt: ProtectedAccountIssuerReceiptWire,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct AccountIssuerRequest {
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    operation: AccountIssuerRequestOperation,
}

impl AccountIssuerRequest {
    pub fn issue_current_authority(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
    ) -> Result<Self, ProtocolError> {
        Self::new(
            correlation_id,
            idempotency_key,
            key_id,
            AccountIssuerRequestOperation::IssueCurrentAuthority {
                provider,
                provider_subject,
            },
        )
    }

    pub fn acknowledge_receipt(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
        protected_receipt: ProtectedAccountIssuerReceiptWire,
    ) -> Result<Self, ProtocolError> {
        Self::new(
            correlation_id,
            idempotency_key,
            key_id,
            AccountIssuerRequestOperation::AcknowledgeReceipt {
                provider,
                provider_subject,
                protected_receipt,
            },
        )
    }

    pub fn kind(&self) -> AccountIssuerMessageKind {
        match &self.operation {
            AccountIssuerRequestOperation::IssueCurrentAuthority { .. } => {
                AccountIssuerMessageKind::IssueCurrentAuthority
            }
            AccountIssuerRequestOperation::AcknowledgeReceipt { .. } => {
                AccountIssuerMessageKind::AcknowledgeReceipt
            }
        }
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

    pub fn operation(&self) -> &AccountIssuerRequestOperation {
        &self.operation
    }

    pub fn into_operation(self) -> AccountIssuerRequestOperation {
        self.operation
    }

    fn new(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        operation: AccountIssuerRequestOperation,
    ) -> Result<Self, ProtocolError> {
        let request = Self {
            correlation_id,
            idempotency_key,
            key_id,
            operation,
        };
        crate::account_issuer_v2_codec::validate_request(&request)?;
        Ok(request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerReceipt {
    kind: AccountIssuerMessageKind,
    receipt_id: AccountIssuerField,
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    lineage: AccountIssuerReceiptLineage,
    result_digest: AccountIssuerField,
    signed_transport_digest: AccountIssuerField,
}

pub struct AccountIssuerReceiptInput {
    pub kind: AccountIssuerMessageKind,
    pub receipt_id: AccountIssuerField,
    pub correlation_id: AccountIssuerField,
    pub idempotency_key: AccountIssuerField,
    pub key_id: AccountIssuerField,
    pub lineage: AccountIssuerReceiptLineage,
    pub result_digest: AccountIssuerField,
    pub signed_transport_digest: AccountIssuerField,
}

impl AccountIssuerReceipt {
    pub fn new(input: AccountIssuerReceiptInput) -> Result<Self, ProtocolError> {
        let receipt = Self {
            kind: input.kind,
            receipt_id: input.receipt_id,
            correlation_id: input.correlation_id,
            idempotency_key: input.idempotency_key,
            key_id: input.key_id,
            lineage: input.lineage,
            result_digest: input.result_digest,
            signed_transport_digest: input.signed_transport_digest,
        };
        for field in [
            receipt.receipt_id.as_bytes(),
            receipt.correlation_id.as_bytes(),
            receipt.idempotency_key.as_bytes(),
            receipt.key_id.as_bytes(),
            receipt.result_digest.as_bytes(),
            receipt.signed_transport_digest.as_bytes(),
        ] {
            crate::account_issuer_v2_codec::validate_text_field(field)?;
        }
        if !account_issuer_receipt_validation::valid_sha256_field(
            receipt.receipt_id.as_bytes(),
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX.as_bytes(),
        ) || !account_issuer_receipt_validation::valid_sha256_field(
            receipt.key_id.as_bytes(),
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX.as_bytes(),
        ) || !account_issuer_receipt_validation::valid_sha256_field(
            receipt.result_digest.as_bytes(),
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PAYLOAD_DIGEST_PREFIX.as_bytes(),
        ) || !account_issuer_receipt_validation::valid_sha256_field(
            receipt.signed_transport_digest.as_bytes(),
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNED_TRANSPORT_DIGEST_PREFIX.as_bytes(),
        ) {
            return Err(ProtocolError::InvalidDiscriminant(0));
        }
        Ok(receipt)
    }

    pub fn kind(&self) -> AccountIssuerMessageKind {
        self.kind
    }

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

    pub fn lineage(&self) -> &AccountIssuerReceiptLineage {
        &self.lineage
    }

    pub fn result_digest(&self) -> &AccountIssuerField {
        &self.result_digest
    }

    pub fn signed_transport_digest(&self) -> &AccountIssuerField {
        &self.signed_transport_digest
    }
}
