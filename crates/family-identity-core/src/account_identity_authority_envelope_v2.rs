//! Canonical inner v2 Account authority envelope.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM,
};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Error;

pub(crate) struct CanonicalAuthorityProducerV2Envelope {
    pub(crate) operation: AccountIdentityAuthorityProducerV2Operation,
    pub(crate) receipt_id: String,
    pub(crate) key_id: String,
    pub(crate) service_binding_id: String,
    pub(crate) key_generation: u64,
    pub(crate) enrollment_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) session_generation: u64,
    pub(crate) correlation_id: String,
    pub(crate) idempotency_key: String,
    pub(crate) issued_at: String,
    pub(crate) expires_at: String,
    pub(crate) payload: Vec<u8>,
}

pub(crate) struct ParsedAuthorityProducerV2Envelope {
    pub(crate) signing_bytes: Vec<u8>,
    pub(crate) signature: [u8; 64],
    pub(crate) operation: AccountIdentityAuthorityProducerV2Operation,
    pub(crate) receipt_id: String,
    pub(crate) key_id: String,
    pub(crate) service_binding_id: String,
    pub(crate) key_generation: u64,
    pub(crate) enrollment_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) session_generation: u64,
    pub(crate) correlation_id: String,
    pub(crate) idempotency_key: String,
    pub(crate) issued_at: String,
    pub(crate) expires_at: String,
    pub(crate) payload: Vec<u8>,
}

#[path = "account_identity_authority_envelope_v2_parse.rs"]
mod parse;

pub(crate) fn encode(
    envelope: &CanonicalAuthorityProducerV2Envelope,
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerV2Error> {
    validate_text(&envelope.key_id)?;
    validate_text(&envelope.receipt_id)?;
    validate_text(&envelope.service_binding_id)?;
    validate_text(&envelope.correlation_id)?;
    validate_text(&envelope.idempotency_key)?;
    validate_text(&envelope.issued_at)?;
    validate_text(&envelope.expires_at)?;
    if envelope.key_generation == 0
        || envelope.key_generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
        || envelope.enrollment_generation == 0
        || envelope.enrollment_generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
        || envelope.authority_generation == 0
        || envelope.authority_generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
        || envelope.session_generation == 0
        || envelope.session_generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
        || envelope.payload.is_empty()
        || envelope.payload.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }

    let key_generation = envelope.key_generation.to_be_bytes();
    let enrollment_generation = envelope.enrollment_generation.to_be_bytes();
    let authority_generation = envelope.authority_generation.to_be_bytes();
    let session_generation = envelope.session_generation.to_be_bytes();
    let fields: [&[u8]; 16] = [
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.as_bytes(),
        envelope.receipt_id.as_bytes(),
        envelope.key_id.as_bytes(),
        envelope.service_binding_id.as_bytes(),
        &key_generation,
        &enrollment_generation,
        &authority_generation,
        &session_generation,
        envelope.correlation_id.as_bytes(),
        envelope.idempotency_key.as_bytes(),
        envelope.issued_at.as_bytes(),
        envelope.expires_at.as_bytes(),
    ];
    let mut bytes = Vec::with_capacity(
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN.len()
            + fields.len() * 8
            + envelope.payload.len(),
    );
    bytes.extend_from_slice(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN);
    bytes.push(envelope.operation.message_kind());
    for field in fields {
        append_field(&mut bytes, field)?;
    }
    append_field(&mut bytes, &envelope.payload)?;
    Ok(bytes)
}

pub(crate) fn wire(
    signing_bytes: Vec<u8>,
    signature: [u8; 64],
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerV2Error> {
    let length = signing_bytes
        .len()
        .checked_add(signature.len())
        .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    if length > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let mut wire = signing_bytes;
    wire.extend_from_slice(&signature);
    Ok(wire)
}

pub(crate) fn parse(
    wire: &[u8],
) -> Result<ParsedAuthorityProducerV2Envelope, AccountIdentityAuthorityProducerV2Error> {
    parse::parse(wire)
}

fn validate_text(value: &str) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    if value.trim().is_empty()
        || value.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES
        || value
            .chars()
            .any(|character| character <= '\u{001f}' || character == '\u{007f}')
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(())
}

fn append_field(
    target: &mut Vec<u8>,
    field: &[u8],
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    if field.is_empty() || field.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let length = u32::try_from(field.len())
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(field);
    Ok(())
}
