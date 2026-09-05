use ocentra_account_issuer_owner::delivery::{
    AccountIssuerDeliveryError, DeliveryFailure, ProtectedAccountIssuerReceipt,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES;

#[test]
fn delivery_failure_and_receipt_boundaries_remain_fail_closed(
) -> Result<(), AccountIssuerDeliveryError> {
    assert!(matches!(
        DeliveryFailure::from_bytes(&[]),
        Err(AccountIssuerDeliveryError::Rejected)
    ));
    let _accepted_failure = DeliveryFailure::from_bytes(&[0; 1_024])?;
    assert!(matches!(
        DeliveryFailure::from_bytes(&[0; 1_025]),
        Err(AccountIssuerDeliveryError::Rejected)
    ));

    assert!(matches!(
        ProtectedAccountIssuerReceipt::from_wire(Vec::new()),
        Err(AccountIssuerDeliveryError::Rejected)
    ));
    let _accepted_receipt = ProtectedAccountIssuerReceipt::from_wire(vec![0])?;
    let _accepted_max_receipt = ProtectedAccountIssuerReceipt::from_wire(vec![
        0;
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES
    ])?;
    assert!(matches!(
        ProtectedAccountIssuerReceipt::from_wire(vec![
            0;
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES
                + 1
        ]),
        Err(AccountIssuerDeliveryError::Rejected)
    ));
    Ok(())
}
