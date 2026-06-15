use ocentra_eventing::EventingError;
use ocentra_parent_agent_protocol::{
    child_domain_child_device_id, child_domain_child_profile_id, child_domain_evidence_ref,
    child_domain_observed_at, child_domain_policy_violation_id_from_policy_request_id,
    ChildDomainEventType, ChildDomainPolicyRuleRef, ChildDomainPolicySeverity,
    ChildDomainPolicyViolationDetectedEvent, ChildDomainRefSuffix, ChildRuntimeDomain,
};
use ocentra_policy_control_core::policy_request::{
    PolicyAssistantConfirmationState, PolicyAssistantPreviewId, PolicyDurationMinutes,
    PolicyRequestKind, PolicyRequestOrigin, PolicyRequestStatus, PolicyRequestTarget,
    PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyHouseholdId, PolicyRuleAction,
    PolicyRuleId, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

use ocentra_child_policy_core::policy_control_request::{
    build_policy_control_request_from_child_violation, ChildPolicyControlRequestInput,
};

#[test]
fn child_origin_violation_becomes_pending_parent_review_request() {
    let request = build_policy_control_request_from_child_violation(
        &violation(),
        ChildPolicyControlRequestInput {
            household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
            source_document_id: ParentPolicyDocumentId::parse("policy-source-browser-default")
                .expect("policy source document id"),
            policy_version: PolicyVersion::new(3).expect("policy version"),
            request_kind: PolicyRequestKind::AskParent,
            target: PolicyRequestTarget {
                kind: PolicyTargetKind::Site,
                reference_id: PolicyTargetReferenceId::parse("site:example-video")
                    .expect("policy target ref"),
            },
            requested_action: PolicyRuleAction::AskParent,
            rule_id: Some(
                PolicyRuleId::parse("rule-browser-video-default").expect("policy rule id"),
            ),
            requested_bonus_minutes: None,
            origin: PolicyRequestOrigin::Child,
            assistant_preview_id: None,
            expires_at: PolicyRequestTimestamp::parse("2026-06-13T22:30:00Z")
                .expect("policy request expiry"),
            audit_reference_ids: vec![
                PolicyAuditReferenceId::parse("audit-request-browser-video")
                    .expect("policy audit ref"),
                PolicyAuditReferenceId::parse("audit-request-browser-video")
                    .expect("policy audit ref"),
            ],
        },
    )
    .expect("child-origin request");

    assert_eq!(request.status, PolicyRequestStatus::PendingParentReview);
    assert_eq!(request.origin, PolicyRequestOrigin::Child);
    assert_eq!(
        request.assistant_confirmation_state,
        PolicyAssistantConfirmationState::NotRequired
    );
    assert_eq!(
        request.requested_at.as_str(),
        violation().detected_at.as_str()
    );
    assert_eq!(
        request.device_id.as_ref().map(|value| value.as_str()),
        Some("child-device-default")
    );
    assert_eq!(request.audit_reference_ids.len(), 1);
    assert!(request
        .request_id
        .as_str()
        .contains(violation().violation_id.as_str()));
    assert!(request.submission_key.as_str().contains("child"));
}

#[test]
fn assistant_draft_violation_becomes_preview_only_bonus_time_request() {
    let request = build_policy_control_request_from_child_violation(
        &violation(),
        ChildPolicyControlRequestInput {
            household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
            source_document_id: ParentPolicyDocumentId::parse("policy-source-browser-default")
                .expect("policy source document id"),
            policy_version: PolicyVersion::new(4).expect("policy version"),
            request_kind: PolicyRequestKind::BonusTime,
            target: PolicyRequestTarget {
                kind: PolicyTargetKind::App,
                reference_id: PolicyTargetReferenceId::parse("app:video-player")
                    .expect("policy target ref"),
            },
            requested_action: PolicyRuleAction::Allow,
            rule_id: Some(PolicyRuleId::parse("rule-bonus-time-default").expect("policy rule id")),
            requested_bonus_minutes: Some(PolicyDurationMinutes::new(15).expect("bonus minutes")),
            origin: PolicyRequestOrigin::AssistantDraft,
            assistant_preview_id: Some(
                PolicyAssistantPreviewId::parse("assistant-preview-browser-video")
                    .expect("assistant preview id"),
            ),
            expires_at: PolicyRequestTimestamp::parse("2026-06-13T22:45:00Z")
                .expect("policy request expiry"),
            audit_reference_ids: vec![PolicyAuditReferenceId::parse("audit-preview-browser-video")
                .expect("policy audit ref")],
        },
    )
    .expect("assistant preview request");

    assert_eq!(request.status, PolicyRequestStatus::PreviewOnly);
    assert_eq!(request.origin, PolicyRequestOrigin::AssistantDraft);
    assert_eq!(
        request.assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmationRequired
    );
    assert_eq!(
        request
            .assistant_preview_id
            .as_ref()
            .map(|value| value.as_str()),
        Some("assistant-preview-browser-video")
    );
    assert_eq!(request.scope.request_kind, PolicyRequestKind::BonusTime);
    assert_eq!(
        request
            .scope
            .requested_bonus_minutes
            .map(|value| value.value()),
        Some(15)
    );
    assert!(request.submission_key.as_str().contains("assistant-draft"));
}

#[test]
fn policy_control_request_bridge_rejects_wrong_event_shape() {
    let mut wrong_event = violation();
    wrong_event.event_type = ChildRuntimeDomain::Browser.policy_evaluation_requested_event_type();

    let wrong_event_error =
        build_policy_control_request_from_child_violation(&wrong_event, child_origin_input())
            .expect_err("must reject wrong event type");
    assert_eq!(
        wrong_event_error,
        EventingError::InvalidValue {
            field: "policy_control_request.event_type",
            value: String::from("browser.policy.evaluation.requested"),
        }
    );

    let mut missing_evidence = violation();
    missing_evidence.evidence_refs.clear();
    let missing_evidence_error =
        build_policy_control_request_from_child_violation(&missing_evidence, child_origin_input())
            .expect_err("must reject missing evidence");
    assert_eq!(
        missing_evidence_error,
        EventingError::InvalidValue {
            field: "policy_control_request.evidence_refs",
            value: String::from("empty"),
        }
    );
}

fn child_origin_input() -> ChildPolicyControlRequestInput {
    ChildPolicyControlRequestInput {
        household_id: PolicyHouseholdId::parse("household-default").expect("household id"),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-browser-default")
            .expect("policy source document id"),
        policy_version: PolicyVersion::new(3).expect("policy version"),
        request_kind: PolicyRequestKind::AskParent,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Site,
            reference_id: PolicyTargetReferenceId::parse("site:example-video")
                .expect("policy target ref"),
        },
        requested_action: PolicyRuleAction::AskParent,
        rule_id: Some(PolicyRuleId::parse("rule-browser-video-default").expect("policy rule id")),
        requested_bonus_minutes: None,
        origin: PolicyRequestOrigin::Child,
        assistant_preview_id: None,
        expires_at: PolicyRequestTimestamp::parse("2026-06-13T22:30:00Z")
            .expect("policy request expiry"),
        audit_reference_ids: vec![
            PolicyAuditReferenceId::parse("audit-request-browser-video").expect("policy audit ref")
        ],
    }
}

fn violation() -> ChildDomainPolicyViolationDetectedEvent {
    let request_id = ocentra_parent_agent_protocol::child_domain_policy_request_id(
        ChildRuntimeDomain::Browser,
        ChildDomainRefSuffix::DefaultPolicyRequest,
    );

    ChildDomainPolicyViolationDetectedEvent {
        event_type: ChildDomainEventType::policy_violation_detected(),
        domain: ChildRuntimeDomain::Browser,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        violation_id: child_domain_policy_violation_id_from_policy_request_id(&request_id),
        policy_rule_ref: ChildDomainPolicyRuleRef::parse("child-domain.policy.default")
            .expect("policy rule ref"),
        severity: ChildDomainPolicySeverity::parse("review").expect("policy severity"),
        detected_at: child_domain_observed_at(),
        evidence_refs: vec![child_domain_evidence_ref(
            ChildRuntimeDomain::Browser,
            ChildDomainRefSuffix::DefaultEvidence,
        )],
    }
}
