use super::TestResult;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, resolve_parent_policy_approval, ChildPolicyRequest,
    ParentPolicyApproval, PolicyApprovalDecision, PolicyApprovalId, PolicyDurationMinutes,
    PolicyRequestId, PolicyRequestKind, PolicyRequestScope, PolicyRequestSubmissionKey,
    PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

fn timestamp(value: impl std::fmt::Display) -> TestResult<PolicyRequestTimestamp> {
    Ok(test_ok!(
        PolicyRequestTimestamp::parse(value.to_string()),
        "policy request timestamp"
    ))
}

fn request() -> TestResult<ChildPolicyRequest> {
    Ok(ChildPolicyRequest {
        schema_version: test_ok!(
            policy_request_schema_version(),
            "policy request schema version"
        ),
        request_id: test_ok!(
            PolicyRequestId::parse("request-version-skew"),
            "policy request id"
        ),
        submission_key: test_ok!(
            PolicyRequestSubmissionKey::parse("request-version-skew-submit"),
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
        scope: PolicyRequestScope {
            request_kind: PolicyRequestKind::BonusTime,
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
            requested_bonus_minutes: Some(test_ok!(PolicyDurationMinutes::new(30), "minutes")),
        },
        requested_at: timestamp("2026-06-13T20:00:00Z")?,
        expires_at: timestamp("2026-06-13T22:00:00Z")?,
        audit_reference_ids: vec![test_ok!(
            PolicyAuditReferenceId::parse("audit-request-created"),
            "policy audit ref"
        )],
        resolved_approval_id: None,
        resolved_at: None,
    })
}

fn approval(request: &ChildPolicyRequest) -> TestResult<ParentPolicyApproval> {
    Ok(ParentPolicyApproval {
        approval_id: test_ok!(
            PolicyApprovalId::parse("request-version-skew-grant"),
            "policy approval id"
        ),
        request_id: request.request_id.clone(),
        household_id: request.household_id.clone(),
        policy_version: request.policy_version,
        actor_id: test_ok!(PolicyActorId::parse("actor-parent"), "policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
        decision: PolicyApprovalDecision::Grant,
        approved_action: None,
        approved_bonus_minutes: None,
        override_expires_at: None,
        decided_at: timestamp("2026-06-13T20:05:00Z")?,
        audit_reference_id: test_ok!(
            PolicyAuditReferenceId::parse("audit-parent-decision"),
            "policy audit ref"
        ),
    })
}

#[test]
fn policy_request_serde_rejects_zero_schema_version() -> TestResult {
    let error = test_err!(
        serde_json::from_str::<ChildPolicyRequest>(
            r#"{
            "schema_version": 0,
            "request_id": "request-version-skew",
            "submission_key": "request-version-skew-submit",
            "household_id": "household-default",
            "child_profile_id": "child-primary",
            "device_id": "device-laptop",
            "source_document_id": "policy-source-default",
            "policy_version": 7,
            "origin": "child",
            "assistant_preview_id": null,
            "assistant_confirmation_state": "not-required",
            "status": "pending-parent-review",
            "scope": {
                "request_kind": "bonus-time",
                "target": {
                    "kind": "category",
                    "reference_id": "category-gaming"
                },
                "requested_action": "time-limit",
                "rule_id": "rule-school-night",
                "requested_bonus_minutes": 30
            },
            "requested_at": "2026-06-13T20:00:00Z",
            "expires_at": "2026-06-13T22:00:00Z",
            "audit_reference_ids": ["audit-request-created"],
            "resolved_approval_id": null,
            "resolved_at": null
        }"#,
        ),
        "policy request schema version zero must be rejected"
    );

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn approval_with_stale_policy_version_is_rejected() -> TestResult {
    let request = request()?;
    let mut stale_approval = approval(&request)?;
    stale_approval.policy_version = test_ok!(PolicyVersion::new(6), "policy version");

    let error = test_err!(
        resolve_parent_policy_approval(&request, stale_approval, None),
        "stale approval must not resolve request"
    );
    assert!(error.to_string().contains("policy_request.policy_version"));
    Ok(())
}
