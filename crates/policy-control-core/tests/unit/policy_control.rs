use super::TestResult;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_policy_control_core::policy_authority::{
    evaluate_policy_control, resolve_policy_conflict, resolve_policy_evaluation_request,
    AiResultAuthorityState, EvidenceReferenceState, ParentAuthorityState,
    PolicyActionAuthorizationState, PolicyConflictInput, PolicyConflictResolutionState,
    PolicyConflictState, PolicyControlAggregateId, PolicyControlInput, PolicyControlRequestId,
    PolicyDecisionMode, PolicyDecisionResolvedEvent, PolicyDecisionSource,
    PolicyEnforcementExecutionState, PolicyEvaluationRequestedEvent, PolicyManualReviewState,
};

#[test]
fn enforce_mode_allows_adapter_only_after_policy_authority_boundary() {
    let decision = evaluate_policy_control(PolicyControlInput {
        mode: PolicyDecisionMode::Enforce,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    });

    assert_eq!(
        decision.action_authorization_state,
        PolicyActionAuthorizationState::Authorized
    );
    assert_eq!(
        decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MayExecute
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
}

#[test]
fn ai_authority_claim_forces_manual_review() {
    let decision = evaluate_policy_control(PolicyControlInput {
        mode: PolicyDecisionMode::Enforce,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state: AiResultAuthorityState::ClaimsAuthority,
    });

    assert_eq!(
        decision.action_authorization_state,
        PolicyActionAuthorizationState::Blocked
    );
    assert_eq!(
        decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MustNotExecute
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::Required
    );
}

#[test]
fn preview_mode_does_not_execute_enforcement_adapter() {
    let decision = evaluate_policy_control(PolicyControlInput {
        mode: PolicyDecisionMode::Preview,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    });

    assert_eq!(
        decision.action_authorization_state,
        PolicyActionAuthorizationState::Authorized
    );
    assert_eq!(
        decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MustNotExecute
    );
}

#[test]
fn missing_evidence_blocks_policy_authority_even_with_parent_authority() {
    let decision = evaluate_policy_control(PolicyControlInput {
        mode: PolicyDecisionMode::Enforce,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Missing,
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    });

    assert_eq!(
        decision.action_authorization_state,
        PolicyActionAuthorizationState::Blocked
    );
    assert_eq!(
        decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MustNotExecute
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::Required
    );
}

#[test]
fn observe_only_mode_can_authorize_policy_record_without_adapter_execution() {
    let decision = evaluate_policy_control(PolicyControlInput {
        mode: PolicyDecisionMode::ObserveOnly,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    });

    assert_eq!(
        decision.action_authorization_state,
        PolicyActionAuthorizationState::Authorized
    );
    assert_eq!(
        decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MustNotExecute
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
}

#[test]
fn conflict_resolution_uses_parent_policy_only_with_authority_and_stable_evidence() {
    let decision = resolve_policy_conflict(PolicyConflictInput {
        parent_authority_state: ParentAuthorityState::Authorized,
        conflict_state: PolicyConflictState::NoConflict,
        requested_source: PolicyDecisionSource::ParentPolicy,
        evidence_reference_state: EvidenceReferenceState::Stable,
    });

    assert_eq!(
        decision.resolution_state,
        PolicyConflictResolutionState::UseParentPolicy
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::NotRequired
    );
}

#[test]
fn ai_evidence_never_becomes_policy_authority_during_conflict_resolution() {
    let decision = resolve_policy_conflict(PolicyConflictInput {
        parent_authority_state: ParentAuthorityState::Authorized,
        conflict_state: PolicyConflictState::NoConflict,
        requested_source: PolicyDecisionSource::AiEvidence,
        evidence_reference_state: EvidenceReferenceState::Stable,
    });

    assert_eq!(
        decision.resolution_state,
        PolicyConflictResolutionState::ManualReview
    );
    assert_eq!(
        decision.manual_review_state,
        PolicyManualReviewState::Required
    );
}

#[test]
fn policy_evaluation_request_resolves_to_typed_decision_event() -> TestResult {
    let request = PolicyEvaluationRequestedEvent {
        aggregate_id: test_ok!(
            PolicyControlAggregateId::parse("policy-control-family-default"),
            "policy control aggregate"
        ),
        request_id: test_ok!(
            PolicyControlRequestId::parse("policy-control-request-default"),
            "policy control request"
        ),
        input: PolicyControlInput {
            mode: PolicyDecisionMode::Enforce,
            parent_authority_state: ParentAuthorityState::Authorized,
            evidence_reference_state: EvidenceReferenceState::Stable,
            ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
        },
        conflict_input: PolicyConflictInput {
            parent_authority_state: ParentAuthorityState::Authorized,
            conflict_state: PolicyConflictState::NoConflict,
            requested_source: PolicyDecisionSource::ParentPolicy,
            evidence_reference_state: EvidenceReferenceState::Stable,
        },
    };

    let decision: PolicyDecisionResolvedEvent = resolve_policy_evaluation_request(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_request_id, request.request_id);
    assert_eq!(
        decision.decision.action_authorization_state,
        PolicyActionAuthorizationState::Authorized
    );
    assert_eq!(
        decision.conflict_decision.resolution_state,
        PolicyConflictResolutionState::UseParentPolicy
    );
    assert_eq!(
        request
            .contract()
            .map_err(|error| std::io::Error::other(format!("policy request contract: {error}")))?
            .event_type
            .as_str(),
        "policy-control.evaluation.requested"
    );
    assert_eq!(
        decision
            .contract()
            .map_err(|error| std::io::Error::other(format!("policy decision contract: {error}")))?
            .event_type
            .as_str(),
        "policy-control.decision.resolved"
    );
    Ok(())
}
