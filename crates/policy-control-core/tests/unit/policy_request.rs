use ocentra_policy_control_core::policy_request::{
    confirm_assistant_policy_request_preview, expire_child_policy_request,
    policy_request_schema_version, register_child_policy_request, resolve_parent_policy_approval,
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, ParentPolicyApproval,
    PolicyApprovalDecision, PolicyApprovalId, PolicyAssistantConfirmationState,
    PolicyAssistantPreviewId, PolicyDurationMinutes, PolicyOverrideState, PolicyRequestId,
    PolicyRequestKind, PolicyRequestOrigin, PolicyRequestResolution, PolicyRequestScope,
    PolicyRequestStatus, PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

const REQUESTED_AT: &str = "2026-06-13T20:00:00Z";
const EXPIRES_AT: &str = "2026-06-13T22:00:00Z";
const MODIFIED_EXPIRES_AT: &str = "2026-06-13T21:00:00Z";

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
        requested_at: timestamp(REQUESTED_AT),
        expires_at: timestamp(EXPIRES_AT),
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

fn assert_active_override_minutes(
    resolution: &PolicyRequestResolution,
    expected_minutes: u16,
    expected_expires_at: &str,
) {
    let temporary_override = resolution
        .temporary_override
        .as_ref()
        .expect("temporary override");

    assert_eq!(resolution.request.resolved_approval_id.is_some(), true);
    assert_eq!(temporary_override.state, PolicyOverrideState::Active);
    assert_eq!(
        temporary_override
            .approved_bonus_minutes
            .expect("approved bonus minutes")
            .value(),
        expected_minutes
    );
    assert_eq!(temporary_override.expires_at.as_str(), expected_expires_at);
}

#[test]
fn double_submit_is_idempotent_and_grant_replay_is_override_safe() {
    let registered = register_child_policy_request(None, child_request())
        .expect("registered child policy request");

    let duplicate = ChildPolicyRequest {
        request_id: PolicyRequestId::parse("request-bonus-time-duplicate").expect("request id"),
        ..registered.clone()
    };
    let deduped = register_child_policy_request(Some(&registered), duplicate)
        .expect("duplicate submit returns existing request");
    assert_eq!(deduped, registered);

    let parent_approval = approval(&registered, PolicyApprovalDecision::Grant);
    let resolved = resolve_parent_policy_approval(&registered, parent_approval.clone(), None)
        .expect("grant resolves child request");
    assert_eq!(resolved.request.status, PolicyRequestStatus::Approved);
    assert_active_override_minutes(&resolved, 30, EXPIRES_AT);

    let replay = resolve_parent_policy_approval(
        &resolved.request,
        parent_approval,
        resolved.temporary_override.as_ref(),
    )
    .expect("same approval id is replay safe");
    assert_eq!(replay, resolved);
}

#[test]
fn duplicate_submission_key_with_changed_payload_is_rejected() {
    let registered = register_child_policy_request(None, child_request())
        .expect("registered child policy request");
    let mut changed = registered.clone();
    changed.request_id = PolicyRequestId::parse("request-bonus-time-changed").expect("request id");
    changed.scope.requested_bonus_minutes = Some(PolicyDurationMinutes::new(45).expect("minutes"));

    let error = register_child_policy_request(Some(&registered), changed)
        .expect_err("changed payload cannot reuse submission key");
    assert!(error.to_string().contains("duplicate submission key"));
}

#[test]
fn parent_can_deny_request_without_creating_override() {
    let request = ChildPolicyRequest {
        scope: request_scope(PolicyRequestKind::AskParent, None),
        ..child_request()
    };
    let resolved = resolve_parent_policy_approval(
        &request,
        approval(&request, PolicyApprovalDecision::Deny),
        None,
    )
    .expect("deny resolves request");

    assert_eq!(resolved.request.status, PolicyRequestStatus::Denied);
    assert!(resolved.temporary_override.is_none());
    assert_eq!(
        resolved
            .request
            .resolved_approval_id
            .as_ref()
            .expect("resolved approval id")
            .as_str(),
        "request-bonus-time-deny"
    );
}

#[test]
fn parent_can_modify_bonus_time_request() {
    let request = child_request();
    let mut parent_approval = approval(&request, PolicyApprovalDecision::Modify);
    parent_approval.approved_bonus_minutes = Some(PolicyDurationMinutes::new(15).expect("minutes"));
    parent_approval.override_expires_at = Some(timestamp(MODIFIED_EXPIRES_AT));

    let resolved = resolve_parent_policy_approval(&request, parent_approval, None)
        .expect("modify resolves child request");

    assert_eq!(resolved.request.status, PolicyRequestStatus::Modified);
    assert_active_override_minutes(&resolved, 15, MODIFIED_EXPIRES_AT);
}

#[test]
fn parent_can_expire_pending_request_without_creating_override() {
    let request = child_request();
    let resolved = resolve_parent_policy_approval(
        &request,
        approval(&request, PolicyApprovalDecision::Expire),
        None,
    )
    .expect("parent can explicitly expire request");

    assert_eq!(resolved.request.status, PolicyRequestStatus::Expired);
    assert!(resolved.temporary_override.is_none());
    assert_eq!(
        resolved
            .request
            .resolved_approval_id
            .as_ref()
            .expect("resolved approval id")
            .as_str(),
        "request-bonus-time-expire"
    );
}

#[test]
fn expired_request_cannot_be_approved() {
    let expired_request = expire_child_policy_request(
        &child_request(),
        timestamp("2026-06-13T22:05:00Z"),
        audit_ref("audit-request-expired"),
    )
    .expect("request expires");

    let error = resolve_parent_policy_approval(
        &expired_request,
        approval(&expired_request, PolicyApprovalDecision::Grant),
        None,
    )
    .expect_err("expired request must not become active override");
    assert!(error
        .to_string()
        .contains("expired-request-cannot-be-approved"));
}

#[test]
fn assistant_draft_stays_preview_only_until_parent_confirms() {
    let preview = register_child_policy_request(None, assistant_preview_request())
        .expect("registered assistant preview request");

    let preview_error = resolve_parent_policy_approval(
        &preview,
        approval(&preview, PolicyApprovalDecision::Grant),
        None,
    )
    .expect_err("preview-only assistant request cannot resolve");
    assert!(preview_error.to_string().contains("assistant-preview-only"));

    let confirmed = confirm_assistant_policy_request_preview(
        &preview,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-parent").expect("actor id"),
            actor_role: ParentPolicyActorRole::Parent,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp("2026-06-13T20:03:00Z"),
            audit_reference_id: audit_ref("audit-assistant-confirmed"),
        },
    )
    .expect("parent confirms assistant preview");
    assert_eq!(confirmed.status, PolicyRequestStatus::PendingParentReview);
    assert_eq!(
        confirmed.assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmed
    );

    let resolved = resolve_parent_policy_approval(
        &confirmed,
        approval(&confirmed, PolicyApprovalDecision::Grant),
        None,
    )
    .expect("confirmed assistant preview can resolve");
    assert_eq!(resolved.request.status, PolicyRequestStatus::Approved);
}

#[test]
fn child_and_support_roles_cannot_confirm_or_self_approve() {
    let preview = assistant_preview_request();
    let child_confirmation_error = confirm_assistant_policy_request_preview(
        &preview,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-child").expect("actor id"),
            actor_role: ParentPolicyActorRole::Child,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp("2026-06-13T20:03:00Z"),
            audit_reference_id: audit_ref("audit-child-confirm-attempt"),
        },
    )
    .expect_err("child cannot confirm assistant preview");
    assert!(child_confirmation_error.to_string().contains("child"));

    let mut support_approval = approval(&child_request(), PolicyApprovalDecision::Grant);
    support_approval.actor_id = PolicyActorId::parse("actor-support").expect("actor id");
    support_approval.actor_role = ParentPolicyActorRole::Support;

    let support_error = resolve_parent_policy_approval(&child_request(), support_approval, None)
        .expect_err("support cannot approve child request");
    assert!(support_error.to_string().contains("support"));
}

#[test]
fn observer_and_revoked_parent_cannot_confirm_or_approve() {
    let preview = assistant_preview_request();
    let observer_confirmation_error = confirm_assistant_policy_request_preview(
        &preview,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-observer").expect("actor id"),
            actor_role: ParentPolicyActorRole::Observer,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp("2026-06-13T20:03:00Z"),
            audit_reference_id: audit_ref("audit-observer-confirm-attempt"),
        },
    )
    .expect_err("observer cannot confirm assistant preview");
    assert!(observer_confirmation_error.to_string().contains("observer"));

    let mut revoked_parent_approval = approval(&child_request(), PolicyApprovalDecision::Grant);
    revoked_parent_approval.actor_id =
        PolicyActorId::parse("actor-revoked-parent").expect("actor id");
    revoked_parent_approval.actor_state = PolicySourceActorState::Revoked;

    let revoked_parent_error =
        resolve_parent_policy_approval(&child_request(), revoked_parent_approval, None)
            .expect_err("revoked parent cannot approve child request");
    assert!(revoked_parent_error
        .to_string()
        .contains("policy_request.actor_state"));
}

#[test]
fn wrong_parent_household_cannot_approve_request() {
    let request = child_request();
    let mut wrong_parent_approval = approval(&request, PolicyApprovalDecision::Grant);
    wrong_parent_approval.actor_id = PolicyActorId::parse("actor-other-parent").expect("actor id");
    wrong_parent_approval.household_id =
        PolicyHouseholdId::parse("household-other").expect("policy household id");

    let error = resolve_parent_policy_approval(&request, wrong_parent_approval, None)
        .expect_err("wrong household parent cannot approve request");
    assert!(error.to_string().contains("policy_request.household_id"));
}
