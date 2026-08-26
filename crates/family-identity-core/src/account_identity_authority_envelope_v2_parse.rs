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

use super::ParsedAuthorityProducerV2Envelope;
use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Error;

pub(super) fn parse(
    wire: &[u8],
) -> Result<ParsedAuthorityProducerV2Envelope, AccountIdentityAuthorityProducerV2Error> {
    if wire.len() <= 64 || wire.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let signing_length = wire
        .len()
        .checked_sub(64)
        .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let signing_bytes = wire[..signing_length].to_vec();
    let signature = wire[signing_length..]
        .try_into()
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidSignature)?;
    let mut cursor = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN.len();
    if signing_bytes.get(..cursor) != Some(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN) {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let operation = operation_from_kind(
        *take_exact(&signing_bytes, &mut cursor, 1)?
            .first()
            .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)?,
    )?;
    let version = take_text(&signing_bytes, &mut cursor)?;
    if version != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let audience = take_text(&signing_bytes, &mut cursor)?;
    let environment = take_text(&signing_bytes, &mut cursor)?;
    let algorithm = take_text(&signing_bytes, &mut cursor)?;
    let service = take_text(&signing_bytes, &mut cursor)?;
    if audience != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE
        || environment != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT
        || algorithm != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM
        || service != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
    {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let receipt_id = take_text(&signing_bytes, &mut cursor)?;
    let key_id = take_text(&signing_bytes, &mut cursor)?;
    let service_binding_id = take_text(&signing_bytes, &mut cursor)?;
    let key_generation = take_u64(&signing_bytes, &mut cursor)?;
    let enrollment_generation = take_u64(&signing_bytes, &mut cursor)?;
    let authority_generation = take_u64(&signing_bytes, &mut cursor)?;
    let session_generation = take_u64(&signing_bytes, &mut cursor)?;
    let correlation_id = take_text(&signing_bytes, &mut cursor)?;
    let idempotency_key = take_text(&signing_bytes, &mut cursor)?;
    let issued_at = take_text(&signing_bytes, &mut cursor)?;
    let expires_at = take_text(&signing_bytes, &mut cursor)?;
    let payload = take_field(&signing_bytes, &mut cursor)?;
    if cursor != signing_bytes.len() {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(ParsedAuthorityProducerV2Envelope {
        signing_bytes,
        signature,
        operation,
        receipt_id,
        key_id,
        service_binding_id,
        key_generation,
        enrollment_generation,
        authority_generation,
        session_generation,
        correlation_id,
        idempotency_key,
        issued_at,
        expires_at,
        payload,
    })
}

fn operation_from_kind(
    kind: u8,
) -> Result<AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Error> {
    match kind {
        6 => Ok(AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority),
        7 => Ok(AccountIdentityAuthorityProducerV2Operation::AcknowledgeReceipt),
        _ => Err(AccountIdentityAuthorityProducerV2Error::InvalidWire),
    }
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

fn take_exact<'a>(
    bytes: &'a [u8],
    cursor: &mut usize,
    length: usize,
) -> Result<&'a [u8], AccountIdentityAuthorityProducerV2Error> {
    let end = cursor
        .checked_add(length)
        .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    let field = bytes
        .get(*cursor..end)
        .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    *cursor = end;
    Ok(field)
}

fn take_field(
    bytes: &[u8],
    cursor: &mut usize,
) -> Result<Vec<u8>, AccountIdentityAuthorityProducerV2Error> {
    let length = take_exact(bytes, cursor, 4)?;
    let length = u32::from_be_bytes([length[0], length[1], length[2], length[3]]) as usize;
    if length == 0 || length > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(take_exact(bytes, cursor, length)?.to_vec())
}

fn take_text(
    bytes: &[u8],
    cursor: &mut usize,
) -> Result<String, AccountIdentityAuthorityProducerV2Error> {
    let value = take_field(bytes, cursor)?;
    let value = String::from_utf8(value)
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    validate_text(&value)?;
    Ok(value)
}

fn take_u64(
    bytes: &[u8],
    cursor: &mut usize,
) -> Result<u64, AccountIdentityAuthorityProducerV2Error> {
    let value = take_field(bytes, cursor)?;
    if value.len() != 8 {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    let value = u64::from_be_bytes(
        value
            .try_into()
            .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)?,
    );
    (value > 0 && value <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION)
        .then_some(value)
        .ok_or(AccountIdentityAuthorityProducerV2Error::InvalidWire)
}
