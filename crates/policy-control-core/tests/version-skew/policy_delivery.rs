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
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

fn sample_policy_source_document(version: u64) -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-delivery")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(version).expect("policy version"),
        source_surface: PolicySourceWriteSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceDocumentStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect("policy device id")],
        rules: vec![ParentPolicyRule {
            rule_id: PolicyRuleId::parse("rule-school-night-block").expect("policy rule id"),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("policy target reference"),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(
                PolicyScheduleId::parse("schedule-school-night").expect("policy schedule id"),
            ),
            priority: 100,
            reason_code: PolicyReasonCode::parse("school-night").expect("policy reason code"),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: PolicyScheduleId::parse("schedule-school-night")
                .expect("policy schedule id"),
            timezone_name: PolicyTimezoneName::parse("America/Toronto")
                .expect("policy timezone name"),
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
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-policy-confirmed").expect("policy audit ref")
        ],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    }
}

fn sample_delivery_target() -> PolicyDeliveryTarget {
    PolicyDeliveryTarget {
        child_profile_id: PolicyChildProfileId::parse("child-primary").expect("child profile id"),
        device_id: PolicyDeviceId::parse("device-laptop").expect("policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    }
}

fn sample_queued_delivery() -> PolicyDeliveryRecord {
    let compiled = compile_domain_policy_artifact(
        &sample_policy_source_document(7),
        PolicyConsumerDomain::Tracking,
    )
    .expect("compiled domain policy artifact");

    queue_policy_delivery(
        &compiled,
        sample_delivery_target(),
        PolicyDeliveryId::parse("delivery-version-skew").expect("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-queued").expect("policy attempt id"),
        vec![audit_ref("audit-policy-queued")],
    )
    .expect("queued policy delivery")
}

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(value).expect("policy audit ref")
}

fn transition(
    sequence: u64,
    attempt_id: &str,
    state: PolicyDeliveryState,
) -> PolicyDeliveryTransition {
    PolicyDeliveryTransition {
        attempt_id: PolicyDeliveryAttemptId::parse(attempt_id).expect("policy attempt id"),
        sequence: PolicyDeliverySequence::new(sequence).expect("policy delivery sequence"),
        state,
        audit_reference_ids: vec![audit_ref(&format!("audit-{attempt_id}-{sequence}"))],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}

#[test]
fn policy_delivery_serde_rejects_zero_schema_version() {
    let error = serde_json::from_str::<PolicyDeliveryRecord>(
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
    )
    .expect_err("policy delivery schema version zero must be rejected");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn delivery_replay_same_sequence_is_duplicate_and_older_sequence_is_stale() {
    let queued = sample_queued_delivery();
    let delivered = transition(3, "attempt-delivered", PolicyDeliveryState::Delivered);
    let delivered_record = apply_policy_delivery_transition(&queued, delivered.clone())
        .expect("deliver policy")
        .into_record();

    let replay = apply_policy_delivery_transition(&delivered_record, delivered)
        .expect("same-sequence replay is idempotent");
    let stale = apply_policy_delivery_transition(
        &delivered_record,
        transition(2, "attempt-stale", PolicyDeliveryState::Queued),
    )
    .expect("older transition is ignored");

    match replay {
        PolicyDeliveryApplyOutcome::Duplicate(record) => assert_eq!(record, delivered_record),
        other => panic!("expected duplicate replay outcome, got {other:?}"),
    }

    match stale {
        PolicyDeliveryApplyOutcome::Stale(record) => assert_eq!(record, delivered_record),
        other => panic!("expected stale replay outcome, got {other:?}"),
    }
}

#[test]
fn conflicting_same_sequence_replay_is_rejected() {
    let queued = sample_queued_delivery();
    let delivered_record = apply_policy_delivery_transition(
        &queued,
        transition(3, "attempt-delivered", PolicyDeliveryState::Delivered),
    )
    .expect("deliver policy")
    .into_record();

    let error = apply_policy_delivery_transition(
        &delivered_record,
        transition(3, "attempt-conflict", PolicyDeliveryState::Acknowledged),
    )
    .expect_err("conflicting replay must be rejected");

    assert!(error
        .to_string()
        .contains("conflicting replay for sequence 3"));
}

#[test]
fn queued_delivery_serialization_preserves_source_metadata_fields() {
    let queued = sample_queued_delivery();

    let payload = serde_json::to_value(&queued).expect("serialize policy delivery record");

    assert_eq!(
        payload["source_audit_reference_ids"][0],
        "audit-policy-confirmed"
    );
    assert!(payload["source_superseded_by_policy_version"].is_null());
    assert!(payload["source_rollback_ref"].is_null());

    let round_trip: PolicyDeliveryRecord =
        serde_json::from_value(payload).expect("deserialize policy delivery record");
    assert_eq!(
        round_trip.source_audit_reference_ids,
        queued.source_audit_reference_ids
    );
    assert_eq!(
        round_trip.source_superseded_by_policy_version,
        queued.source_superseded_by_policy_version
    );
    assert_eq!(round_trip.source_rollback_ref, queued.source_rollback_ref);
}

#[test]
fn policy_delivery_round_trips_explicit_wp04_delivery_states() {
    let queued = sample_queued_delivery();
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
        let (attempt_id, audit_reference_id) = transition_meta.expect("transition metadata");
        let mut transition = transition(2, attempt_id, state);
        transition.audit_reference_ids = vec![audit_ref(audit_reference_id)];
        transition.reason_code = reason_code.map(|value| {
            PolicyReasonCode::parse(value).expect("policy reason code for explicit wp04 state")
        });
        if state == PolicyDeliveryState::Superseded {
            transition.superseded_by_policy_version =
                Some(PolicyVersion::new(8).expect("policy version"));
        }

        let record = apply_policy_delivery_transition(&queued, transition)
            .expect("explicit wp04 delivery state transition")
            .into_record();

        let serialized = serde_json::to_value(&record).expect("serialize policy delivery record");
        assert_eq!(serialized["state"], expected_state);

        let round_trip: PolicyDeliveryRecord =
            serde_json::from_value(serialized).expect("deserialize policy delivery record");
        assert_eq!(round_trip.state, state);
    }
}
