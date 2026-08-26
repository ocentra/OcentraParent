//! Protected AccountIssuer v2 messages.
//!
//! These values are untrusted transport envelopes.  The inner authority is
//! only accepted after the family-owned v2 parser verifies its signature and
//! the Account-owned currentness transaction re-checks the binding.

use crate::account_issuer_contract::AccountIssuerField;
use crate::types::ProtocolError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AccountIssuerMessageKind {
    IssueCurrentAuthority = 6,
    AcknowledgeReceipt = 7,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerRequest {
    kind: AccountIssuerMessageKind,
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    inner_wire: Vec<u8>,
}

impl AccountIssuerRequest {
    pub fn new(
        kind: AccountIssuerMessageKind,
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        inner_wire: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        let request = Self {
            kind,
            correlation_id,
            idempotency_key,
            key_id,
            inner_wire,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn kind(&self) -> AccountIssuerMessageKind {
        self.kind
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

    pub fn inner_wire(&self) -> &[u8] {
        self.inner_wire.as_slice()
    }

    fn validate(&self) -> Result<(), ProtocolError> {
        validate_text_field(self.correlation_id.as_bytes())?;
        validate_text_field(self.idempotency_key.as_bytes())?;
        validate_text_field(self.key_id.as_bytes())?;
        validate_inner_field(self.inner_wire.as_slice())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerReceipt {
    kind: AccountIssuerMessageKind,
    receipt_id: AccountIssuerField,
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    result_digest: AccountIssuerField,
}

impl AccountIssuerReceipt {
    pub fn new(
        kind: AccountIssuerMessageKind,
        receipt_id: AccountIssuerField,
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        result_digest: AccountIssuerField,
    ) -> Result<Self, ProtocolError> {
        let receipt = Self {
            kind,
            receipt_id,
            correlation_id,
            idempotency_key,
            key_id,
            result_digest,
        };
        receipt.validate()?;
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

    pub fn result_digest(&self) -> &AccountIssuerField {
        &self.result_digest
    }

    fn validate(&self) -> Result<(), ProtocolError> {
        for field in [
            self.receipt_id.as_bytes(),
            self.correlation_id.as_bytes(),
            self.idempotency_key.as_bytes(),
            self.key_id.as_bytes(),
            self.result_digest.as_bytes(),
        ] {
            validate_text_field(field)?;
        }
        Ok(())
    }
}

fn validate_text_field(field: &[u8]) -> Result<(), ProtocolError> {
    if field.is_empty() {
        return Err(ProtocolError::EmptyField);
    }
    if field.len() > crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_FIELD_BYTES {
        return Err(ProtocolError::FieldTooLarge);
    }
    Ok(())
}

fn validate_inner_field(field: &[u8]) -> Result<(), ProtocolError> {
    if field.is_empty() {
        return Err(ProtocolError::EmptyField);
    }
    if field.len() > crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_INNER_BYTES {
        return Err(ProtocolError::FieldTooLarge);
    }
    Ok(())
}
