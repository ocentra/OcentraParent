//! Durable delivery-attempt boundary.
//!
//! Delivery completion is intentionally not represented by a public trait or
//! boolean. The owner can claim an outbox row and durably record failure; a
//! later protected adapter must present a verified receipt to transition the
//! row to acknowledged.

use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use ocentra_family_identity_core::account_identity_authority_producer_v2::
    AccountIdentityAuthorityProducerV2Request;
use ocentra_schema::account_identity_authority_producer_v2::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES, ACCOUNT_ISSUER_DELIVERY_ERROR,
    ACCOUNT_ISSUER_DIGEST_HEX,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::AccountIssuerField;
use ring::digest::{digest, SHA256};

#[derive(Debug)]
pub enum AccountIssuerDeliveryError {
    OwnerUnavailable,
    Rejected,
}

impl std::fmt::Display for AccountIssuerDeliveryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_DELIVERY_ERROR)
    }
}

impl std::error::Error for AccountIssuerDeliveryError {}

pub struct DeliveryClaim {
    pub(crate) inner: AccountIdentityIssuerOutboxClaim,
}

impl DeliveryClaim {
    /// Return the owner-derived receipt identity without exposing claim or
    /// lease internals. The field is only a typed match key; acknowledgement
    /// still consumes this exact claim and revalidates it transactionally.
    pub fn receipt_id(&self) -> Result<AccountIssuerField, AccountIssuerDeliveryError> {
        AccountIssuerField::from_wire(self.inner.receipt_id().as_bytes().to_vec())
            .map_err(|_| AccountIssuerDeliveryError::Rejected)
    }
}

pub struct DeliveryFailure {
    pub(crate) code: DeliveryFailureCode,
    pub(crate) detail_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryFailureCode {
    TransportRejected,
}

impl DeliveryFailure {
    pub fn from_bytes(detail: Vec<u8>) -> Result<Self, AccountIssuerDeliveryError> {
        if detail.is_empty() || detail.len() > 1_024 {
            return Err(AccountIssuerDeliveryError::Rejected);
        }
        let value = digest(&SHA256, detail.as_slice());
        let mut hex = String::with_capacity(value.as_ref().len() * 2);
        for byte in value.as_ref() {
            hex.push(char::from(
                ACCOUNT_ISSUER_DIGEST_HEX[usize::from(byte >> 4)],
            ));
            hex.push(char::from(
                ACCOUNT_ISSUER_DIGEST_HEX[usize::from(byte & 0x0f)],
            ));
        }
        Ok(Self {
            code: DeliveryFailureCode::TransportRejected,
            detail_digest: format!("sha256:delivery-detail:{hex}"),
        })
    }
}

/// Unsigned owner request which a protected signer adapter may carry across
/// the broker boundary.  It cannot complete issuance without the durable
/// account key and a verified protected signature.
pub struct PreparedAcknowledgeReceipt {
    pub(crate) request: AccountIdentityAuthorityProducerV2Request,
}

impl PreparedAcknowledgeReceipt {
    pub(crate) fn signing_bytes(&self) -> &[u8] {
        self.request.signing_bytes()
    }
}

/// Untrusted protected-adapter output.  The family transaction verifies the
/// signature, operation, exact stored receipt, current key, and lease before
/// accepting it; construction alone never acknowledges anything.
pub struct ProtectedAccountIssuerReceipt {
    wire: Vec<u8>,
}

impl ProtectedAccountIssuerReceipt {
    pub fn from_wire(wire: Vec<u8>) -> Result<Self, AccountIssuerDeliveryError> {
        if wire.is_empty() || wire.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES {
            return Err(AccountIssuerDeliveryError::Rejected);
        }
        Ok(Self { wire })
    }

    pub(crate) fn wire(&self) -> &[u8] {
        self.wire.as_slice()
    }
}
