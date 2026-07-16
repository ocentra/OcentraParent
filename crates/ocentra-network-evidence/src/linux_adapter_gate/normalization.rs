use super::{NetworkLinuxAdapterGateError, NetworkLinuxAdapterGateInput};

pub(super) struct NormalizedLinuxAdapterGateInput {
    pub linux_adapter_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub distro_ref: String,
    pub kernel_ref: String,
}

pub(super) struct NetworkLinuxAdapterArtifactRefs {
    pub distro_kernel_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub adapter_api_capability_proof_ref: Option<String>,
    pub adapter_plan_proof_ref: Option<String>,
    pub service_manager_scope_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub lab_result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
}

pub(super) fn normalize_linux_adapter_gate_input(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<NormalizedLinuxAdapterGateInput, NetworkLinuxAdapterGateError> {
    Ok(NormalizedLinuxAdapterGateInput {
        linux_adapter_gate_ref: normalize_ref(&input.linux_adapter_gate_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyLinuxAdapterGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        distro_ref: normalize_ref(&input.distro_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyDistroRef)?,
        kernel_ref: normalize_ref(&input.kernel_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyKernelRef)?,
    })
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkLinuxAdapterGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkLinuxAdapterGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkLinuxAdapterGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkLinuxAdapterGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkLinuxAdapterGateError::EmptyLocalAiResultRef),
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
