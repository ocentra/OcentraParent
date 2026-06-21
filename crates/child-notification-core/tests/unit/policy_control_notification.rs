use ocentra_child_notification_core::policy_control_notification::{
    build_policy_control_parent_notification, PolicyControlNotificationState,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryAttemptId,
    PolicyDeliveryId, PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTarget,
    PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, ChildPolicyRequest, PolicyApprovalId,
    PolicyAssistantConfirmationState, PolicyDurationMinutes, PolicyOverrideState, PolicyRequestId,
    PolicyRequestKind, PolicyRequestOrigin, PolicyRequestScope, PolicyRequestStatus,
    PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
    PolicyTemporaryOverride,
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

fn timestamp(value: &str) -> PolicyRequestTimestamp {
    PolicyRequestTimestamp::parse(value).expect("policy request timestamp")
}

fn audit_ref(value: &str) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(value).expect("policy audit ref")
}

fn request_scope(kind: PolicyRequestKind, minutes: Option<u16>) -> PolicyRequestScope {
    PolicyRequestScope {
        request_kind: kind,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect("policy target ref"),
        },
        requested_action: PolicyRuleAction::TimeLimit,
        rule_id: Some(PolicyRuleId::parse("rule-school-night").expect("policy rule id")),
        requested_bonus_minutes: minutes
            .map(PolicyDurationMinutes::new)
            .transpose()
            .expect("policy duration"),
    }
}

fn approved_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        schema_version: policy_request_schema_version().expect("policy request schema version"),
        request_id: PolicyRequestId::parse("request-bonus-time").expect("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-bonus-time-submit")
            .expect("policy submission key"),
        household_id: PolicyHouseholdId::parse("household-default").expect("policy household id"),
        child_profile_id: PolicyChildProfileId::parse("child-primary").expect("child profile id"),
        device_id: Some(PolicyDeviceId::parse("device-laptop").expect("policy device id")),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect("policy source id"),
        policy_version: PolicyVersion::new(7).expect("policy version"),
        origin: PolicyRequestOrigin::Child,
        assistant_preview_id: None,
        assistant_confirmation_state: PolicyAssistantConfirmationState::NotRequired,
        status: PolicyRequestStatus::Approved,
        scope: request_scope(PolicyRequestKind::BonusTime, Some(30)),
        requested_at: timestamp("2026-06-13T20:00:00Z"),
        expires_at: timestamp("2026-06-13T22:00:00Z"),
        audit_reference_ids: vec![
            audit_ref("audit-request-created"),
            audit_ref("audit-parent-decision"),
        ],
        resolved_approval_id: Some(
            PolicyApprovalId::parse("request-bonus-time-grant").expect("policy approval id"),
        ),
        resolved_at: Some(timestamp("2026-06-13T20:05:00Z")),
    }
}

fn preview_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        origin: PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: Some(
            ocentra_policy_control_core::policy_request::PolicyAssistantPreviewId::parse(
                "assistant-preview-default",
            )
            .expect("assistant preview id"),
        ),
        assistant_confirmation_state: PolicyAssistantConfirmationState::ParentConfirmationRequired,
        status: PolicyRequestStatus::PreviewOnly,
        request_id: PolicyRequestId::parse("request-assistant-preview").expect("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-assistant-preview-submit")
            .expect("policy submission key"),
        scope: request_scope(PolicyRequestKind::AskParent, None),
        audit_reference_ids: vec![audit_ref("audit-request-created")],
        resolved_approval_id: None,
        resolved_at: None,
        ..approved_request()
    }
}

fn approved_override() -> PolicyTemporaryOverride {
    PolicyTemporaryOverride {
        schema_version: policy_request_schema_version().expect("policy request schema version"),
        override_id: ocentra_policy_control_core::policy_request::PolicyOverrideId::parse(
            "policy-override:request-bonus-time-grant",
        )
        .expect("policy override id"),
        source_request_id: approved_request().request_id.clone(),
        source_approval_id: approved_request()
            .resolved_approval_id
            .clone()
            .expect("resolved approval id"),
        household_id: approved_request().household_id.clone(),
        child_profile_id: approved_request().child_profile_id.clone(),
        device_id: approved_request().device_id.clone(),
        source_document_id: approved_request().source_document_id.clone(),
        policy_version: approved_request().policy_version,
        request_kind: PolicyRequestKind::BonusTime,
        target: approved_request().scope.target.clone(),
        approved_action: PolicyRuleAction::TimeLimit,
        approved_bonus_minutes: Some(PolicyDurationMinutes::new(30).expect("minutes")),
        effective_at: timestamp("2026-06-13T20:05:00Z"),
        expires_at: timestamp("2026-06-13T22:00:00Z"),
        state: PolicyOverrideState::Active,
        audit_reference_ids: vec![audit_ref("audit-parent-decision")],
    }
}

fn sample_policy_source_document() -> ParentPolicySourceDocument {
    ParentPolicySourceDocument {
        schema_version: parent_policy_source_schema_version()
            .expect("policy source schema version"),
        document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect("policy source document id"),
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        policy_version: PolicyVersion::new(7).expect("policy version"),
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

fn queued_delivery() -> ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord {
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

#[test]
fn preview_only_request_stays_confirmation_gated() {
    let notification = build_policy_control_parent_notification(&preview_request(), None, None)
        .expect("preview notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::PreviewOnly
    );
    assert!(notification.source_approval_id.is_none());
    assert!(notification.source_override_id.is_none());
    assert!(notification.delivery_parent_visible_state.is_none());
}

#[test]
fn approved_request_and_queued_delivery_keep_override_and_audit_context() {
    let request = approved_request();
    let temporary_override = approved_override();
    let queued = queued_delivery();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&queued),
    )
    .expect("queued delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryPending
    );
    assert_eq!(
        notification
            .source_approval_id
            .as_ref()
            .expect("source approval id")
            .as_str(),
        "request-bonus-time-grant"
    );
    assert_eq!(
        notification
            .source_override_id
            .as_ref()
            .expect("source override id")
            .as_str(),
        "policy-override:request-bonus-time-grant"
    );
    assert_eq!(notification.audit_reference_ids.len(), 3);
}

#[test]
fn applied_delivery_promotes_parent_visible_state() {
    let request = approved_request();
    let temporary_override = approved_override();
    let queued = queued_delivery();
    let applied = apply_policy_delivery_transition(
        &queued,
        PolicyDeliveryTransition {
            attempt_id: PolicyDeliveryAttemptId::parse("attempt-applied")
                .expect("policy attempt id"),
            sequence: PolicyDeliverySequence::new(2).expect("policy delivery sequence"),
            state: PolicyDeliveryState::Applied,
            audit_reference_ids: vec![audit_ref("audit-policy-applied")],
            reason_code: None,
            superseded_by_policy_version: None,
            rollback_reference_state: None,
        },
    )
    .expect("applied transition")
    .into_record();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&applied),
    )
    .expect("applied delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryApplied
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect("delivery parent visible state"),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Applied
    );
    assert_eq!(notification.audit_reference_ids.len(), 3);
}

#[test]
fn retry_and_partial_delivery_states_stay_parent_visible_as_degraded() {
    let request = approved_request();
    let temporary_override = approved_override();
    let queued = queued_delivery();
    let retry = PolicyDeliveryTransition {
        attempt_id: PolicyDeliveryAttemptId::parse("attempt-retry").expect("policy attempt id"),
        sequence: PolicyDeliverySequence::new(2).expect("policy delivery sequence"),
        state: PolicyDeliveryState::RetryScheduled,
        audit_reference_ids: vec![audit_ref("audit-policy-retry")],
        reason_code: Some(PolicyReasonCode::parse("adapter-timeout").expect("reason code")),
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    };
    let retry = apply_policy_delivery_transition(&queued, retry)
        .expect("retry transition")
        .into_record();

    retry
        .reason_code
        .as_ref()
        .expect("retry reason code is preserved");

    let notification =
        build_policy_control_parent_notification(&request, Some(&temporary_override), Some(&retry))
            .expect("retry delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryDegraded
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect("delivery parent visible state"),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::Degraded
    );
}

#[test]
fn blocked_delivery_states_surface_manual_required_notifications() {
    let request = approved_request();
    let temporary_override = approved_override();
    let queued = queued_delivery();
    let blocked = apply_policy_delivery_transition(
        &queued,
        PolicyDeliveryTransition {
            attempt_id: PolicyDeliveryAttemptId::parse("attempt-blocked-permission")
                .expect("policy attempt id"),
            sequence: PolicyDeliverySequence::new(2).expect("policy delivery sequence"),
            state: PolicyDeliveryState::BlockedByPermission,
            audit_reference_ids: vec![audit_ref("audit-policy-blocked-permission")],
            reason_code: Some(
                PolicyReasonCode::parse("device-permission-lost").expect("reason code"),
            ),
            superseded_by_policy_version: None,
            rollback_reference_state: None,
        },
    )
    .expect("blocked transition")
    .into_record();

    let notification = build_policy_control_parent_notification(
        &request,
        Some(&temporary_override),
        Some(&blocked),
    )
    .expect("blocked delivery notification");

    assert_eq!(
        notification.state,
        PolicyControlNotificationState::DeliveryManualRequired
    );
    assert_eq!(
        notification
            .delivery_parent_visible_state
            .expect("delivery parent visible state"),
        ocentra_policy_control_core::policy_delivery::PolicyDeliveryParentVisibleState::ManualRequired
    );
}

#[test]
fn denied_request_cannot_fake_override_or_delivery() {
    let mut denied = approved_request();
    denied.status = PolicyRequestStatus::Denied;
    denied.resolved_at = Some(timestamp("2026-06-13T20:05:00Z"));

    let error = build_policy_control_parent_notification(&denied, Some(&approved_override()), None)
        .expect_err("denied request must not carry override");
    assert!(error
        .to_string()
        .contains("policy_control_notification.override_id"));
}

#[test]
fn replay_rejected_request_is_rejected_for_parent_notification() {
    let mut replay_rejected = approved_request();
    replay_rejected.status = PolicyRequestStatus::ReplayRejected;
    replay_rejected.resolved_approval_id = None;
    replay_rejected.resolved_at = None;

    let error = build_policy_control_parent_notification(&replay_rejected, None, None)
        .expect_err("replay-rejected status must not produce a parent notification");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.status",
            value: "replay-rejected".to_string(),
        }
    );
}
