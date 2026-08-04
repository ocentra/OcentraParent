use super::policy_delivery_helpers as helpers;
use super::TestResult;
use helpers::{
    audit_ref, execution_receipt, reason, sample_queued_delivery, transition, transition_or_context,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, apply_policy_delivery_transition_with_execution_receipt,
    validate_policy_delivery_execution_receipt, PolicyDeliveryApplyOutcome,
    PolicyDeliveryParentVisibleState, PolicyDeliveryState,
};
use ocentra_policy_control_core::policy_source::{PolicyConsumerDomain, PolicyVersion};

#[test]
fn queued_delivery_starts_pending_per_child_device_domain() -> TestResult {
    let queued = sample_queued_delivery()?;

    assert_eq!(queued.target.child_profile_id.as_str(), "child-primary");
    assert_eq!(queued.target.device_id.as_str(), "device-laptop");
    assert_eq!(queued.target.domain, PolicyConsumerDomain::Tracking);
    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert_eq!(
        queued.source_audit_reference_ids,
        vec![audit_ref("audit-policy-confirmed")?]
    );
    assert!(queued.source_superseded_by_policy_version.is_none());
    assert!(queued.source_rollback_ref.is_none());
    assert_eq!(
        queued.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!queued.is_active());
    Ok(())
}

#[test]
fn duplicate_and_older_transitions_are_safe_noops() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered.clone()),
        "deliver policy"
    )
    .into_record();

    let duplicate = test_ok!(
        apply_policy_delivery_transition(&delivered_record, delivered),
        "duplicate delivery is idempotent"
    );
    let stale = transition_or_context(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(1, "attempt-stale", PolicyDeliveryState::Queued)?,
        ),
        "older queued replay is ignored",
    )?;

    assert!(matches!(
        duplicate,
        PolicyDeliveryApplyOutcome::Duplicate(_)
    ));
    assert!(matches!(stale, PolicyDeliveryApplyOutcome::Stale(_)));
    assert_eq!(delivered_record.state, PolicyDeliveryState::Delivered);
    Ok(())
}

#[test]
fn conflicting_same_sequence_replay_is_rejected() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered),
        "deliver policy"
    )
    .into_record();

    let conflict = test_err!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(
                2,
                "attempt-delivering-conflict",
                PolicyDeliveryState::Delivering,
            )?,
        ),
        "same-sequence replay with changed state must be rejected"
    );

    assert_eq!(
        conflict,
        EventingError::InvalidValue {
            field: "policy_delivery.sequence",
            value: "conflicting replay for sequence 2 with mismatched transition provenance"
                .to_string(),
        }
    );
    Ok(())
}

#[test]
fn delivering_state_stays_pending_until_ack_or_apply() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivering = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivering", PolicyDeliveryState::Delivering)?,
        ),
        "delivering transition"
    )
    .into_record();

    assert_eq!(delivering.state, PolicyDeliveryState::Delivering);
    assert_eq!(
        delivering.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!delivering.is_active());
    Ok(())
}

#[test]
fn acknowledged_evidence_cannot_advance_without_trusted_adapter_authority() -> TestResult {
    let queued = sample_queued_delivery()?;
    let acknowledged_transition =
        transition(2, "attempt-acknowledged", PolicyDeliveryState::Acknowledged)?;
    let receipt = execution_receipt(&queued, &acknowledged_transition);
    test_ok!(
        validate_policy_delivery_execution_receipt(
            &queued,
            &acknowledged_transition,
            Some(&receipt),
        ),
        "validate acknowledged receipt evidence"
    );
    let error = test_err!(
        apply_policy_delivery_transition(&queued, acknowledged_transition),
        "receipt evidence alone cannot advance acknowledged"
    );

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for acknowledged".to_string(),
        }
    );
    assert_eq!(
        queued.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    assert!(!queued.is_active());
    Ok(())
}

#[test]
fn offline_delivery_is_degraded_and_requires_reason_code() -> TestResult {
    let queued = sample_queued_delivery()?;
    let mut offline_transition = transition(2, "attempt-offline", PolicyDeliveryState::Offline)?;
    offline_transition.reason_code = Some(reason("network-offline")?);

    let offline = test_ok!(
        apply_policy_delivery_transition(&queued, offline_transition),
        "mark policy delivery offline"
    )
    .into_record();

    assert_eq!(offline.state, PolicyDeliveryState::Offline);
    assert_eq!(
        offline.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(offline.reason_code, Some(reason("network-offline")?));
    assert!(!offline.is_active());
    Ok(())
}

#[test]
fn queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let payload = test_ok!(
        serde_json::to_value(&queued),
        "serialize policy delivery record"
    );
    let debug = format!("{queued:?}");

    assert!(payload.get("child_profile_ids").is_none());
    assert!(payload.get("device_ids").is_none());
    assert!(payload.get("rules").is_none());
    assert!(payload.get("schedules").is_none());
    assert!(payload.get("retention").is_none());
    assert_eq!(
        payload["source_audit_reference_ids"],
        serde_json::json!(["audit-policy-confirmed"])
    );
    assert_eq!(payload["target"]["child_profile_id"], "child-primary");
    assert_eq!(debug.find("rule-school-night-block"), None);
    assert_eq!(debug.find("schedule-school-night"), None);
    assert_eq!(debug.find("school-night"), None);
    Ok(())
}

#[test]
fn applied_receipt_evidence_cannot_advance_without_trusted_adapter_authority() -> TestResult {
    let queued = sample_queued_delivery()?;
    let applied_transition = transition(4, "attempt-applied", PolicyDeliveryState::Applied)?;
    let receipt = execution_receipt(&queued, &applied_transition);
    test_ok!(
        validate_policy_delivery_execution_receipt(&queued, &applied_transition, Some(&receipt)),
        "validate applied receipt evidence"
    );
    let error = test_err!(
        apply_policy_delivery_transition(&queued, applied_transition),
        "receipt evidence alone cannot advance applied"
    );

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for applied".to_string(),
        }
    );
    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert!(!queued.is_active());
    assert_eq!(
        queued.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Pending
    );
    Ok(())
}

#[test]
fn typed_adapter_receipt_advances_applied_delivery_and_rolls_back_idempotently() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver compiled policy before adapter execution"
    )
    .into_record();

    let applied_transition = transition(3, "attempt-applied", PolicyDeliveryState::Applied)?;
    let applied_receipt = execution_receipt(&delivered, &applied_transition);
    let applied = test_ok!(
        apply_policy_delivery_transition_with_execution_receipt(
            &delivered,
            applied_transition.clone(),
            applied_receipt.clone(),
        ),
        "adapter receipt advances applied delivery"
    )
    .into_record();
    assert_eq!(applied.state, PolicyDeliveryState::Applied);
    assert_eq!(applied.execution_receipt(), Some(&applied_receipt));
    assert!(applied.is_active());

    let duplicate_applied = test_ok!(
        apply_policy_delivery_transition_with_execution_receipt(
            &applied,
            applied_transition,
            applied_receipt,
        ),
        "duplicate applied receipt is idempotent"
    );
    assert!(matches!(
        duplicate_applied,
        PolicyDeliveryApplyOutcome::Duplicate(_)
    ));

    let mut rollback_transition =
        transition(4, "attempt-rollback", PolicyDeliveryState::RolledBack)?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Applied);
    let rollback_receipt = execution_receipt(&applied, &rollback_transition);
    let rolled_back = test_ok!(
        apply_policy_delivery_transition_with_execution_receipt(
            &applied,
            rollback_transition,
            rollback_receipt.clone(),
        ),
        "adapter rollback receipt restores prior policy state reference"
    )
    .into_record();
    assert_eq!(rolled_back.state, PolicyDeliveryState::RolledBack);
    assert_eq!(
        rolled_back.rollback_reference_state,
        Some(PolicyDeliveryState::Applied)
    );
    assert_eq!(rolled_back.execution_receipt(), Some(&rollback_receipt));
    assert!(!rolled_back.is_active());
    Ok(())
}

#[test]
fn forged_adapter_receipt_cannot_advance_applied_delivery() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver compiled policy before forged receipt check"
    )
    .into_record();
    let applied_transition = transition(3, "attempt-applied", PolicyDeliveryState::Applied)?;
    let mut forged_receipt = execution_receipt(&delivered, &applied_transition);
    forged_receipt.audit_reference_ids = vec![audit_ref("audit-forged")?];

    let error = test_err!(
        apply_policy_delivery_transition_with_execution_receipt(
            &delivered,
            applied_transition,
            forged_receipt,
        ),
        "forged adapter receipt must be rejected"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.audit_reference_ids",
            value: "expected audit references to match execution receipt".to_string(),
        }
    );
    assert_eq!(delivered.state, PolicyDeliveryState::Delivered);
    assert!(!delivered.is_active());
    Ok(())
}

#[test]
fn late_intermediate_events_are_stale_after_newer_non_execution_progress() -> TestResult {
    let queued = sample_queued_delivery()?;
    let degraded = test_ok!(
        apply_policy_delivery_transition(&queued, {
            let mut transition = transition(4, "attempt-degraded", PolicyDeliveryState::Degraded)?;
            transition.reason_code = Some(reason("adapter-authority-unavailable")?);
            transition
        },),
        "advance to newer degraded state"
    )
    .into_record();
    let stale_delivered = test_ok!(
        apply_policy_delivery_transition(
            &degraded,
            transition(3, "attempt-delivered-late", PolicyDeliveryState::Delivered)?,
        ),
        "late delivered event is ignored"
    );

    assert!(matches!(
        stale_delivered,
        PolicyDeliveryApplyOutcome::Stale(_)
    ));
    assert_eq!(degraded.state, PolicyDeliveryState::Degraded);
    assert!(!degraded.is_active());
    Ok(())
}

#[test]
fn retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let delivering = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivering", PolicyDeliveryState::Delivering)?,
        ),
        "delivering transition"
    )
    .into_record();

    let mut retry_transition = transition(3, "attempt-retry", PolicyDeliveryState::RetryScheduled)?;
    retry_transition.reason_code = Some(reason("adapter-timeout")?);

    let retry = test_ok!(
        apply_policy_delivery_transition(&delivering, retry_transition),
        "retry transition is accepted"
    )
    .into_record();

    let mut partial_transition = transition(
        4,
        "attempt-partial",
        PolicyDeliveryState::PartialDomainApply,
    )?;
    partial_transition.reason_code = Some(reason("domain-subset-applied")?);

    let partial = test_ok!(
        apply_policy_delivery_transition(&retry, partial_transition),
        "partial-domain-apply transition is accepted"
    )
    .into_record();

    let mut expired_transition = transition(
        5,
        "attempt-expired",
        PolicyDeliveryState::ExpiredBeforeDelivery,
    )?;
    expired_transition.reason_code = Some(reason("delivery-window-expired")?);

    let expired = test_ok!(
        apply_policy_delivery_transition(&retry, expired_transition),
        "expired transition is accepted"
    )
    .into_record();

    assert_eq!(
        retry.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        partial.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        expired.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        test_some!(partial.reason_code.as_ref(), "partial reason code").as_str(),
        "domain-subset-applied"
    );
    assert_eq!(
        test_some!(expired.reason_code.as_ref(), "expired reason code").as_str(),
        "delivery-window-expired"
    );
    assert!(!partial.is_active());
    assert!(!expired.is_active());
    Ok(())
}

#[test]
fn blocked_and_manual_required_transitions_require_reason_and_surface_manual_required() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let missing_reason_error = test_err!(
        apply_policy_delivery_transition(
            &queued,
            transition(
                2,
                "attempt-blocked-permission-missing-reason",
                PolicyDeliveryState::BlockedByPermission,
            )?,
        ),
        "blocked-by-permission without reason is invalid"
    );
    assert_eq!(
        missing_reason_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "missing reason code for blocked-by-permission".to_string(),
        }
    );

    let mut blocked_permission = transition(
        2,
        "attempt-blocked-permission",
        PolicyDeliveryState::BlockedByPermission,
    )?;
    blocked_permission.reason_code = Some(reason("device-permission-lost")?);
    let blocked_permission = test_ok!(
        apply_policy_delivery_transition(&queued, blocked_permission),
        "blocked-by-permission transition"
    )
    .into_record();

    let mut blocked_capability = transition(
        3,
        "attempt-blocked-capability",
        PolicyDeliveryState::BlockedByCapability,
    )?;
    blocked_capability.reason_code = Some(reason("adapter-capability-missing")?);
    let blocked_capability = test_ok!(
        apply_policy_delivery_transition(&blocked_permission, blocked_capability),
        "blocked-by-capability transition"
    )
    .into_record();

    let mut manual_required = transition(
        4,
        "attempt-manual-required",
        PolicyDeliveryState::ManualRequired,
    )?;
    manual_required.reason_code = Some(reason("parent-confirmation-required")?);
    let manual_required = test_ok!(
        apply_policy_delivery_transition(&blocked_capability, manual_required),
        "manual-required transition"
    )
    .into_record();

    assert_eq!(
        blocked_permission.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        blocked_capability.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        manual_required.parent_visible_state(),
        PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert!(!manual_required.is_active());
    Ok(())
}

#[test]
fn rejected_and_rolled_back_transitions_require_reason_and_reference_context() -> TestResult {
    let queued = sample_queued_delivery()?;

    let rejected_error = test_err!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-rejected", PolicyDeliveryState::Rejected)?,
        ),
        "rejected transition without reason is invalid"
    );
    assert_eq!(
        rejected_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "missing reason code for rejected".to_string(),
        }
    );

    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver before rollback evidence validation"
    )
    .into_record();

    let mut rollback_transition =
        transition(3, "attempt-rollback", PolicyDeliveryState::RolledBack)?;
    rollback_transition.reason_code = Some(reason("adapter-failed")?);
    rollback_transition.rollback_reference_state = Some(PolicyDeliveryState::Delivered);
    let receipt = execution_receipt(&delivered, &rollback_transition);

    test_ok!(
        validate_policy_delivery_execution_receipt(
            &delivered,
            &rollback_transition,
            Some(&receipt),
        ),
        "validate rollback receipt evidence"
    );
    let rollback_error = test_err!(
        apply_policy_delivery_transition(&delivered, rollback_transition),
        "receipt evidence alone cannot advance rolled-back"
    );
    assert_eq!(
        rollback_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for rolled-back".to_string(),
        }
    );
    Ok(())
}

#[test]
fn superseded_transition_requires_newer_policy_version_and_blocks_regressions() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver before supersede"
    )
    .into_record();

    let mut invalid_superseded = transition(
        3,
        "attempt-superseded-invalid",
        PolicyDeliveryState::Superseded,
    )?;
    invalid_superseded.superseded_by_policy_version =
        Some(test_ok!(PolicyVersion::new(3), "version"));

    let invalid_error = test_err!(
        apply_policy_delivery_transition(&delivered, invalid_superseded),
        "same-version supersede is invalid"
    );
    assert_eq!(
        invalid_error,
        EventingError::InvalidValue {
            field: "policy_delivery.superseded_by_policy_version",
            value: "replacement policy version 3 must be newer than 3".to_string(),
        }
    );

    let mut superseded = transition(4, "attempt-superseded", PolicyDeliveryState::Superseded)?;
    superseded.superseded_by_policy_version = Some(test_ok!(PolicyVersion::new(4), "version"));

    let superseded_record = test_ok!(
        apply_policy_delivery_transition(&delivered, superseded),
        "superseded transition"
    )
    .into_record();

    let regression_error = test_err!(
        apply_policy_delivery_transition(
            &superseded_record,
            transition(5, "attempt-regression", PolicyDeliveryState::Delivered)?,
        ),
        "superseded delivery cannot regress to delivered"
    );

    assert_eq!(superseded_record.state, PolicyDeliveryState::Superseded);
    assert_eq!(
        superseded_record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Superseded
    );
    assert_eq!(
        regression_error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "invalid transition superseded -> delivered".to_string(),
        }
    );
    Ok(())
}

#[test]
fn superseded_before_ack_stays_superseded_and_never_becomes_active() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(2, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "delivered transition"
    )
    .into_record();

    let mut superseded = transition(
        3,
        "attempt-superseded-before-ack",
        PolicyDeliveryState::Superseded,
    )?;
    superseded.superseded_by_policy_version = Some(test_ok!(PolicyVersion::new(4), "version"));

    let superseded_record = test_ok!(
        apply_policy_delivery_transition(&delivered, superseded),
        "superseded before ack transition"
    )
    .into_record();

    assert_eq!(superseded_record.state, PolicyDeliveryState::Superseded);
    assert_eq!(
        superseded_record.parent_visible_state(),
        PolicyDeliveryParentVisibleState::Superseded
    );
    assert_eq!(
        test_some!(
            superseded_record.superseded_by_policy_version,
            "replacement policy version"
        )
        .value(),
        4
    );
    assert!(!superseded_record.is_active());
    Ok(())
}
