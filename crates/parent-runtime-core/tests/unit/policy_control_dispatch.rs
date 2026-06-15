use ocentra_eventing::DomainEvent;
use ocentra_parent_runtime_core::policy_control_dispatch::{
    parent_runtime_policy_control_dispatch_evaluated_event, route_parent_policy_control_delivery,
    route_parent_policy_control_delivery_from_origin, ParentPolicyControlAcknowledgementState,
    ParentRuntimePolicyControlOriginState, ParentRuntimePolicyControlPublishState,
    ParentRuntimePolicyControlWaitState,
    PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE,
};
use ocentra_policy_control_core::policy_authority::{
    EvidenceReferenceState, ParentAuthorityState, PolicyActionAuthorizationState,
    PolicyConflictDecision, PolicyConflictResolutionState, PolicyControlAggregateId,
    PolicyControlDecision, PolicyControlDecisionId, PolicyControlRequestId,
    PolicyDecisionResolvedEvent, PolicyEnforcementExecutionState, PolicyManualReviewState,
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
    PolicyScheduleWindow, PolicySourceDocumentStatus, PolicySourceWriteSurface, PolicyTargetKind,
    PolicyTargetReferenceId, PolicyTimezoneName, PolicyVersion,
};

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
    let offline = apply_policy_delivery_transition(
        &queued,
        PolicyDeliveryTransition {
            attempt_id: PolicyDeliveryAttemptId::parse("attempt-offline")
                .expect("policy delivery attempt id"),
            sequence: ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(2)
                .expect("policy delivery sequence"),
            state: PolicyDeliveryState::Offline,
            audit_reference_ids: vec![audit_ref("audit-policy-offline")],
            reason_code: Some(reason("child-offline")),
            superseded_by_policy_version: None,
            rollback_reference_state: None,
        },
    )
    .expect("offline transition")
    .into_record();
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
        dispatch
            .contract()
            .expect("policy control dispatch contract")
            .event_type
            .as_str(),
        PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE
    );
}

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-household-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(3).expect("policy version"),
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
    let compiled = compile_domain_policy_artifact(
        &sample_policy_source_document(),
        PolicyConsumerDomain::Tracking,
    )
    .expect("compiled domain policy artifact");

    queue_policy_delivery(
        &compiled,
        PolicyDeliveryTarget {
            child_profile_id: PolicyChildProfileId::parse("child-primary")
                .expect("child profile id"),
            device_id: PolicyDeviceId::parse("device-laptop").expect("policy device id"),
            domain: PolicyConsumerDomain::Tracking,
        },
        PolicyDeliveryId::parse("delivery-policy-household-default").expect("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-queued").expect("policy attempt id"),
        vec![audit_ref("audit-policy-queued")],
    )
    .expect("queued policy delivery")
}

fn authorized_decision_event() -> PolicyDecisionResolvedEvent {
    PolicyDecisionResolvedEvent {
        aggregate_id: PolicyControlAggregateId::parse(
            "policy-control-aggregate:child-primary:tracking",
        )
        .expect("policy control aggregate id"),
        decision_id: PolicyControlDecisionId::parse("policy-control-decision-1")
            .expect("policy control decision id"),
        source_request_id: PolicyControlRequestId::parse("policy-control-request-1")
            .expect("policy control request id"),
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

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(value).expect("policy audit ref")
}

fn reason(value: &str) -> PolicyReasonCode {
    PolicyReasonCode::parse(value).expect("policy reason code")
}
