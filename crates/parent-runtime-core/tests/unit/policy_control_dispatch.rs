use ocentra_eventing::envelope::DomainEvent;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_runtime_core::policy_control_dispatch::{
    parent_runtime_policy_control_dispatch_evaluated_event, route_parent_policy_control_delivery,
    route_parent_policy_control_delivery_from_origin, ParentPolicyControlAcknowledgementState,
    ParentRuntimePolicyControlOriginState, ParentRuntimePolicyControlPublishState,
    ParentRuntimePolicyControlWaitState,
    PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyActionAuthorizationState, PolicyConflictDecision, PolicyConflictResolutionState,
    PolicyControlAggregateId, PolicyControlDecision, PolicyControlDecisionId,
    PolicyControlRequestId, PolicyDecisionResolvedEvent, PolicyEnforcementExecutionState,
    PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryAttemptId,
    PolicyDeliveryId, PolicyDeliveryState, PolicyDeliveryTarget, PolicyDeliveryTransition,
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

macro_rules! result_or_panic {
    ($result:expr, $context:expr $(,)?) => {
        $result.expect($context)
    };
}

macro_rules! audit_ref {
    ($value:expr) => {
        result_or_panic!(PolicyAuditReferenceId::parse($value), "policy audit ref")
    };
}

macro_rules! reason {
    ($value:expr) => {
        result_or_panic!(PolicyReasonCode::parse($value), "policy reason code")
    };
}

#[test]
fn queued_authorized_delivery_publishes_and_awaits_child_acknowledgement() {
    let delivery = sample_queued_delivery();
    let decision = authorized_decision_event();

    let dispatch = route_parent_policy_control_delivery(
        &delivery,
        &decision,
        ParentPolicyControlAcknowledgementState::Required,
    );

    assert_eq!(
        dispatch.child_runtime_publish_state,
        ParentRuntimePolicyControlPublishState::Publish
    );
    assert_eq!(
        dispatch.child_acknowledgement_wait_state,
        ParentRuntimePolicyControlWaitState::Await
    );
    assert_eq!(
        dispatch.parent_visible_state,
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Pending
    );
}

#[test]
fn untrusted_origin_blocks_policy_delivery_before_child_runtime_publish() {
    let delivery = sample_queued_delivery();
    let decision = authorized_decision_event();

    let dispatch = route_parent_policy_control_delivery_from_origin(
        &delivery,
        &decision,
        ParentPolicyControlAcknowledgementState::Required,
        ParentRuntimePolicyControlOriginState::Untrusted,
    );

    assert_eq!(
        dispatch.child_runtime_publish_state,
        ParentRuntimePolicyControlPublishState::DoNotPublish
    );
    assert_eq!(
        dispatch.child_acknowledgement_wait_state,
        ParentRuntimePolicyControlWaitState::DoNotAwait
    );
    assert_eq!(
        dispatch.parent_visible_state,
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::ManualRequired
    );
}

#[test]
fn degraded_delivery_can_be_republished_after_offline_state_clears() {
    let queued = sample_queued_delivery();
    let offline = apply_offline_delivery_transition(&queued).into_record();
    let decision = authorized_decision_event();

    let dispatch = parent_runtime_policy_control_dispatch_evaluated_event(
        &offline,
        &decision,
        ParentPolicyControlAcknowledgementState::NotRequired,
    );

    assert_eq!(
        dispatch.decision.child_runtime_publish_state,
        ParentRuntimePolicyControlPublishState::Publish
    );
    assert_eq!(
        dispatch.decision.child_acknowledgement_wait_state,
        ParentRuntimePolicyControlWaitState::DoNotAwait
    );
    assert_eq!(
        dispatch.decision.parent_visible_state,
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Degraded
    );
    assert_eq!(
        result_or_panic!(dispatch.contract(), "policy control dispatch contract")
            .event_type
            .as_str(),
        PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE
    );
}

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: result_or_panic!(
            parent_policy_source_schema_version(),
            "policy source schema version",
        ),
        document_id: result_or_panic!(
            ParentPolicyDocumentId::parse("policy-source-household-default"),
            "policy source document id",
        ),
        household_id: result_or_panic!(
            PolicyHouseholdId::parse("household-default"),
            "household id",
        ),
        policy_version: result_or_panic!(PolicyVersion::new(3), "policy version"),
        source_surface: PolicySourceSurface::ParentPortal,
        actor_id: result_or_panic!(PolicyActorId::parse("actor-parent"), "policy actor id"),
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
    let compiled = result_or_panic!(
        compile_domain_policy_artifact(
            &sample_policy_source_document(),
            PolicyConsumerDomain::Tracking,
        ),
        "compiled domain policy artifact",
    );

    result_or_panic!(
        queue_policy_delivery(
            &compiled,
            sample_policy_delivery_target(),
            result_or_panic!(
                PolicyDeliveryId::parse("delivery-policy-household-default"),
                "policy delivery id",
            ),
            result_or_panic!(
                PolicyDeliveryAttemptId::parse("attempt-queued"),
                "policy attempt id",
            ),
            vec![audit_ref!("audit-policy-queued")],
        ),
        "queued policy delivery",
    )
}

fn authorized_decision_event() -> PolicyDecisionResolvedEvent {
    PolicyDecisionResolvedEvent {
        aggregate_id: result_or_panic!(
            PolicyControlAggregateId::parse("policy-control-aggregate:child-primary:tracking"),
            "policy control aggregate id",
        ),
        decision_id: result_or_panic!(
            PolicyControlDecisionId::parse("policy-control-decision-1"),
            "policy control decision id",
        ),
        source_request_id: result_or_panic!(
            PolicyControlRequestId::parse("policy-control-request-1"),
            "policy control request id",
        ),
        decision: PolicyControlDecision {
            action_authorization_state: PolicyActionAuthorizationState::Authorized,
            enforcement_execution_state: PolicyEnforcementExecutionState::MayExecute,
            manual_review_state: PolicyManualReviewState::NotRequired,
        },
        conflict_decision: PolicyConflictDecision {
            resolution_state: PolicyConflictResolutionState::UseParentPolicy,
            manual_review_state: PolicyManualReviewState::NotRequired,
        },
    }
}

fn sample_child_profile_id() -> PolicyChildProfileId {
    result_or_panic!(
        PolicyChildProfileId::parse("child-primary"),
        "child profile id"
    )
}

fn sample_device_id() -> PolicyDeviceId {
    result_or_panic!(PolicyDeviceId::parse("device-laptop"), "policy device id")
}

fn sample_schedule_id() -> PolicyScheduleId {
    result_or_panic!(
        PolicyScheduleId::parse("schedule-school-night"),
        "policy schedule id",
    )
}

fn sample_policy_rule_target() -> PolicyRuleTarget {
    PolicyRuleTarget {
        kind: PolicyTargetKind::Category,
        reference_id: result_or_panic!(
            PolicyTargetReferenceId::parse("category-gaming"),
            "policy target reference",
        ),
    }
}

fn sample_policy_rule() -> ParentPolicyRule {
    ParentPolicyRule {
        rule_id: result_or_panic!(
            PolicyRuleId::parse("rule-school-night-block"),
            "policy rule id",
        ),
        target: sample_policy_rule_target(),
        action: PolicyRuleAction::Block,
        schedule_id: Some(sample_schedule_id()),
        priority: 100,
        reason_code: result_or_panic!(
            PolicyReasonCode::parse("school-night"),
            "policy reason code",
        ),
        enabled: true,
    }
}

fn sample_schedule_window() -> PolicyScheduleWindow {
    PolicyScheduleWindow {
        schedule_id: sample_schedule_id(),
        timezone_name: result_or_panic!(
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

fn apply_offline_delivery_transition(
    queued: &ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord,
) -> ocentra_policy_control_core::policy_delivery::PolicyDeliveryApplyOutcome {
    result_or_panic!(
        apply_policy_delivery_transition(queued, sample_offline_delivery_transition()),
        "offline transition",
    )
}

fn sample_offline_delivery_transition() -> PolicyDeliveryTransition {
    PolicyDeliveryTransition {
        attempt_id: result_or_panic!(
            PolicyDeliveryAttemptId::parse("attempt-offline"),
            "policy delivery attempt id",
        ),
        sequence: result_or_panic!(
            ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(2),
            "policy delivery sequence",
        ),
        state: PolicyDeliveryState::Offline,
        audit_reference_ids: vec![audit_ref!("audit-policy-offline")],
        reason_code: Some(reason!("child-offline")),
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
}
