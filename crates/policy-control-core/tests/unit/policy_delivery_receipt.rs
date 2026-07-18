#[path = "policy_delivery_helpers.rs"]
mod helpers;

use std::any::TypeId;

use super::TestResult;
use helpers::{reason, sample_queued_delivery, transition};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_adapter_execution, apply_policy_delivery_transition,
    validate_policy_delivery_execution_receipt, PolicyDeliveryAdapterExecution,
    PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt, PolicyDeliveryId,
    PolicyDeliveryParentVisibleState, PolicyDeliverySequence, PolicyDeliveryState,
    PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{ParentPolicyDocumentId, PolicyScheduleId};

fn execution_receipt(
    current: &ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> PolicyDeliveryExecutionReceipt {
    PolicyDeliveryExecutionReceipt {
        delivery_id: current.delivery_id.clone(),
        attempt_id: transition.attempt_id.clone(),
        sequence: transition.sequence,
        state: transition.state,
        audit_reference_ids: transition.audit_reference_ids.clone(),
        rollback_reference_state: transition.rollback_reference_state,
    }
}

fn adapter_execution(
    current: &ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> PolicyDeliveryAdapterExecution {
    PolicyDeliveryAdapterExecution {
        transition: transition.clone(),
        receipt: execution_receipt(current, transition),
    }
}

fn execution_receipt_with_sequence(
    current: &ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    sequence: u64,
) -> PolicyDeliveryExecutionReceipt {
    PolicyDeliveryExecutionReceipt {
        sequence: test_ok!(
            PolicyDeliverySequence::new(sequence),
            "policy delivery receipt sequence"
        ),
        ..execution_receipt(current, transition)
    }
}

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
    let stale_transition = transition(1, "attempt-stale-receipt", PolicyDeliveryState::Queued)?;
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
