use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_runtime_core::policy_control_dispatch::{
    ParentPolicyControlAcknowledgementState, ParentRuntimePolicyControlOriginState,
    ParentRuntimePolicyControlPublishState,
};
use ocentra_parent_runtime_core::policy_control_update_flow::publish_parent_policy_control_delivery_event_flow;
use ocentra_policy_control_core::policy_authority::{
    AiResultAuthorityState, EvidenceReferenceState, ParentAuthorityState, PolicyConflictInput,
    PolicyConflictState, PolicyControlAggregateId, PolicyControlInput, PolicyControlRequestId,
    PolicyDecisionMode, PolicyDecisionSource, PolicyEvaluationRequestedEvent,
};
use ocentra_policy_control_core::policy_delivery::{
    queue_policy_delivery, PolicyDeliveryApplyOutcome, PolicyDeliveryAttemptId, PolicyDeliveryId,
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

macro_rules! result_or_unreachable {
    ($result:expr, $context:expr $(,)?) => {
        $result.expect($context)
    };
}

macro_rules! option_or_unreachable {
    ($option:expr, $context:expr $(,)?) => {
        $option.expect($context)
    };
}

macro_rules! audit_ref {
    ($value:expr) => {
        result_or_unreachable!(PolicyAuditReferenceId::parse($value), "policy audit ref")
    };
}

macro_rules! reason {
    ($value:expr) => {
        result_or_unreachable!(PolicyReasonCode::parse($value), "policy reason code")
    };
}

#[test]
fn parent_runtime_policy_control_flow_rejects_receipt_required_child_transitions() {
    let queued_delivery = sample_queued_delivery();
    let evaluation_event = authorized_evaluation_event(PolicyDecisionMode::Enforce);
    let cases = [
        (
            acknowledged_attempt_id(),
            PolicyDeliveryState::Acknowledged,
            "acknowledged",
        ),
        (
            applied_attempt_id(),
            PolicyDeliveryState::Applied,
            "applied",
        ),
    ];

    for (attempt_id, state, state_name) in cases {
        assert_eq!(
            publish_parent_policy_control_delivery_event_flow(
                &queued_delivery,
                &evaluation_event,
                &[transition(3, attempt_id, state)],
                ParentPolicyControlAcknowledgementState::Required,
                ParentRuntimePolicyControlOriginState::TrustedLocalUi,
            ),
            Err(EventingError::InvalidValue {
                field: "policy_delivery.state",
                value: format!("missing adapter execution receipt for {state_name}"),
            })
        );
    }

    assert_eq!(queued_delivery.state, PolicyDeliveryState::Queued);
    assert!(!queued_delivery.is_active());
}

#[test]
fn parent_runtime_policy_control_flow_rejects_when_dispatch_is_blocked() {
    let queued_delivery = sample_queued_delivery();
    let evaluation_event = authorized_evaluation_event(PolicyDecisionMode::Enforce);

    let flow_report = publish_parent_policy_control_delivery_event_flow(
        &queued_delivery,
        &evaluation_event,
        &[],
        ParentPolicyControlAcknowledgementState::Required,
        ParentRuntimePolicyControlOriginState::Untrusted,
    );
    let flow_report = result_or_unreachable!(flow_report, "policy control delivery flow");

    assert_eq!(
        flow_report
            .dispatch_event
            .decision
            .child_runtime_publish_state,
        ParentRuntimePolicyControlPublishState::DoNotPublish
    );
    assert_eq!(flow_report.attempted_transitions.len(), 1);
    assert_eq!(
        flow_report.final_record.state,
        PolicyDeliveryState::Rejected
    );
    assert_eq!(
        flow_report.final_record.parent_visible_state(),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::ManualRequired
    );
    assert_eq!(
        option_or_unreachable!(
            flow_report.final_record.reason_code.as_ref(),
            "policy rejection reason code",
        )
        .as_str(),
        "parent-runtime-dispatch-blocked"
    );
}

#[test]
fn parent_runtime_policy_control_flow_keeps_parent_surface_degraded_when_child_stays_offline() {
    let queued_delivery = sample_queued_delivery();
    let evaluation_event = authorized_evaluation_event(PolicyDecisionMode::Enforce);
    let mut offline_transition = transition(3, offline_attempt_id(), PolicyDeliveryState::Offline);
    offline_transition.reason_code = Some(reason!("child-offline"));

    let flow_report = publish_parent_policy_control_delivery_event_flow(
        &queued_delivery,
        &evaluation_event,
        &[offline_transition],
        ParentPolicyControlAcknowledgementState::Required,
        ParentRuntimePolicyControlOriginState::TrustedLocalUi,
    );
    let flow_report = result_or_unreachable!(flow_report, "policy control delivery flow");

    assert_eq!(flow_report.delivery_outcomes.len(), 2);
    assert_eq!(flow_report.final_record.state, PolicyDeliveryState::Offline);
    assert_eq!(
        flow_report.final_record.parent_visible_state(),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        option_or_unreachable!(
            flow_report.final_record.reason_code.as_ref(),
            "offline reason code",
        )
        .as_str(),
        "child-offline"
    );
    assert!(matches!(
        flow_report.delivery_outcomes[1],
        PolicyDeliveryApplyOutcome::Advanced(_)
    ));
}

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: result_or_unreachable!(
            parent_policy_source_schema_version(),
            "policy source schema version",
        ),
        document_id: result_or_unreachable!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id",
        ),
        household_id: result_or_unreachable!(
            PolicyHouseholdId::parse("household-default"),
            "household id",
        ),
        policy_version: result_or_unreachable!(PolicyVersion::new(3), "policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: result_or_unreachable!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceStatus::Confirmed,
        child_profile_ids: vec![sample_child_profile_id()],
        device_ids: vec![sample_device_id()],
        rules: vec![sample_policy_rule()],
        schedules: vec![sample_schedule_window()],
        audit_reference_ids: vec![audit_ref!("audit-policy-confirmed")],
        superseded_by_policy_version: None,
        rollback_ref: None,
        retention: PolicyRetentionMetadata {
            export_allowed: true,
            delete_allowed: true,
            sync_allowed: false,
        },
    }
}

fn sample_queued_delivery() -> ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord {
    let compiled = result_or_unreachable!(
        compile_domain_policy_artifact(
            &sample_policy_source_document(),
            PolicyConsumerDomain::Tracking,
        ),
        "compiled domain policy artifact",
    );

    result_or_unreachable!(
        queue_policy_delivery(
            &compiled,
            sample_policy_delivery_target(),
            result_or_unreachable!(
                PolicyDeliveryId::parse("delivery-policy-household-default"),
                "policy delivery id",
            ),
            result_or_unreachable!(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id",
            ),
            vec![audit_ref!("audit-policy-queued")],
        ),
        "queued policy delivery",
    )
}

fn authorized_evaluation_event(mode: PolicyDecisionMode) -> PolicyEvaluationRequestedEvent {
    PolicyEvaluationRequestedEvent {
        aggregate_id: result_or_unreachable!(
            PolicyControlAggregateId::parse("policy-control-aggregate:child-primary:tracking"),
            "policy control aggregate id",
        ),
        request_id: result_or_unreachable!(
            PolicyControlRequestId::parse("policy-control-request-1"),
            "policy control request id",
        ),
        input: authorized_policy_control_input(mode),
        conflict_input: authorized_policy_conflict_input(),
    }
}

fn transition(
    sequence: u64,
    attempt_id: PolicyDeliveryAttemptId,
    state: PolicyDeliveryState,
) -> PolicyDeliveryTransition {
    PolicyDeliveryTransition {
        audit_reference_ids: vec![transition_audit_reference(&attempt_id, sequence)],
        attempt_id,
        sequence: result_or_unreachable!(
            ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(sequence),
            "policy delivery sequence",
        ),
        state,
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}

fn sample_child_profile_id() -> PolicyChildProfileId {
    result_or_unreachable!(
        PolicyChildProfileId::parse("child-primary"),
        "child profile id"
    )
}

fn sample_device_id() -> PolicyDeviceId {
    result_or_unreachable!(PolicyDeviceId::parse("device-laptop"), "policy device id")
}

fn sample_schedule_id() -> PolicyScheduleId {
    result_or_unreachable!(
        PolicyScheduleId::parse("schedule-school-night"),
        "policy schedule id",
    )
}

fn sample_policy_rule_target() -> PolicyRuleTarget {
    PolicyRuleTarget {
        kind: PolicyTargetKind::Category,
        reference_id: result_or_unreachable!(
            PolicyTargetReferenceId::parse("category-gaming"),
            "policy target reference",
        ),
    }
}

fn sample_policy_rule() -> ParentPolicyRule {
    ParentPolicyRule {
        rule_id: result_or_unreachable!(
            PolicyRuleId::parse("rule-school-night-block"),
            "policy rule id",
        ),
        target: sample_policy_rule_target(),
        action: PolicyRuleAction::Block,
        schedule_id: Some(sample_schedule_id()),
        priority: 100,
        reason_code: result_or_unreachable!(
            PolicyReasonCode::parse("school-night"),
            "policy reason code",
        ),
        enabled: true,
    }
}

fn sample_schedule_window() -> PolicyScheduleWindow {
    PolicyScheduleWindow {
        schedule_id: sample_schedule_id(),
        timezone_name: result_or_unreachable!(
            PolicyTimezoneName::parse("America/Toronto"),
            "policy timezone name",
        ),
        starts_at: "21:00".to_string(),
        ends_at: "07:00".to_string(),
        time_budget: sample_schedule_time_budget(),
    }
}

fn sample_schedule_time_budget() -> PolicyScheduleTimeBudget {
    PolicyScheduleTimeBudget {
        budget_window_minutes: 120,
        reset: sample_budget_reset_rule(),
        carryover: sample_budget_carryover_rule(),
        grace_period_minutes: 5,
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_until: None,
        bonus_expiry_minutes: 30,
        clock_source: PolicyScheduleClockSource::TrustedService,
        offline_recovery: PolicyScheduleOfflineRecovery::RecomputeFromJournal,
    }
}

fn sample_budget_reset_rule() -> PolicyScheduleBudgetResetRule {
    PolicyScheduleBudgetResetRule {
        kind: PolicyScheduleBudgetResetKind::Daily,
        local_time: "00:00".to_string(),
        day: None,
    }
}

fn sample_budget_carryover_rule() -> PolicyScheduleBudgetCarryoverRule {
    PolicyScheduleBudgetCarryoverRule {
        mode: PolicyScheduleBudgetCarryoverMode::DiscardUnused,
        max_minutes: None,
    }
}

fn sample_policy_delivery_target() -> PolicyDeliveryTarget {
    PolicyDeliveryTarget {
        child_profile_id: sample_child_profile_id(),
        device_id: sample_device_id(),
        domain: PolicyConsumerDomain::Tracking,
    }
}

fn acknowledged_attempt_id() -> PolicyDeliveryAttemptId {
    result_or_unreachable!(
        PolicyDeliveryAttemptId::parse("attempt-acknowledged"),
        "policy attempt id",
    )
}

fn applied_attempt_id() -> PolicyDeliveryAttemptId {
    result_or_unreachable!(
        PolicyDeliveryAttemptId::parse("attempt-applied"),
        "policy attempt id",
    )
}

fn offline_attempt_id() -> PolicyDeliveryAttemptId {
    result_or_unreachable!(
        PolicyDeliveryAttemptId::parse("attempt-offline"),
        "policy attempt id",
    )
}

fn transition_audit_reference(
    attempt_id: &PolicyDeliveryAttemptId,
    sequence: u64,
) -> PolicyAuditReferenceId {
    audit_ref!(&format!("audit-{}-{sequence}", attempt_id.as_str()))
}

fn authorized_policy_control_input(mode: PolicyDecisionMode) -> PolicyControlInput {
    PolicyControlInput {
        mode,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    }
}

fn authorized_policy_conflict_input() -> PolicyConflictInput {
    PolicyConflictInput {
        parent_authority_state: ParentAuthorityState::Authorized,
        conflict_state: PolicyConflictState::NoConflict,
        requested_source: PolicyDecisionSource::ParentPolicy,
        evidence_reference_state: EvidenceReferenceState::Stable,
    }
}
