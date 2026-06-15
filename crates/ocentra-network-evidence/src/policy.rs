use serde::{Deserialize, Serialize};

use crate::NetworkEvidenceGrade;

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
    let policy_decision_ref = normalize_ref(&input.policy_decision_ref)
        .ok_or(NetworkEvidencePolicyMappingError::EmptyPolicyDecisionRef)?;
    let parent_rule_ref = normalize_ref(&input.parent_rule_ref)
        .ok_or(NetworkEvidencePolicyMappingError::EmptyParentRuleRef)?;
    let evidence_refs = normalized_refs(
        &input.evidence_refs,
        NetworkEvidencePolicyMappingError::EmptyEvidenceRef,
    )?;
    let local_ai_result_ref = normalized_optional_ref(
        input.local_ai_result_ref.as_deref(),
        NetworkEvidencePolicyMappingError::EmptyLocalAiResultRef,
    )?;
    let adapter_capability_proof_ref = normalized_optional_ref(
        input.adapter_capability_proof_ref.as_deref(),
        NetworkEvidencePolicyMappingError::EmptyAdapterCapabilityProofRef,
    )?;
    let evidence_grade = input.evidence_grade;
    let requested_action = input.requested_action;
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

fn mapped_mode_and_action(
    grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> (NetworkEvidencePolicyMode, NetworkEvidencePolicyAction) {
    match grade {
        NetworkEvidenceGrade::A => (
            NetworkEvidencePolicyMode::DryRun,
            dry_run_action(requested_action),
        ),
        NetworkEvidenceGrade::B => probable_mode_and_action(requested_action),
        NetworkEvidenceGrade::C => (
            NetworkEvidencePolicyMode::ParentReview,
            NetworkEvidencePolicyAction::AskParent,
        ),
        NetworkEvidenceGrade::D => (
            NetworkEvidencePolicyMode::ObserveOnly,
            NetworkEvidencePolicyAction::None,
        ),
    }
}

fn dry_run_action(requested_action: NetworkEvidencePolicyAction) -> NetworkEvidencePolicyAction {
    match requested_action {
        NetworkEvidencePolicyAction::None => NetworkEvidencePolicyAction::Monitor,
        action => action,
    }
}

fn probable_mode_and_action(
    requested_action: NetworkEvidencePolicyAction,
) -> (NetworkEvidencePolicyMode, NetworkEvidencePolicyAction) {
    match requested_action {
        NetworkEvidencePolicyAction::Block | NetworkEvidencePolicyAction::Limit => (
            NetworkEvidencePolicyMode::ParentReview,
            NetworkEvidencePolicyAction::AskParent,
        ),
        NetworkEvidencePolicyAction::None => (
            NetworkEvidencePolicyMode::DryRun,
            NetworkEvidencePolicyAction::Monitor,
        ),
        action => (NetworkEvidencePolicyMode::DryRun, action),
    }
}

fn normalized_refs(
    refs: &[String],
    empty_error: NetworkEvidencePolicyMappingError,
) -> Result<Vec<String>, NetworkEvidencePolicyMappingError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(empty_error);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(empty_error);
    }
    Ok(normalized)
}

fn normalized_optional_ref(
    value: Option<&str>,
    empty_error: NetworkEvidencePolicyMappingError,
) -> Result<Option<String>, NetworkEvidencePolicyMappingError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(empty_error),
        None => Ok(None),
    }
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
