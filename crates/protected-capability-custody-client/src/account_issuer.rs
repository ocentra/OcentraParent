//! Typed client boundary for the AccountIssuer v2 session.
//!
//! This module deliberately does not expose the generic custody request
//! (`RequestKind` plus an operation buffer). AccountIssuer commands have their
//! own kinds, validated fields, and opaque owner-produced payloads. Transport
//! authentication and broker availability are kept behind the authenticated
//! session in `account_issuer_rpc`.

use std::fmt;

use crate::account_issuer_rpc::AccountIssuerClientError;
use ocentra_protected_capability_custody_protocol::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt as ProtocolAccountIssuerReceipt,
    AccountIssuerRequest as ProtocolAccountIssuerRequest,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::{
    AccountIssuerField, ACCOUNT_ISSUER_MAX_INNER_BYTES,
};
use ocentra_protected_capability_custody_protocol::constants;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;

/// Opaque, bounded bytes produced by the Account-owned authority/receipt
/// serializer. The client can carry this payload, but cannot interpret it or
/// mint signer, key, path, SQL, or authority state from it.
#[derive(Clone, Eq, PartialEq)]
pub struct AccountIssuerPayload {
    wire: Vec<u8>,
}

impl AccountIssuerPayload {
    pub fn from_wire(wire: Vec<u8>) -> Result<Self, AccountIssuerClientError> {
        if wire.is_empty() {
            return Err(AccountIssuerClientError::Protocol(
                ProtocolError::EmptyField,
            ));
        }
        if wire.len() > ACCOUNT_ISSUER_MAX_INNER_BYTES {
            return Err(AccountIssuerClientError::Protocol(
                ProtocolError::FieldTooLarge,
            ));
        }
        Ok(Self { wire })
    }

    pub(crate) fn into_wire(self) -> Vec<u8> {
        self.wire
    }
}

impl fmt::Debug for AccountIssuerPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple(constants::DEBUG_REDACTED)
            .field(&self.wire.len())
            .finish()
    }
}

/// A typed IssueCurrentAuthority command. The inner payload remains opaque to
/// this transport crate and is interpreted only by the Account owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCurrentAuthorityRequest {
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    authority_payload: AccountIssuerPayload,
}

impl IssueCurrentAuthorityRequest {
    pub fn new(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        authority_payload: AccountIssuerPayload,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
            key_id,
            authority_payload,
        }
    }

    pub(crate) fn into_protocol(
        self,
    ) -> Result<ProtocolAccountIssuerRequest, AccountIssuerClientError> {
        ProtocolAccountIssuerRequest::new(
            AccountIssuerMessageKind::IssueCurrentAuthority,
            self.correlation_id,
            self.idempotency_key,
            self.key_id,
            self.authority_payload.into_wire(),
        )
        .map_err(AccountIssuerClientError::Protocol)
    }
}

/// A typed AcknowledgeReceipt command. Receipt verification remains an
/// Account-owned operation; this payload is only transported through the
/// authenticated broker session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcknowledgeReceiptRequest {
    correlation_id: AccountIssuerField,
    idempotency_key: AccountIssuerField,
    key_id: AccountIssuerField,
    receipt_payload: AccountIssuerPayload,
}

impl AcknowledgeReceiptRequest {
    pub fn new(
        correlation_id: AccountIssuerField,
        idempotency_key: AccountIssuerField,
        key_id: AccountIssuerField,
        receipt_payload: AccountIssuerPayload,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
            key_id,
            receipt_payload,
        }
    }

    pub(crate) fn into_protocol(
        self,
    ) -> Result<ProtocolAccountIssuerRequest, AccountIssuerClientError> {
        ProtocolAccountIssuerRequest::new(
            AccountIssuerMessageKind::AcknowledgeReceipt,
            self.correlation_id,
            self.idempotency_key,
            self.key_id,
            self.receipt_payload.into_wire(),
        )
        .map_err(AccountIssuerClientError::Protocol)
    }
}

/// The verified AccountIssuer receipt returned by the authenticated broker
/// session. All fields are transport-contract values; signer and authority
/// verification remain outside this client crate.
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
}
