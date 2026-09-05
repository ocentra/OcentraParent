//! Authenticated AccountIssuer v2 session envelopes.
//!
//! The generic protected-custody lifecycle request is intentionally not used
//! here. AccountIssuer kinds 6 and 7 have their own typed wire contract and
//! their own request/receipt binding. This module reuses the broker-generated
//! session authenticator without exposing that key or accepting generic
//! operation bytes from a caller.

#[path = "account_issuer_session_codec.rs"]
mod codec;
#[path = "account_issuer_session_digest.rs"]
mod digest;

use crate::account_issuer::{AccountIssuerReceipt, AccountIssuerRequest};
use crate::handshake::UntrustedBrokerHello;
use crate::types::{
    AuthenticationDomain, AuthenticationTag, BootstrapAuthenticator, CorrelationId, Nonce,
    ProtocolError, ProtocolGeneration, ProtocolVersion, SessionHandle, SessionTranscriptDigest,
};

pub(crate) const REQUEST_TAG: u8 = 1;
pub(crate) const RECEIPT_TAG: u8 = 2;
pub(crate) const ACCOUNT_ISSUER_SESSION_DOMAIN: &[u8] =
    crate::account_issuer_contract::ACCOUNT_ISSUER_TRANSPORT_DOMAIN;
pub(crate) const ACCOUNT_ISSUER_SESSION_REQUEST_DOMAIN: &[u8] =
    crate::constants::REQUEST_DIGEST_DOMAIN.as_bytes();
pub(crate) const ACCOUNT_ISSUER_SESSION_RECEIPT_DOMAIN: &[u8] =
    crate::constants::RESPONSE_DIGEST_DOMAIN.as_bytes();

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SessionBinding {
    pub(crate) version: ProtocolVersion,
    pub(crate) protocol_generation: ProtocolGeneration,
    pub(crate) client_nonce: Nonce,
    pub(crate) broker_nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_id: u32,
    pub(crate) client_process_epoch: u64,
    pub(crate) client_session_id: u32,
    pub(crate) broker_process_id: u32,
    pub(crate) broker_session_id: u32,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) watermark: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) transcript_digest: SessionTranscriptDigest,
    pub(crate) sequence: u64,
    pub(crate) expires_at_unix_millis: u64,
}

impl SessionBinding {
    pub(crate) fn from_hello(
        hello: &UntrustedBrokerHello,
        sequence: u64,
        expires_at_unix_millis: u64,
    ) -> Result<Self, ProtocolError> {
        if sequence == 0 {
            return Err(ProtocolError::InvalidSequence);
        }
        if expires_at_unix_millis == 0
            || expires_at_unix_millis > hello.session_expires_at_unix_millis()
        {
            return Err(ProtocolError::InvalidExpiry);
        }
        if hello.client_process_id() == 0
            || hello.client_session_id() == 0
            || hello.broker_process_id() == 0
            || hello.broker_session_id() == 0
        {
            return Err(ProtocolError::InvalidProcessId);
        }
        Ok(Self {
            version: hello.version(),
            protocol_generation: hello.protocol_generation(),
            client_nonce: hello.client_nonce(),
            broker_nonce: hello.broker_nonce(),
            correlation: hello.correlation(),
            client_process_id: hello.client_process_id(),
            client_process_epoch: hello.client_process_epoch(),
            client_session_id: hello.client_session_id(),
            broker_process_id: hello.broker_process_id(),
            broker_session_id: hello.broker_session_id(),
            broker_epoch: hello.broker_epoch(),
            broker_key_epoch: hello.broker_key_epoch(),
            writer_lease_epoch: hello.writer_lease_epoch(),
            watermark: hello.watermark(),
            session_handle: hello.session_handle(),
            transcript_digest: hello.transcript_digest(),
            sequence,
            expires_at_unix_millis,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AuthenticatedAccountIssuerRequest {
    binding: SessionBinding,
    request: AccountIssuerRequest,
    request_digest: [u8; crate::constants::REQUEST_DIGEST_BYTES],
    authentication_tag: AuthenticationTag,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UntrustedAccountIssuerRequest {
    binding: SessionBinding,
    request: AccountIssuerRequest,
    request_digest: [u8; crate::constants::REQUEST_DIGEST_BYTES],
    authentication_tag: AuthenticationTag,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AuthenticatedAccountIssuerReceipt {
    binding: SessionBinding,
    request_digest: [u8; crate::constants::REQUEST_DIGEST_BYTES],
    receipt: AccountIssuerReceipt,
    authentication_tag: AuthenticationTag,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UntrustedAccountIssuerReceipt {
    binding: SessionBinding,
    request_digest: [u8; crate::constants::REQUEST_DIGEST_BYTES],
    receipt: AccountIssuerReceipt,
    authentication_tag: AuthenticationTag,
}

impl AuthenticatedAccountIssuerRequest {
    pub fn authenticate(
        hello: &UntrustedBrokerHello,
        request: AccountIssuerRequest,
        sequence: u64,
        expires_at_unix_millis: u64,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<Self, ProtocolError> {
        // The pipe transcript has a binary CorrelationId while the AccountIssuer
        // contract carries its own validated textual correlation field. Both
        // are bound: the former is in SessionBinding and the latter is echoed
        // by validate_receipt_binding below.
        let binding = SessionBinding::from_hello(hello, sequence, expires_at_unix_millis)?;
        let request_wire = crate::account_issuer_v2_codec::encode_request(&request)?;
        let request_digest = digest::request(&binding, &request_wire);
        let authentication_tag =
            authenticator.authenticate(AuthenticationDomain::Request, &request_digest)?;
        Ok(Self {
            binding,
            request,
            request_digest,
            authentication_tag,
        })
    }

    pub fn request(&self) -> &AccountIssuerRequest {
        &self.request
    }

    pub fn request_digest(&self) -> [u8; crate::constants::REQUEST_DIGEST_BYTES] {
        self.request_digest
    }
}

impl UntrustedAccountIssuerRequest {
    pub fn into_authenticated_session(
        self,
        hello: &UntrustedBrokerHello,
        now_unix_millis: u64,
        expected_sequence: u64,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<AuthenticatedAccountIssuerRequest, ProtocolError> {
        let expected = SessionBinding::from_hello(
            hello,
            expected_sequence,
            self.binding.expires_at_unix_millis,
        )?;
        if self.binding != expected {
            return Err(ProtocolError::AuthenticationFailed);
        }
        if now_unix_millis == 0
            || now_unix_millis >= self.binding.expires_at_unix_millis
            || self.binding.expires_at_unix_millis > hello.session_expires_at_unix_millis()
            || self
                .binding
                .expires_at_unix_millis
                .saturating_sub(now_unix_millis)
                > crate::constants::MAX_REQUEST_TTL_MILLIS
        {
            return Err(ProtocolError::InvalidExpiry);
        }
        let request_wire = crate::account_issuer_v2_codec::encode_request(&self.request)?;
        if digest::request(&self.binding, &request_wire) != self.request_digest {
            return Err(ProtocolError::AuthenticationFailed);
        }
        authenticator.verify(
            AuthenticationDomain::Request,
            &self.request_digest,
            self.authentication_tag,
        )?;
        Ok(AuthenticatedAccountIssuerRequest {
            binding: self.binding,
            request: self.request,
            request_digest: self.request_digest,
            authentication_tag: self.authentication_tag,
        })
    }

    pub fn request(&self) -> &AccountIssuerRequest {
        &self.request
    }

    pub fn request_digest(&self) -> [u8; crate::constants::REQUEST_DIGEST_BYTES] {
        self.request_digest
    }
}

impl AuthenticatedAccountIssuerReceipt {
    pub fn authenticate(
        request: AuthenticatedAccountIssuerRequest,
        receipt: AccountIssuerReceipt,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<Self, ProtocolError> {
        validate_receipt_binding(request.request(), &receipt)?;
        let receipt_wire = crate::account_issuer_v2_codec::encode_receipt(&receipt)?;
        let response_digest =
            digest::receipt(&request.binding, request.request_digest, &receipt_wire);
        let authentication_tag =
            authenticator.authenticate(AuthenticationDomain::Response, &response_digest)?;
        Ok(Self {
            binding: request.binding,
            request_digest: request.request_digest,
            receipt,
            authentication_tag,
        })
    }

    pub fn receipt(&self) -> &AccountIssuerReceipt {
        &self.receipt
    }

    pub fn request_digest(&self) -> [u8; crate::constants::REQUEST_DIGEST_BYTES] {
        self.request_digest
    }
}

impl UntrustedAccountIssuerReceipt {
    pub fn into_verified_receipt(
        self,
        request: &AuthenticatedAccountIssuerRequest,
        hello: &UntrustedBrokerHello,
        now_unix_millis: u64,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<AccountIssuerReceipt, ProtocolError> {
        if self.binding.ne(&request.binding)
            || self.request_digest != request.request_digest
            || self.binding.ne(&SessionBinding::from_hello(
                hello,
                request.binding.sequence,
                request.binding.expires_at_unix_millis,
            )?)
        {
            return Err(ProtocolError::AuthenticationFailed);
        }
        if now_unix_millis == 0 || now_unix_millis >= self.binding.expires_at_unix_millis {
            return Err(ProtocolError::InvalidExpiry);
        }
        validate_receipt_binding(request.request(), &self.receipt)?;
        let receipt_wire = crate::account_issuer_v2_codec::encode_receipt(&self.receipt)?;
        let response_digest = digest::receipt(&self.binding, self.request_digest, &receipt_wire);
        authenticator.verify(
            AuthenticationDomain::Response,
            &response_digest,
            self.authentication_tag,
        )?;
        Ok(self.receipt)
    }
}

fn validate_receipt_binding(
    request: &AccountIssuerRequest,
    receipt: &AccountIssuerReceipt,
) -> Result<(), ProtocolError> {
    let (provider, provider_subject) = match request.operation() {
        crate::account_issuer::AccountIssuerRequestOperation::IssueCurrentAuthority {
            provider,
            provider_subject,
        }
        | crate::account_issuer::AccountIssuerRequestOperation::AcknowledgeReceipt {
            provider,
            provider_subject,
            ..
        } => (provider, provider_subject),
    };
    if receipt.kind() != request.kind()
        || receipt.correlation_id().as_bytes() != request.correlation_id().as_bytes()
        || receipt.idempotency_key().as_bytes() != request.idempotency_key().as_bytes()
        || receipt.key_id().as_bytes() != request.key_id().as_bytes()
        || receipt.lineage().provider() != provider
        || receipt.lineage().provider_subject() != provider_subject
    {
        return Err(ProtocolError::AuthenticationFailed);
    }
    Ok(())
}

pub fn encode_request(
    request: &AuthenticatedAccountIssuerRequest,
) -> Result<Vec<u8>, ProtocolError> {
    codec::encode_request(request)
}

pub fn decode_request(frame: &[u8]) -> Result<UntrustedAccountIssuerRequest, ProtocolError> {
    codec::decode_request(frame)
}

pub fn encode_receipt(
    receipt: &AuthenticatedAccountIssuerReceipt,
) -> Result<Vec<u8>, ProtocolError> {
    codec::encode_receipt(receipt)
}

pub fn decode_receipt(frame: &[u8]) -> Result<UntrustedAccountIssuerReceipt, ProtocolError> {
    codec::decode_receipt(frame)
}
