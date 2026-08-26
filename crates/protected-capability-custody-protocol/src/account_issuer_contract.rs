//! Fixed AccountIssuer v2 transport contract.

use ocentra_schema::account_identity_authority_producer_v2::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNER_CAPABILITY_DOMAIN,
};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerField(String);

impl AccountIssuerField {
    pub(crate) fn from_wire(bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        if bytes.is_empty() {
            return Err(ProtocolError::EmptyField);
        }
        if bytes.len() > ACCOUNT_ISSUER_MAX_FIELD_BYTES {
            return Err(ProtocolError::FieldTooLarge);
        }
        let value = String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
        if value.trim().is_empty() {
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
    pub(crate) fn decode(frame: &[u8]) -> Result<Self, ProtocolError> {
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

/// Opaque public-key enrollment capability.  Only the protected custody
/// adapter may create this value in a later wave; callers can carry it to the
/// owner but cannot mint one from a raw public key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtectedAccountIssuerKeyRegistration {
    public_key: [u8; 65],
}

impl ProtectedAccountIssuerKeyRegistration {
    pub fn public_key(&self) -> &[u8; 65] {
        &self.public_key
    }

    pub(crate) fn from_protected_adapter(public_key: [u8; 65]) -> Self {
        Self { public_key }
    }
}
