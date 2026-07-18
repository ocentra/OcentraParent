#[path = "policy_delivery_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::{
    adapter_execution, execution_receipt, mutate_provenance_child_profile,
    mutate_provenance_device, mutate_provenance_domain, mutate_provenance_household,
    mutate_provenance_policy_version, mutate_provenance_reason_code,
    mutate_provenance_source_document, reason, sample_queued_delivery, transition,
    transition_or_context,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_adapter_execution, apply_policy_delivery_transition,
    validate_policy_delivery_adapter_execution, validate_policy_delivery_execution_receipt,
    PolicyDeliveryAdapterExecution, PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryId, PolicyDeliveryParentVisibleState, PolicyDeliveryRecord,
    PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{PolicyReasonCode, PolicyVersion};

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
            value: "expected attempt attempt-acknowledged-receipt but receipt reported attempt-mismatch"
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

    let provenance_cases: [(&str, String, fn(&mut PolicyDeliveryExecutionReceipt)); 6] = [
        (
            "policy_source.document_id",
            "expected source document policy-source-household-default but receipt reported policy-source-mismatch"
                .to_string(),
            mutate_provenance_source_document,
        ),
        (
            "policy_source.household_id",
            "expected household household-default but receipt reported household-mismatch"
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
            "expected child profile child-primary but receipt reported child-mismatch".to_string(),
            mutate_provenance_child_profile,
        ),
        (
            "policy_source.device_id",
            "expected device device-laptop but receipt reported device-mismatch".to_string(),
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
fn adapter_execution_validation_matches_apply_for_forbidden_receipts_and_invalid_audits(
) -> TestResult {
    let queued = sample_queued_delivery()?;
    let cases = [
        {
            let mut transition = transition(
                2,
                "attempt-forbidden-delivered",
                PolicyDeliveryState::Delivered,
            )?;
            transition.audit_reference_ids = vec![test_ok!(
                helpers::audit_ref("audit-forbidden-delivered"),
                "audit ref"
            )];
            ("forbidden receipt state", transition)
        },
        {
            let mut transition =
                transition(2, "attempt-empty-audit", PolicyDeliveryState::Acknowledged)?;
            transition.audit_reference_ids.clear();
            ("empty audit refs", transition)
        },
        {
            let mut transition = transition(
                2,
                "attempt-duplicate-audit",
                PolicyDeliveryState::Acknowledged,
            )?;
            let duplicate_audit = test_ok!(helpers::audit_ref("audit-duplicate"), "audit ref");
            transition.audit_reference_ids = vec![duplicate_audit.clone(), duplicate_audit];
            ("duplicate audit refs", transition)
        },
    ];

    for (label, transition) in cases {
        let execution = adapter_execution(&queued, &transition);
        let validate_error = test_err!(
            validate_policy_delivery_adapter_execution(&queued, &execution),
            label
        );
        let apply_error = test_err!(
            apply_policy_delivery_adapter_execution(&queued, execution.clone()),
            label
        );

        assert_eq!(validate_error, apply_error);
    }

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
            value: format!(
                "stale execution receipt for sequence 1 on {}",
                queued.delivery_id.as_str()
            ),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_duplicate_receipts() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_transition = transition(
        2,
        "attempt-delivered-receipt",
        PolicyDeliveryState::Delivered,
    )?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition.clone()),
        "deliver policy"
    )
    .into_record();
    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let acknowledged_record = test_ok!(
        apply_policy_delivery_adapter_execution(
            &delivered_record,
            adapter_execution(&delivered_record, &acknowledged_transition),
        ),
        "acknowledged delivery with receipt"
    )
    .into_record();
    let acknowledged_receipt = execution_receipt(&delivered_record, &acknowledged_transition);

    let duplicate_error = test_err!(
        validate_policy_delivery_execution_receipt(
            &acknowledged_record,
            &acknowledged_transition,
            Some(&acknowledged_receipt),
        ),
        "duplicate execution receipt must fail"
    );
    assert_eq!(
        duplicate_error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: format!(
                "duplicate execution receipt for sequence 3 on {}",
                acknowledged_record.delivery_id.as_str()
            ),
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
    let applied_record = test_ok!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(3, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "apply policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        4,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);
    let mut receipt = adapter_execution(&applied_record, &rollback_transition).receipt;
    receipt.rollback_reference_state = Some(PolicyDeliveryState::Delivered);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &applied_record,
            &rollback_transition,
            Some(&receipt),
        ),
        "rollback reference mismatch must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.rollback_reference_state",
            value: "expected rollback reference state applied but receipt reported delivered"
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
    let applied_record = test_ok!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(3, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "apply policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        4,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);
    let rollback_execution = adapter_execution(&applied_record, &rollback_transition);
    let missing_reference_error = test_err!(
        validate_policy_delivery_execution_receipt(
            &applied_record,
            &rollback_transition,
            Some(&PolicyDeliveryExecutionReceipt {
                rollback_reference_state: None,
                ..rollback_execution.receipt.clone()
            }),
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
fn rolled_back_execution_receipt_applies_successfully() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();
    let applied_record = test_ok!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(3, "attempt-applied", PolicyDeliveryState::Applied)?,
        ),
        "apply policy"
    )
    .into_record();
    let mut rollback_transition = transition(
        4,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);
    let rollback_execution = adapter_execution(&applied_record, &rollback_transition);

    let rolled_back = test_ok!(
        apply_policy_delivery_adapter_execution(&applied_record, rollback_execution),
        "rolled back delivery with receipt"
    )
    .into_record();

    assert_eq!(rolled_back.state, PolicyDeliveryState::RolledBack);
    assert_eq!(
        rolled_back.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert!(!rolled_back.is_active());
    Ok(())
}
