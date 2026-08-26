use crate::account_issuer::{AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerRequest};
use crate::account_issuer_contract::{
    AccountIssuerField, ACCOUNT_ISSUER_MAX_FIELD_BYTES, ACCOUNT_ISSUER_MAX_INNER_BYTES,
    ACCOUNT_ISSUER_MAX_WIRE_BYTES, ACCOUNT_ISSUER_PROTOCOL_VERSION, ACCOUNT_ISSUER_SERVICE,
    ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use crate::types::ProtocolError;

const REQUEST_TAG: u8 = 1;
const RECEIPT_TAG: u8 = 2;

pub(super) fn request(frame: &[u8]) -> Result<AccountIssuerRequest, ProtocolError> {
    let (kind, fields) = decode(frame, REQUEST_TAG, 4)?;
    let mut fields = fields.into_iter();
    AccountIssuerRequest::new(
        kind,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
        next_bytes(&mut fields)?,
    )
}

pub(super) fn receipt(frame: &[u8]) -> Result<AccountIssuerReceipt, ProtocolError> {
    let (kind, fields) = decode(frame, RECEIPT_TAG, 5)?;
    let mut fields = fields.into_iter();
    AccountIssuerReceipt::new(
        kind,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
        next_field(&mut fields)?,
    )
}

fn decode(
    wire: &[u8],
    expected_tag: u8,
    field_count: usize,
) -> Result<(AccountIssuerMessageKind, Vec<Vec<u8>>), ProtocolError> {
    if wire.is_empty() {
        return Err(ProtocolError::EmptyFrame);
    }
    if wire.len() > ACCOUNT_ISSUER_MAX_WIRE_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let mut cursor = 0usize;
    let domain = take_exact(wire, &mut cursor, ACCOUNT_ISSUER_TRANSPORT_DOMAIN.len())?;
    if domain != ACCOUNT_ISSUER_TRANSPORT_DOMAIN {
        return Err(ProtocolError::InvalidDomain);
    }
    let version = take_exact(wire, &mut cursor, 2)?;
    let version = u16::from_be_bytes([version[0], version[1]]);
    if version != ACCOUNT_ISSUER_PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion(version));
    }
    let tag = take_exact(wire, &mut cursor, 1)?[0];
    if tag != expected_tag {
        return Err(ProtocolError::InvalidDiscriminant(tag));
    }
    let kind = AccountIssuerMessageKind::from_wire(take_exact(wire, &mut cursor, 1)?[0])?;
    let service = take_text_field(wire, &mut cursor)?;
    if service.as_slice() != ACCOUNT_ISSUER_SERVICE.as_bytes() {
        return Err(ProtocolError::InvalidDomain);
    }
    let mut fields = Vec::with_capacity(field_count);
    for _ in 0..field_count {
        let field = (expected_tag == REQUEST_TAG && fields.len() == 3)
            .then(|| take_inner_field(wire, &mut cursor))
            .unwrap_or_else(|| take_text_field(wire, &mut cursor))?;
        fields.push(field);
    }
    if cursor != wire.len() {
        return Err(ProtocolError::TrailingBytes);
    }
    Ok((kind, fields))
}

fn take_exact<'a>(
    wire: &'a [u8],
    cursor: &mut usize,
    length: usize,
) -> Result<&'a [u8], ProtocolError> {
    let end = cursor.checked_add(length).ok_or(ProtocolError::Truncated)?;
    let value = wire.get(*cursor..end).ok_or(ProtocolError::Truncated)?;
    *cursor = end;
    Ok(value)
}

fn take_text_field(wire: &[u8], cursor: &mut usize) -> Result<Vec<u8>, ProtocolError> {
    let field = take_length_prefixed(wire, cursor, ACCOUNT_ISSUER_MAX_FIELD_BYTES)?;
    String::from_utf8(field.clone()).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
    Ok(field)
}

fn take_inner_field(wire: &[u8], cursor: &mut usize) -> Result<Vec<u8>, ProtocolError> {
    take_length_prefixed(wire, cursor, ACCOUNT_ISSUER_MAX_INNER_BYTES)
}

fn take_length_prefixed(
    wire: &[u8],
    cursor: &mut usize,
    max_length: usize,
) -> Result<Vec<u8>, ProtocolError> {
    let length = take_exact(wire, cursor, 4)?;
    let length = u32::from_be_bytes([length[0], length[1], length[2], length[3]]) as usize;
    if length == 0 {
        return Err(ProtocolError::EmptyField);
    }
    if length > max_length {
        return Err(ProtocolError::FieldTooLarge);
    }
    Ok(take_exact(wire, cursor, length)?.to_vec())
}

fn next_field<I>(fields: &mut I) -> Result<AccountIssuerField, ProtocolError>
where
    I: Iterator<Item = Vec<u8>>,
{
    let value = fields.next().ok_or(ProtocolError::Truncated)?;
    AccountIssuerField::from_wire(value)
}

fn next_bytes<I>(fields: &mut I) -> Result<Vec<u8>, ProtocolError>
where
    I: Iterator<Item = Vec<u8>>,
{
    fields.next().ok_or(ProtocolError::Truncated)
}
