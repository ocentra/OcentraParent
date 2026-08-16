use ocentra_child_policy_core::policy_control_request_handoff::{
    confirm_policy_control_request_handoff, expire_policy_control_request_handoff,
    register_policy_control_request_handoff, resolve_policy_control_request_handoff,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, AssistantPolicyRequestConfirmation, ChildPolicyRequest,
    ParentPolicyApproval, PolicyApprovalDecision, PolicyApprovalId, PolicyAssistantPreviewId,
    PolicyDurationMinutes, PolicyRequestId, PolicyRequestKind, PolicyRequestScope,
    PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

fn timestamp(value: PolicyRequestTimestamp) -> PolicyRequestTimestamp {
    value
}

fn audit_ref(value: PolicyAuditReferenceId) -> PolicyAuditReferenceId {
    value
}

fn request_scope(kind: PolicyRequestKind, minutes: Option<u16>) -> PolicyRequestScope {
    PolicyRequestScope {
        request_kind: kind,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect_value("policy target ref"),
        },
        requested_action: PolicyRuleAction::TimeLimit,
        rule_id: Some(PolicyRuleId::parse("rule-school-night").expect_value("policy rule id")),
        requested_bonus_minutes: minutes
            .map(PolicyDurationMinutes::new)
            .transpose()
            .expect_value("policy duration"),
    }
}

fn child_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        schema_version: policy_request_schema_version()
            .expect_value("policy request schema version"),
        request_id: PolicyRequestId::parse("request-bonus-time").expect_value("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-bonus-time-submit")
            .expect_value("policy submission key"),
        household_id: PolicyHouseholdId::parse("household-default")
            .expect_value("policy household id"),
        child_profile_id: PolicyChildProfileId::parse("child-primary")
            .expect_value("child profile id"),
        device_id: Some(PolicyDeviceId::parse("device-laptop").expect_value("policy device id")),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        origin: PolicyRequestOrigin::Child,
        assistant_preview_id: None,
        assistant_confirmation_state: PolicyAssistantConfirmationState::NotRequired,
        status: PolicyRequestStatus::PendingParentReview,
        scope: request_scope(PolicyRequestKind::BonusTime, Some(30)),
        requested_at: timestamp(
            PolicyRequestTimestamp::parse("2026-06-13T20:00:00Z")
                .expect_value("policy request timestamp"),
        ),
        expires_at: timestamp(
            PolicyRequestTimestamp::parse("2026-06-13T22:00:00Z")
                .expect_value("policy request timestamp"),
        ),
        audit_reference_ids: vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-request-created").expect_value("policy audit ref"),
        )],
        resolved_approval_id: None,
        resolved_at: None,
    }
}

fn assistant_preview_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        origin: PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: Some(
            PolicyAssistantPreviewId::parse("assistant-preview-default")
                .expect_value("assistant preview id"),
        ),
        assistant_confirmation_state: PolicyAssistantConfirmationState::ParentConfirmationRequired,
        status: PolicyRequestStatus::PreviewOnly,
        request_id: PolicyRequestId::parse("request-assistant-preview")
            .expect_value("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-assistant-preview-submit")
            .expect_value("policy submission key"),
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
        .expect_value("policy approval id"),
        request_id: request.request_id.clone(),
        household_id: request.household_id.clone(),
        policy_version: request.policy_version,
        actor_id: PolicyActorId::parse("actor-parent").expect_value("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
        decision,
        approved_action: None,
        approved_bonus_minutes: None,
        override_expires_at: None,
        decided_at: timestamp(
            PolicyRequestTimestamp::parse("2026-06-13T20:05:00Z")
                .expect_value("policy request timestamp"),
        ),
        audit_reference_id: audit_ref(
            PolicyAuditReferenceId::parse("audit-parent-decision").expect_value("policy audit ref"),
        ),
    }
}

#[test]
fn request_handoff_dedupes_duplicate_submission_key() {
    let request = child_request();
    let first = register_policy_control_request_handoff(None, request.clone())
        .expect_value("first registration");
    let second = register_policy_control_request_handoff(Some(&first.request), request)
        .expect_value("duplicate registration");

    assert_eq!(second.request, first.request);
    assert_eq!(second.temporary_override, None);
}

#[test]
fn request_handoff_rejects_submission_key_payload_drift() {
    let first = child_request();
    let mut drifted = child_request();
    drifted.expires_at = timestamp(
        PolicyRequestTimestamp::parse("2026-06-13T22:30:00Z")
            .expect_value("policy request timestamp"),
    );

    let error = register_policy_control_request_handoff(Some(&first), drifted)
        .expect_err_value("drifted duplicate must fail");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.submission_key",
            value:
                "duplicate submission key request-bonus-time-submit with different request payload"
                    .to_string(),
        }
    );
}

#[test]
fn assistant_preview_requires_parent_confirmation_before_resolution() {
    let request = assistant_preview_request();
    let approval = approval(&request, PolicyApprovalDecision::Grant);

    let error = resolve_policy_control_request_handoff(&request, approval, None)
        .expect_err_value("preview cannot resolve before confirmation");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.assistant_confirmation_state",
            value: "assistant-preview-only".to_string(),
        }
    );
}

#[test]
fn assistant_preview_confirmation_moves_request_into_parent_review() {
    let preview = register_policy_control_request_handoff(None, assistant_preview_request())
        .expect_value("preview registration");
    let confirmed = confirm_policy_control_request_handoff(
        &preview.request,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-parent").expect_value("actor id"),
            actor_role: ParentPolicyActorRole::Parent,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp(
                PolicyRequestTimestamp::parse("2026-06-13T20:03:00Z")
                    .expect_value("policy request timestamp"),
            ),
            audit_reference_id: audit_ref(
                PolicyAuditReferenceId::parse("audit-assistant-confirmed")
                    .expect_value("policy audit ref"),
            ),
        },
    )
    .expect_value("confirmed preview");

    assert_eq!(
        confirmed.request.status,
        PolicyRequestStatus::PendingParentReview
    );
    assert_eq!(confirmed.request.audit_reference_ids.len(), 2);
    assert_eq!(confirmed.temporary_override, None);
}

#[test]
fn grant_modify_deny_and_expire_resolution_shape_is_correct() {
    let request = child_request();

    let granted = resolve_policy_control_request_handoff(
        &request,
        approval(&request, PolicyApprovalDecision::Grant),
        None,
    )
    .expect_value("grant resolution");
    assert_eq!(granted.request.status, PolicyRequestStatus::Approved);
    assert_eq!(
        granted
            .temporary_override
            .as_ref()
            .map(|override_value| override_value.source_request_id.clone()),
        Some(request.request_id.clone())
    );
    assert_eq!(
        granted
            .temporary_override
            .as_ref()
            .and_then(|override_value| override_value.approved_bonus_minutes)
            .map(|approved_bonus_minutes| approved_bonus_minutes.value()),
        Some(30)
    );
    assert_eq!(
        granted
            .temporary_override
            .as_ref()
            .map(|override_value| override_value.audit_reference_ids.clone()),
        Some(vec![audit_ref(
            PolicyAuditReferenceId::parse("audit-parent-decision").expect_value("policy audit ref"),
        )])
    );

    let modified = resolve_policy_control_request_handoff(
        &request,
        ParentPolicyApproval {
            decision: PolicyApprovalDecision::Modify,
            approved_action: Some(PolicyRuleAction::Allow),
            approved_bonus_minutes: Some(PolicyDurationMinutes::new(45).expect_value("minutes")),
            override_expires_at: Some(timestamp(
                PolicyRequestTimestamp::parse("2026-06-13T21:00:00Z")
                    .expect_value("policy request timestamp"),
            )),
            ..approval(&request, PolicyApprovalDecision::Modify)
        },
        None,
    )
    .expect_value("modify resolution");
    assert_eq!(modified.request.status, PolicyRequestStatus::Modified);
    assert_eq!(
        modified
            .temporary_override
            .as_ref()
            .and_then(|value| value.approved_bonus_minutes)
            .map(|value| value.value()),
        Some(45)
    );

    let denied = resolve_policy_control_request_handoff(
        &request,
        approval(&request, PolicyApprovalDecision::Deny),
        None,
    )
    .expect_value("deny resolution");
    assert_eq!(denied.request.status, PolicyRequestStatus::Denied);
    assert_eq!(denied.temporary_override, None);

    let expired = resolve_policy_control_request_handoff(
        &request,
        approval(&request, PolicyApprovalDecision::Expire),
        None,
    )
    .expect_value("expire resolution");
    assert_eq!(expired.request.status, PolicyRequestStatus::Expired);
    assert_eq!(expired.temporary_override, None);
}

#[test]
fn expired_request_cannot_be_approved() {
    let expired = expire_policy_control_request_handoff(
        &child_request(),
        timestamp(
            PolicyRequestTimestamp::parse("2026-06-13T22:05:00Z")
                .expect_value("policy request timestamp"),
        ),
        audit_ref(
            PolicyAuditReferenceId::parse("audit-request-expired").expect_value("policy audit ref"),
        ),
    )
    .expect_value("request expires");

    let error = resolve_policy_control_request_handoff(
        &expired.request,
        approval(&expired.request, PolicyApprovalDecision::Grant),
        None,
    )
    .expect_err_value("expired request cannot be approved");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.status",
            value: "expired-request-cannot-be-approved".to_string(),
        }
    );
}

#[test]
fn approval_replay_is_override_safe() {
    let request = child_request();
    let parent_approval = approval(&request, PolicyApprovalDecision::Grant);
    let resolved = resolve_policy_control_request_handoff(&request, parent_approval.clone(), None)
        .expect_value("grant resolves handoff");
    let replay = resolve_policy_control_request_handoff(
        &resolved.request,
        parent_approval,
        resolved.temporary_override.as_ref(),
    )
    .expect_value("replay is safe");

    assert_eq!(replay.request, resolved.request);
    assert_eq!(replay.temporary_override, resolved.temporary_override);
}

#[test]
fn observer_and_revoked_parent_are_denied_through_request_handoff() {
    let preview = assistant_preview_request();
    let observer_confirmation_error = confirm_policy_control_request_handoff(
        &preview,
        AssistantPolicyRequestConfirmation {
            actor_id: PolicyActorId::parse("actor-observer").expect_value("actor id"),
            actor_role: ParentPolicyActorRole::Observer,
            actor_state: PolicySourceActorState::Active,
            confirmed_at: timestamp(
                PolicyRequestTimestamp::parse("2026-06-13T20:03:00Z")
                    .expect_value("policy request timestamp"),
            ),
            audit_reference_id: audit_ref(
                PolicyAuditReferenceId::parse("audit-observer-confirm-attempt")
                    .expect_value("policy audit ref"),
            ),
        },
    )
    .expect_err_value("observer cannot confirm preview through handoff");
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
        PolicyActorId::parse("actor-revoked-parent").expect_value("actor id");
    revoked_parent_approval.actor_state = PolicySourceActorState::Revoked;

    let revoked_parent_error =
        resolve_policy_control_request_handoff(&request, revoked_parent_approval, None)
            .expect_err_value("revoked parent cannot approve through handoff");
    assert_eq!(
        revoked_parent_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_state",
            value: "revoked".to_string(),
        }
    );
}
