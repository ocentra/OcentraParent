use ocentra_protected_capability_custody_client::account_issuer::{
    AcknowledgeReceiptRequest, AcknowledgeReceiptWire, IssueCurrentAuthorityRequest,
};
use ocentra_protected_capability_custody_client::account_issuer_rpc::AccountIssuerClientError;
#[cfg(not(windows))]
use ocentra_protected_capability_custody_client::account_issuer_rpc::AccountIssuerRpc;
use ocentra_protected_capability_custody_protocol::account_issuer::ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES;
use ocentra_protected_capability_custody_protocol::account_issuer_contract::AccountIssuerField;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

fn field(value: &str) -> Result<AccountIssuerField, ProtocolError> {
    AccountIssuerField::from_wire(value.as_bytes().to_vec())
}

fn subject() -> Result<AccountIdentityProviderSubject, ProtocolError> {
    AccountIdentityProviderSubject::parse("authjs-subject-1").ok_or(ProtocolError::EmptyField)
}

fn issue_request() -> Result<IssueCurrentAuthorityRequest, ProtocolError> {
    Ok(IssueCurrentAuthorityRequest::new(
        field("correlation-1")?,
        field("idempotency-1")?,
        field(&format!("sha256:ecdsa-p256:{}", "0".repeat(64)))?,
        AccountIdentityProvider::Authjs,
        subject()?,
    ))
}

#[test]
fn typed_issue_request_preserves_its_identity_selector() -> Result<(), ProtocolError> {
    assert_eq!(issue_request()?, issue_request()?);
    Ok(())
}

#[test]
fn acknowledge_receipt_wire_rejects_empty_and_over_bound_payloads() {
    assert!(matches!(
        AcknowledgeReceiptWire::try_from(Vec::new()),
        Err(AccountIssuerClientError::Protocol(
            ProtocolError::EmptyField
        ))
    ));
    assert!(matches!(
        AcknowledgeReceiptWire::try_from(vec![0; ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES + 1]),
        Err(AccountIssuerClientError::Protocol(
            ProtocolError::FieldTooLarge
        ))
    ));
}

#[cfg(not(windows))]
#[test]
fn account_issuer_rpc_requires_the_protected_windows_broker() {
    assert!(matches!(
        AccountIssuerRpc::connect(),
        Err(AccountIssuerClientError::ManualRequired)
    ));
}

#[test]
fn typed_acknowledge_request_retains_protected_receipt_boundary() -> Result<(), ProtocolError> {
    let wire = match AcknowledgeReceiptWire::try_from(vec![0x01, 0x02, 0x03]) {
        Ok(value) => value,
        Err(_) => return Err(ProtocolError::EmptyField),
    };
    let request = AcknowledgeReceiptRequest::new(
        field("correlation-1")?,
        field("idempotency-1")?,
        field(&format!("sha256:ecdsa-p256:{}", "0".repeat(64)))?,
        AccountIdentityProvider::Authjs,
        subject()?,
        wire,
    );
    let equivalent_wire = match AcknowledgeReceiptWire::try_from(vec![0x01, 0x02, 0x03]) {
        Ok(value) => value,
        Err(_) => return Err(ProtocolError::EmptyField),
    };
    let equivalent = AcknowledgeReceiptRequest::new(
        field("correlation-1")?,
        field("idempotency-1")?,
        field(&format!("sha256:ecdsa-p256:{}", "0".repeat(64)))?,
        AccountIdentityProvider::Authjs,
        subject()?,
        equivalent_wire,
    );
    assert_eq!(request, equivalent);
    Ok(())
}
