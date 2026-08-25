//! Account-owned signed current-authority producer transport.
//!
//! The only issuer input is the opaque `VerifiedAccountIdentityAuthority`
//! produced after a durable Account repository read. The transport is not
//! serde material and cannot be constructed from a caller-selected handoff.

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ed25519_dalek::VerifyingKey;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use ocentra_schema::account_identity_authority_producer::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS;
use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
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

    pub(crate) fn from_signed_parts(
        signing_bytes: Vec<u8>,
        payload: &[u8],
        signature: [u8; 64],
    ) -> Result<Self, AccountIdentityAuthorityProducerError> {
        let wire = envelope::wire(signing_bytes, signature)?;
        Ok(Self {
            wire,
            payload_digest: format!("sha256:{:x}", Sha256::digest(payload)),
        })
    }
}

/// Internal seam for a durable Account-owned signer and public-key registry.
/// No implementation is supplied here; absent custody remains typed
/// `SignerCustodyUnavailable` rather than generating a process key.
pub(crate) trait AccountIdentityAuthorityProducerCustody: Send + Sync {
    fn signing_key_id(&self) -> &str;
    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError>;
    fn sign(&self, payload: &[u8]) -> Result<[u8; 64], AccountIdentityAuthorityProducerError>;
}

pub(crate) fn issue(
    authority: &VerifiedAccountIdentityAuthority,
    custody: &dyn AccountIdentityAuthorityProducerCustody,
) -> Result<AccountIdentityAuthorityProducerTransport, AccountIdentityAuthorityProducerError> {
    issue_at(authority, custody, Utc::now())
}

pub(crate) fn issue_at(
    authority: &VerifiedAccountIdentityAuthority,
    custody: &dyn AccountIdentityAuthorityProducerCustody,
    issued_at: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerTransport, AccountIdentityAuthorityProducerError> {
    let handoff = authority.handoff();
    let payload = serde_json::to_vec(handoff)
        .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    let canonical_payload = serde_json::to_vec(
        &serde_json::from_slice::<AccountIdentityCurrentMemberDeviceAuthorityHandoff>(&payload)
            .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?,
    )
    .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    if canonical_payload != payload {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    let expires_at = issued_at
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerError::AuthorityExpired)?;
    let issued_at = issued_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let key_id = custody.signing_key_id().trim();
    if key_id.is_empty() {
        return Err(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable);
    }
    let envelope = envelope::CanonicalAuthorityProducerEnvelope {
        key_id: key_id.to_owned(),
        issued_at,
        expires_at,
        payload,
    };
    let signing_bytes = envelope::encode(&envelope)?;
    let verifying_key = custody.verification_key(key_id)?;
    if expected_key_id(&verifying_key) != key_id {
        return Err(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable);
    }
    let signature = custody.sign(&signing_bytes)?;
    verifying_key
        .verify_strict(
            &signing_bytes,
            &ed25519_dalek::Signature::from_bytes(&signature),
        )
        .map_err(|_| AccountIdentityAuthorityProducerError::SignatureInvalid)?;
    AccountIdentityAuthorityProducerTransport::from_signed_parts(
        signing_bytes,
        &envelope.payload,
        signature,
    )
}

pub(crate) fn verify(
    wire: &[u8],
    custody: &dyn AccountIdentityAuthorityProducerCustody,
) -> Result<AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityAuthorityProducerError>
{
    verify_at(wire, custody, Utc::now())
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
        .map_err(|_| AccountIdentityAuthorityProducerError::SignatureInvalid)?;
    Ok(parsed.handoff)
}

pub(crate) fn expected_key_id(verifying_key: &VerifyingKey) -> String {
    format!("sha256:{:x}", Sha256::digest(verifying_key.as_bytes()))
}
