use super::policy_delivery_helpers as helpers;
use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, validate_policy_delivery_execution_receipt,
    PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt, PolicyDeliveryId,
    PolicyDeliveryRecord, PolicyDeliveryState, PolicyDeliveryTransition,
};

type ReceiptMutation = fn(&mut PolicyDeliveryExecutionReceipt);

struct RedactionFixture {
    delivered_record: Box<PolicyDeliveryRecord>,
    acknowledged_transition: Box<PolicyDeliveryTransition>,
    base_receipt: Box<PolicyDeliveryExecutionReceipt>,
}

const SENSITIVE_IDENTIFIERS: &[&str] = &[
    "delivery-policy-household-default",
    "delivery-sensitive-reported",
    "household-default",
    "household-mismatch",
    "policy-source-household-default",
    "policy-source-mismatch",
    "child-primary",
    "child-mismatch",
    "device-laptop",
    "device-mismatch",
    "attempt-sensitive-delivered",
    "attempt-sensitive-acknowledged",
    "attempt-sensitive-reported",
    "attempt-sensitive-stale",
    "audit-attempt-sensitive-delivered-2",
    "audit-sensitive-acknowledged",
    "audit-sensitive-reported",
    "audit-sensitive-conflicting",
    "reason-sensitive-transition",
    "reason-sensitive-reported",
];

#[test]
fn formatted_receipt_validation_errors_redact_sensitive_identifiers() -> TestResult {
    let fixture = redaction_fixture()?;
    assert_identity_mismatch_errors_redact(&fixture)?;
    assert_receipt_required_hydration_errors_redact(&fixture)?;
    Ok(())
}

fn redaction_fixture() -> TestResult<RedactionFixture> {
    let queued = helpers::sample_queued_delivery()?;
    let delivered_transition = helpers::transition(
        2,
        "attempt-sensitive-delivered",
        PolicyDeliveryState::Delivered,
    )?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition),
        "deliver policy before receipt redaction matrix"
    )
    .into_record();
    let mut acknowledged_transition = helpers::transition(
        3,
        "attempt-sensitive-acknowledged",
        PolicyDeliveryState::Acknowledged,
    )?;
    acknowledged_transition.audit_reference_ids =
        vec![helpers::audit_ref("audit-sensitive-acknowledged")?];
    let receipt = helpers::execution_receipt(&delivered_record, &acknowledged_transition);

    Ok(RedactionFixture {
        delivered_record: Box::new(delivered_record),
        acknowledged_transition: Box::new(acknowledged_transition),
        base_receipt: Box::new(receipt),
    })
}

fn assert_identity_mismatch_errors_redact(fixture: &RedactionFixture) -> TestResult {
    let mutations: Vec<ReceiptMutation> = vec![
        mutate_delivery_id as ReceiptMutation,
        helpers::mutate_provenance_household,
        helpers::mutate_provenance_source_document,
        helpers::mutate_provenance_child_profile,
        helpers::mutate_provenance_device,
        mutate_attempt_id,
        mutate_audit_reference,
    ];
    for mutate in mutations {
        let mut candidate = Box::new((*fixture.base_receipt).clone());
        mutate(&mut candidate);
        let error = test_err!(
            validate_policy_delivery_execution_receipt(
                &fixture.delivered_record,
                &fixture.acknowledged_transition,
                Some(&candidate),
            ),
            "receipt identity mismatch must fail"
        );
        assert_formatted_error_redacts(&error);
    }
    let mut reason_transition = (*fixture.acknowledged_transition).clone();
    reason_transition.reason_code = Some(helpers::reason("reason-sensitive-transition")?);
    let mut reason_receipt =
        helpers::execution_receipt(&fixture.delivered_record, &reason_transition);
    mutate_reason_code(&mut reason_receipt);
    let reason_error = test_err!(
        validate_policy_delivery_execution_receipt(
            &fixture.delivered_record,
            &reason_transition,
            Some(&reason_receipt),
        ),
        "receipt reason identity mismatch must fail"
    );
    assert_formatted_error_redacts(&reason_error);
    Ok(())
}

fn assert_receipt_required_hydration_errors_redact(fixture: &RedactionFixture) -> TestResult {
    let stale_transition = helpers::transition(
        1,
        "attempt-sensitive-stale",
        PolicyDeliveryState::Acknowledged,
    )?;
    let stale_receipt = helpers::execution_receipt(&fixture.delivered_record, &stale_transition);
    let stale = test_err!(
        validate_policy_delivery_execution_receipt(
            &fixture.delivered_record,
            &stale_transition,
            Some(&stale_receipt),
        ),
        "stale receipt must fail"
    );
    let mut acknowledged_payload = test_ok!(
        serde_json::to_value(&fixture.delivered_record),
        "serialize existing receipt replay fixture"
    );
    acknowledged_payload["state"] = serde_json::json!("acknowledged");
    acknowledged_payload["last_sequence"] = serde_json::json!(3);
    acknowledged_payload["last_attempt_id"] = serde_json::json!("attempt-sensitive-acknowledged");
    acknowledged_payload["audit_reference_ids"] =
        serde_json::json!(["audit-sensitive-acknowledged"]);
    acknowledged_payload["execution_receipt"] = test_ok!(
        serde_json::to_value(&fixture.base_receipt),
        "serialize existing receipt evidence"
    );
    let hydration_error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(acknowledged_payload),
        "generic schema-v2 acknowledged hydration must fail"
    );

    assert_formatted_error_redacts(&stale);
    assert_outputs_redact([format!("{hydration_error:?}"), hydration_error.to_string()]);
    Ok(())
}

fn assert_formatted_error_redacts(error: &EventingError) {
    assert_outputs_redact([format!("{error:?}"), error.to_string()]);
}

fn assert_outputs_redact(outputs: impl IntoIterator<Item = String>) {
    for output in outputs {
        for sensitive_identifier in SENSITIVE_IDENTIFIERS {
            assert!(
                !output.contains(sensitive_identifier),
                "formatted receipt validation error exposed {sensitive_identifier}: {output}"
            );
        }
    }
}

fn mutate_delivery_id(receipt: &mut PolicyDeliveryExecutionReceipt) {
    receipt.delivery_id = test_ok!(
        PolicyDeliveryId::parse("delivery-sensitive-reported"),
        "reported delivery id"
    );
}

fn mutate_attempt_id(receipt: &mut PolicyDeliveryExecutionReceipt) {
    receipt.attempt_id = test_ok!(
        PolicyDeliveryAttemptId::parse("attempt-sensitive-reported"),
        "reported attempt id"
    );
}

fn mutate_audit_reference(receipt: &mut PolicyDeliveryExecutionReceipt) {
    receipt.audit_reference_ids = vec![test_ok!(
        helpers::audit_ref("audit-sensitive-reported"),
        "reported audit reference"
    )];
}

fn mutate_reason_code(receipt: &mut PolicyDeliveryExecutionReceipt) {
    receipt.reason_code = Some(test_ok!(
        helpers::reason("reason-sensitive-reported"),
        "reported reason code"
    ));
}
