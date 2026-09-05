use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey};

use super::AccountIdentityIssuerError;
use crate::account_identity_authority_producer::AccountIdentityAuthorityProducerCustody;
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_transport_codec.rs"]
mod codec;

pub(crate) struct StoredIssuerTransportExpectation<'a> {
    pub(crate) receipt_id: &'a str,
    pub(crate) service_label: &'a str,
    pub(crate) binding_id: &'a str,
    pub(crate) account_id: &'a str,
    pub(crate) household_id: &'a str,
    pub(crate) authority_generation: u64,
    pub(crate) key_id: &'a str,
    pub(crate) key_version: u64,
    pub(crate) issued_at_millis: i64,
    pub(crate) expires_at_millis: i64,
}

/// Validate a retained outbox transport without treating wall-clock expiry as
/// storage corruption. Canonical framing, both signatures, receipt derivation,
/// and every persisted context field are still checked at the original issue
/// instant.
pub(crate) fn validate_stored_wire(
    wire: &[u8],
    expected: &StoredIssuerTransportExpectation<'_>,
    public_key: &[u8],
) -> Result<(), AccountIdentityIssuerError> {
    let issued_at = DateTime::from_timestamp_millis(expected.issued_at_millis)
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)?;
    let parsed = codec::parse(wire, issued_at)?;
    if parsed.receipt_id != expected.receipt_id
        || parsed.service_label != expected.service_label
        || parsed.binding_id != expected.binding_id
        || parsed.account_id != expected.account_id
        || parsed.household_id != expected.household_id
        || parsed.authority_generation != expected.authority_generation
        || parsed.key_id != expected.key_id
        || parsed.key_version != expected.key_version
        || parsed.issued_at.timestamp_millis() != expected.issued_at_millis
        || parsed.expires_at.timestamp_millis() != expected.expires_at_millis
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let public_key: [u8; 32] = public_key
        .try_into()
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    if crate::account_identity_authority_producer::expected_key_id(&verifying_key)
        != expected.key_id
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    verifying_key
        .verify_strict(
            &parsed.signing_bytes,
            &Signature::from_bytes(&parsed.signature),
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let custody = VerificationCustody {
        key_id: expected.key_id.to_owned(),
        public_key: verifying_key,
    };
    let handoff = crate::account_identity_authority_producer::verify_at(
        parsed.inner_wire.as_slice(),
        &custody,
        issued_at,
    )
    .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    handoff
        .validate_shape()
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    (handoff.mapping.account_id.to_string() == expected.account_id
        && handoff.member.account_id.to_string() == expected.account_id
        && handoff.member.household_id.to_string() == expected.household_id
        && handoff.binding.account_id.to_string() == expected.account_id
        && handoff.binding.household_id.to_string() == expected.household_id
        && handoff.member.authority_generation == expected.authority_generation
        && handoff.binding.authority_generation == expected.authority_generation)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)
}

struct ParsedTransport {
    signing_bytes: Vec<u8>,
    signature: [u8; 64],
    service_label: String,
    binding_id: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    key_id: String,
    key_version: u64,
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    receipt_id: String,
    inner_wire: Vec<u8>,
}

struct VerificationCustody {
    key_id: String,
    public_key: VerifyingKey,
}

impl AccountIdentityAuthorityProducerCustody for VerificationCustody {
    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError> {
        (key_id == self.key_id)
            .then_some(self.public_key)
            .ok_or(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable)
    }
}
