//! Account-owned signed current-authority producer transport.
//!
//! The only issuer input is the opaque `VerifiedAccountIdentityAuthority`
//! produced after a durable Account repository read. The transport is not
//! serde material and cannot be constructed from a caller-selected handoff.

use chrono::{DateTime, Utc};
use ed25519_dalek::VerifyingKey;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use sha2::{Digest, Sha256};

use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_producer_envelope.rs"]
mod envelope;
#[path = "account_identity_authority_producer_parse.rs"]
mod parse;

pub struct AccountIdentityAuthorityProducerTransport {
    wire: Vec<u8>,
    payload_digest: String,
}

impl AccountIdentityAuthorityProducerTransport {
    pub fn wire_bytes(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }
}

/// Internal seam for a durable Account-owned signer and public-key registry.
/// No implementation is supplied here; absent custody remains typed
/// `SignerCustodyUnavailable` rather than generating a process key.
pub(crate) trait AccountIdentityAuthorityProducerCustody: Send + Sync {
    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError>;
}

pub(crate) fn verify_at(
    wire: &[u8],
    custody: &dyn AccountIdentityAuthorityProducerCustody,
    now: DateTime<Utc>,
) -> Result<AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityAuthorityProducerError>
{
    let parsed = parse::parse_wire_at(wire, now)?;
    let verifying_key = custody.verification_key(&parsed.envelope.key_id)?;
    if expected_key_id(&verifying_key) != parsed.envelope.key_id {
        return Err(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable);
    }
    verifying_key
        .verify_strict(
            &parsed.signing_bytes,
            &ed25519_dalek::Signature::from_bytes(&parsed.signature),
        )
        .map_err(|_error| AccountIdentityAuthorityProducerError::SignatureInvalid)?;
    Ok(parsed.handoff)
}

pub(crate) fn expected_key_id(verifying_key: &VerifyingKey) -> String {
    format!("sha256:{:x}", Sha256::digest(verifying_key.as_bytes()))
}
