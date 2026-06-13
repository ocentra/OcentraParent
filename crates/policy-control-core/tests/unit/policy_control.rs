use ocentra_policy_control_core::{
    evaluate_policy_control, AiResultAuthorityState, EvidenceReferenceState, ParentAuthorityState,
    PolicyActionAuthorizationState, PolicyControlInput, PolicyDecisionMode,
    PolicyEnforcementExecutionState, PolicyManualReviewState,
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
    assert_eq!(decision.manual_review_state, PolicyManualReviewState::NotRequired);
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
    assert_eq!(decision.manual_review_state, PolicyManualReviewState::Required);
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
    assert_eq!(decision.manual_review_state, PolicyManualReviewState::Required);
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
    assert_eq!(decision.manual_review_state, PolicyManualReviewState::NotRequired);
}
