//! Fixed AccountIssuer v2 transport contract.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Binding, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNER_CAPABILITY_DOMAIN,
};
use sha2::{Digest, Sha256};

use crate::types::ProtocolError;

pub const ACCOUNT_ISSUER_PROTOCOL_VERSION: u16 = 2;
pub const ACCOUNT_ISSUER_TRANSPORT_DOMAIN: &[u8] =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN;
pub const ACCOUNT_ISSUER_SERVICE: &str = "ocentra.account-authority-producer.cloudflare.v2";
pub const ACCOUNT_ISSUER_MAX_FIELD_BYTES: usize = 1_024;
pub const ACCOUNT_ISSUER_MAX_INNER_BYTES: usize = 64 * 1_024;
pub const ACCOUNT_ISSUER_MAX_WIRE_BYTES: usize = 128 * 1_024;
pub const ACCOUNT_ISSUER_SIGNER_CAPABILITY_DOMAIN: &[u8] =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNER_CAPABILITY_DOMAIN;
pub const ACCOUNT_ISSUER_SIGNER_CAPABILITY_BYTES: usize = 32 + 64;

/// The exact digest of one owner-created AccountIssuer v2 signing request.
///
/// This is a protected request-digest capability, not a generic hash input.
/// Its only constructor is the validated owner-request preparation path and
/// its bytes cannot be changed after construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtectedAccountIssuerRequestDigest([u8; 32]);

impl ProtectedAccountIssuerRequestDigest {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// One non-Clone, protocol-owned AccountIssuer v2 request prepared by the
/// Account owner for the protected signer.
///
/// The canonical payload is consumed only to validate the signing domain and
/// derive the exact digest. It is never exposed to the platform signer or to a
/// caller-selected generic signing API; only the validated binding and typed
/// digest capability cross the protected boundary.
pub struct PreparedAccountIssuerV2Request {
    binding: AccountIdentityAuthorityProducerV2Binding,
    request_digest: ProtectedAccountIssuerRequestDigest,
}

impl PreparedAccountIssuerV2Request {
    pub fn from_owner_request(
        canonical_payload: &[u8],
        binding: AccountIdentityAuthorityProducerV2Binding,
    ) -> Result<Self, ProtocolError> {
        if canonical_payload.is_empty()
            || canonical_payload.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES
            || !canonical_payload.starts_with(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN)
        {
            return Err(ProtocolError::InvalidDomain);
        }
        binding
            .validate_shape()
            .map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
        let request_digest =
            ProtectedAccountIssuerRequestDigest(Sha256::digest(canonical_payload).into());
        Ok(Self {
            binding,
            request_digest,
        })
    }

    pub fn binding(&self) -> &AccountIdentityAuthorityProducerV2Binding {
        &self.binding
    }

    pub fn request_digest(&self) -> &ProtectedAccountIssuerRequestDigest {
        &self.request_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerField(String);

impl AccountIssuerField {
    pub fn from_wire(bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        if bytes.is_empty() {
            return Err(ProtocolError::EmptyField);
        }
        if bytes.len() > ACCOUNT_ISSUER_MAX_FIELD_BYTES {
            return Err(ProtocolError::FieldTooLarge);
        }
        let value = String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
        if value.trim().is_empty()
            || value
                .chars()
                .any(|character| character <= '\u{001f}' || character == '\u{007f}')
        {
            return Err(ProtocolError::EmptyField);
        }
        Ok(Self(value))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// An opaque proof returned by the protected broker/Windows signing seam.
///
/// This is deliberately a signed-request capability, not a public signer
/// trait or a caller-provided key.  Decoding is untrusted; the owner still
/// binds the request digest and verifies the P-256 signature against the
/// account-owned key before any durable receipt is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtectedAccountIssuerSignerCapability {
    request_digest: [u8; 32],
    signature: [u8; 64],
}

impl ProtectedAccountIssuerSignerCapability {
    /// Construct an untrusted signed-request capability returned by the
    /// protected platform signer. The digest is copied from the exact typed
    /// request, so the platform boundary cannot pair a signature with a
    /// caller-selected digest. The Account owner must still compare the
    /// capability to its own request and verify the signature against the
    /// current Account-owned public key before durable state changes.
    pub fn from_prepared_request(
        request: &PreparedAccountIssuerV2Request,
        signature: [u8; 64],
    ) -> Self {
        Self {
            request_digest: *request.request_digest.as_bytes(),
            signature,
        }
    }

    pub fn decode(frame: &[u8]) -> Result<Self, ProtocolError> {
        let expected =
            ACCOUNT_ISSUER_SIGNER_CAPABILITY_DOMAIN.len() + ACCOUNT_ISSUER_SIGNER_CAPABILITY_BYTES;
        if frame.len() != expected
            || frame[..ACCOUNT_ISSUER_SIGNER_CAPABILITY_DOMAIN.len()]
                != *ACCOUNT_ISSUER_SIGNER_CAPABILITY_DOMAIN
        {
            return Err(ProtocolError::InvalidDomain);
        }
        let mut cursor = ACCOUNT_ISSUER_SIGNER_CAPABILITY_DOMAIN.len();
        let request_digest = frame[cursor..cursor + 32]
            .try_into()
            .map_err(|_| ProtocolError::InvalidFrameLength)?;
        cursor += 32;
        let signature = frame[cursor..cursor + 64]
            .try_into()
            .map_err(|_| ProtocolError::InvalidFrameLength)?;
        Ok(Self {
            request_digest,
            signature,
        })
    }

    pub fn request_digest(&self) -> &[u8; 32] {
        &self.request_digest
    }

    pub fn signature(&self) -> &[u8; 64] {
        &self.signature
    }
}
