use crate::account_issuer::{AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerRequest};
use crate::account_issuer_contract::{
    ACCOUNT_ISSUER_MAX_FIELD_BYTES, ACCOUNT_ISSUER_MAX_INNER_BYTES, ACCOUNT_ISSUER_MAX_WIRE_BYTES,
    ACCOUNT_ISSUER_PROTOCOL_VERSION, ACCOUNT_ISSUER_SERVICE, ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use crate::types::ProtocolError;

const REQUEST_TAG: u8 = 1;
const RECEIPT_TAG: u8 = 2;

pub(super) fn request(request: &AccountIssuerRequest) -> Result<Vec<u8>, ProtocolError> {
    let fields = [
        request.correlation_id().as_bytes(),
        request.idempotency_key().as_bytes(),
        request.key_id().as_bytes(),
        request.inner_wire(),
    ];
    encode(REQUEST_TAG, request.kind(), &fields)
}

pub(super) fn receipt(receipt: &AccountIssuerReceipt) -> Result<Vec<u8>, ProtocolError> {
    let fields = [
        receipt.receipt_id().as_bytes(),
        receipt.correlation_id().as_bytes(),
        receipt.idempotency_key().as_bytes(),
        receipt.key_id().as_bytes(),
        receipt.result_digest().as_bytes(),
    ];
    encode(RECEIPT_TAG, receipt.kind(), &fields)
}

fn encode(
    tag: u8,
    kind: AccountIssuerMessageKind,
    fields: &[&[u8]],
) -> Result<Vec<u8>, ProtocolError> {
    let mut wire = Vec::with_capacity(ACCOUNT_ISSUER_MAX_FIELD_BYTES * fields.len());
    wire.extend_from_slice(ACCOUNT_ISSUER_TRANSPORT_DOMAIN);
    wire.extend_from_slice(&ACCOUNT_ISSUER_PROTOCOL_VERSION.to_be_bytes());
    wire.push(tag);
    wire.push(kind.as_wire());
    append_field(
        &mut wire,
        ACCOUNT_ISSUER_SERVICE.as_bytes(),
        ACCOUNT_ISSUER_MAX_FIELD_BYTES,
    )?;
    for (index, field) in fields.iter().enumerate() {
        let max_length = (tag == REQUEST_TAG && index == 3)
            .then_some(ACCOUNT_ISSUER_MAX_INNER_BYTES)
            .unwrap_or(ACCOUNT_ISSUER_MAX_FIELD_BYTES);
        append_field(&mut wire, field, max_length)?;
    }
    if wire.len() > ACCOUNT_ISSUER_MAX_WIRE_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    Ok(wire)
}

fn append_field(
    target: &mut Vec<u8>,
    field: &[u8],
    max_length: usize,
) -> Result<(), ProtocolError> {
    if field.is_empty() {
        return Err(ProtocolError::EmptyField);
    }
    if field.len() > max_length {
        return Err(ProtocolError::FieldTooLarge);
    }
    let length = u32::try_from(field.len()).map_err(|_| ProtocolError::FieldTooLarge)?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(field);
    Ok(())
}
