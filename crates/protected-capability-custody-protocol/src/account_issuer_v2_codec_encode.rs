use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use crate::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerRequest,
    AccountIssuerRequestOperation, ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
};
use crate::account_issuer_contract::{
    ACCOUNT_ISSUER_MAX_FIELD_BYTES, ACCOUNT_ISSUER_MAX_WIRE_BYTES, ACCOUNT_ISSUER_PROTOCOL_VERSION,
    ACCOUNT_ISSUER_SERVICE, ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use crate::types::ProtocolError;

#[path = "account_issuer_v2_codec_encode_receipt_lineage.rs"]
mod receipt_lineage;

const REQUEST_TAG: u8 = 1;
const RECEIPT_TAG: u8 = 2;
const AUTHJS_PROVIDER: u8 = 1;
const FIREBASE_PROVIDER: u8 = 2;

pub(super) fn request(request: &AccountIssuerRequest) -> Result<Vec<u8>, ProtocolError> {
    let mut wire = header(REQUEST_TAG, request.kind())?;
    append_field(
        &mut wire,
        request.correlation_id().as_bytes(),
        ACCOUNT_ISSUER_MAX_FIELD_BYTES,
    )?;
    append_field(
        &mut wire,
        request.idempotency_key().as_bytes(),
        ACCOUNT_ISSUER_MAX_FIELD_BYTES,
    )?;
    append_field(
        &mut wire,
        request.key_id().as_bytes(),
        ACCOUNT_ISSUER_MAX_FIELD_BYTES,
    )?;
    match request.operation() {
        AccountIssuerRequestOperation::IssueCurrentAuthority {
            provider,
            provider_subject,
        } => {
            append_provider(&mut wire, provider);
            append_field(
                &mut wire,
                provider_subject.as_str().as_bytes(),
                ACCOUNT_ISSUER_MAX_FIELD_BYTES,
            )?;
        }
        AccountIssuerRequestOperation::AcknowledgeReceipt {
            provider,
            provider_subject,
            protected_receipt,
        } => {
            append_provider(&mut wire, provider);
            append_field(
                &mut wire,
                provider_subject.as_str().as_bytes(),
                ACCOUNT_ISSUER_MAX_FIELD_BYTES,
            )?;
            append_field(
                &mut wire,
                protected_receipt.as_bytes(),
                ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
            )?;
        }
    }
    finish(wire)
}

pub(super) fn receipt(receipt: &AccountIssuerReceipt) -> Result<Vec<u8>, ProtocolError> {
    let mut wire = header(RECEIPT_TAG, receipt.kind())?;
    for field in [
        receipt.receipt_id().as_bytes(),
        receipt.correlation_id().as_bytes(),
        receipt.idempotency_key().as_bytes(),
        receipt.key_id().as_bytes(),
    ] {
        append_field(&mut wire, field, ACCOUNT_ISSUER_MAX_FIELD_BYTES)?;
    }
    receipt_lineage::append(&mut wire, receipt.lineage())?;
    for field in [
        receipt.result_digest().as_bytes(),
        receipt.signed_transport_digest().as_bytes(),
    ] {
        append_field(&mut wire, field, ACCOUNT_ISSUER_MAX_FIELD_BYTES)?;
    }
    finish(wire)
}

fn header(tag: u8, kind: AccountIssuerMessageKind) -> Result<Vec<u8>, ProtocolError> {
    let mut wire = Vec::with_capacity(ACCOUNT_ISSUER_MAX_FIELD_BYTES * 6);
    wire.extend_from_slice(ACCOUNT_ISSUER_TRANSPORT_DOMAIN);
    wire.extend_from_slice(&ACCOUNT_ISSUER_PROTOCOL_VERSION.to_be_bytes());
    wire.push(tag);
    wire.push(kind.as_wire());
    append_field(
        &mut wire,
        ACCOUNT_ISSUER_SERVICE.as_bytes(),
        ACCOUNT_ISSUER_MAX_FIELD_BYTES,
    )?;
    Ok(wire)
}

fn append_provider(target: &mut Vec<u8>, provider: &AccountIdentityProvider) {
    let value = match provider {
        AccountIdentityProvider::Authjs => AUTHJS_PROVIDER,
        AccountIdentityProvider::Firebase => FIREBASE_PROVIDER,
    };
    target.push(value);
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
    let length = u32::try_from(field.len()).map_err(|_error| ProtocolError::FieldTooLarge)?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(field);
    Ok(())
}

fn finish(wire: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
    if wire.len() > ACCOUNT_ISSUER_MAX_WIRE_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    Ok(wire)
}
