use super::{
    NetworkWindowsFirewallAdapterProofError, NetworkWindowsFirewallAdapterProofInput,
    NetworkWindowsFirewallArtifactRefs, NetworkWindowsFirewallRequiredArtifact,
    NormalizedWindowsFirewallInput,
};

pub(super) fn normalize_windows_firewall_input(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<NormalizedWindowsFirewallInput, NetworkWindowsFirewallAdapterProofError> {
    Ok(NormalizedWindowsFirewallInput {
        firewall_adapter_plan_ref: normalize_ref(&input.firewall_adapter_plan_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyFirewallAdapterPlanRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        windows_os_scope_ref: normalize_ref(&input.windows_os_scope_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyWindowsOsScopeRef)?,
        target_ref: normalize_ref(&input.target_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyTargetRef)?,
        firewall_rule_ref: normalize_ref(&input.firewall_rule_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyFirewallRuleRef)?,
    })
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<NetworkWindowsFirewallArtifactRefs, NetworkWindowsFirewallAdapterProofError> {
    Ok(NetworkWindowsFirewallArtifactRefs {
        adapter_authorization_ref: normalized_artifact_ref(
            input.adapter_authorization_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
        )?,
        adapter_capability_proof_ref: normalized_artifact_ref(
            input.adapter_capability_proof_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
        )?,
        apply_artifact_ref: normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
        )?,
        rollback_artifact_ref: normalized_artifact_ref(
            input.rollback_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::AuditEvent,
        )?,
    })
}

fn normalized_refs(
    refs: &[String],
) -> Result<Vec<String>, NetworkWindowsFirewallAdapterProofError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkWindowsFirewallAdapterProofError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkWindowsFirewallAdapterProofError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkWindowsFirewallAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkWindowsFirewallRequiredArtifact,
) -> Result<Option<String>, NetworkWindowsFirewallAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyRequiredArtifactRef(artifact)),
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
