use ocentra_child_runtime::policy_control_runtime_flow::{
    apply_policy_control_delivery_handoff, confirm_policy_control_request_handoff,
    expire_policy_control_request_handoff, queue_policy_control_delivery_handoff,
    register_policy_control_request_handoff, resolve_policy_control_request_handoff,
};
use ocentra_eventing::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliverySequence, PolicyDeliveryState,
    PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, AssistantPolicyRequestConfirmation, ChildPolicyRequest,
    ParentPolicyApproval, PolicyApprovalDecision, PolicyApprovalId,
    PolicyAssistantConfirmationState, PolicyAssistantPreviewId, PolicyDurationMinutes,
    PolicyRequestId, PolicyRequestKind, PolicyRequestOrigin, PolicyRequestScope,
    PolicyRequestStatus, PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    compile_domain_policy_artifact, parent_policy_source_schema_version, ParentPolicyActorRole,
    ParentPolicyDocumentId, ParentPolicyRule, ParentPolicySourceDocument, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRuleAction, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetCarryoverRule,
    PolicyScheduleBudgetResetKind, PolicyScheduleBudgetResetRule, PolicyScheduleClockSource,
    PolicyScheduleId, PolicyScheduleOfflineRecovery, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceActorState, PolicySourceDocumentStatus,
    PolicySourceWriteSurface, PolicyTargetKind, PolicyTargetReferenceId, PolicyTimezoneName,
    PolicyVersion,
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

fn child_request() -> ChildPolicyRequest {
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
        status: PolicyRequestStatus::PendingParentReview,
        scope: request_scope(PolicyRequestKind::BonusTime, Some(30)),
        requested_at: timestamp("2026-06-13T20:00:00Z"),
        expires_at: timestamp("2026-06-13T22:00:00Z"),
        audit_reference_ids: vec![audit_ref("audit-request-created")],
        resolved_approval_id: None,
        resolved_at: None,
    }
}

fn assistant_preview_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        origin: PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: Some(
            PolicyAssistantPreviewId::parse("assistant-preview-default")
                .expect("assistant preview id"),
        ),
        assistant_confirmation_state: PolicyAssistantConfirmationState::ParentConfirmationRequired,
        status: PolicyRequestStatus::PreviewOnly,
        request_id: PolicyRequestId::parse("request-assistant-preview").expect("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-assistant-preview-submit")
            .expect("policy submission key"),
        scope: request_scope(PolicyRequestKind::AskParent, None),
        ..child_request()
    }
}

fn approval(
    request: &ChildPolicyRequest,
    decision: PolicyApprovalDecision,
) -> ParentPolicyApproval {
    ParentPolicyApproval {
        approval_id: PolicyApprovalId::parse(format!(
            "{}-{}",
            request.request_id.as_str(),
            match decision {
                PolicyApprovalDecision::Grant => "grant",
                PolicyApprovalDecision::Deny => "deny",
                PolicyApprovalDecision::Modify => "modify",
                PolicyApprovalDecision::Expire => "expire",
            }
        ))
        .expect("policy approval id"),
        request_id: request.request_id.clone(),
        household_id: request.household_id.clone(),
        policy_version: request.policy_version,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
        decision,
        approved_action: None,
        approved_bonus_minutes: None,
        override_expires_at: None,
        decided_at: timestamp("2026-06-13T20:05:00Z"),
        audit_reference_id: audit_ref("audit-parent-decision"),
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

fn sample_delivery_target() -> PolicyDeliveryTarget {
    PolicyDeliveryTarget {
        child_profile_id: PolicyChildProfileId::parse("child-primary").expect("child profile id"),
        device_id: PolicyDeviceId::parse("device-laptop").expect("policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    }
}

#[test]
fn request_handoff_makes_pending_review_parent_visible() {
    let report = register_policy_control_request_handoff(None, child_request())
        .expect("registered child policy request handoff");

    assert_eq!(
        report.request.status,
        PolicyRequestStatus::PendingParentReview
    );
    assert_eq!(
        report.parent_notification.state,
        ocentra_child_notification_core::policy_control_notification::PolicyControlNotificationState::PendingParentReview
    );
    assert_eq!(report.parent_notification.audit_reference_ids.len(), 1);
}

#[test]
fn assistant_preview_and_confirmation_stay_gated_until_parent_review() {
    let preview = register_policy_control_request_handoff(None, assistant_preview_request())
        .expect("registered assistant preview handoff");
    assert_eq!(
        preview.parent_notification.state,
        ocentra_child_notification_core::policy_control_notification::PolicyControlNotificationState::PreviewOnly
    );

    let confirmed = confirm_policy_control_request_handoff(
        &preview.request,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-parent").expect("actor id"),
            actor_role: ParentPolicyActorRole::Parent,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp("2026-06-13T20:03:00Z"),
            audit_reference_id: audit_ref("audit-assistant-confirmed"),
        },
    )
    .expect("confirmed assistant preview handoff");

    assert_eq!(
        confirmed.request.status,
        PolicyRequestStatus::PendingParentReview
    );
    assert_eq!(confirmed.parent_notification.audit_reference_ids.len(), 2);
}

#[test]
fn resolved_request_can_queue_and_apply_delivery_without_losing_audit_refs() {
    let request = child_request();
    let resolved = resolve_policy_control_request_handoff(
        &request,
        approval(&request, PolicyApprovalDecision::Grant),
        None,
    )
    .expect("grant resolves handoff");
    let compiled = compile_domain_policy_artifact(
        &sample_policy_source_document(),
        PolicyConsumerDomain::Tracking,
    )
    .expect("compiled domain policy artifact");

    let queued = queue_policy_control_delivery_handoff(
        &compiled,
        sample_delivery_target(),
        &resolved.request,
        resolved.temporary_override.as_ref(),
        PolicyDeliveryId::parse("delivery-policy-household-default").expect("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-queued").expect("policy attempt id"),
        vec![audit_ref("audit-policy-queued")],
    )
    .expect("queued delivery handoff");

    assert_eq!(
        queued.parent_notification.state,
        ocentra_child_notification_core::policy_control_notification::PolicyControlNotificationState::DeliveryPending
    );
    assert_eq!(queued.parent_notification.audit_reference_ids.len(), 3);
    assert_eq!(
        queued.delivery.source_audit_reference_ids,
        vec![audit_ref("audit-policy-confirmed")]
    );
    assert!(queued.delivery.source_superseded_by_policy_version.is_none());
    assert!(queued.delivery.source_rollback_ref.is_none());

    let applied = apply_policy_control_delivery_handoff(
        &resolved.request,
        resolved.temporary_override.as_ref(),
        &queued.delivery,
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
    .expect("applied delivery handoff");

    assert_eq!(
        applied.parent_notification.state,
        ocentra_child_notification_core::policy_control_notification::PolicyControlNotificationState::DeliveryApplied
    );
    assert_eq!(applied.parent_notification.audit_reference_ids.len(), 3);
}

#[test]
fn replay_and_expire_paths_do_not_create_extra_delivery_truth() {
    let request = child_request();
    let parent_approval = approval(&request, PolicyApprovalDecision::Grant);
    let resolved = resolve_policy_control_request_handoff(&request, parent_approval.clone(), None)
        .expect("grant resolves handoff");
    let replay = resolve_policy_control_request_handoff(
        &resolved.request,
        parent_approval,
        resolved.temporary_override.as_ref(),
    )
    .expect("replay is safe");

    assert_eq!(replay.request, resolved.request);
    assert_eq!(replay.temporary_override, resolved.temporary_override);

    let expired = expire_policy_control_request_handoff(
        &child_request(),
        timestamp("2026-06-13T22:05:00Z"),
        audit_ref("audit-request-expired"),
    )
    .expect("request expires");

    let compiled = compile_domain_policy_artifact(
        &sample_policy_source_document(),
        PolicyConsumerDomain::Tracking,
    )
    .expect("compiled domain policy artifact");
    let error = queue_policy_control_delivery_handoff(
        &compiled,
        sample_delivery_target(),
        &expired.request,
        None,
        PolicyDeliveryId::parse("delivery-expired-request").expect("policy delivery id"),
        PolicyDeliveryAttemptId::parse("attempt-expired").expect("policy attempt id"),
        vec![audit_ref("audit-policy-queued")],
    )
    .expect_err("expired request cannot queue delivery handoff");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_control_notification.delivery_state",
            value: "expired".to_string(),
        }
    );
}

#[test]
fn observer_and_revoked_parent_denials_survive_runtime_flow() {
    let preview = assistant_preview_request();
    let observer_confirmation_error = confirm_policy_control_request_handoff(
        &preview,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-observer").expect("actor id"),
            actor_role: ParentPolicyActorRole::Observer,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp("2026-06-13T20:03:00Z"),
            audit_reference_id: audit_ref("audit-observer-confirm-attempt"),
        },
    )
    .expect_err("observer cannot confirm preview through runtime flow");
    assert_eq!(
        observer_confirmation_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_role",
            value: "observer".to_string(),
        }
    );

    let request = child_request();
    let mut revoked_parent_approval = approval(&request, PolicyApprovalDecision::Grant);
    revoked_parent_approval.actor_id =
        PolicyActorId::parse("actor-revoked-parent").expect("actor id");
    revoked_parent_approval.actor_state = PolicySourceActorState::Revoked;

    let revoked_parent_error =
        resolve_policy_control_request_handoff(&request, revoked_parent_approval, None)
            .expect_err("revoked parent cannot approve through runtime flow");
    assert_eq!(
        revoked_parent_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_state",
            value: "revoked".to_string(),
        }
    );
}
