use super::*;

pub(super) fn normalize_action_result_input(
    input: &NetworkActionResultInput,
) -> Result<NormalizedActionResultInput, NetworkActionResultError> {
    Ok(NormalizedActionResultInput {
        action_result_ref: refs::normalize_ref(&input.action_result_ref)
            .ok_or(NetworkActionResultError::EmptyActionResultRef)?,
        policy_decision_ref: refs::normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkActionResultError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: refs::normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkActionResultError::EmptyParentRuleRef)?,
        evidence_refs: refs::normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: refs::normalized_optional_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
            NetworkActionResultError::EmptyLocalAiResultRef,
        )?,
        target_ref: refs::normalize_ref(&input.target_ref)
            .ok_or(NetworkActionResultError::EmptyTargetRef)?,
    })
}
