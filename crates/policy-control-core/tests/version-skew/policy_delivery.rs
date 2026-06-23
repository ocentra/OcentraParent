use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryApplyOutcome,
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliverySequence,
    PolicyDeliveryState, PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version, ParentPolicyActorRole,
    ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

fn sample_policy_source_document(version: u64) -> TestResult<ParentPolicySourceDocument> {
    Ok(ParentPolicySourceDocument {
        schema_version: test_ok!(
            parent_policy_source_schema_version(),
            "policy source schema version"
        ),
        document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-delivery"),
            "policy source document id"
        ),
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "household id"
        ),
        policy_version: test_ok!(PolicyVersion::new(version), "policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceStatus::Confirmed,
        child_profile_ids: vec![test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        )],
        device_ids: vec![test_ok!(
            PolicyDeviceId::parse("device-laptop"),
            "policy device id"
        )],
        rules: vec![ParentPolicyRule {
            rule_id: test_ok!(
                PolicyRuleId::parse("rule-school-night-block"),
                "policy rule id"
            ),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: test_ok!(
                    PolicyTargetReferenceId::parse("category-gaming"),
                    "policy target reference"
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id"
            )),
            priority: 100,
            reason_code: test_ok!(
                PolicyReasonCode::parse("school-night"),
                "policy reason code"
            ),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: test_ok!(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id"
            ),
            timezone_name: test_ok!(
                PolicyTimezoneName::parse("America/Toronto"),
                "policy timezone name"
            ),
            starts_at: "21:00".to_string(),
            ends_at: "07:00".to_string(),
            time_budget: PolicyScheduleTimeBudget {
                budget_window_minutes: 120,
                reset: PolicyScheduleBudgetResetRule {
                    kind: PolicyScheduleBudgetResetKind::Daily,
                    local_time: "00:00".to_string(),
                    day: None,
                },
                carryover: PolicyScheduleBudgetCarryoverRule {
                    mode: PolicyScheduleBudgetCarryoverMode::DiscardUnused,
                    max_minutes: None,
                },
                grace_period_minutes: 5,
                effective_from: "2026-01-01T00:00:00Z".to_string(),
                effective_until: None,
                bonus_expiry_minutes: 30,
                clock_source: PolicyScheduleClockSource::TrustedService,
                offline_recovery: PolicyScheduleOfflineRecovery::RecomputeFromJournal,
            },
        }],
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-policy-confirmed"),
            "policy audit ref"
        )],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    })
}

fn sample_delivery_target() -> TestResult<PolicyDeliveryTarget> {
    Ok(PolicyDeliveryTarget {
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: test_ok!(PolicyDeviceId::parse("device-laptop"), "policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    })
}

fn sample_queued_delivery() -> TestResult<PolicyDeliveryRecord> {
    let source = sample_policy_source_document(7)?;
    let compiled = test_ok!(
        compile_domain_policy_artifact(&source, PolicyConsumerDomain::Tracking),
        "compiled domain policy artifact"
    );

    Ok(test_ok!(
        queue_policy_delivery(
            &compiled,
            sample_delivery_target()?,
            test_ok!(
                PolicyDeliveryId::parse("delivery-version-skew"),
                "policy delivery id"
            ),
            test_ok!(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id"
            ),
            vec![audit_ref("audit-policy-queued")?],
        ),
        "queued policy delivery"
    ))
}

fn audit_ref(value: &str) -> TestResult<PolicyAuditReferenceId> {
    Ok(test_ok!(
        PolicyAuditReferenceId::parse(value),
        "policy audit ref"
    ))
}

fn transition(
    sequence: u64,
    attempt_id: &str,
    state: PolicyDeliveryState,
) -> TestResult<PolicyDeliveryTransition> {
    Ok(PolicyDeliveryTransition {
        attempt_id: test_ok!(
            PolicyDeliveryAttemptId::parse(attempt_id),
            "policy attempt id"
        ),
        sequence: test_ok!(
            PolicyDeliverySequence::new(sequence),
            "policy delivery sequence"
        ),
        state,
        audit_reference_ids: vec![audit_ref(&format!("audit-{attempt_id}-{sequence}"))?],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    })
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
    let stale = apply_policy_delivery_transition(
        &delivered_record,
        transition(2, "attempt-stale", PolicyDeliveryState::Queued)?,
    )
    .map_err(|error| std::io::Error::other(format!("older transition is ignored: {error}")))?;

    match replay {
        PolicyDeliveryApplyOutcome::Duplicate(record) => assert_eq!(record, delivered_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected duplicate replay outcome, got {other:?}"
            ))
            .into())
        }
    }

    match stale {
        PolicyDeliveryApplyOutcome::Stale(record) => assert_eq!(record, delivered_record),
        other => {
            return Err(std::io::Error::other(format!(
                "expected stale replay outcome, got {other:?}"
            ))
            .into())
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
