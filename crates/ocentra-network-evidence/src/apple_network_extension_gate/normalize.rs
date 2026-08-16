use super::{
    NetworkAppleNetworkExtensionArtifactRefs, NetworkAppleNetworkExtensionGateError,
    NetworkAppleNetworkExtensionGateInput, NetworkAppleNetworkExtensionRequiredArtifact,
    NormalizedAppleNetworkExtensionGateInput,
};

pub(super) fn normalize_apple_network_extension_gate_input(
    input: &NetworkAppleNetworkExtensionGateInput,
) -> Result<NormalizedAppleNetworkExtensionGateInput, NetworkAppleNetworkExtensionGateError> {
    Ok(NormalizedAppleNetworkExtensionGateInput {
        apple_network_extension_gate_ref: normalize_ref(&input.apple_network_extension_gate_ref)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyAppleNetworkExtensionGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        bundle_ref: normalize_ref(&input.bundle_ref)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyBundleRef)?,
        network_extension_ref: normalize_ref(&input.network_extension_ref)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyNetworkExtensionRef)?,
    })
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkAppleNetworkExtensionGateInput,
) -> Result<NetworkAppleNetworkExtensionArtifactRefs, NetworkAppleNetworkExtensionGateError> {
    Ok(NetworkAppleNetworkExtensionArtifactRefs {
        developer_team_proof_ref: normalized_artifact_ref(
            input.developer_team_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof,
        )?,
        entitlement_approval_proof_ref: normalized_artifact_ref(
            input.entitlement_approval_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof,
        )?,
        provisioning_profile_proof_ref: normalized_artifact_ref(
            input.provisioning_profile_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof,
        )?,
        signing_proof_ref: normalized_artifact_ref(
            input.signing_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::SigningProof,
        )?,
        device_or_testflight_proof_ref: normalized_artifact_ref(
            input.device_or_testflight_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof,
        )?,
        network_extension_declaration_ref: normalized_artifact_ref(
            input.network_extension_declaration_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration,
        )?,
        extension_configuration_proof_ref: normalized_artifact_ref(
            input.extension_configuration_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent,
        )?,
        supervision_or_mdm_proof_ref: normalized_artifact_ref(
            input.supervision_or_mdm_proof_ref.as_deref(),
            NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof,
        )?,
    })
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkAppleNetworkExtensionGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkAppleNetworkExtensionGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkAppleNetworkExtensionGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkAppleNetworkExtensionGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) -> Result<Option<String>, NetworkAppleNetworkExtensionGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkAppleNetworkExtensionGateError::EmptyRequiredArtifactRef(artifact)),
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
