use super::*;

#[test]
fn acknowledged_record_with_stored_receipt_round_trips_as_pending_evidence() -> TestResult {
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

    let record: PolicyDeliveryRecord = test_ok!(
        serde_json::from_value(serialized),
        "hydrate acknowledged evidence record"
    );
    assert_eq!(record.state, PolicyDeliveryState::Acknowledged);
    assert_eq!(
        record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
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
        "invalid eventing value for policy_delivery.state: generic applied record hydration is unsupported"
    );
    Ok(())
}
