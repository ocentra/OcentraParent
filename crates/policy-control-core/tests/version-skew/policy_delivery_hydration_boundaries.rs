use super::{policy_delivery::helpers, TestResult};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, PolicyDeliveryParentVisibleState, PolicyDeliveryRecord,
    PolicyDeliveryState,
};
use ocentra_policy_control_core::policy_source::PolicyReasonCode;

#[test]
fn policy_delivery_serde_rejects_future_schema_version_before_state_hydration() -> TestResult {
    let error = test_err!(
        serde_json::from_str::<PolicyDeliveryRecord>(
            r#"{"schema_version":3,"delivery_id":"delivery-future-schema","household_id":"household-default","policy_version":7,"source_document_id":"policy-source-delivery","target":{"child_profile_id":"child-primary","device_id":"device-laptop","domain":"tracking"},"state":"applied","last_sequence":1,"last_attempt_id":"attempt-future-schema","audit_reference_ids":["audit-future-schema"],"reason_code":null,"superseded_by_policy_version":null,"rollback_reference_state":null}"#,
        ),
        "future policy delivery schema version must be rejected before state hydration"
    );
    assert_eq!(error.to_string(), "invalid eventing value for policy_delivery.schema_version: unsupported future schema version 3");
    Ok(())
}

#[test]
fn advancing_hydrated_schema_v1_acknowledged_writes_v2_and_preserves_source_audit_semantics(
) -> TestResult {
    let acknowledged: PolicyDeliveryRecord = test_ok!(
        serde_json::from_str(
            r#"{"schema_version":1,"delivery_id":"delivery-legacy-acknowledged-advance","household_id":"household-default","policy_version":7,"source_document_id":"policy-source-delivery","target":{"child_profile_id":"child-primary","device_id":"device-laptop","domain":"tracking"},"state":"acknowledged","last_sequence":2,"last_attempt_id":"attempt-legacy-acknowledged","audit_reference_ids":["audit-legacy-acknowledged"],"source_audit_reference_ids":["audit-policy-confirmed"],"reason_code":null,"superseded_by_policy_version":null,"rollback_reference_state":null}"#
        ),
        "hydrate schema-v1 receiptless acknowledged record for advance"
    );
    let mut transition = test_ok!(
        helpers::transition(
            3,
            "attempt-manual-required",
            PolicyDeliveryState::ManualRequired
        ),
        "manual-required transition"
    );
    transition.reason_code = Some(test_ok!(
        PolicyReasonCode::parse("legacy-receipt-unverified"),
        "manual-required reason"
    ));
    let advanced = test_ok!(
        apply_policy_delivery_transition(&acknowledged, transition),
        "advance legacy acknowledged record"
    )
    .into_record();
    assert_eq!(advanced.schema_version.value(), 2);
    assert_eq!(advanced.state, PolicyDeliveryState::ManualRequired);
    assert_eq!(
        advanced.source_audit_reference_ids,
        acknowledged.source_audit_reference_ids
    );
    assert_eq!(advanced.source_document_id, acknowledged.source_document_id);
    assert_eq!(
        advanced.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert!(!advanced.is_active());
    Ok(())
}
