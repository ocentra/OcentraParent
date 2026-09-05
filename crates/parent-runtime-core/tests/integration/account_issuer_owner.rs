#![cfg(not(windows))]

use ocentra_parent_runtime_core::account_issuer_owner::{
    acknowledge_receipt, issue_current_authority,
};
use ocentra_protected_capability_custody_client::account_issuer::{
    AcknowledgeReceiptRequest, AcknowledgeReceiptWire, IssueCurrentAuthorityRequest,
};
use ocentra_protected_capability_custody_client::account_issuer_rpc::AccountIssuerClientError;
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

fn key_id() -> Result<AccountIssuerField, ProtocolError> {
    field(&format!("sha256:ecdsa-p256:{}", "0".repeat(64)))
}

#[test]
fn parent_issue_and_acknowledge_paths_fail_closed_without_the_broker_owner(
) -> Result<(), ProtocolError> {
    let issue = IssueCurrentAuthorityRequest::new(
        field("correlation-1")?,
        field("idempotency-1")?,
        key_id()?,
        AccountIdentityProvider::Authjs,
        subject()?,
    );
    assert!(matches!(
        issue_current_authority(issue),
        Err(AccountIssuerClientError::ManualRequired)
    ));

    let protected_receipt = match AcknowledgeReceiptWire::try_from(vec![0x01, 0x02, 0x03]) {
        Ok(value) => value,
        Err(_) => return Err(ProtocolError::EmptyField),
    };
    let acknowledge = AcknowledgeReceiptRequest::new(
        field("correlation-1")?,
        field("idempotency-1")?,
        key_id()?,
        AccountIdentityProvider::Authjs,
        subject()?,
        protected_receipt,
    );
    assert!(matches!(
        acknowledge_receipt(acknowledge),
        Err(AccountIssuerClientError::ManualRequired)
    ));
    Ok(())
}
