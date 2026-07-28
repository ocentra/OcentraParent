use ocentra_child_policy_core::policy_control_delivery_handoff::{
    apply_policy_control_delivery_handoff, apply_trusted_adapter_delivery_handoff,
    queue_policy_control_delivery_handoff,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryApplyOutcome, PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryId, PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTarget,
    PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version,
    supersede_parent_policy_source_document, ParentPolicyActorRole, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
};

fn audit_ref(value: PolicyAuditReferenceId) -> PolicyAuditReferenceId {
    value
}

fn reason(value: PolicyReasonCode) -> PolicyReasonCode {
    value
}

fn policy_schedule_time_budget() -> PolicyScheduleTimeBudget {
    PolicyScheduleTimeBudget {
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
    }
}

fn policy_schedule_window() -> PolicyScheduleWindow {
    PolicyScheduleWindow {
        schedule_id: PolicyScheduleId::parse("schedule-school-night")
            .expect_value("policy schedule id"),
        timezone_name: PolicyTimezoneName::parse("America/Toronto")
            .expect_value("policy timezone name"),
        starts_at: "21:00".to_string(),
        ends_at: "07:00".to_string(),
        time_budget: policy_schedule_time_budget(),
    }
}

fn policy_source_rule() -> ParentPolicyRule {
    ParentPolicyRule {
        rule_id: PolicyRuleId::parse("rule-school-night-block").expect_value("policy rule id"),
        target: PolicyRuleTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect_value("policy target reference"),
        },
        action: PolicyRuleAction::Block,
        schedule_id: Some(
            PolicyScheduleId::parse("schedule-school-night").expect_value("policy schedule id"),
        ),
        priority: 100,
        reason_code: PolicyReasonCode::parse("school-night").expect_value("policy reason code"),
        enabled: true,
    }
}

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect_value("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect_value("household id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: PolicyActorId::parse("actor-parent").expect_value("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceStatus::Confirmed,
        child_profile_ids: vec![
            PolicyChildProfileId::parse("child-primary").expect_value("child profile id")
        ],
        device_ids: vec![PolicyDeviceId::parse("device-laptop").expect_value("policy device id")],
        rules: vec![policy_source_rule()],
        schedules: vec![policy_schedule_window()],
        audit_reference_ids: vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-policy-confirmed")
                .expect_value("policy audit ref"),
        )],
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
        child_profile_id: PolicyChildProfileId::parse("child-primary")
            .expect_value("child profile id"),
        device_id: PolicyDeviceId::parse("device-laptop").expect_value("policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    }
}

fn queued_delivery_from_source(
    source: &ParentPolicySourceDocument,
) -> ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord {
    let compiled = compile_domain_policy_artifact(source, PolicyConsumerDomain::Tracking)
        .expect_value("compiled domain policy artifact");

    queue_policy_control_delivery_handoff(
        &compiled,
        sample_delivery_target(),
        PolicyDeliveryId::parse("delivery-policy-household-default")
            .expect_value("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-queued").expect_value("policy attempt id"),
        vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-policy-queued").expect_value("policy audit ref"),
        )],
    )
    .expect_value("queued delivery handoff")
    .delivery
}

fn queued_delivery() -> ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord {
    queued_delivery_from_source(&sample_policy_source_document())
}

fn transition(
    sequence: u64,
    attempt_id: PolicyDeliveryAttemptId,
    state: PolicyDeliveryState,
) -> PolicyDeliveryTransition {
    let attempt_id_text = attempt_id.as_str().to_owned();

    PolicyDeliveryTransition {
        attempt_id,
        sequence: PolicyDeliverySequence::new(sequence).expect_value("policy delivery sequence"),
        state,
        audit_reference_ids: vec![audit_ref(
            PolicyAuditReferenceId::parse(format!("audit-{attempt_id_text}-{sequence}"))
                .expect_value("policy audit ref"),
        )],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}

#[test]
fn delivery_queue_starts_pending_per_child_device_domain() {
    let queued = queued_delivery();

    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert_eq!(queued.last_sequence.value(), 1);
    assert_eq!(
        queued.delivery_id.as_str(),
        "delivery-policy-household-default"
    );
    assert_eq!(queued.target.domain, PolicyConsumerDomain::Tracking);
    assert_eq!(
        queued.source_audit_reference_ids,
        vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-policy-confirmed")
                .expect_value("policy audit ref"),
        )]
    );
    assert!(queued.source_superseded_by_policy_version.is_none());
    assert!(queued.source_rollback_ref.is_none());
}

#[test]
fn delivery_queue_preserves_source_artifact_metadata() {
    let source = supersede_parent_policy_source_document(
        &sample_policy_source_document(),
        PolicyVersion::new(8).expect_value("policy version"),
        audit_ref(
            PolicyAuditReferenceId::parse("audit-policy-superseded")
                .expect_value("policy audit ref"),
        ),
    )
    .expect_value("superseded policy source document");

    let queued = queued_delivery_from_source(&source);

    assert_eq!(
        queued.source_audit_reference_ids,
        source.audit_reference_ids
    );
    assert_eq!(
        queued
            .source_superseded_by_policy_version
            .expect_value("replacement policy version")
            .value(),
        8
    );
    assert!(queued.source_rollback_ref.is_none());
    assert_eq!(
        queued.audit_reference_ids,
        vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-policy-queued").expect_value("policy audit ref"),
        )]
    );
    assert!(queued.superseded_by_policy_version.is_none());
}

#[test]
fn delivery_duplicate_and_stale_transitions_are_noops() {
    let queued = queued_delivery();
    let delivered = apply_policy_control_delivery_handoff(
        &queued,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-delivered").expect_value("policy attempt id"),
            PolicyDeliveryState::Delivered,
        ),
    )
    .expect_value("delivered transition");

    let duplicate = apply_policy_control_delivery_handoff(
        &delivered.delivery,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-delivered").expect_value("policy attempt id"),
            PolicyDeliveryState::Delivered,
        ),
    )
    .expect_value("duplicate transition");
    assert!(matches!(
        duplicate.outcome,
        PolicyDeliveryApplyOutcome::Duplicate(_)
    ));

    let stale = apply_policy_control_delivery_handoff(
        &delivered.delivery,
        transition(
            1,
            PolicyDeliveryAttemptId::parse("attempt-queued").expect_value("policy attempt id"),
            PolicyDeliveryState::Queued,
        ),
    )
    .expect_value("stale transition");
    assert!(matches!(
        stale.outcome,
        PolicyDeliveryApplyOutcome::Stale(_)
    ));
}

#[test]
fn delivery_handoff_surfaces_receipt_required_states_as_manual_required() {
    let queued = queued_delivery();
    let cases = [
        (
            PolicyDeliveryState::Acknowledged,
            "attempt-acknowledged-without-receipt",
        ),
        (
            PolicyDeliveryState::Applied,
            "attempt-applied-without-receipt",
        ),
    ];

    for (state, attempt_id) in cases {
        let report = apply_policy_control_delivery_handoff(
            &queued,
            transition(
                2,
                PolicyDeliveryAttemptId::parse(attempt_id).expect_value("policy attempt id"),
                state,
            ),
        )
        .expect_value("receipt-required child handoff surfaces dependency state");

        assert_eq!(report.delivery.state, PolicyDeliveryState::ManualRequired);
        assert_eq!(
            report
                .delivery
                .reason_code
                .as_ref()
                .map(|value| value.as_str()),
            Some("trusted-adapter-required")
        );
        assert!(!report.delivery.is_active());
    }

    assert_eq!(queued.state, PolicyDeliveryState::Queued);
    assert!(!queued.is_active());
}

#[test]
fn trusted_adapter_handoff_persists_applied_execution_receipt() {
    let queued = queued_delivery();
    let delivered = apply_policy_control_delivery_handoff(
        &queued,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-delivered").expect_value("policy attempt id"),
            PolicyDeliveryState::Delivered,
        ),
    )
    .expect_value("delivered transition");
    let applied_transition = transition(
        3,
        PolicyDeliveryAttemptId::parse("attempt-applied").expect_value("policy attempt id"),
        PolicyDeliveryState::Applied,
    );
    let receipt = PolicyDeliveryExecutionReceipt {
        delivery_id: delivered.delivery.delivery_id.clone(),
        household_id: delivered.delivery.household_id.clone(),
        policy_version: delivered.delivery.policy_version,
        source_document_id: delivered.delivery.source_document_id.clone(),
        target: delivered.delivery.target.clone(),
        attempt_id: applied_transition.attempt_id.clone(),
        sequence: applied_transition.sequence,
        state: applied_transition.state,
        audit_reference_ids: applied_transition.audit_reference_ids.clone(),
        reason_code: None,
        rollback_reference_state: None,
    };

    let applied = apply_trusted_adapter_delivery_handoff(
        &delivered.delivery,
        applied_transition,
        receipt.clone(),
    )
    .expect_value("trusted adapter receipt applies delivery");

    assert_eq!(applied.delivery.state, PolicyDeliveryState::Applied);
    assert_eq!(applied.delivery.execution_receipt(), Some(&receipt));
    assert!(applied.delivery.is_active());
}

#[test]
fn delivery_offline_and_expired_before_delivery_stay_degraded_or_fail_closed() {
    let queued = queued_delivery();
    let mut offline_transition = transition(
        2,
        PolicyDeliveryAttemptId::parse("attempt-offline").expect_value("policy attempt id"),
        PolicyDeliveryState::Offline,
    );
    offline_transition.reason_code = Some(reason(
        PolicyReasonCode::parse("child-offline").expect_value("policy reason code"),
    ));

    let offline = apply_policy_control_delivery_handoff(&queued, offline_transition)
        .expect_value("offline transition");
    assert_eq!(offline.delivery.state, PolicyDeliveryState::Offline);
    assert_eq!(
        offline.delivery.parent_visible_state(),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Degraded
    );

    let mut invalid_queued_reason = transition(
        2,
        PolicyDeliveryAttemptId::parse("attempt-expired").expect_value("policy attempt id"),
        PolicyDeliveryState::Queued,
    );
    invalid_queued_reason.reason_code = Some(reason(
        PolicyReasonCode::parse("expired-before-delivery").expect_value("policy reason code"),
    ));
    let invalid_error = apply_policy_control_delivery_handoff(&queued, invalid_queued_reason)
        .expect_err_value("queued transition with reason must fail");
    assert_eq!(
        invalid_error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "unexpected reason code present for queued".to_string(),
        }
    );
}

#[test]
fn delivery_rollback_requires_reason_and_reference_state() {
    let queued = queued_delivery();
    let error = apply_policy_control_delivery_handoff(
        &queued,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-rolled-back").expect_value("policy attempt id"),
            PolicyDeliveryState::RolledBack,
        ),
    )
    .expect_err_value("rollback without context must fail");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.reason_code",
            value: "missing reason code for rolled-back".to_string(),
        }
    );
}

#[test]
fn delivery_rollback_with_valid_context_still_requires_trusted_execution_receipt() {
    let queued = queued_delivery();
    let delivered = apply_policy_control_delivery_handoff(
        &queued,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-delivered-before-rollback-receipt")
                .expect_value("policy attempt id"),
            PolicyDeliveryState::Delivered,
        ),
    )
    .expect_value("delivery must reach a valid rollback source state");
    let mut rollback = transition(
        3,
        PolicyDeliveryAttemptId::parse("attempt-rollback-without-receipt")
            .expect_value("policy attempt id"),
        PolicyDeliveryState::RolledBack,
    );
    rollback.reason_code = Some(reason(
        PolicyReasonCode::parse("adapter-failed").expect_value("policy reason code"),
    ));
    rollback.rollback_reference_state = Some(PolicyDeliveryState::Delivered);

    let error = apply_policy_control_delivery_handoff(&delivered.delivery, rollback)
        .expect_err_value("rollback without trusted execution receipt must fail closed");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.state",
            value: "missing adapter execution receipt for rolled-back".to_string(),
        }
    );
}

#[test]
fn delivery_supersede_requires_newer_policy_version() {
    let queued = queued_delivery();
    let mut superseded = transition(
        2,
        PolicyDeliveryAttemptId::parse("attempt-superseded").expect_value("policy attempt id"),
        PolicyDeliveryState::Superseded,
    );
    superseded.superseded_by_policy_version =
        Some(PolicyVersion::new(7).expect_value("policy version"));

    let error = apply_policy_control_delivery_handoff(&queued, superseded)
        .expect_err_value("same-version supersede must fail");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_delivery.superseded_by_policy_version",
            value: "replacement policy version 7 must be newer than 7".to_string(),
        }
    );
}

#[test]
fn request_override_and_delivery_audit_refs_are_preserved_without_duplication() {
    let queued = queued_delivery();
    let delivered = apply_policy_control_delivery_handoff(
        &queued,
        transition(
            2,
            PolicyDeliveryAttemptId::parse("attempt-delivered").expect_value("policy attempt id"),
            PolicyDeliveryState::Delivered,
        ),
    )
    .expect_value("delivered transition");

    assert_eq!(
        delivered.delivery.audit_reference_ids,
        vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-attempt-delivered-2")
                .expect_value("policy audit ref"),
        )]
    );
}
