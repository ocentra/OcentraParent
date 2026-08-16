#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::{
    action::{
        compare_policy_action_strictness, select_stricter_policy_action, PolicyContractAction,
    },
    preview::{PolicyContractDecision, PolicyContractDecisionHandoffState},
};

pub struct PolicyContractScreenAiStricterParentRuleInput {
    pub source_decision: PolicyContractDecision,
    pub stricter_parent_rule_enabled: bool,
    pub stricter_parent_rule_action: PolicyContractAction,
    pub expected_final_action: PolicyContractAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScreenAiStricterParentRuleProof {
    pub final_action: PolicyContractAction,
    pub stricter_parent_rule_action: PolicyContractAction,
    pub final_decision: PolicyContractDecision,
    pub source_decision: PolicyContractDecision,
    pub stricter_parent_rule_id: String,
    pub all_claim_boundaries_false: bool,
}

pub fn screen_ai_stricter_parent_rule_input_is_ready(
    input: &PolicyContractScreenAiStricterParentRuleInput,
) -> bool {
    input.source_decision.dry_run
        && input.source_decision.enforcement_handoff_state
            != PolicyContractDecisionHandoffState::HandedOff
        && input.source_decision.local_ai_result_id.is_some()
        && input.stricter_parent_rule_enabled
        && compare_policy_action_strictness(
            input.stricter_parent_rule_action,
            input.source_decision.action,
        ) > 0
        && input.expected_final_action
            == select_stricter_policy_action(
                input.stricter_parent_rule_action,
                input.source_decision.action,
            )
}

pub fn screen_ai_stricter_parent_rule_proof_is_honest(
    proof: &PolicyContractScreenAiStricterParentRuleProof,
) -> bool {
    proof.final_action == proof.stricter_parent_rule_action
        && proof.final_decision.action == proof.stricter_parent_rule_action
        && proof.final_decision.local_ai_result_id == proof.source_decision.local_ai_result_id
        && proof.final_decision.evidence_reference_count
            == proof.source_decision.evidence_reference_count
        && proof
            .final_decision
            .rule_ids
            .iter()
            .any(|rule_id| rule_id == &proof.stricter_parent_rule_id)
        && proof.final_decision.dry_run
        && proof.final_decision.enforcement_handoff_state
            != PolicyContractDecisionHandoffState::HandedOff
        && proof.all_claim_boundaries_false
}
