mod normalization;
mod routing;

use serde::{Deserialize, Serialize};

use self::{
    normalization::{normalize_ref, normalized_optional_ref, normalized_refs},
    routing::mapped_mode_and_action,
};
use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidencePolicyAction {
    None,
    AskParent,
    WarnChild,
    Monitor,
    Limit,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidencePolicyMode {
    ObserveOnly,
    DryRun,
    ParentReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEvidencePolicyMappingInput {
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub requested_action: NetworkEvidencePolicyAction,
    pub adapter_capability_proof_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEvidencePolicyMapping {
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub requested_action: NetworkEvidencePolicyAction,
    pub mapped_action: NetworkEvidencePolicyAction,
    pub mode: NetworkEvidencePolicyMode,
    pub adapter_capability_proof_ref: Option<String>,
    pub adapter_action_authorized: bool,
    pub enforcement_command_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEvidencePolicyMappingError {
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyAdapterCapabilityProofRef,
}

pub fn map_network_evidence_grade_to_policy(
    input: NetworkEvidencePolicyMappingInput,
) -> Result<NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError> {
    let NetworkEvidencePolicyMappingInput {
        policy_decision_ref,
        parent_rule_ref,
        evidence_refs,
        local_ai_result_ref,
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref,
    } = input;
    let policy_decision_ref = normalize_ref(&policy_decision_ref)
        .ok_or(NetworkEvidencePolicyMappingError::EmptyPolicyDecisionRef)?;
    let parent_rule_ref = normalize_ref(&parent_rule_ref)
        .ok_or(NetworkEvidencePolicyMappingError::EmptyParentRuleRef)?;
    let evidence_refs = normalized_refs(
        &evidence_refs,
        NetworkEvidencePolicyMappingError::EmptyEvidenceRef,
    )?;
    let local_ai_result_ref = normalized_optional_ref(
        local_ai_result_ref.as_deref(),
        NetworkEvidencePolicyMappingError::EmptyLocalAiResultRef,
    )?;
    let adapter_capability_proof_ref = normalized_optional_ref(
        adapter_capability_proof_ref.as_deref(),
        NetworkEvidencePolicyMappingError::EmptyAdapterCapabilityProofRef,
    )?;
    let (mode, mapped_action) = mapped_mode_and_action(evidence_grade, requested_action);

    Ok(NetworkEvidencePolicyMapping {
        policy_decision_ref,
        parent_rule_ref,
        evidence_refs,
        local_ai_result_ref,
        evidence_grade,
        requested_action,
        mapped_action,
        mode,
        adapter_capability_proof_ref,
        adapter_action_authorized: false,
        enforcement_command_authorized: false,
    })
}
