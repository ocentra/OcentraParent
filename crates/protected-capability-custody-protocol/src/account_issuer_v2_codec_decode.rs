use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use crate::account_issuer::account_issuer_receipt_lineage::AccountIssuerReceiptLineage;
use crate::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerReceipt, AccountIssuerRequest,
    ProtectedAccountIssuerReceiptWire, ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
};
use crate::account_issuer_contract::{
    AccountIssuerField, ACCOUNT_ISSUER_MAX_FIELD_BYTES, ACCOUNT_ISSUER_MAX_WIRE_BYTES,
    ACCOUNT_ISSUER_PROTOCOL_VERSION, ACCOUNT_ISSUER_SERVICE, ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use crate::account_issuer_v2_codec::require;
use crate::types::ProtocolError;

const REQUEST_TAG: u8 = 1;
const RECEIPT_TAG: u8 = 2;
const AUTHJS_PROVIDER: u8 = 1;
const FIREBASE_PROVIDER: u8 = 2;

pub(super) fn request(frame: &[u8]) -> Result<AccountIssuerRequest, ProtocolError> {
    let (kind, mut cursor) = decode_header(frame, REQUEST_TAG)?;
    let correlation_id = next_field(frame, &mut cursor)?;
    let idempotency_key = next_field(frame, &mut cursor)?;
    let key_id = next_field(frame, &mut cursor)?;
    let provider = take_provider(frame, &mut cursor)?;
    let provider_subject = take_provider_subject(frame, &mut cursor)?;
    match kind {
        AccountIssuerMessageKind::IssueCurrentAuthority => {
            finish(frame, cursor)?;
            AccountIssuerRequest::issue_current_authority(
                correlation_id,
                idempotency_key,
                key_id,
                provider,
                provider_subject,
            )
        }
        AccountIssuerMessageKind::AcknowledgeReceipt => {
            let protected_receipt =
                ProtectedAccountIssuerReceiptWire::try_from(take_length_prefixed(
                    frame,
                    &mut cursor,
                    ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
                )?)?;
            finish(frame, cursor)?;
            AccountIssuerRequest::acknowledge_receipt(
                correlation_id,
                idempotency_key,
                key_id,
                provider,
                provider_subject,
                protected_receipt,
            )
        }
    }
}

pub(super) fn receipt(frame: &[u8]) -> Result<AccountIssuerReceipt, ProtocolError> {
    let (kind, mut cursor) = decode_header(frame, RECEIPT_TAG)?;
    let receipt_id = next_field(frame, &mut cursor)?;
    let correlation_id = next_field(frame, &mut cursor)?;
    let idempotency_key = next_field(frame, &mut cursor)?;
    let key_id = next_field(frame, &mut cursor)?;
    let provider = take_provider(frame, &mut cursor)?;
    let provider_subject = take_provider_subject(frame, &mut cursor)?;
    let lineage = AccountIssuerReceiptLineage::new(
        provider,
        provider_subject,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        take_generation(frame, &mut cursor)?,
        take_generation(frame, &mut cursor)?,
        take_generation(frame, &mut cursor)?,
        take_generation(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
    )?;
    let receipt = AccountIssuerReceipt::new(
        kind,
        receipt_id,
        correlation_id,
        idempotency_key,
        key_id,
        lineage,
        next_field(frame, &mut cursor)?,
        next_field(frame, &mut cursor)?,
    )?;
    finish(frame, cursor)?;
    Ok(receipt)
}

fn take_generation(wire: &[u8], cursor: &mut usize) -> Result<u64, ProtocolError> {
    let bytes: [u8; 8] = take_exact(wire, cursor, 8)?
        .try_into()
        .map_err(|_| ProtocolError::Truncated)?;
    Ok(u64::from_be_bytes(bytes))
}

fn decode_header(
    wire: &[u8],
    expected_tag: u8,
) -> Result<(AccountIssuerMessageKind, usize), ProtocolError> {
    require(!wire.is_empty(), ProtocolError::EmptyFrame)?;
    require(
        wire.len() <= ACCOUNT_ISSUER_MAX_WIRE_BYTES,
        ProtocolError::FrameTooLarge,
    )?;
    let mut cursor = 0usize;
    let domain = take_exact(wire, &mut cursor, ACCOUNT_ISSUER_TRANSPORT_DOMAIN.len())?;
    require(
        domain == ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
        ProtocolError::InvalidDomain,
    )?;
    let version = take_exact(wire, &mut cursor, 2)?;
    let version = u16::from_be_bytes([version[0], version[1]]);
    require(
        version == ACCOUNT_ISSUER_PROTOCOL_VERSION,
        ProtocolError::UnsupportedVersion(version),
    )?;
    let tag = take_exact(wire, &mut cursor, 1)?[0];
    require(tag == expected_tag, ProtocolError::InvalidDiscriminant(tag))?;
    let kind = AccountIssuerMessageKind::from_wire(take_exact(wire, &mut cursor, 1)?[0])?;
    let service = take_text_field(wire, &mut cursor)?;
    require(
        service.as_slice() == ACCOUNT_ISSUER_SERVICE.as_bytes(),
        ProtocolError::InvalidDomain,
    )?;
    Ok((kind, cursor))
}

fn take_provider(
    wire: &[u8],
    cursor: &mut usize,
) -> Result<AccountIdentityProvider, ProtocolError> {
    match take_exact(wire, cursor, 1)?[0] {
        AUTHJS_PROVIDER => Ok(AccountIdentityProvider::Authjs),
        FIREBASE_PROVIDER => Ok(AccountIdentityProvider::Firebase),
        other => Err(ProtocolError::InvalidDiscriminant(other)),
    }
}

fn take_provider_subject(
    wire: &[u8],
    cursor: &mut usize,
) -> Result<AccountIdentityProviderSubject, ProtocolError> {
    let value = take_text_field(wire, cursor)?;
    let value = String::from_utf8(value).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
    AccountIdentityProviderSubject::parse(value).ok_or(ProtocolError::EmptyField)
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
    take_length_prefixed(wire, cursor, ACCOUNT_ISSUER_MAX_FIELD_BYTES).and_then(|field| {
        String::from_utf8(field.clone()).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
        Ok(field)
    })
}

fn take_length_prefixed(
    wire: &[u8],
    cursor: &mut usize,
    max_length: usize,
) -> Result<Vec<u8>, ProtocolError> {
    let length = take_exact(wire, cursor, 4)?;
    let length = u32::from_be_bytes([length[0], length[1], length[2], length[3]]) as usize;
    require(length != 0, ProtocolError::EmptyField)?;
    require(length <= max_length, ProtocolError::FieldTooLarge)?;
    Ok(take_exact(wire, cursor, length)?.to_vec())
}

fn next_field(wire: &[u8], cursor: &mut usize) -> Result<AccountIssuerField, ProtocolError> {
    AccountIssuerField::from_wire(take_text_field(wire, cursor)?)
}

fn finish(wire: &[u8], cursor: usize) -> Result<(), ProtocolError> {
    require(cursor == wire.len(), ProtocolError::TrailingBytes)
}
