use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Claims, AccountIdentityAuthorityProducerV2Operation,
    AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES,
};
use ring::digest::{digest, SHA256};
use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};
use sha2::{Digest, Sha256};

use super::{
    AccountIdentityAuthorityProducerV2Error, AccountIdentityAuthorityProducerV2Verified,
    AccountIdentityAuthorityProducerV2VerifiedReceipt,
};

pub(crate) fn verify(
    wire: &[u8],
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Verified, AccountIdentityAuthorityProducerV2Error> {
    validate_public_key(public_key)?;
    let parsed = crate::account_identity_authority_envelope_v2::parse(wire)?;
    if expected_key_id(public_key) != parsed.key_id {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId);
    }
    verify_signature(public_key, &parsed.signing_bytes, &parsed.signature)?;
    super::account_identity_authority_producer_v2_time::validate_lifetime(
        &parsed.issued_at,
        &parsed.expires_at,
        now,
    )?;
    if parsed.operation != AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority {
        return Err(AccountIdentityAuthorityProducerV2Error::UnsupportedOperation);
    }
    let claims: AccountIdentityAuthorityProducerV2Claims = serde_json::from_slice(&parsed.payload)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    claims
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    let canonical_payload = serde_json::to_vec(&claims)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::AuthorityInvalid)?;
    if canonical_payload != parsed.payload {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(AccountIdentityAuthorityProducerV2Verified {
        operation: parsed.operation,
        receipt_id: parsed.receipt_id,
        key_id: parsed.key_id,
        service_binding_id: parsed.service_binding_id,
        key_generation: parsed.key_generation,
        enrollment_generation: parsed.enrollment_generation,
        authority_generation: parsed.authority_generation,
        session_generation: parsed.session_generation,
        correlation_id: parsed.correlation_id,
        idempotency_key: parsed.idempotency_key,
        issued_at: parsed.issued_at,
        expires_at: parsed.expires_at,
        payload_digest: payload_digest(canonical_payload.as_slice()),
        claims,
    })
}

pub(crate) fn verify_receipt(
    wire: &[u8],
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<
    AccountIdentityAuthorityProducerV2VerifiedReceipt,
    AccountIdentityAuthorityProducerV2Error,
> {
    validate_public_key(public_key)?;
    let parsed = crate::account_identity_authority_envelope_v2::parse(wire)?;
    if expected_key_id(public_key) != parsed.key_id {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId);
    }
    verify_signature(public_key, &parsed.signing_bytes, &parsed.signature)?;
    super::account_identity_authority_producer_v2_time::validate_lifetime(
        &parsed.issued_at,
        &parsed.expires_at,
        now,
    )?;
    if parsed.operation != AccountIdentityAuthorityProducerV2Operation::AcknowledgeReceipt {
        return Err(AccountIdentityAuthorityProducerV2Error::UnsupportedOperation);
    }
    let receipt: AccountIdentityAuthorityProducerV2Receipt =
        serde_json::from_slice(&parsed.payload)
            .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    receipt
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let canonical_receipt = serde_json::to_vec(&receipt)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    if canonical_receipt != parsed.payload
        || receipt.operation != AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    if receipt.receipt_id != parsed.receipt_id
        || receipt.key_id != parsed.key_id
        || receipt.service_binding_id != parsed.service_binding_id
        || receipt.key_generation != parsed.key_generation
        || receipt.enrollment_generation != parsed.enrollment_generation
        || receipt.authority_generation != parsed.authority_generation
        || receipt.session_generation != parsed.session_generation
        || receipt.correlation_id != parsed.correlation_id
        || receipt.idempotency_key != parsed.idempotency_key
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(AccountIdentityAuthorityProducerV2VerifiedReceipt { receipt })
}

pub(crate) fn expected_key_id(
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
) -> String {
    let mut digest = Sha256::new();
    digest.update(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN);
    digest.update(public_key);
    format!(
        "{}{:x}",
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
        digest.finalize()
    )
}

pub(crate) fn validate_public_key(
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    if super::sec1::parse_uncompressed_p256(public_key).is_none() {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidPublicKey);
    }
    Ok(())
}

fn payload_digest(payload: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = digest(&SHA256, payload);
    let mut text = String::with_capacity(digest.as_ref().len() * 2);
    for byte in digest.as_ref() {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    format!("sha256:{text}")
}

pub(super) fn verify_signature(
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    signing_bytes: &[u8],
    signature: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    validate_public_key(public_key)?;
    if !is_low_s(&signature[32..]) {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidSignature);
    }
    UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key)
        .verify(signing_bytes, signature)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::SignatureInvalid)
}

fn is_low_s(value: &[u8]) -> bool {
    const P256_HALF_ORDER: [u8; 32] = [
        0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xde, 0x73, 0x7d, 0x56, 0xd3, 0x8b, 0xcf, 0x42, 0x79, 0xdc, 0xe5, 0x61, 0x7e, 0x31,
        0x92, 0xa8,
    ];
    value.len() == P256_HALF_ORDER.len()
        && value <= P256_HALF_ORDER.as_slice()
        && value.iter().any(|byte| *byte != 0)
}
