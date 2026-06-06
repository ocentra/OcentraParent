use serde::{Deserialize, Serialize};

use crate::{
    map_network_evidence_grade_to_policy, CategoryFreshnessState, CategoryMatchKind,
    DomainCategoryLookup, NetworkCategory, NetworkEvidenceGrade, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskTargetPolicyHandoffState {
    PolicyDryRun,
    ParentReviewRequired,
    ObserveOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskTargetPolicyHandoffInput {
    pub risk_target_ref: String,
    pub category_lookup: DomainCategoryLookup,
    pub requested_action: NetworkEvidencePolicyAction,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub live_adapter_mutation_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub broad_platform_support_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskTargetPolicyHandoff {
    pub risk_target_ref: String,
    pub normalized_domain: String,
    pub matched_domain: Option<String>,
    pub category: NetworkCategory,
    pub source_id: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub handoff_state: NetworkRiskTargetPolicyHandoffState,
    pub parent_review_required: bool,
    pub evidence_refs: Vec<String>,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub live_adapter_mutation_executed: bool,
    pub broad_platform_support: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRiskTargetPolicyHandoffError {
    EmptyRiskTargetRef,
    EmptyEvidenceRef,
    EmptyEvidenceRefs,
    CategoryLookupExactUrlRejected,
    CategoryLookupDecryptedPayloadRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    LiveAdapterMutationClaimRejected,
    EnforcementCommandClaimRejected,
    BroadPlatformSupportClaimRejected,
    PolicyMapping(NetworkEvidencePolicyMappingError),
}

pub fn map_network_risk_target_to_policy_handoff(
    input: NetworkRiskTargetPolicyHandoffInput,
) -> Result<NetworkRiskTargetPolicyHandoff, NetworkRiskTargetPolicyHandoffError> {
    validate_claims(&input)?;
    let risk_target_ref = normalize_ref(&input.risk_target_ref)
        .ok_or(NetworkRiskTargetPolicyHandoffError::EmptyRiskTargetRef)?;
    let evidence_refs = normalized_refs(&input.evidence_refs)?;
    let evidence_grade = risk_target_evidence_grade(&input.category_lookup);
    let policy_mapping = map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: input.policy_decision_ref,
        parent_rule_ref: input.parent_rule_ref,
        evidence_refs: evidence_refs.clone(),
        local_ai_result_ref: input.local_ai_result_ref,
        evidence_grade,
        requested_action: input.requested_action,
        adapter_capability_proof_ref: input.adapter_capability_proof_ref,
    })?;
    let handoff_state = handoff_state(policy_mapping.mode);

    Ok(NetworkRiskTargetPolicyHandoff {
        risk_target_ref,
        normalized_domain: input.category_lookup.normalized_domain,
        matched_domain: input.category_lookup.matched_domain,
        category: input.category_lookup.category,
        source_id: input.category_lookup.source_id,
        evidence_grade,
        parent_review_required: handoff_state
            == NetworkRiskTargetPolicyHandoffState::ParentReviewRequired,
        policy_mapping,
        handoff_state,
        evidence_refs,
        exact_url_available: false,
        decrypted_payload_available: false,
        live_adapter_mutation_executed: false,
        broad_platform_support: false,
        enforcement_commands_published: 0,
    })
}

fn validate_claims(
    input: &NetworkRiskTargetPolicyHandoffInput,
) -> Result<(), NetworkRiskTargetPolicyHandoffError> {
    if input.category_lookup.exact_url_available {
        return Err(NetworkRiskTargetPolicyHandoffError::CategoryLookupExactUrlRejected);
    }
    if input.category_lookup.decrypted_payload_available {
        return Err(NetworkRiskTargetPolicyHandoffError::CategoryLookupDecryptedPayloadRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkRiskTargetPolicyHandoffError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkRiskTargetPolicyHandoffError::DecryptedPayloadClaimRejected);
    }
    if input.live_adapter_mutation_claimed {
        return Err(NetworkRiskTargetPolicyHandoffError::LiveAdapterMutationClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkRiskTargetPolicyHandoffError::EnforcementCommandClaimRejected);
    }
    if input.broad_platform_support_claimed {
        return Err(NetworkRiskTargetPolicyHandoffError::BroadPlatformSupportClaimRejected);
    }
    Ok(())
}

fn risk_target_evidence_grade(lookup: &DomainCategoryLookup) -> NetworkEvidenceGrade {
    if lookup.match_kind == CategoryMatchKind::NoMatch
        || lookup.category == NetworkCategory::Unknown
    {
        return NetworkEvidenceGrade::D;
    }
    if !matches!(lookup.freshness, CategoryFreshnessState::Fresh { .. }) {
        return NetworkEvidenceGrade::C;
    }
    if lookup
        .confidence_percent
        .is_some_and(|confidence| confidence >= 90)
    {
        NetworkEvidenceGrade::B
    } else {
        NetworkEvidenceGrade::C
    }
}

fn handoff_state(mode: NetworkEvidencePolicyMode) -> NetworkRiskTargetPolicyHandoffState {
    match mode {
        NetworkEvidencePolicyMode::DryRun => NetworkRiskTargetPolicyHandoffState::PolicyDryRun,
        NetworkEvidencePolicyMode::ParentReview => {
            NetworkRiskTargetPolicyHandoffState::ParentReviewRequired
        }
        NetworkEvidencePolicyMode::ObserveOnly => NetworkRiskTargetPolicyHandoffState::ObserveOnly,
    }
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkRiskTargetPolicyHandoffError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkRiskTargetPolicyHandoffError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkRiskTargetPolicyHandoffError::EmptyEvidenceRefs);
    }
    Ok(normalized)
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

impl From<NetworkEvidencePolicyMappingError> for NetworkRiskTargetPolicyHandoffError {
    fn from(error: NetworkEvidencePolicyMappingError) -> Self {
        Self::PolicyMapping(error)
    }
}
