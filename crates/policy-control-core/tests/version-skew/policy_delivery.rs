#[path = "policy_delivery_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::{
    assert_explicit_wp04_delivery_state_round_trip, attempt, audit_ref, reason,
    sample_delivery_target, sample_policy_source_document, sample_queued_delivery, transition,
    transition_or_context,
};
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryApplyOutcome,
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliverySequence,
    PolicyDeliveryState, PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version,
    rollback_parent_policy_source_document, supersede_parent_policy_source_document,
    ParentPolicyActorRole, ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument,
    PolicyActorId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata,
    PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};
#[test]
fn policy_delivery_serde_rejects_zero_schema_version() -> TestResult {
    let error = test_err!(
        serde_json::from_str::<PolicyDeliveryRecord>(
            r#"{
            "schema_version": 0,
            "delivery_id": "delivery-version-skew",
            "household_id": "household-default",
            "policy_version": 7,
            "source_document_id": "policy-source-delivery",
            "target": {
                "child_profile_id": "child-primary",
                "device_id": "device-laptop",
                "domain": "tracking"
            },
            "state": "queued",
            "last_sequence": 1,
            "last_attempt_id": "attempt-queued",
            "audit_reference_ids": ["audit-policy-queued"],
            "reason_code": null,
            "superseded_by_policy_version": null,
            "rollback_reference_state": null
        }"#,
        ),
        "policy delivery schema version zero must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn delivery_replay_same_sequence_is_duplicate_and_older_sequence_is_stale() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered = transition(3, "attempt-delivered", PolicyDeliveryState::Delivered)?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(&queued, delivered.clone()),
        "deliver policy"
    )
    .into_record();

    let replay = test_ok!(
        apply_policy_delivery_transition(&delivered_record, delivered),
        "same-sequence replay is idempotent"
    );
    let stale = transition_or_context(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(2, "attempt-stale", PolicyDeliveryState::Queued)?,
        ),
        "older transition is ignored",
    )?;

    match replay {
        PolicyDeliveryApplyOutcome::Duplicate(record) => assert_eq!(record, delivered_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected duplicate replay outcome, got {other:?}"
            ))
            .into());
        }
    }

    match stale {
        PolicyDeliveryApplyOutcome::Stale(record) => assert_eq!(record, delivered_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected stale replay outcome, got {other:?}"
            ))
            .into());
        }
    }
    Ok(())
}

#[test]
fn conflicting_same_sequence_replay_is_rejected() -> TestResult {
    let queued = sample_queued_delivery()?;
    let delivered_record = test_ok!(
        apply_policy_delivery_transition(
            &queued,
            transition(3, "attempt-delivered", PolicyDeliveryState::Delivered)?,
        ),
        "deliver policy"
    )
    .into_record();

    let error = test_err!(
        apply_policy_delivery_transition(
            &delivered_record,
            transition(3, "attempt-conflict", PolicyDeliveryState::Acknowledged)?,
        ),
        "conflicting replay must be rejected"
    );

    assert!(error
        .to_string()
        .contains("conflicting replay for sequence 3"));
    Ok(())
}

#[test]
fn queued_delivery_serialization_preserves_source_metadata_fields() -> TestResult {
    let queued = sample_queued_delivery()?;

    let payload = test_ok!(
        serde_json::to_value(&queued),
        "serialize policy delivery record"
    );

    assert_eq!(
        payload["source_audit_reference_ids"][0],
        "audit-policy-confirmed"
    );
    assert!(payload["source_superseded_by_policy_version"].is_null());
    assert!(payload["source_rollback_ref"].is_null());

    let round_trip: PolicyDeliveryRecord = test_ok!(
        serde_json::from_value(payload),
        "deserialize policy delivery record"
    );
    assert_eq!(
        round_trip.source_audit_reference_ids,
        queued.source_audit_reference_ids
    );
    assert_eq!(
        round_trip.source_superseded_by_policy_version,
        queued.source_superseded_by_policy_version
    );
    assert_eq!(round_trip.source_rollback_ref, queued.source_rollback_ref);
    Ok(())
}

#[test]
fn policy_delivery_round_trips_explicit_wp04_delivery_states() -> TestResult {
    let queued = sample_queued_delivery()?;
    let cases = [
        (
            PolicyDeliveryState::Delivering,
            "delivering",
            Some(("attempt-delivering", "audit-attempt-delivering-2")),
            None,
        ),
        (
            PolicyDeliveryState::Acknowledged,
            "acknowledged",
            Some(("attempt-acknowledged", "audit-attempt-acknowledged-2")),
            None,
        ),
        (
            PolicyDeliveryState::Offline,
            "offline",
            Some(("attempt-offline", "audit-attempt-offline-2")),
            Some("network-offline"),
        ),
        (
            PolicyDeliveryState::Superseded,
            "superseded",
            Some(("attempt-superseded", "audit-attempt-superseded-2")),
            None,
        ),
        (
            PolicyDeliveryState::RetryScheduled,
            "retry-scheduled",
            Some(("attempt-retry", "audit-attempt-retry-2")),
            Some("adapter-timeout"),
        ),
        (
            PolicyDeliveryState::PartialDomainApply,
            "partial-domain-apply",
            Some(("attempt-partial", "audit-attempt-partial-2")),
            Some("domain-subset-applied"),
        ),
        (
            PolicyDeliveryState::BlockedByPermission,
            "blocked-by-permission",
            Some((
                "attempt-blocked-permission",
                "audit-attempt-blocked-permission-2",
            )),
            Some("device-permission-lost"),
        ),
        (
            PolicyDeliveryState::BlockedByCapability,
            "blocked-by-capability",
            Some((
                "attempt-blocked-capability",
                "audit-attempt-blocked-capability-2",
            )),
            Some("adapter-capability-missing"),
        ),
        (
            PolicyDeliveryState::ManualRequired,
            "manual-required",
            Some(("attempt-manual-required", "audit-attempt-manual-required-2")),
            Some("parent-confirmation-required"),
        ),
        (
            PolicyDeliveryState::ExpiredBeforeDelivery,
            "expired-before-delivery",
            Some(("attempt-expired", "audit-attempt-expired-2")),
            Some("delivery-window-expired"),
        ),
    ];

    for (state, expected_state, transition_meta, reason_code) in cases {
        let (attempt_id, audit_reference_id) = test_some!(transition_meta, "transition metadata");
        let mut transition = transition(2, attempt_id, state)?;
        transition.audit_reference_ids = vec![audit_ref(audit_reference_id)?];
        transition.reason_code = test_ok!(
            reason_code.map(PolicyReasonCode::parse).transpose(),
            "policy reason code for explicit wp04 state"
        );
        if state == PolicyDeliveryState::Superseded {
            transition.superseded_by_policy_version =
                Some(test_ok!(PolicyVersion::new(8), "policy version"));
        }

        let record = test_ok!(
            apply_policy_delivery_transition(&queued, transition),
            "explicit wp04 delivery state transition"
        )
        .into_record();

        let serialized = test_ok!(
            serde_json::to_value(&record),
            "serialize policy delivery record"
        );
        assert_eq!(serialized["state"], expected_state);

        let round_trip: PolicyDeliveryRecord = test_ok!(
            serde_json::from_value(serialized),
            "deserialize policy delivery record"
        );
        assert_eq!(round_trip.state, state);
    }
    Ok(())
}
