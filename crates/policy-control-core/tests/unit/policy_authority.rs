use super::TestResult;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_policy_control_core::policy_authority::{
    resolve_policy_conflict, resolve_policy_evaluation_request, AiResultAuthorityState,
    EvidenceReferenceState, ParentAuthorityState, PolicyActionAuthorizationState,
    PolicyConflictDecision, PolicyConflictInput, PolicyConflictResolutionState,
    PolicyConflictState, PolicyControlAggregateId, PolicyControlInput, PolicyControlRequestId,
    PolicyDecisionMode, PolicyDecisionSource, PolicyEnforcementExecutionState,
    PolicyEvaluationRequestedEvent, PolicyManualReviewState,
};

const POLICY_AGGREGATE_ID: &str = "policy-control-household-default";
const POLICY_REQUEST_ID: &str = "policy-control-request-default";
const POLICY_EVALUATION_EVENT_TYPE: &str = "policy-control.evaluation.requested";
const POLICY_DECISION_EVENT_TYPE: &str = "policy-control.decision.resolved";

fn policy_input(ai_result_authority_state: AiResultAuthorityState) -> PolicyControlInput {
    PolicyControlInput {
        mode: PolicyDecisionMode::Enforce,
        parent_authority_state: ParentAuthorityState::Authorized,
        evidence_reference_state: EvidenceReferenceState::Stable,
        ai_result_authority_state,
    }
}

fn conflict_input(requested_source: PolicyDecisionSource) -> PolicyConflictInput {
    PolicyConflictInput {
        parent_authority_state: ParentAuthorityState::Authorized,
        conflict_state: PolicyConflictState::NoConflict,
        requested_source,
        evidence_reference_state: EvidenceReferenceState::Stable,
    }
}

#[test]
fn parent_policy_with_stable_evidence_can_authorize_enforcement() -> TestResult {
    let request = PolicyEvaluationRequestedEvent {
        aggregate_id: test_ok!(
            PolicyControlAggregateId::parse(POLICY_AGGREGATE_ID),
            "policy aggregate id"
        ),
        request_id: test_ok!(
            PolicyControlRequestId::parse(POLICY_REQUEST_ID),
            "policy request id"
        ),
        input: policy_input(AiResultAuthorityState::EvidenceOnly),
        conflict_input: conflict_input(PolicyDecisionSource::ParentPolicy),
    };

    let decision = resolve_policy_evaluation_request(&request);

    assert_eq!(
        decision.decision.action_authorization_state,
        PolicyActionAuthorizationState::Authorized
    );
    assert_eq!(
        decision.decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MayExecute
    );
    assert_eq!(
        decision.conflict_decision.resolution_state,
        PolicyConflictResolutionState::UseParentPolicy
    );
    assert_eq!(
        request
            .contract()
            .map_err(|error| std::io::Error::other(format!("policy evaluation contract: {error}")))?
            .event_type
            .as_str(),
        POLICY_EVALUATION_EVENT_TYPE
    );
    assert_eq!(
        decision
            .contract()
            .map_err(|error| std::io::Error::other(format!("policy decision contract: {error}")))?
            .event_type
            .as_str(),
        POLICY_DECISION_EVENT_TYPE
    );
    Ok(())
}

#[test]
fn ai_result_claiming_authority_blocks_enforcement_and_requires_review() -> TestResult {
    let request = PolicyEvaluationRequestedEvent {
        aggregate_id: test_ok!(
            PolicyControlAggregateId::parse(POLICY_AGGREGATE_ID),
            "policy aggregate id"
        ),
        request_id: test_ok!(
            PolicyControlRequestId::parse(POLICY_REQUEST_ID),
            "policy request id"
        ),
        input: policy_input(AiResultAuthorityState::ClaimsAuthority),
        conflict_input: conflict_input(PolicyDecisionSource::AiEvidence),
    };

    let decision = resolve_policy_evaluation_request(&request);

    assert_eq!(
        decision.decision.action_authorization_state,
        PolicyActionAuthorizationState::Blocked
    );
    assert_eq!(
        decision.decision.enforcement_execution_state,
        PolicyEnforcementExecutionState::MustNotExecute
    );
    assert_eq!(
        decision.decision.manual_review_state,
        PolicyManualReviewState::Required
    );
    assert_eq!(
        decision.conflict_decision.resolution_state,
        PolicyConflictResolutionState::ManualReview
    );
    Ok(())
}

#[test]
fn missing_evidence_reference_forces_manual_review_conflict_resolution() -> TestResult {
    let decision = resolve_policy_conflict(PolicyConflictInput {
        evidence_reference_state: EvidenceReferenceState::Missing,
        ..conflict_input(PolicyDecisionSource::ParentPolicy)
    });

    assert_eq!(
        decision,
        PolicyConflictDecision {
            resolution_state: PolicyConflictResolutionState::ManualReview,
            manual_review_state: PolicyManualReviewState::Required,
        }
    );
    Ok(())
}
