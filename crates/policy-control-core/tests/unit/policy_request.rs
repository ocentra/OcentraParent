use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_policy_control_core::policy_request::{
    confirm_assistant_policy_request_preview, expire_child_policy_request,
    policy_request_schema_version, register_child_policy_request, resolve_parent_policy_approval,
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, ParentPolicyApproval,
    PolicyApprovalDecision, PolicyApprovalId, PolicyAssistantPreviewId, PolicyDurationMinutes,
    PolicyOverrideState, PolicyRequestId, PolicyRequestKind, PolicyRequestResolution,
    PolicyRequestScope, PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

#[path = "policy_request/boundaries.rs"]
mod boundaries;

const REQUESTED_AT: &str = "2026-06-13T20:00:00Z";
const EXPIRES_AT: &str = "2026-06-13T22:00:00Z";
const MODIFIED_EXPIRES_AT: &str = "2026-06-13T21:00:00Z";

fn timestamp(value: impl std::fmt::Display) -> TestResult<PolicyRequestTimestamp> {
    Ok(test_ok!(
        PolicyRequestTimestamp::parse(value.to_string()),
        "policy request timestamp"
    ))
}

fn audit_ref(value: impl std::fmt::Display) -> TestResult<PolicyAuditReferenceId> {
    Ok(test_ok!(
        PolicyAuditReferenceId::parse(value.to_string()),
        "policy audit ref"
    ))
}

fn request_scope(kind: PolicyRequestKind, minutes: Option<u16>) -> TestResult<PolicyRequestScope> {
    Ok(PolicyRequestScope {
        request_kind: kind,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Category,
            reference_id: test_ok!(
                PolicyTargetReferenceId::parse("category-gaming"),
                "policy target ref"
            ),
        },
        requested_action: PolicyRuleAction::TimeLimit,
        rule_id: Some(test_ok!(
            PolicyRuleId::parse("rule-school-night"),
            "policy rule id"
        )),
        requested_bonus_minutes: test_ok!(
            minutes.map(PolicyDurationMinutes::new).transpose(),
            "policy duration"
        ),
    })
}

fn child_request() -> TestResult<ChildPolicyRequest> {
    Ok(ChildPolicyRequest {
        schema_version: test_ok!(
            policy_request_schema_version(),
            "policy request schema version"
        ),
        request_id: test_ok!(
            PolicyRequestId::parse("request-bonus-time"),
            "policy request id"
        ),
        submission_key: test_ok!(
            PolicyRequestSubmissionKey::parse("request-bonus-time-submit"),
            "policy submission key"
        ),
        household_id: test_ok!(
            PolicyHouseholdId::parse("household-default"),
            "policy household id"
        ),
        child_profile_id: test_ok!(
            PolicyChildProfileId::parse("child-primary"),
            "child profile id"
        ),
        device_id: Some(test_ok!(
            PolicyDeviceId::parse("device-laptop"),
            "policy device id"
        )),
        source_document_id: test_ok!(
            ParentPolicyDocumentId::parse("policy-source-default"),
            "policy source id"
        ),
        policy_version: test_ok!(PolicyVersion::new(7), "policy version"),
        origin: PolicyRequestOrigin::Child,
        assistant_preview_id: None,
        assistant_confirmation_state: PolicyAssistantConfirmationState::NotRequired,
        status: PolicyRequestStatus::PendingParentReview,
        scope: request_scope(PolicyRequestKind::BonusTime, Some(30))?,
        requested_at: timestamp(REQUESTED_AT)?,
        expires_at: timestamp(EXPIRES_AT)?,
        audit_reference_ids: vec![audit_ref("audit-request-created")?],
        resolved_approval_id: None,
        resolved_at: None,
    })
}

fn assistant_preview_request() -> TestResult<ChildPolicyRequest> {
    Ok(ChildPolicyRequest {
        origin: PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: Some(test_ok!(
            PolicyAssistantPreviewId::parse("assistant-preview-default"),
            "assistant preview id"
        )),
        assistant_confirmation_state: PolicyAssistantConfirmationState::ParentConfirmationRequired,
        status: PolicyRequestStatus::PreviewOnly,
        request_id: test_ok!(
            PolicyRequestId::parse("request-assistant-preview"),
            "policy request id"
        ),
        submission_key: test_ok!(
            PolicyRequestSubmissionKey::parse("request-assistant-preview-submit"),
            "policy submission key"
        ),
        scope: request_scope(PolicyRequestKind::AskParent, None)?,
        ..child_request()?
    })
}

fn approval(
    request: &ChildPolicyRequest,
    decision: PolicyApprovalDecision,
) -> TestResult<ParentPolicyApproval> {
    Ok(ParentPolicyApproval {
        approval_id: test_ok!(
            PolicyApprovalId::parse(format!(
                "{}-{}",
                request.request_id.as_str(),
                match decision {
                    PolicyApprovalDecision::Grant => "grant",
                    PolicyApprovalDecision::Deny => "deny",
                    PolicyApprovalDecision::Modify => "modify",
                    PolicyApprovalDecision::Expire => "expire",
                }
            )),
            "policy approval id"
        ),
        request_id: request.request_id.clone(),
        household_id: request.household_id.clone(),
        policy_version: request.policy_version,
        actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
        decision,
        approved_action: None,
        approved_bonus_minutes: None,
        override_expires_at: None,
        decided_at: timestamp("2026-06-13T20:05:00Z")?,
        audit_reference_id: audit_ref("audit-parent-decision")?,
    })
}

fn assert_active_override_minutes(
    resolution: &PolicyRequestResolution,
    expected_minutes: u16,
    expected_expires_at: impl std::fmt::Display,
) -> TestResult {
    let expected_expires_at = expected_expires_at.to_string();
    let temporary_override =
        test_some!(resolution.temporary_override.as_ref(), "temporary override");

    assert_eq!(
        resolution.request.resolved_approval_id.as_ref(),
        Some(&temporary_override.source_approval_id)
    );
    assert_eq!(temporary_override.state, PolicyOverrideState::Active);
    assert_eq!(
        test_some!(
            temporary_override.approved_bonus_minutes,
            "approved bonus minutes"
        )
        .value(),
        expected_minutes
    );
    assert_eq!(temporary_override.expires_at.as_str(), expected_expires_at);
    Ok(())
}

#[test]
fn double_submit_is_idempotent_and_grant_replay_is_override_safe() -> TestResult {
    let registered = test_ok!(
        register_child_policy_request(None, child_request()?),
        "registered child policy request"
    );

    let duplicate = ChildPolicyRequest {
        request_id: test_ok!(
            PolicyRequestId::parse("request-bonus-time-duplicate"),
            "request id"
        ),
        ..registered.clone()
    };
    let deduped = test_ok!(
        register_child_policy_request(Some(&registered), duplicate),
        "duplicate submit returns existing request"
    );
    assert_eq!(deduped, registered);

    let parent_approval = approval(&registered, PolicyApprovalDecision::Grant)?;
    let resolved = test_ok!(
        resolve_parent_policy_approval(&registered, parent_approval.clone(), None),
        "grant resolves child request"
    );
    assert_eq!(resolved.request.status, PolicyRequestStatus::Approved);
    assert_active_override_minutes(&resolved, 30, EXPIRES_AT)?;

    let replay = test_ok!(
        resolve_parent_policy_approval(
            &resolved.request,
            parent_approval,
            resolved.temporary_override.as_ref(),
        ),
        "same approval id is replay safe"
    );
    assert_eq!(replay, resolved);
    Ok(())
}

#[test]
fn replay_with_changed_decision_is_rejected() -> TestResult {
    let request = child_request()?;
    let granted_approval = approval(&request, PolicyApprovalDecision::Grant)?;
    let resolved = test_ok!(
        resolve_parent_policy_approval(&request, granted_approval.clone(), None),
        "grant resolves child request"
    );

    let mut stale_replay = approval(&resolved.request, PolicyApprovalDecision::Deny)?;
    stale_replay.approval_id = granted_approval.approval_id;

    let error = test_err!(
        resolve_parent_policy_approval(
            &resolved.request,
            stale_replay,
            resolved.temporary_override.as_ref(),
        ),
        "reused approval id with changed decision must be rejected"
    );

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.approval_id",
            value: "request-bonus-time-grant".to_string(),
        }
    );
    Ok(())
}

#[test]
fn replay_rejected_status_is_rejected_for_core_request_registration() -> TestResult {
    let mut request = child_request()?;
    request.status = PolicyRequestStatus::ReplayRejected;

    let error = test_err!(
        register_child_policy_request(None, request),
        "replay-rejected is not a core request status"
    );

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.status",
            value: "replay-rejected".to_string(),
        }
    );
    Ok(())
}

#[test]
fn duplicate_submission_key_with_changed_payload_is_rejected() -> TestResult {
    let registered = test_ok!(
        register_child_policy_request(None, child_request()?),
        "registered child policy request"
    );
    let mut changed = registered.clone();
    changed.request_id = test_ok!(
        PolicyRequestId::parse("request-bonus-time-changed"),
        "request id"
    );
    changed.scope.requested_bonus_minutes =
        Some(test_ok!(PolicyDurationMinutes::new(45), "minutes"));

    let error = test_err!(
        register_child_policy_request(Some(&registered), changed),
        "changed payload cannot reuse submission key"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.submission_key",
            value:
                "duplicate submission key request-bonus-time-submit with different request payload"
                    .to_string(),
        }
    );
    Ok(())
}

#[test]
fn parent_can_deny_request_without_creating_override() -> TestResult {
    let request = ChildPolicyRequest {
        scope: request_scope(PolicyRequestKind::AskParent, None)?,
        ..child_request()?
    };
    let resolved = test_ok!(
        resolve_parent_policy_approval(
            &request,
            approval(&request, PolicyApprovalDecision::Deny)?,
            None,
        ),
        "deny resolves request"
    );

    assert_eq!(resolved.request.status, PolicyRequestStatus::Denied);
    assert!(resolved.temporary_override.is_none());
    assert_eq!(
        test_some!(
            resolved.request.resolved_approval_id.as_ref(),
            "resolved approval id"
        )
        .as_str(),
        "request-bonus-time-deny"
    );
    Ok(())
}

#[test]
fn parent_can_modify_bonus_time_request() -> TestResult {
    let request = child_request()?;
    let mut parent_approval = approval(&request, PolicyApprovalDecision::Modify)?;
    parent_approval.approved_bonus_minutes =
        Some(test_ok!(PolicyDurationMinutes::new(15), "minutes"));
    parent_approval.override_expires_at = Some(timestamp(MODIFIED_EXPIRES_AT)?);

    let resolved = test_ok!(
        resolve_parent_policy_approval(&request, parent_approval, None),
        "modify resolves child request"
    );

    assert_eq!(resolved.request.status, PolicyRequestStatus::Modified);
    assert_active_override_minutes(&resolved, 15, MODIFIED_EXPIRES_AT)?;
    Ok(())
}

#[test]
fn parent_can_expire_pending_request_without_creating_override() -> TestResult {
    let request = child_request()?;
    let resolved = test_ok!(
        resolve_parent_policy_approval(
            &request,
            approval(&request, PolicyApprovalDecision::Expire)?,
            None,
        ),
        "parent can explicitly expire request"
    );

    assert_eq!(resolved.request.status, PolicyRequestStatus::Expired);
    assert!(resolved.temporary_override.is_none());
    assert_eq!(
        test_some!(
            resolved.request.resolved_approval_id.as_ref(),
            "resolved approval id"
        )
        .as_str(),
        "request-bonus-time-expire"
    );
    Ok(())
}

#[test]
fn expired_request_cannot_be_approved() -> TestResult {
    let request = child_request()?;
    let expired_request = test_ok!(
        expire_child_policy_request(
            &request,
            timestamp("2026-06-13T22:05:00Z")?,
            audit_ref("audit-request-expired")?,
        ),
        "request expires"
    );

    let error = test_err!(
        resolve_parent_policy_approval(
            &expired_request,
            approval(&expired_request, PolicyApprovalDecision::Grant)?,
            None,
        ),
        "expired request must not become active override"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.status",
            value: "expired-request-cannot-be-approved".to_string(),
        }
    );
    Ok(())
}

#[test]
fn assistant_draft_stays_preview_only_until_parent_confirms() -> TestResult {
    let preview = test_ok!(
        register_child_policy_request(None, assistant_preview_request()?),
        "registered assistant preview request"
    );

    let preview_error = test_err!(
        resolve_parent_policy_approval(
            &preview,
            approval(&preview, PolicyApprovalDecision::Grant)?,
            None,
        ),
        "preview-only assistant request cannot resolve"
    );
    assert_eq!(
        preview_error,
        EventingError::InvalidValue {
            field: "policy_request.assistant_confirmation_state",
            value: "assistant-preview-only".to_string(),
        }
    );

    let confirmed = test_ok!(
        confirm_assistant_policy_request_preview(
            &preview,
            AssistantPolicyRequestConfirmation {
                actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "actor id"),
                actor_role: ParentPolicyActorRole::Parent,
                actor_state: PolicySourceActorState::Active,
                confirmed_at: timestamp("2026-06-13T20:03:00Z")?,
                audit_reference_id: audit_ref("audit-assistant-confirmed")?,
            },
        ),
        "parent confirms assistant preview"
    );
    assert_eq!(confirmed.status, PolicyRequestStatus::PendingParentReview);
    assert_eq!(
        confirmed.assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmed
    );

    let resolved = test_ok!(
        resolve_parent_policy_approval(
            &confirmed,
            approval(&confirmed, PolicyApprovalDecision::Grant)?,
            None,
        ),
        "confirmed assistant preview can resolve"
    );
    assert_eq!(resolved.request.status, PolicyRequestStatus::Approved);
    Ok(())
}

#[test]
fn child_and_support_roles_cannot_confirm_or_self_approve() -> TestResult {
    let preview = assistant_preview_request()?;
    let child_confirmation_error = test_err!(
        confirm_assistant_policy_request_preview(
            &preview,
            AssistantPolicyRequestConfirmation {
                actor_id: test_ok!(PolicyActorId::parse("actor-child"), "actor id"),
                actor_role: ParentPolicyActorRole::Child,
                actor_state: PolicySourceActorState::Active,
                confirmed_at: timestamp("2026-06-13T20:03:00Z")?,
                audit_reference_id: audit_ref("audit-child-confirm-attempt")?,
            },
        ),
        "child cannot confirm assistant preview"
    );
    assert_eq!(
        child_confirmation_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_role",
            value: "child".to_string(),
        }
    );

    let child_request = child_request()?;
    let mut support_approval = approval(&child_request, PolicyApprovalDecision::Grant)?;
    support_approval.actor_id = test_ok!(PolicyActorId::parse("actor-support"), "actor id");
    support_approval.actor_role = ParentPolicyActorRole::Support;

    let support_error = test_err!(
        resolve_parent_policy_approval(&child_request, support_approval, None),
        "support cannot approve child request"
    );
    assert_eq!(
        support_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_role",
            value: "support".to_string(),
        }
    );
    Ok(())
}

#[test]
fn observer_and_revoked_parent_cannot_confirm_or_approve() -> TestResult {
    let preview = assistant_preview_request()?;
    let observer_confirmation_error = test_err!(
        confirm_assistant_policy_request_preview(
            &preview,
            AssistantPolicyRequestConfirmation {
                actor_id: test_ok!(PolicyActorId::parse("actor-observer"), "actor id"),
                actor_role: ParentPolicyActorRole::Observer,
                actor_state: PolicySourceActorState::Active,
                confirmed_at: timestamp("2026-06-13T20:03:00Z")?,
                audit_reference_id: audit_ref("audit-observer-confirm-attempt")?,
            },
        ),
        "observer cannot confirm assistant preview"
    );
    assert_eq!(
        observer_confirmation_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_role",
            value: "observer".to_string(),
        }
    );

    let child_request = child_request()?;
    let mut revoked_parent_approval = approval(&child_request, PolicyApprovalDecision::Grant)?;
    revoked_parent_approval.actor_id =
        test_ok!(PolicyActorId::parse("actor-revoked-parent"), "actor id");
    revoked_parent_approval.actor_state = PolicySourceActorState::Revoked;

    let revoked_parent_error = test_err!(
        resolve_parent_policy_approval(&child_request, revoked_parent_approval, None),
        "revoked parent cannot approve child request"
    );
    assert_eq!(
        revoked_parent_error,
        EventingError::InvalidValue {
            field: "policy_request.actor_state",
            value: "revoked".to_string(),
        }
    );
    Ok(())
}

#[test]
fn wrong_parent_household_cannot_approve_request() -> TestResult {
    let request = child_request()?;
    let mut wrong_parent_approval = approval(&request, PolicyApprovalDecision::Grant)?;
    wrong_parent_approval.actor_id =
        test_ok!(PolicyActorId::parse("actor-other-parent"), "actor id");
    wrong_parent_approval.household_id = test_ok!(
        PolicyHouseholdId::parse("household-other"),
        "policy household id"
    );

    let error = test_err!(
        resolve_parent_policy_approval(&request, wrong_parent_approval, None),
        "wrong household parent cannot approve request"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "policy_request.household_id",
            value: "household-other".to_string(),
        }
    );
    Ok(())
}
