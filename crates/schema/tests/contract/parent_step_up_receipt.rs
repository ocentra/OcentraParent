use ocentra_schema::parent_step_up_receipt::{
    ParentStepUpAuthorityReceipt, PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION,
};

#[test]
fn parent_step_up_receipt_uses_stable_camel_case_wire_shape() {
    let receipt = ParentStepUpAuthorityReceipt {
        schema_version: PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION.to_owned(),
        receipt_id: "receipt-1".to_owned(),
        issuer: "account-authority".to_owned(),
        audience: "ocentra-parent".to_owned(),
        key_id: "key-1".to_owned(),
        family_id: "family-1".to_owned(),
        parent_account_id: "parent-1".to_owned(),
        action_device_id: "device-1".to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some("child-1".to_owned()),
        action: "pair-child-device".to_owned(),
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-08-05T23:00:00.000Z".to_owned(),
        expires_at: "2026-08-05T23:05:00.000Z".to_owned(),
        signature: "encoded-signature".to_owned(),
    };

    let encoded = serde_json::to_value(&receipt).unwrap_or(serde_json::Value::Null);
    assert_eq!(
        encoded["schemaVersion"],
        PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION
    );
    assert_eq!(encoded["receiptId"], "receipt-1");
    assert_eq!(encoded["actionDeviceId"], "device-1");
    assert_eq!(encoded["targetChildProfileId"], "child-1");
    assert!(encoded.get("action_device_id").is_none());
}
