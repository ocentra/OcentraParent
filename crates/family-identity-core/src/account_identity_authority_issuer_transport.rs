use chrono::{DateTime, Duration, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use getrandom::fill;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use ocentra_schema::account_identity_authority_producer::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS;

use super::key_custody::RegisteredProducerCustody;
use super::key_registry::RegisteredIssuerKey;
use super::service_binding::AccountIdentityIssuerServiceBinding;
use super::AccountIdentityIssuerError;
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer::{
    AccountIdentityAuthorityProducerCustody, AccountIdentityAuthorityProducerTransport,
};
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_transport_codec.rs"]
mod codec;

pub(crate) struct AccountIdentityIssuerTransport {
    wire: Vec<u8>,
    receipt_id: String,
    key_id: String,
    key_version: u64,
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl AccountIdentityIssuerTransport {
    pub(crate) fn wire_bytes(&self) -> &[u8] {
        &self.wire
    }

    pub(crate) fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    pub(crate) fn key_id(&self) -> &str {
        &self.key_id
    }

    pub(crate) fn key_version(&self) -> u64 {
        self.key_version
    }

    pub(crate) fn issued_at(&self) -> DateTime<Utc> {
        self.issued_at
    }

    pub(crate) fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}

pub(crate) struct VerifiedIssuerTransport {
    handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    receipt_id: String,
    key_id: String,
    key_version: u64,
}

impl VerifiedIssuerTransport {
    pub(crate) fn into_handoff(self) -> AccountIdentityCurrentMemberDeviceAuthorityHandoff {
        self.handoff
    }

    pub(crate) fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    pub(crate) fn key_id(&self) -> &str {
        &self.key_id
    }

    pub(crate) fn key_version(&self) -> u64 {
        self.key_version
    }
}

pub(crate) fn issue(
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    registered: &RegisteredIssuerKey,
    custody: &RegisteredProducerCustody<'_>,
    inner: AccountIdentityAuthorityProducerTransport,
    issued_at: DateTime<Utc>,
) -> Result<AccountIdentityIssuerTransport, AccountIdentityIssuerError> {
    let expires_at = issued_at
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityIssuerError::InvalidClock)?;
    let mut nonce = [0_u8; codec::NONCE_BYTES];
    fill(&mut nonce).map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let signing_bytes = codec::encode(
        binding,
        authority,
        registered,
        issued_at,
        expires_at,
        &nonce,
        inner.wire_bytes(),
    )?;
    let signature = custody.sign(&signing_bytes)?;
    registered
        .verifying_key
        .verify_strict(&signing_bytes, &Signature::from_bytes(&signature))
        .map_err(|_| AccountIdentityIssuerError::SignerCustodyUnavailable)?;
    let mut wire = signing_bytes.clone();
    wire.extend_from_slice(&signature);
    let receipt_id = codec::receipt_id(binding, authority, registered, &nonce, &signing_bytes);
    Ok(AccountIdentityIssuerTransport {
        wire,
        receipt_id,
        key_id: registered.handle.key_id().to_owned(),
        key_version: registered.handle.key_version(),
        issued_at,
        expires_at,
    })
}

pub(crate) fn verify(
    wire: &[u8],
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    registered: &RegisteredIssuerKey,
    now: DateTime<Utc>,
) -> Result<VerifiedIssuerTransport, AccountIdentityIssuerError> {
    let parsed = codec::parse(wire, now)?;
    if parsed.service_label != binding.service().label()
        || parsed.binding_id != binding.binding_id()
        || parsed.account_id != authority.account_id().to_string()
        || parsed.household_id != authority.household_id().to_string()
        || parsed.authority_generation != authority.authority_generation()
        || parsed.key_id != registered.handle.key_id()
        || parsed.key_version != registered.handle.key_version()
    {
        return Err(AccountIdentityIssuerError::TransportContextMismatch);
    }
    registered
        .verifying_key
        .verify_strict(
            &parsed.signing_bytes,
            &Signature::from_bytes(&parsed.signature),
        )
        .map_err(|_| AccountIdentityIssuerError::InvalidTransport)?;
    let custody = VerificationCustody {
        key_id: registered.handle.key_id().to_owned(),
        public_key: registered.verifying_key,
    };
    let handoff = crate::account_identity_authority_producer::verify_at(
        parsed.inner_wire.as_slice(),
        &custody,
        now,
    )
    .map_err(AccountIdentityIssuerError::Producer)?;
    if handoff.member.account_id != *authority.account_id()
        || handoff.member.household_id != *authority.household_id()
        || handoff.member.authority_generation != authority.authority_generation()
        || handoff.binding.account_id != *authority.account_id()
        || handoff.binding.household_id != *authority.household_id()
        || handoff.binding.authority_generation != authority.authority_generation()
    {
        return Err(AccountIdentityIssuerError::TransportContextMismatch);
    }
    Ok(VerifiedIssuerTransport {
        handoff,
        receipt_id: parsed.receipt_id,
        key_id: parsed.key_id,
        key_version: parsed.key_version,
    })
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
    receipt_id: String,
    inner_wire: Vec<u8>,
}

struct VerificationCustody {
    key_id: String,
    public_key: VerifyingKey,
}

impl AccountIdentityAuthorityProducerCustody for VerificationCustody {
    fn signing_key_id(&self) -> &str {
        "verification-only"
    }

    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError> {
        (key_id == self.key_id)
            .then_some(self.public_key)
            .ok_or(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable)
    }

    fn sign(&self, _payload: &[u8]) -> Result<[u8; 64], AccountIdentityAuthorityProducerError> {
        Err(AccountIdentityAuthorityProducerError::SignerCustodyUnavailable)
    }
}
