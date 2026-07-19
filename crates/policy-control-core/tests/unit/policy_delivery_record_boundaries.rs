use super::policy_delivery_helpers as helpers;
use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, validate_policy_delivery_execution_receipt,
    validate_policy_delivery_record, PolicyDeliveryParentVisibleState, PolicyDeliveryRecord,
    PolicyDeliverySequence, PolicyDeliveryState,
};

const SENSITIVE_IDENTIFIERS: &[&str] = &[
    "delivery-policy-household-default",
    "household-default",
    "policy-source-household-default",
    "child-primary",
    "device-laptop",
    "attempt-queued",
    "attempt-debug-sensitive",
    "audit-policy-queued",
    "audit-policy-confirmed",
    "audit-debug-sensitive",
    "reason-debug-sensitive",
];

#[test]
fn operational_debug_redacts_identifiers_while_wire_contract_preserves_them() -> TestResult {
    let queued = helpers::sample_queued_delivery()?;
    let mut transition = helpers::transition(
        2,
        "attempt-debug-sensitive",
        PolicyDeliveryState::BlockedByPermission,
    )?;
    transition.audit_reference_ids = vec![helpers::audit_ref("audit-debug-sensitive")?];
    transition.reason_code = Some(helpers::reason("reason-debug-sensitive")?);
    let outcome = test_ok!(
        apply_policy_delivery_transition(&queued, transition.clone()),
        "apply diagnostic redaction transition"
    );
    let outputs = vec![
        format!("{:?}", queued.delivery_id),
        format!("{:?}", queued.last_attempt_id),
        format!("{:?}", queued.target),
        format!("{queued:?}"),
        format!("{transition:?}"),
        format!("{outcome:?}"),
    ];

    assert_outputs_redact(outputs, SENSITIVE_IDENTIFIERS);

    let wire = test_ok!(
        serde_json::to_value(&queued),
        "serialize delivery wire contract"
    );
    assert_eq!(wire["delivery_id"], "delivery-policy-household-default");
    assert_eq!(wire["target"]["child_profile_id"], "child-primary");
    assert_eq!(wire["target"]["device_id"], "device-laptop");
    Ok(())
}

#[test]
fn transition_validation_diagnostics_redact_raw_sentinels() -> TestResult {
    let queued = helpers::sample_queued_delivery()?;
    let delivered_transition =
        helpers::transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition),
        "deliver before conflicting replay"
    )
    .into_record();
    let conflict = test_err!(
        apply_policy_delivery_transition(
            &delivered,
            helpers::transition(
                2,
                "attempt-conflict-sensitive",
                PolicyDeliveryState::Delivering,
            )?,
        ),
        "conflicting replay must fail"
    );
    assert_eq!(
        conflict,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "conflicting replay for sequence 2 with mismatched transition provenance"
                .to_string(),
        }
    );

    let mut unexpected_reason =
        helpers::transition(2, "attempt-reason-sensitive", PolicyDeliveryState::Queued)?;
    unexpected_reason.reason_code = Some(helpers::reason("reason-sensitive-unexpected")?);
    let reason_error = test_err!(
        apply_policy_delivery_transition(&queued, unexpected_reason),
        "unexpected reason must fail"
    );
    assert_eq!(
        reason_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "unexpected reason code present for queued".to_string(),
        }
    );

    let mut duplicate_audit =
        helpers::transition(2, "attempt-audit-sensitive", PolicyDeliveryState::Delivered)?;
    let audit = helpers::audit_ref("audit-sensitive-duplicate")?;
    duplicate_audit.audit_reference_ids = vec![audit.clone(), audit];
    let audit_error = test_err!(
        apply_policy_delivery_transition(&queued, duplicate_audit),
        "duplicate audit must fail"
    );
    assert_eq!(
        audit_error,
        EventingError::InvalidValue {
            field: "policy_delivery.audit_reference_ids",
            value: "duplicate audit reference".to_string(),
        }
    );

    let outputs = vec![
        format!("{conflict:?}"),
        conflict.to_string(),
        format!("{reason_error:?}"),
        reason_error.to_string(),
        format!("{audit_error:?}"),
        audit_error.to_string(),
    ];
    let sentinels = [
        "delivery-policy-household-default",
        "attempt-conflict-sensitive",
        "reason-sensitive-unexpected",
        "audit-sensitive-duplicate",
    ];
    assert_outputs_redact(outputs, &sentinels);
    Ok(())
}

#[test]
fn applied_state_without_receipt_evidence_fails_closed() -> TestResult {
    let mut forged = helpers::sample_queued_delivery()?;
    forged.state = PolicyDeliveryState::Applied;
    forged.last_sequence = PolicyDeliverySequence::new(2)?;
    let expected = EventingError::InvalidValue {
        field: "policy_delivery.state",
        value: "missing adapter execution receipt for applied".to_string(),
    };

    assert_eq!(
        test_err!(
            validate_policy_delivery_record(&forged),
            "forged applied record must fail validation"
        ),
        expected
    );
    assert!(!forged.is_active());
    assert_eq!(
        forged.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );

    let payload = test_ok!(
        serde_json::to_value(&forged),
        "serialize forged applied record"
    );
    let hydration_error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(payload),
        "hydrate forged applied record"
    );
    assert_eq!(
        hydration_error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}

#[test]
fn fully_matching_public_receipt_remains_untrusted_for_applied_hydration() -> TestResult {
    let queued = helpers::sample_queued_delivery()?;
    let transition =
        helpers::transition(2, "attempt-receipt-validated", PolicyDeliveryState::Applied)?;
    let receipt = helpers::execution_receipt(&queued, &transition);
    test_ok!(
        validate_policy_delivery_execution_receipt(&queued, &transition, Some(&receipt)),
        "matching receipt remains structurally valid evidence"
    );

    let mut payload = test_ok!(
        serde_json::to_value(&queued),
        "serialize queued record before forged applied hydration"
    );
    payload["state"] = serde_json::json!("applied");
    payload["last_sequence"] = serde_json::json!(2);
    payload["last_attempt_id"] = serde_json::json!("attempt-receipt-validated");
    payload["audit_reference_ids"] = serde_json::json!(["audit-attempt-receipt-validated-2"]);
    payload["execution_receipt"] = test_ok!(
        serde_json::to_value(&receipt),
        "serialize caller-fabricated receipt evidence"
    );
    assert_eq!(
        payload["execution_receipt"]["attempt_id"],
        "attempt-receipt-validated"
    );
    let hydration_error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(payload),
        "public receipt evidence cannot establish Applied authenticity"
    );
    assert_eq!(
        hydration_error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}

fn assert_outputs_redact(outputs: Vec<String>, sensitive_identifiers: &[&str]) {
    for output in outputs {
        for sensitive_identifier in sensitive_identifiers {
            assert_eq!(
                output.find(sensitive_identifier),
                None,
                "formatted delivery surface exposed {sensitive_identifier}: {output}"
            );
        }
    }
}
