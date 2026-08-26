use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_schema::account_identity_authority::AccountIdentityProvider;
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Binding, AccountIdentityAuthorityProducerV2Claims,
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_ENROLLMENT_GENERATION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES,
};
use ring::digest::{digest, SHA256};

use super::{
    account_identity_authority_producer_v2_verify, AccountIdentityAuthorityProducerV2Error,
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
    VerifiedAccountIdentityAuthority,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority as Authority;

impl AccountIdentityAuthorityProducerV2Request {
    pub fn signing_bytes(&self) -> &[u8] {
        self.signing_bytes.as_slice()
    }

    pub fn operation(&self) -> AccountIdentityAuthorityProducerV2Operation {
        self.operation
    }

    pub fn binding(&self) -> &AccountIdentityAuthorityProducerV2Binding {
        &self.binding
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.binding.enrollment_generation
    }

    pub fn issued_at(&self) -> &str {
        self.issued_at.as_str()
    }

    pub fn expires_at(&self) -> &str {
        self.expires_at.as_str()
    }

    /// Finish an owner-created request with a platform signature. The
    /// signature is checked immediately with ring and low-S is enforced before
    /// any transport can be returned.
    pub fn finalize(
        self,
        signature: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
    ) -> Result<AccountIdentityAuthorityProducerV2Transport, AccountIdentityAuthorityProducerV2Error>
    {
        account_identity_authority_producer_v2_verify::verify_signature(
            &self.public_key,
            &self.signing_bytes,
            &signature,
        )?;
        let wire =
            crate::account_identity_authority_envelope_v2::wire(self.signing_bytes, signature)?;
        let receipt_id = self.binding.receipt_id.clone();
        Ok(AccountIdentityAuthorityProducerV2Transport {
            wire,
            receipt: AccountIdentityAuthorityProducerV2Receipt {
                receipt_id,
                operation: self.operation,
                account_id: self.binding.account_id,
                household_id: self.binding.household_id,
                service_binding_id: self.binding.service_binding_id,
                correlation_id: self.binding.correlation_id,
                idempotency_key: self.binding.idempotency_key,
                payload_digest: self.payload_digest,
                key_id: self.binding.key_id,
                key_generation: self.binding.key_generation,
                enrollment_generation: self.binding.enrollment_generation,
                authority_generation: self.binding.authority_generation,
                session_generation: self.binding.session_generation,
                issued_at: self.issued_at,
                expires_at: self.expires_at,
            },
        })
    }
}

impl AccountIdentityAuthorityProducerV2Transport {
    pub fn wire_bytes(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub fn receipt(&self) -> &AccountIdentityAuthorityProducerV2Receipt {
        &self.receipt
    }

    pub(crate) fn clone_durable(&self) -> Self {
        crate::account_identity_authority_producer_v2::from_durable_transport(
            self.wire.clone(),
            self.receipt.clone(),
        )
    }
}

pub(crate) fn issue_request(
    authority: &Authority,
    key_id: &str,
    key_generation: u64,
    enrollment_generation: u64,
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    service_binding_id: &str,
    correlation_id: &str,
    idempotency_key: &str,
    issued_at: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Error> {
    validate_issue_key(
        key_id,
        key_generation,
        enrollment_generation,
        public_key,
        service_binding_id,
    )?;
    let expires_at = issued_at
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)?;
    let payload = canonical_authority_claims_payload(authority)?;
    let payload_digest = format!("sha256:{}", sha256_hex(payload.as_slice()));
    let receipt_id = receipt_id_for(correlation_id, idempotency_key, &payload_digest);
    let issued_at_text = issued_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at_text = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let binding = AccountIdentityAuthorityProducerV2Binding {
        account_id: authority.account_id().to_string(),
        household_id: authority.household_id().to_string(),
        receipt_id: receipt_id.clone(),
        service_binding_id: service_binding_id.to_owned(),
        key_id: key_id.to_owned(),
        key_generation,
        enrollment_generation,
        authority_generation: authority.authority_generation(),
        session_generation: authority.session_generation(),
        correlation_id: correlation_id.to_owned(),
        idempotency_key: idempotency_key.to_owned(),
    };
    binding
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let envelope =
        crate::account_identity_authority_envelope_v2::CanonicalAuthorityProducerV2Envelope {
            operation: AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
            receipt_id: receipt_id.clone(),
            key_id: key_id.to_owned(),
            service_binding_id: service_binding_id.to_owned(),
            key_generation,
            enrollment_generation,
            authority_generation: authority.authority_generation(),
            session_generation: authority.session_generation(),
            correlation_id: correlation_id.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
            issued_at: issued_at_text.clone(),
            expires_at: expires_at_text.clone(),
            payload: payload.clone(),
        };
    let signing_bytes = crate::account_identity_authority_envelope_v2::encode(&envelope)?;
    Ok(AccountIdentityAuthorityProducerV2Request {
        signing_bytes,
        public_key: *public_key,
        operation: envelope.operation,
        binding,
        payload_digest,
        issued_at: issued_at_text,
        expires_at: expires_at_text,
    })
}

pub(crate) fn acknowledge_request(
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Error> {
    super::account_identity_authority_producer_v2_verify::validate_public_key(public_key)?;
    if super::account_identity_authority_producer_v2_verify::expected_key_id(public_key)
        != receipt.key_id
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId);
    }
    let payload = serde_json::to_vec(receipt)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let expires_at = now
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)?;
    let issued_at_text = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at_text = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let payload_digest = format!("sha256:{}", sha256_hex(payload.as_slice()));
    let binding = AccountIdentityAuthorityProducerV2Binding {
        account_id: receipt.account_id.clone(),
        household_id: receipt.household_id.clone(),
        receipt_id: receipt.receipt_id.clone(),
        service_binding_id: receipt.service_binding_id.clone(),
        key_id: receipt.key_id.clone(),
        key_generation: receipt.key_generation,
        enrollment_generation: receipt.enrollment_generation,
        authority_generation: receipt.authority_generation,
        session_generation: receipt.session_generation,
        correlation_id: receipt.correlation_id.clone(),
        idempotency_key: receipt.idempotency_key.clone(),
    };
    binding
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let envelope =
        crate::account_identity_authority_envelope_v2::CanonicalAuthorityProducerV2Envelope {
            operation: AccountIdentityAuthorityProducerV2Operation::AcknowledgeReceipt,
            receipt_id: receipt.receipt_id.clone(),
            key_id: receipt.key_id.clone(),
            service_binding_id: receipt.service_binding_id.clone(),
            key_generation: receipt.key_generation,
            enrollment_generation: receipt.enrollment_generation,
            authority_generation: receipt.authority_generation,
            session_generation: receipt.session_generation,
            correlation_id: receipt.correlation_id.clone(),
            idempotency_key: receipt.idempotency_key.clone(),
            issued_at: issued_at_text.clone(),
            expires_at: expires_at_text.clone(),
            payload: payload.clone(),
        };
    let signing_bytes = crate::account_identity_authority_envelope_v2::encode(&envelope)?;
    Ok(AccountIdentityAuthorityProducerV2Request {
        signing_bytes,
        public_key: *public_key,
        operation: envelope.operation,
        binding,
        payload_digest,
        issued_at: issued_at_text,
        expires_at: expires_at_text,
    })
}

fn sha256_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = digest(&SHA256, value);
    let mut text = String::with_capacity(digest.as_ref().len() * 2);
    for byte in digest.as_ref() {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn receipt_id_for(correlation_id: &str, idempotency_key: &str, payload_digest: &str) -> String {
    let mut framed = Vec::new();
    framed.extend_from_slice(b"ocentra.account-authority-producer.receipt-id.v2\0");
    for value in [
        correlation_id.as_bytes(),
        idempotency_key.as_bytes(),
        payload_digest.as_bytes(),
    ] {
        framed.extend_from_slice(&(value.len() as u64).to_be_bytes());
        framed.extend_from_slice(value);
    }
    format!("sha256:receipt:{}", sha256_hex(framed.as_slice()))
}

fn validate_issue_key(
    key_id: &str,
    key_generation: u64,
    enrollment_generation: u64,
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    service_binding_id: &str,
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    super::account_identity_authority_producer_v2_verify::validate_public_key(public_key)?;
    let expected =
        super::account_identity_authority_producer_v2_verify::expected_key_id(public_key);
    if key_id != expected
        || !key_id.starts_with(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX)
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId);
    }
    if key_generation == 0
        || enrollment_generation == 0
        || enrollment_generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_ENROLLMENT_GENERATION
        || service_binding_id.trim().is_empty()
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(())
}

fn canonical_authority_claims_payload(
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerV2Error> {
    let handoff = authority.handoff();
    let claims = AccountIdentityAuthorityProducerV2Claims {
        account_id: handoff.mapping.account_id.to_string(),
        household_id: handoff.binding.household_id.to_string(),
        provider: provider_label(&handoff.mapping.provider).to_owned(),
        provider_subject: handoff.mapping.provider_subject.as_str().to_owned(),
        member_id: handoff.member.member_id.as_str().to_owned(),
        device_id: handoff.member.device_id.as_str().to_owned(),
        session_id: handoff.member.session_id.as_str().to_owned(),
    };
    claims
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    let payload = serde_json::to_vec(&claims)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    let canonical_payload = serde_json::to_vec(
        &serde_json::from_slice::<AccountIdentityAuthorityProducerV2Claims>(&payload)
            .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?,
    )
    .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    (canonical_payload == payload)
        .then_some(payload)
        .ok_or(AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)
}

fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}
