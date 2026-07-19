use super::*;

#[test]
fn applied_record_hydration_requires_execution_receipt_evidence() -> TestResult {
    let queued = sample_queued_delivery()?;
    let transition = transition(2, "attempt-applied-hydration", PolicyDeliveryState::Applied)?;
    let applied = test_ok!(
        apply_policy_delivery_adapter_execution(&queued, adapter_execution(&queued, &transition)),
        "apply receipt-validated delivery before hydration"
    )
    .into_record();
    let mut payload = test_ok!(
        serde_json::to_value(&applied),
        "serialize receipt-validated applied delivery"
    );
    let hydrated: PolicyDeliveryRecord = test_ok!(
        serde_json::from_value(payload.clone()),
        "hydrate receipt-validated applied delivery"
    );
    assert_eq!(hydrated, applied);
    assert!(hydrated.is_active());

    assert_mismatched_receipt_fails(&payload);

    let removed = test_some!(
        test_some!(payload.as_object_mut(), "applied delivery object").remove("execution_receipt"),
        "execution receipt evidence"
    );
    assert_eq!(removed["state"], "applied");
    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(payload),
        "applied hydration without receipt evidence must fail"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: missing adapter execution receipt for applied"
    );
    Ok(())
}

fn assert_mismatched_receipt_fails(payload: &serde_json::Value) {
    let mut mismatched_payload = payload.clone();
    mismatched_payload["execution_receipt"]["attempt_id"] =
        serde_json::Value::String("attempt-receipt-mismatch".to_string());
    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(mismatched_payload),
        "Applied hydration with mismatched receipt evidence must fail"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.attempt_id: delivery record receipt evidence mismatch: expected=record, reported=execution-receipt"
    );
}
