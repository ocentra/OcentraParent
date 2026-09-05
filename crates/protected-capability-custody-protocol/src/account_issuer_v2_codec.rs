//! Length-delimited AccountIssuer v2 transport codec.

use std::fmt;

use crate::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerRequest, AccountIssuerRequestOperation,
    ProtectedAccountIssuerReceiptWire, ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
};
use crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_FIELD_BYTES;
use crate::constants::DEBUG_REDACTED;
use crate::types::ProtocolError;

#[path = "account_issuer_v2_codec_decode.rs"]
mod decode;
#[path = "account_issuer_v2_codec_encode.rs"]
mod encode;

use crate::account_issuer::AccountIssuerReceipt;

pub fn encode_request(request: &AccountIssuerRequest) -> Result<Vec<u8>, ProtocolError> {
    encode::request(request)
}

pub fn decode_request(frame: &[u8]) -> Result<AccountIssuerRequest, ProtocolError> {
    decode::request(frame)
}

pub fn encode_receipt(receipt: &AccountIssuerReceipt) -> Result<Vec<u8>, ProtocolError> {
    encode::receipt(receipt)
}

pub fn decode_receipt(frame: &[u8]) -> Result<AccountIssuerReceipt, ProtocolError> {
    decode::receipt(frame)
}

impl AccountIssuerMessageKind {
    pub(crate) fn from_wire(value: u8) -> Result<Self, ProtocolError> {
        match value {
            6 => Ok(Self::IssueCurrentAuthority),
            7 => Ok(Self::AcknowledgeReceipt),
            other => Err(ProtocolError::InvalidMessageKind(other)),
        }
    }

    pub const fn as_wire(self) -> u8 {
        self as u8
    }
}

impl TryFrom<Vec<u8>> for ProtectedAccountIssuerReceiptWire {
    type Error = ProtocolError;

    fn try_from(wire: Vec<u8>) -> Result<Self, Self::Error> {
        require(!wire.is_empty(), ProtocolError::EmptyField)?;
        require(
            wire.len() <= ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
            ProtocolError::FieldTooLarge,
        )?;
        Ok(Self { wire })
    }
}

impl ProtectedAccountIssuerReceiptWire {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub fn into_wire(self) -> Vec<u8> {
        self.wire
    }
}

impl fmt::Debug for ProtectedAccountIssuerReceiptWire {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple(DEBUG_REDACTED)
            .field(&self.wire.len())
            .finish()
    }
}

impl PartialEq for ProtectedAccountIssuerReceiptWire {
    fn eq(&self, other: &Self) -> bool {
        self.wire == other.wire
    }
}

impl Eq for ProtectedAccountIssuerReceiptWire {}

pub(crate) fn validate_request(request: &AccountIssuerRequest) -> Result<(), ProtocolError> {
    for field in [
        request.correlation_id().as_bytes(),
        request.idempotency_key().as_bytes(),
        request.key_id().as_bytes(),
    ] {
        validate_text_field(field)?;
    }
    match request.operation() {
        AccountIssuerRequestOperation::IssueCurrentAuthority {
            provider_subject, ..
        }
        | AccountIssuerRequestOperation::AcknowledgeReceipt {
            provider_subject, ..
        } => {
            let field = provider_subject.as_str().as_bytes();
            validate_text_field(field)?;
            require(
                field.iter().all(|byte| *byte > 0x1f && *byte != 0x7f),
                ProtocolError::EmptyField,
            )?;
        }
    }
    Ok(())
}

pub(crate) fn validate_text_field(field: &[u8]) -> Result<(), ProtocolError> {
    require(!field.is_empty(), ProtocolError::EmptyField)?;
    require(
        field.len() <= ACCOUNT_ISSUER_MAX_FIELD_BYTES,
        ProtocolError::FieldTooLarge,
    )
}

pub(crate) fn require(condition: bool, error: ProtocolError) -> Result<(), ProtocolError> {
    condition.then_some(()).ok_or(error)
}
