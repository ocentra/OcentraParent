use ocentra_schema::account_identity_authority_producer::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_AUDIENCE, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_ENVIRONMENT,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_ALGORITHM,
};

use super::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

const DOMAIN_SEPARATOR: &[u8] = b"ocentra.account-authority-producer.signing.v1\0";

pub(crate) struct CanonicalAuthorityProducerEnvelope {
    pub(crate) key_id: String,
    pub(crate) issued_at: String,
    pub(crate) expires_at: String,
    pub(crate) payload: Vec<u8>,
}

pub(crate) fn encode(
    envelope: &CanonicalAuthorityProducerEnvelope,
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerError> {
    if envelope.key_id.trim().is_empty()
        || envelope.issued_at.trim().is_empty()
        || envelope.expires_at.trim().is_empty()
        || envelope.key_id.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES
        || envelope.issued_at.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES
        || envelope.expires_at.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES
        || envelope.payload.is_empty()
        || envelope.payload.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES
    {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }

    let mut bytes = Vec::with_capacity(
        DOMAIN_SEPARATOR.len()
            + 4 * 4
            + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES.min(envelope.key_id.len())
            + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES.min(envelope.issued_at.len())
            + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES.min(envelope.expires_at.len())
            + envelope.payload.len(),
    );
    bytes.extend_from_slice(DOMAIN_SEPARATOR);
    for field in [
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SCHEMA_VERSION,
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_AUDIENCE,
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_ENVIRONMENT,
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_ALGORITHM,
    ] {
        append_field(&mut bytes, field.as_bytes())?;
    }
    append_field(&mut bytes, envelope.key_id.as_bytes())?;
    append_field(&mut bytes, envelope.issued_at.as_bytes())?;
    append_field(&mut bytes, envelope.expires_at.as_bytes())?;
    append_field(&mut bytes, &envelope.payload)?;
    Ok(bytes)
}

pub(crate) fn wire(
    signing_bytes: Vec<u8>,
    signature: [u8; 64],
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerError> {
    let wire_length = signing_bytes
        .len()
        .checked_add(signature.len())
        .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)?;
    if wire_length > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    let mut wire = signing_bytes;
    wire.extend_from_slice(&signature);
    Ok(wire)
}

pub(crate) fn domain_separator() -> &'static [u8] {
    DOMAIN_SEPARATOR
}

fn append_field(
    bytes: &mut Vec<u8>,
    field: &[u8],
) -> Result<(), AccountIdentityAuthorityProducerError> {
    let length = u32::try_from(field.len())
        .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(field);
    Ok(())
}
