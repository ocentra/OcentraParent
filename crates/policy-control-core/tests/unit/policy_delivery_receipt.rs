#[path = "policy_delivery_helpers.rs"]
mod helpers;

use std::any::TypeId;

use super::TestResult;
use helpers::{
    adapter_execution, assert_unexpected_adapter_execution_receipt, execution_receipt,
    execution_receipt_with_sequence, mutate_provenance_child_profile, mutate_provenance_device,
    mutate_provenance_domain, mutate_provenance_household, mutate_provenance_policy_version,
    reason, sample_queued_delivery, transition,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_adapter_execution, apply_policy_delivery_transition,
    validate_policy_delivery_adapter_execution, validate_policy_delivery_execution_receipt,
    PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt, PolicyDeliveryId,
    PolicyDeliveryParentVisibleState, PolicyDeliveryState,
};
use ocentra_policy_control_core::policy_source::{ParentPolicyDocumentId, PolicyScheduleId};

#[test]
fn policy_delivery_ids_are_distinct_opaque_types_with_delivery_specific_validation() -> TestResult {
    assert_ne!(
        TypeId::of::<PolicyDeliveryId>(),
        TypeId::of::<ParentPolicyDocumentId>()
    );
    assert_ne!(
        TypeId::of::<PolicyDeliveryAttemptId>(),
        TypeId::of::<PolicyScheduleId>()
    );

    let delivery_error = test_err!(
        PolicyDeliveryId::parse(""),
        "empty delivery id must be rejected"
    );
    assert_eq!(
        delivery_error,
        EventingError::EmptyValue {
            field: "policy_delivery.delivery_id",
        }
    );

    let attempt_error = test_err!(
        PolicyDeliveryAttemptId::parse(""),
        "empty attempt id must be rejected"
    );
    assert_eq!(
        attempt_error,
        EventingError::EmptyValue {
            field: "policy_delivery.attempt_id",
        }
    );
    Ok(())
}

#[test]
fn acknowledged_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let acknowledged_transition = transition(
        2,
        "attempt-acknowledged-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let acknowledged_execution = adapter_execution(&queued, &acknowledged_transition);

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &acknowledged_transition, None,),
        "acknowledged delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for acknowledged".to_string(),
        }
    );

    let acknowledged = test_ok!(
        apply_policy_delivery_adapter_execution(&queued, acknowledged_execution),
        "acknowledged delivery with explicit receipt"
    )
    .into_record();

    assert_eq!(acknowledged.state, PolicyDeliveryState::Acknowledged);
    assert_eq!(
        acknowledged.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!acknowledged.is_active());
    Ok(())
}

#[test]
fn execution_receipt_matrix_rejects_unexpected_receipts_for_non_adapter_states() -> TestResult {
    let queued = sample_queued_delivery()?;
    let states = [
        PolicyDeliveryState::Queued,
        PolicyDeliveryState::Delivering,
        PolicyDeliveryState::Delivered,
        PolicyDeliveryState::Rejected,
        PolicyDeliveryState::Superseded,
        PolicyDeliveryState::Degraded,
        PolicyDeliveryState::Offline,
        PolicyDeliveryState::ExpiredBeforeDelivery,
        PolicyDeliveryState::RetryScheduled,
        PolicyDeliveryState::PartialDomainApply,
        PolicyDeliveryState::BlockedByPermission,
        PolicyDeliveryState::BlockedByCapability,
        PolicyDeliveryState::ManualRequired,
    ];

    for state in states {
        assert_unexpected_adapter_execution_receipt(&queued, state)?;
    }

    Ok(())
}

#[test]
fn applied_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let applied_transition =
        transition(3, "attempt-applied-receipt", PolicyDeliveryState::Applied)?;

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &applied_transition, None),
        "applied delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for applied".to_string(),
        }
    );

    Ok(())
}

#[test]
fn rolled_back_delivery_requires_an_explicit_execution_receipt() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut rollback_transition = transition(
        4,
        "attempt-rollback-receipt",
        PolicyDeliveryState::RolledBack,
    )?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);

    let missing_receipt_error = test_err!(
        validate_policy_delivery_execution_receipt(&queued, &rollback_transition, None),
        "rolled-back delivery missing execution receipt must fail"
    );
    assert_eq!(
        missing_receipt_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for rolled-back".to_string(),
        }
    );

    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_equal_current_receipt_for_later_acknowledged_transition(
) -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_transition = transition(
        2,
        "attempt-delivered-receipt",
        PolicyDeliveryState::Delivered,
    )?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition),
        "deliver policy"
    )
    .into_record();

    let acknowledged_transition = transition(
        3,
        "attempt-acknowledged-receipt",
        PolicyDeliveryState::Acknowledged,
    )?;
    let receipt = execution_receipt_with_sequence(&delivered_record, &acknowledged_transition, 2);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "receipt stuck on current sequence must not authorize a later transition"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "expected receipt sequence 3 but receipt reported 2".to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_receipt_sequence_ahead_of_transition() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_transition = transition(
        2,
        "attempt-delivered-receipt",
        PolicyDeliveryState::Delivered,
    )?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition),
        "deliver policy"
    )
    .into_record();

    let applied_transition =
        transition(3, "attempt-applied-receipt", PolicyDeliveryState::Applied)?;
    let receipt = execution_receipt_with_sequence(&delivered_record, &applied_transition, 4);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &applied_transition,
            Some(&receipt),
        ),
        "receipt ahead of the transition must not validate"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "expected receipt sequence 3 but receipt reported 4".to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_same_sequence_nonduplicate_receipts_as_conflicts(
) -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_transition = transition(
        2,
        "attempt-delivered-receipt",
        PolicyDeliveryState::Delivered,
    )?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered_transition),
        "deliver policy"
    )
    .into_record();

    let acknowledged_transition = transition(
        2,
        "attempt-acknowledged-conflict",
        PolicyDeliveryState::Acknowledged,
    )?;
    let receipt = execution_receipt(&delivered_record, &acknowledged_transition);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "nonduplicate same-sequence receipt must be rejected as a conflict"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "conflicting replay for sequence 2 on delivery-policy-household-default"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_rolled_back_sequence_mismatch() -> TestResult {
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
    let receipt = execution_receipt_with_sequence(&applied_record, &rollback_transition, 3);

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &applied_record,
            &rollback_transition,
            Some(&receipt),
        ),
        "rollback receipt sequence must match its transition"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "expected receipt sequence 4 but receipt reported 3".to_string(),
        }
    );
    Ok(())
}

#[test]
fn execution_receipt_validation_rejects_delivery_identity_mismatch() -> TestResult {
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
    receipt.delivery_id = test_ok!(
        PolicyDeliveryId::parse("delivery-policy-mismatch"),
        "mismatched delivery id"
    );

    let error = test_err!(
        validate_policy_delivery_execution_receipt(
            &delivered_record,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "mismatched delivery id must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.delivery_id",
            value: "expected delivery delivery-policy-household-default but receipt reported delivery-policy-mismatch"
                .to_string(),
        }
    );
    Ok(())
}

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

    let provenance_cases: [(&str, String, fn(&mut PolicyDeliveryExecutionReceipt)); 5] = [
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
            "policy_delivery.state",
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
            value: "stale execution receipt for sequence 1 on delivery-policy-household-default"
                .to_string(),
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
            value:
                "duplicate execution receipt for sequence 3 on delivery-policy-household-default"
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
