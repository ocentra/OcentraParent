use ocentra_protected_capability_custody_protocol::account_issuer::account_issuer_receipt_lineage::AccountIssuerReceiptLineage;
use ocentra_protected_capability_custody_protocol::account_issuer::{
    AccountIssuerMessageKind, AccountIssuerRequest, ProtectedAccountIssuerReceiptWire,
    ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::{
    AccountIssuerField, ACCOUNT_ISSUER_MAX_FIELD_BYTES, ACCOUNT_ISSUER_MAX_WIRE_BYTES,
    ACCOUNT_ISSUER_PROTOCOL_VERSION, ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use ocentra_protected_capability_custody_protocol::types::ProtocolError;
use ocentra_protected_capability_custody_protocol::{
    decode_account_issuer_receipt, decode_account_issuer_request, encode_account_issuer_receipt,
    encode_account_issuer_request,
};
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

fn field(value: &str) -> Result<AccountIssuerField, ProtocolError> {
    AccountIssuerField::from_wire(value.as_bytes().to_vec())
}

fn subject() -> Result<AccountIdentityProviderSubject, ProtocolError> {
    AccountIdentityProviderSubject::parse("authjs-subject-1").ok_or(ProtocolError::EmptyField)
}

fn key_id() -> String {
    format!("sha256:ecdsa-p256:{}", "0".repeat(64))
}

fn receipt_id() -> String {
    format!("sha256:receipt:{}", "0".repeat(64))
}

fn binding_id() -> String {
    format!("sha256:binding:{}", "0".repeat(64))
}

fn digest() -> String {
    format!("sha256:{}", "0".repeat(64))
}

fn signed_transport_digest() -> String {
    format!("sha256:signed-transport:{}", "0".repeat(64))
}

fn issue_request() -> Result<AccountIssuerRequest, ProtocolError> {
    AccountIssuerRequest::issue_current_authority(
        field("correlation-1")?,
        field("idempotency-1")?,
        field(&key_id())?,
        AccountIdentityProvider::Authjs,
        subject()?,
    )
}

fn issue_receipt() -> Result<
    ocentra_protected_capability_custody_protocol::account_issuer::AccountIssuerReceipt,
    ProtocolError,
> {
    let lineage = AccountIssuerReceiptLineage::new(
        AccountIdentityProvider::Authjs,
        subject()?,
        field("account-1")?,
        field("household-1")?,
        field("member-1")?,
        field("device-1")?,
        field("session-1")?,
        field(&binding_id())?,
        1,
        2,
        3,
        4,
        field("2026-08-28T00:00:00.000Z")?,
        field("2026-08-28T00:05:00.000Z")?,
    )?;
    ocentra_protected_capability_custody_protocol::account_issuer::AccountIssuerReceipt::new(
        AccountIssuerMessageKind::IssueCurrentAuthority,
        field(&receipt_id())?,
        field("correlation-1")?,
        field("idempotency-1")?,
        field(&key_id())?,
        lineage,
        field(&digest())?,
        field(&signed_transport_digest())?,
    )
}

#[test]
fn account_issuer_request_and_receipt_round_trip_without_generic_authority_bytes(
) -> Result<(), ProtocolError> {
    let request = issue_request()?;
    let request_frame = encode_account_issuer_request(&request)?;
    let decoded_request = decode_account_issuer_request(&request_frame)?;

    assert_eq!(decoded_request, request);
    assert_eq!(
        decoded_request.kind(),
        AccountIssuerMessageKind::IssueCurrentAuthority
    );
    assert_eq!(decoded_request.correlation_id(), request.correlation_id());
    assert_eq!(decoded_request.idempotency_key(), request.idempotency_key());
    assert_eq!(decoded_request.key_id(), request.key_id());
    assert_eq!(
        request.correlation_id().parse_correlation_id()?.as_str(),
        "correlation-1"
    );
    assert_eq!(
        request.idempotency_key().parse_idempotency_key()?.as_str(),
        "idempotency-1"
    );

    let receipt = issue_receipt()?;
    let receipt_frame = encode_account_issuer_receipt(&receipt)?;
    let decoded_receipt = decode_account_issuer_receipt(&receipt_frame)?;

    assert_eq!(decoded_receipt, receipt);
    assert_eq!(
        decoded_receipt.kind(),
        AccountIssuerMessageKind::IssueCurrentAuthority
    );
    assert_eq!(decoded_receipt.lineage().key_generation(), 1);
    assert_eq!(decoded_receipt.lineage().enrollment_generation(), 2);
    assert_eq!(decoded_receipt.lineage().authority_generation(), 3);
    assert_eq!(decoded_receipt.lineage().session_generation(), 4);
    Ok(())
}

#[test]
fn account_issuer_wire_rejects_domain_version_trailing_and_size_drift() -> Result<(), ProtocolError>
{
    let frame = encode_account_issuer_request(&issue_request()?)?;

    let mut wrong_domain = frame.clone();
    wrong_domain[0] ^= 1;
    assert!(matches!(
        decode_account_issuer_request(&wrong_domain),
        Err(ProtocolError::InvalidDomain)
    ));

    let mut wrong_version = frame.clone();
    wrong_version[ACCOUNT_ISSUER_TRANSPORT_DOMAIN.len() + 1] =
        ACCOUNT_ISSUER_PROTOCOL_VERSION.saturating_add(1) as u8;
    assert!(matches!(
        decode_account_issuer_request(&wrong_version),
        Err(ProtocolError::UnsupportedVersion(_))
    ));

    let mut trailing = frame;
    trailing.push(0);
    assert!(matches!(
        decode_account_issuer_request(&trailing),
        Err(ProtocolError::TrailingBytes)
    ));
    assert!(matches!(
        decode_account_issuer_request(&vec![0; ACCOUNT_ISSUER_MAX_WIRE_BYTES + 1]),
        Err(ProtocolError::FrameTooLarge)
    ));
    Ok(())
}

#[test]
fn account_issuer_fields_and_protected_receipts_are_bounded() {
    assert!(matches!(
        AccountIssuerField::from_wire(Vec::new()),
        Err(ProtocolError::EmptyField)
    ));
    assert!(matches!(
        AccountIssuerField::from_wire(vec![b'x'; ACCOUNT_ISSUER_MAX_FIELD_BYTES + 1]),
        Err(ProtocolError::FieldTooLarge)
    ));
    assert!(matches!(
        ProtectedAccountIssuerReceiptWire::try_from(Vec::new()),
        Err(ProtocolError::EmptyField)
    ));
    assert!(matches!(
        ProtectedAccountIssuerReceiptWire::try_from(vec![
            0;
            ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES
                + 1
        ]),
        Err(ProtocolError::FieldTooLarge)
    ));
}
