#[path = "policy_delivery_helpers.rs"]
mod helpers;

use super::TestResult;
use helpers::{
    audit_ref, sample_delivery_id, sample_queued_delivery, transition, transition_or_context,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_adapter_execution, apply_policy_delivery_transition,
    validate_policy_delivery_execution_receipt, PolicyDeliveryAdapterExecution,
    PolicyDeliveryApplyOutcome, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord,
    PolicyDeliveryState, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{PolicyReasonCode, PolicyVersion};

fn execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> PolicyDeliveryExecutionReceipt {
    PolicyDeliveryExecutionReceipt {
        delivery_id: current.delivery_id.clone(),
        household_id: current.household_id.clone(),
        policy_version: current.policy_version,
        source_document_id: current.source_document_id.clone(),
        target: current.target.clone(),
        attempt_id: transition.attempt_id.clone(),
        sequence: transition.sequence,
        state: transition.state,
        audit_reference_ids: transition.audit_reference_ids.clone(),
        reason_code: transition.reason_code.clone(),
        rollback_reference_state: transition.rollback_reference_state,
    }
}

fn adapter_execution(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> PolicyDeliveryAdapterExecution {
    PolicyDeliveryAdapterExecution {
        transition: transition.clone(),
        receipt: execution_receipt(current, transition),
    }
}

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
fn policy_delivery_execution_receipt_rejects_missing_field_in_version_skew_json() -> TestResult {
    let error = test_err!(
        serde_json::from_str::<PolicyDeliveryExecutionReceipt>(
            r#"{
            "attempt_id": "attempt-acknowledged",
            "sequence": 2,
            "state": "acknowledged",
            "audit_reference_ids": ["audit-attempt-acknowledged-2"],
            "reason_code": null,
            "rollback_reference_state": null
        }"#,
        ),
        "missing receipt field must be rejected"
    );

    assert!(error.to_string().contains("missing field `delivery_id`"));
    Ok(())
}

#[test]
fn policy_delivery_execution_receipt_rejects_missing_provenance_in_version_skew_json() -> TestResult
{
    let queued = sample_queued_delivery()?;
    let transition = transition(
        2,
        "attempt-acknowledged-provenance",
        PolicyDeliveryState::Acknowledged,
    )?;
    let receipt = execution_receipt(&queued, &transition);

    let mut serialized = test_ok!(
        serde_json::to_value(&receipt),
        "serialize execution receipt"
    );
    let serialized_object = serialized
        .as_object_mut()
        .ok_or_else(|| std::io::Error::other("serialized execution receipt must be an object"))?;
    serialized_object.remove("source_document_id");

    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryExecutionReceipt>(serialized),
        "missing provenance field must be rejected"
    );

    assert!(error
        .to_string()
        .contains("missing field `source_document_id`"));
    Ok(())
}

#[test]
fn policy_delivery_adapter_execution_rejects_missing_receipt_in_version_skew_json() -> TestResult {
    let queued = sample_queued_delivery()?;
    let transition = transition(
        2,
        "attempt-acknowledged-version-skew",
        PolicyDeliveryState::Acknowledged,
    )?;
    let execution = adapter_execution(&queued, &transition);

    let mut serialized = test_ok!(
        serde_json::to_value(&execution),
        "serialize adapter execution"
    );
    let serialized_object = serialized
        .as_object_mut()
        .ok_or_else(|| std::io::Error::other("serialized adapter execution must be an object"))?;
    serialized_object.remove("receipt");

    let error = test_err!(
        serde_json::from_value::<PolicyDeliveryAdapterExecution>(serialized),
        "missing adapter receipt must be rejected"
    );

    assert!(error.to_string().contains("missing field `receipt`"));
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
fn execution_receipt_round_trip_and_validation_cover_legacy_payloads() -> TestResult {
    let queued = sample_queued_delivery()?;
    let transition = transition(
        2,
        "attempt-acknowledged-version-skew",
        PolicyDeliveryState::Acknowledged,
    )?;
    let receipt = execution_receipt(&queued, &transition);
    let execution = adapter_execution(&queued, &transition);

    let encoded = test_ok!(
        serde_json::to_value(&receipt),
        "serialize execution receipt"
    );
    let decoded: PolicyDeliveryExecutionReceipt = test_ok!(
        serde_json::from_value(encoded),
        "deserialize execution receipt"
    );

    assert_eq!(decoded, receipt);
    assert_eq!(decoded.source_document_id, queued.source_document_id);
    assert_eq!(decoded.reason_code, transition.reason_code);

    let execution_round_trip: PolicyDeliveryAdapterExecution = test_ok!(
        serde_json::from_value(test_ok!(
            serde_json::to_value(&execution),
            "serialize adapter execution"
        )),
        "deserialize adapter execution"
    );
    assert_eq!(execution_round_trip, execution);

    let acknowledged_record = test_ok!(
        apply_policy_delivery_adapter_execution(&queued, execution),
        "acknowledge with explicit receipt"
    )
    .into_record();

    let duplicate_error = test_err!(
        validate_policy_delivery_execution_receipt(
            &acknowledged_record,
            &transition,
            Some(&receipt),
        ),
        "duplicate receipt must fail after replay"
    );
    assert!(duplicate_error
        .to_string()
        .contains("duplicate execution receipt for sequence 2"));
    Ok(())
}

#[test]
fn queued_delivery_preserves_caller_provided_delivery_id() -> TestResult {
    let queued = sample_queued_delivery()?;

    assert_eq!(queued.delivery_id.as_str(), "delivery-version-skew");
    assert_ne!(queued.delivery_id, sample_delivery_id()?);
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
            transition(3, "attempt-conflict", PolicyDeliveryState::Delivering)?,
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

        let outcome = if state == PolicyDeliveryState::Acknowledged {
            apply_policy_delivery_adapter_execution(
                &queued,
                adapter_execution(&queued, &transition),
            )
        } else {
            apply_policy_delivery_transition(&queued, transition)
        };
        let record = test_ok!(outcome, "explicit wp04 delivery state transition").into_record();

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
