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

#[test]
fn parent_runtime_policy_control_flow_applies_delivered_acknowledged_and_applied_chain() {
    let queued_delivery = sample_queued_delivery();
    let evaluation_event = authorized_evaluation_event(PolicyDecisionMode::Enforce);
    let transitions = vec![
        transition(3, "attempt-acknowledged", PolicyDeliveryState::Acknowledged),
        transition(4, "attempt-applied", PolicyDeliveryState::Applied),
    ];

    let flow_report = publish_parent_policy_control_delivery_event_flow(
        &queued_delivery,
        &evaluation_event,
        &transitions,
        ParentPolicyControlAcknowledgementState::Required,
        ParentRuntimePolicyControlOriginState::TrustedLocalUi,
    );
    let flow_report = result_or_unreachable(flow_report, "policy control delivery flow");

    assert_eq!(
        flow_report
            .dispatch_event
            .decision
            .child_runtime_publish_state,
        ParentRuntimePolicyControlPublishState::Publish
    );
    assert_eq!(flow_report.attempted_transitions.len(), 3);
    assert_eq!(flow_report.delivery_outcomes.len(), 3);
    assert_eq!(flow_report.final_record.state, PolicyDeliveryState::Applied);
    assert!(flow_report.final_record.is_active());
    assert_eq!(
        flow_report.final_record.parent_visible_state(),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Applied
    );
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
    let flow_report = result_or_unreachable(flow_report, "policy control delivery flow");

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
        option_or_unreachable(
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
    let mut offline_transition = transition(3, "attempt-offline", PolicyDeliveryState::Offline);
    offline_transition.reason_code = Some(reason("child-offline"));

    let flow_report = publish_parent_policy_control_delivery_event_flow(
        &queued_delivery,
        &evaluation_event,
        &[offline_transition],
        ParentPolicyControlAcknowledgementState::Required,
        ParentRuntimePolicyControlOriginState::TrustedLocalUi,
    );
    let flow_report = result_or_unreachable(flow_report, "policy control delivery flow");

    assert_eq!(flow_report.delivery_outcomes.len(), 2);
    assert_eq!(flow_report.final_record.state, PolicyDeliveryState::Offline);
    assert_eq!(
        flow_report.final_record.parent_visible_state(),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        option_or_unreachable(
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
        schema_version: result_or_unreachable(
            parent_policy_source_schema_version(),
            "policy source schema version",
        ),
        document_id: result_or_unreachable(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id",
        ),
        household_id: result_or_unreachable(
            PolicyHouseholdId::parse("household-default"),
            "household id",
        ),
        policy_version: result_or_unreachable(PolicyVersion::new(3), "policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: result_or_unreachable(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        status: PolicySourceStatus::Confirmed,
        child_profile_ids: vec![result_or_unreachable(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id",
        )],
        device_ids: vec![result_or_unreachable(
            PolicyDeviceId::parse("device-laptop"),
            "policy device id",
        )],
        rules: vec![ParentPolicyRule {
            rule_id: result_or_unreachable(
                PolicyRuleId::parse("rule-school-night-block"),
                "policy rule id",
            ),
            target: PolicyRuleTarget {
                kind: PolicyTargetKind::Category,
                reference_id: result_or_unreachable(
                    PolicyTargetReferenceId::parse("category-gaming"),
                    "policy target reference",
                ),
            },
            action: PolicyRuleAction::Block,
            schedule_id: Some(result_or_unreachable(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id",
            )),
            priority: 100,
            reason_code: result_or_unreachable(
                PolicyReasonCode::parse("school-night"),
                "policy reason code",
            ),
            enabled: true,
        }],
        schedules: vec![PolicyScheduleWindow {
            schedule_id: result_or_unreachable(
                PolicyScheduleId::parse("schedule-school-night"),
                "policy schedule id",
            ),
            timezone_name: result_or_unreachable(
                PolicyTimezoneName::parse("America/Toronto"),
                "policy timezone name",
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
        audit_reference_ids: vec![audit_ref("audit-policy-confirmed")],
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
    let compiled = result_or_unreachable(
        compile_domain_policy_artifact(
            &sample_policy_source_document(),
            PolicyConsumerDomain::Tracking,
        ),
        "compiled domain policy artifact",
    );

    result_or_unreachable(
        queue_policy_delivery(
            &compiled,
            PolicyDeliveryTarget {
                child_profile_id: result_or_unreachable(
                    PolicyChildProfileId::parse("child-primary"),
                    "child profile id",
                ),
                device_id: result_or_unreachable(
                    PolicyDeviceId::parse("device-laptop"),
                    "policy device id",
                ),
                domain: PolicyConsumerDomain::Tracking,
            },
            result_or_unreachable(
                PolicyDeliveryId::parse("delivery-policy-household-default"),
                "policy delivery id",
            ),
            result_or_unreachable(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id",
            ),
            vec![audit_ref("audit-policy-queued")],
        ),
        "queued policy delivery",
    )
}

fn authorized_evaluation_event(mode: PolicyDecisionMode) -> PolicyEvaluationRequestedEvent {
    PolicyEvaluationRequestedEvent {
        aggregate_id: result_or_unreachable(
            PolicyControlAggregateId::parse("policy-control-aggregate:child-primary:tracking"),
            "policy control aggregate id",
        ),
        request_id: result_or_unreachable(
            PolicyControlRequestId::parse("policy-control-request-1"),
            "policy control request id",
        ),
        input: PolicyControlInput {
            mode,
            parent_authority_state: ParentAuthorityState::Authorized,
            evidence_reference_state: EvidenceReferenceState::Stable,
            ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
        },
        conflict_input: PolicyConflictInput {
            parent_authority_state: ParentAuthorityState::Authorized,
            conflict_state: PolicyConflictState::NoConflict,
            requested_source: PolicyDecisionSource::ParentPolicy,
            evidence_reference_state: EvidenceReferenceState::Stable,
        },
    }
}

fn transition(
    sequence: u64,
    attempt_id: &str,
    state: PolicyDeliveryState,
) -> PolicyDeliveryTransition {
    PolicyDeliveryTransition {
        attempt_id: result_or_unreachable(
            PolicyDeliveryAttemptId::parse(attempt_id),
            "policy attempt id",
        ),
        sequence: result_or_unreachable(
            ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(sequence),
            "policy delivery sequence",
        ),
        state,
        audit_reference_ids: vec![audit_ref(&format!("audit-{attempt_id}-{sequence}"))],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    result_or_unreachable(PolicyAuditReferenceId::parse(value), "policy audit ref")
}

fn reason(value: &str) -> PolicyReasonCode {
    result_or_unreachable(PolicyReasonCode::parse(value), "policy reason code")
}

fn result_or_unreachable<T, E>(result: Result<T, E>, context: &'static str) -> T
where
    E: std::fmt::Debug,
{
    match result {
        Ok(value) => value,
        Err(error) => unreachable!("{context}: {error:?}"),
    }
}

fn option_or_unreachable<T>(option: Option<T>, context: &'static str) -> T {
    match option {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}
