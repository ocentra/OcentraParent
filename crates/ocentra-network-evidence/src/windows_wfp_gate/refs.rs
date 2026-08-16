use super::{
    NetworkWindowsWfpArtifactRefs, NetworkWindowsWfpGateError, NetworkWindowsWfpGateInput,
    NetworkWindowsWfpRequiredArtifact, NormalizedWindowsWfpGateInput,
};

pub(super) fn normalize_windows_wfp_gate_input(
    input: &NetworkWindowsWfpGateInput,
) -> Result<NormalizedWindowsWfpGateInput, NetworkWindowsWfpGateError> {
    Ok(NormalizedWindowsWfpGateInput {
        wfp_gate_ref: normalize_ref(&input.wfp_gate_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        target_ref: normalize_ref(&input.target_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyTargetRef)?,
        wfp_provider_ref: normalize_ref(&input.wfp_provider_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpProviderRef)?,
        wfp_layer_ref: normalize_ref(&input.wfp_layer_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpLayerRef)?,
    })
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkWindowsWfpGateInput,
) -> Result<NetworkWindowsWfpArtifactRefs, NetworkWindowsWfpGateError> {
    Ok(NetworkWindowsWfpArtifactRefs {
        administrator_permission_proof_ref: normalized_artifact_ref(
            input.administrator_permission_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof,
        )?,
        driver_signing_proof_ref: normalized_artifact_ref(
            input.driver_signing_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::DriverSigningProof,
        )?,
        driver_package_proof_ref: normalized_artifact_ref(
            input.driver_package_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::DriverPackageProof,
        )?,
        provider_registration_plan_ref: normalized_artifact_ref(
            input.provider_registration_plan_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan,
        )?,
        layer_capability_matrix_ref: normalized_artifact_ref(
            input.layer_capability_matrix_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::RollbackPlan,
        )?,
        lab_result_artifact_ref: normalized_artifact_ref(
            input.lab_result_artifact_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::LabResultArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::AuditEvent,
        )?,
    })
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkWindowsWfpGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkWindowsWfpGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkWindowsWfpGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkWindowsWfpGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsWfpGateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkWindowsWfpRequiredArtifact,
) -> Result<Option<String>, NetworkWindowsWfpGateError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkWindowsWfpGateError::EmptyRequiredArtifactRef(artifact),
        ),
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
