use super::policy_delivery_helpers as helpers;
use super::TestResult;
use helpers::{
    execution_receipt, mutate_provenance_child_profile, mutate_provenance_device,
    mutate_provenance_domain, mutate_provenance_household, mutate_provenance_policy_version,
    mutate_provenance_source_document, reason, sample_queued_delivery, transition,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, validate_policy_delivery_execution_receipt,
    PolicyDeliveryAttemptId, PolicyDeliveryRecord, PolicyDeliveryState,
};

#[test]
fn execution_receipt_validation_rejects_attempt_identity_mismatch() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-receipt",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let mut receipt = execution_receipt(&delivered_record, &acknowledged_transition);
    receipt.attempt_id = test_ok!(
        PolicyDeliveryAttemptId::parse("attempt-mismatch"),
        "mismatched attempt id"
    );

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "mismatched attempt id must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.attempt_id",
            value: "execution receipt identity mismatch: expected=transition, reported=execution-receipt"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_provenance_mismatches() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-provenance",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-provenance",
        PolicyDeliveryState::Acknowledged,
    )?;

    let provenance_cases: [helpers::ProvenanceCase; 6] = [
        (
            "policy_source.document_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_source_document,
        ),
        (
            "policy_source.household_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_household,
        ),
        (
            "policy_source.policy_version",
            format!(
                "expected policy version {} but receipt reported 8",
                delivered_record.policy_version.value()
            ),
            mutate_provenance_policy_version,
        ),
        (
            "policy_source.child_profile_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_child_profile,
        ),
        (
            "policy_source.device_id",
            "execution receipt identity mismatch: expected=current-record, reported=execution-receipt"
                .to_string(),
            mutate_provenance_device,
        ),
        (
            "policy_delivery.target.domain",
            "expected delivery domain tracking but receipt reported browser".to_string(),
            mutate_provenance_domain,
        ),
    ];

    for (field, value, mutate) in provenance_cases {
        let mut receipt = execution_receipt(&delivered_record, &acknowledged_transition);
        mutate(&mut receipt);

        let error = test_err!(
            validate_policy_delivery_execution_receipt(
                &delivered_record,
                &acknowledged_transition,
                Some(&receipt),
            ),
            "mismatched provenance must fail"
        );
        assert_eq!(error, EventingError::InvalidValue { field, value });
    }

    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_forbidden_receipts_and_invalid_audits() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut forbidden = transition(
        2,
        "attempt-forbidden-delivered",
        PolicyDeliveryState::Delivered,
    )?;
    forbidden.audit_reference_ids = vec![helpers::audit_ref("audit-forbidden-delivered")?];
    let forbidden_receipt = execution_receipt(&queued, &forbidden);
    let forbidden_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &forbidden, Some(&forbidden_receipt)),
        "forbidden receipt state"
    );
    assert!(matches!(
        forbidden_error,
        EventingError::InvalidValue { .. }
    ));

    let mut empty_audit = transition(2, "attempt-empty-audit", PolicyDeliveryState::Acknowledged)?;
    empty_audit.audit_reference_ids.clear();
    let empty_error = test_err!(
        apply_policy_delivery_transition(&queued, empty_audit),
        "empty audit refs"
    );
    assert!(matches!(empty_error, EventingError::InvalidValue { .. }));

    let mut duplicate_audit = transition(
        2,
        "attempt-duplicate-audit",
        PolicyDeliveryState::Acknowledged,
    )?;
    let duplicate = helpers::audit_ref("audit-duplicate")?;
    duplicate_audit.audit_reference_ids = vec![duplicate.clone(), duplicate];
    let duplicate_error = test_err!(
        apply_policy_delivery_transition(&queued, duplicate_audit),
        "duplicate audit refs"
    );
    assert!(matches!(
        duplicate_error,
        EventingError::InvalidValue { .. }
    ));

    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_stale_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-receipt",
                PolicyDeliveryState::Delivered,
            )?,
        ),
        "deliver policy"
    )
    .into_record();
    let stale_transition = transition(
        1,
        "attempt-stale-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let stale_receipt = execution_receipt(&delivered_record, &stale_transition);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &stale_transition,
            Some(&stale_receipt),
        ),
        "stale execution receipt must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "execution receipt sequence mismatch: expected=greater-than-current(2), reported=1 (stale)"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn non_rollback_receipt_rejects_rollback_reference_state() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-reference",
        PolicyDeliveryState::Acknowledged,
    )?;
    let mut receipt = execution_receipt(&delivered_record, &acknowledged_transition);
    receipt.rollback_reference_state = Some(PolicyDeliveryState::Delivered);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "non-rollback receipt must reject rollback reference"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.rollback_reference_state",
            value: "unexpected rollback reference state delivered in execution receipt".to_string(),
        }
    );
    Ok(())
}

#[test]
fn rolled_back_execution_receipt_rejects_prior_state_mismatch() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut rollback_transition = transition(
        2,
        "attempt-rollback-prior-state",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);
    let mut receipt = execution_receipt(&queued, &rollback_transition);
    receipt.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &rollback_transition, Some(&receipt),),
        "rollback prior-state mismatch must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.rollback_reference_state",
            value: "expected rollback reference state queued but transition reported applied"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn rolled_back_execution_receipt_rejects_wrong_reference_state() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        3,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let mut receipt = execution_receipt(&delivered_record, &rollback_transition);
    receipt.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &rollback_transition,
            Some(&receipt),
        ),
        "rollback reference mismatch must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.rollback_reference_state",
            value: "expected rollback reference state delivered but receipt reported applied"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn rolled_back_execution_receipt_requires_rollback_reference_state() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        3,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let mut missing_reference_receipt = execution_receipt(&delivered_record, &rollback_transition);
    missing_reference_receipt.rollback_reference_state = None;
    let missing_reference_error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &rollback_transition,
            Some(&missing_reference_receipt),
        ),
        "rollback execution receipt missing its reference state must fail"
    );
    assert_eq!(
        missing_reference_error,
        EventingError::InvalidValue {
            field: "policy_delivery.rollback_reference_state",
            value: "missing rollback reference state for rolled-back receipt".to_string(),
        }
    );
    Ok(())
}

#[test]
fn rolled_back_execution_receipt_validates_without_minting_adapter_authority() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        3,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let receipt = execution_receipt(&delivered_record, &rollback_transition);

    test_ok!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &rollback_transition,
            Some(&receipt),
        ),
        "validate rolled-back receipt evidence"
    );
    let error = test_err!(
        apply_policy_delivery_transition(&delivered_record, rollback_transition),
        "receipt evidence alone cannot advance rolled-back"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for rolled-back".to_string(),
        }
    );
    Ok(())
}

#[test]
fn generic_stored_rolled_back_receipt_is_rejected_as_unauthenticated() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-delivered-replay",
                PolicyDeliveryState::Delivered
            )?
        ),
        "deliver policy"
    )
    .into_record();
    let mut rollback = transition(
        3,
        "attempt-rollback-replay",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback.reason_code = Some(reason("adapter-failed")?);
    rollback.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let receipt = execution_receipt(&delivered, &rollback);
    let mut payload = test_ok!(
        serde_json::to_value(&delivered),
        "serialize rollback fixture"
    );
    payload["state"] = serde_json::json!("rolled-back");
    payload["last_sequence"] = serde_json::json!(3);
    payload["last_attempt_id"] = serde_json::json!("attempt-rollback-replay");
    payload["audit_reference_ids"] = serde_json::json!(["audit-attempt-rollback-replay-3"]);
    payload["reason_code"] = serde_json::json!("adapter-failed");
    payload["rollback_reference_state"] = serde_json::json!("delivered");
    payload["execution_receipt"] =
        test_ok!(serde_json::to_value(&receipt), "serialize rollback receipt");
    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryRecord>(payload),
        "generic hydration cannot authenticate stored rollback receipt"
    );
    assert_eq!(
        error.to_string(),
        "invalid eventing value for policy_delivery.state: generic receipt-required record hydration is unsupported"
    );
    Ok(())
}
