use super::*;

#[test]
fn schema_v1_receiptless_applied_is_not_legacy_compatible() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut serialized = test_ok!(
        serde_json::to_value(&queued),
        "serialize schema-v1 applied fixture"
    );
    serialized["schema_version"] = serde_json::json!(1);
    serialized["state"] = serde_json::json!("applied");
    serialized["last_sequence"] = serde_json::json!(2);
    serialized["last_attempt_id"] = serde_json::json!("attempt-schema-v1-applied");
    serialized["audit_reference_ids"] = serde_json::json!(["audit-attempt-schema-v1-applied-2"]);

    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(serialized),
        "schema-v1 receiptless applied history is not legacy-compatible"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}

#[test]
fn generic_acknowledged_hydration_rejects_matching_public_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let transition = transition(
        2,
        "attempt-acknowledged-round-trip",
        PolicyDeliveryState::Acknowledged,
    )?;
    let receipt = execution_receipt(&queued, &transition);
    let mut serialized = test_ok!(
        serde_json::to_value(&queued),
        "serialize queued record for acknowledged evidence fixture"
    );
    serialized["state"] = serde_json::json!("acknowledged");
    serialized["last_sequence"] = serde_json::json!(2);
    serialized["last_attempt_id"] = serde_json::json!("attempt-acknowledged-round-trip");
    serialized["audit_reference_ids"] =
        serde_json::json!(["audit-attempt-acknowledged-round-trip-2"]);
    serialized["execution_receipt"] = test_ok!(
        serde_json::to_value(receipt),
        "serialize acknowledged receipt evidence"
    );

    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(serialized),
        "caller-controlled receipt cannot authenticate acknowledged history"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}

#[test]
fn generic_rolled_back_hydration_rejects_matching_public_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-before-rollback",
                PolicyDeliveryState::Delivered
            )?,
        ),
        "deliver before forged rollback"
    )
    .into_record();
    let mut rollback = transition(
        3,
        "attempt-rolled-back-hydration",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback.reason_code = Some(test_ok!(
        PolicyReasonCode::parse("adapter-failed"),
        "rollback reason"
    ));
    rollback.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let receipt = execution_receipt(&delivered, &rollback);
    let mut serialized = test_ok!(
        serde_json::to_value(&delivered),
        "serialize delivered record"
    );
    serialized["state"] = serde_json::json!("rolled-back");
    serialized["last_sequence"] = serde_json::json!(3);
    serialized["last_attempt_id"] = serde_json::json!("attempt-rolled-back-hydration");
    serialized["audit_reference_ids"] =
        serde_json::json!(["audit-attempt-rolled-back-hydration-3"]);
    serialized["reason_code"] = serde_json::json!("adapter-failed");
    serialized["rollback_reference_state"] = serde_json::json!("delivered");
    serialized["execution_receipt"] =
        test_ok!(serde_json::to_value(receipt), "serialize rollback receipt");

    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(serialized),
        "caller-controlled receipt cannot authenticate rolled-back history"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}

#[test]
fn generic_applied_hydration_rejects_fully_matching_forged_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let transition = transition(2, "attempt-applied-hydration", PolicyDeliveryState::Applied)?;
    let receipt = execution_receipt(&queued, &transition);
    let mut forged_payload = test_ok!(
        serde_json::to_value(&queued),
        "serialize queued record as caller-controlled JSON"
    );
    forged_payload["state"] = serde_json::Value::String("applied".to_string());
    forged_payload["last_sequence"] = test_ok!(
        serde_json::to_value(transition.sequence),
        "serialize forged Applied sequence"
    );
    forged_payload["last_attempt_id"] = test_ok!(
        serde_json::to_value(&transition.attempt_id),
        "serialize forged Applied attempt"
    );
    forged_payload["audit_reference_ids"] = test_ok!(
        serde_json::to_value(&transition.audit_reference_ids),
        "serialize forged Applied audit refs"
    );
    forged_payload["execution_receipt"] = test_ok!(
        serde_json::to_value(&receipt),
        "serialize fully matching forged receipt"
    );
    assert_eq!(
        forged_payload["delivery_id"],
        forged_payload["execution_receipt"]["delivery_id"]
    );
    assert_eq!(
        forged_payload["last_attempt_id"],
        forged_payload["execution_receipt"]["attempt_id"]
    );
    assert_eq!(
        forged_payload["last_sequence"],
        forged_payload["execution_receipt"]["sequence"]
    );
    assert_eq!(
        forged_payload["state"],
        forged_payload["execution_receipt"]["state"]
    );
    let caller_receipt: PolicyDeliveryExecutionReceipt = test_ok!(
        serde_json::from_value(forged_payload["execution_receipt"].clone()),
        "public receipt serde remains evidence-only"
    );
    assert_eq!(caller_receipt, receipt);
    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(forged_payload),
        "fully matching caller receipt cannot authenticate Applied hydration"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}
