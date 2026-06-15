use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, resolve_parent_policy_approval, ChildPolicyRequest,
    ParentPolicyApproval, PolicyApprovalDecision, PolicyApprovalId,
    PolicyAssistantConfirmationState, PolicyDurationMinutes, PolicyRequestId, PolicyRequestKind,
    PolicyRequestOrigin, PolicyRequestScope, PolicyRequestStatus, PolicyRequestSubmissionKey,
    PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

fn timestamp(value: &str) -> PolicyRequestTimestamp {
    PolicyRequestTimestamp::parse(value).expect("policy request timestamp")
}

fn request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        schema_version: policy_request_schema_version().expect("policy request schema version"),
        request_id: PolicyRequestId::parse("request-version-skew").expect("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-version-skew-submit")
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
        scope: PolicyRequestScope {
            request_kind: PolicyRequestKind::BonusTime,
            target: PolicyRequestTarget {
                kind: PolicyTargetKind::Category,
                reference_id: PolicyTargetReferenceId::parse("category-gaming")
                    .expect("policy target ref"),
            },
            requested_action: PolicyRuleAction::TimeLimit,
            rule_id: Some(PolicyRuleId::parse("rule-school-night").expect("policy rule id")),
            requested_bonus_minutes: Some(PolicyDurationMinutes::new(30).expect("minutes")),
        },
        requested_at: timestamp("2026-06-13T20:00:00Z"),
        expires_at: timestamp("2026-06-13T22:00:00Z"),
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-request-created").expect("policy audit ref")
        ],
        resolved_approval_id: None,
        resolved_at: None,
    }
}

fn approval(request: &ChildPolicyRequest) -> ParentPolicyApproval {
    ParentPolicyApproval {
        approval_id: PolicyApprovalId::parse("request-version-skew-grant")
            .expect("policy approval id"),
        request_id: request.request_id.clone(),
        household_id: request.household_id.clone(),
        policy_version: request.policy_version,
        actor_id: PolicyActorId::parse("actor-parent").expect("policy actor id"),
        actor_role: ParentPolicyActorRole::Parent,
        actor_state: PolicySourceActorState::Active,
        decision: PolicyApprovalDecision::Grant,
        approved_action: None,
        approved_bonus_minutes: None,
        override_expires_at: None,
        decided_at: timestamp("2026-06-13T20:05:00Z"),
        audit_reference_id: PolicyAuditReferenceId::parse("audit-parent-decision")
            .expect("policy audit ref"),
    }
}

#[test]
fn policy_request_serde_rejects_zero_schema_version() {
    let error = serde_json::from_str::<ChildPolicyRequest>(
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
    )
    .expect_err("policy request schema version zero must be rejected");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn approval_with_stale_policy_version_is_rejected() {
    let request = request();
    let mut stale_approval = approval(&request);
    stale_approval.policy_version = PolicyVersion::new(6).expect("policy version");

    let error = resolve_parent_policy_approval(&request, stale_approval, None)
        .expect_err("stale approval must not resolve request");
    assert!(error.to_string().contains("policy_request.policy_version"));
}
