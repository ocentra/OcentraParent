use ocentra_protected_capability_custody_client::account_issuer::AcknowledgeReceiptWire;
use ocentra_protected_capability_custody_client::account_issuer_rpc::AccountIssuerClientError;
use ocentra_protected_capability_custody_protocol::account_issuer::ACCOUNT_ISSUER_MAX_PROTECTED_RECEIPT_BYTES;
use ocentra_protected_capability_custody_protocol::types::ProtocolError;

#[test]
fn protected_receipt_wire_rejects_empty_and_oversized_inputs() {
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
    assert!(matches!(AcknowledgeReceiptWire::try_from(vec![0]), Ok(_)));
}

#[cfg(not(windows))]
#[test]
fn broker_connection_fails_closed_outside_windows() {
    assert!(matches!(
        ocentra_protected_capability_custody_client::connect(),
        Err(ocentra_protected_capability_custody_client::ClientError::UnsupportedPlatform)
    ));
}
