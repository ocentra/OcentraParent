use super::{
    NetworkAndroidVpnServiceArtifactRefs, NetworkAndroidVpnServiceGateError,
    NetworkAndroidVpnServiceGateInput, NetworkAndroidVpnServiceRequiredArtifact,
    NormalizedAndroidVpnServiceGateInput,
};

pub(super) fn normalize_android_vpn_service_gate_input(
    input: &NetworkAndroidVpnServiceGateInput,
) -> Result<NormalizedAndroidVpnServiceGateInput, NetworkAndroidVpnServiceGateError> {
    Ok(NormalizedAndroidVpnServiceGateInput {
        android_vpn_service_gate_ref: normalize_ref(&input.android_vpn_service_gate_ref)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyAndroidVpnServiceGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        package_ref: normalize_ref(&input.package_ref)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyPackageRef)?,
        vpn_service_ref: normalize_ref(&input.vpn_service_ref)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyVpnServiceRef)?,
    })
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkAndroidVpnServiceGateInput,
) -> Result<NetworkAndroidVpnServiceArtifactRefs, NetworkAndroidVpnServiceGateError> {
    Ok(NetworkAndroidVpnServiceArtifactRefs {
        vpn_service_declaration_ref: normalized_artifact_ref(
            input.vpn_service_declaration_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration,
        )?,
        user_consent_proof_ref: normalized_artifact_ref(
            input.user_consent_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof,
        )?,
        physical_device_proof_ref: normalized_artifact_ref(
            input.physical_device_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof,
        )?,
        package_identity_proof_ref: normalized_artifact_ref(
            input.package_identity_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof,
        )?,
        virtual_interface_proof_ref: normalized_artifact_ref(
            input.virtual_interface_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof,
        )?,
        traffic_observation_proof_ref: normalized_artifact_ref(
            input.traffic_observation_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::AuditEvent,
        )?,
        device_owner_proof_ref: normalized_artifact_ref(
            input.device_owner_proof_ref.as_deref(),
            NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof,
        )?,
    })
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkAndroidVpnServiceGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkAndroidVpnServiceGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkAndroidVpnServiceGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkAndroidVpnServiceGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkAndroidVpnServiceGateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) -> Result<Option<String>, NetworkAndroidVpnServiceGateError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkAndroidVpnServiceGateError::EmptyRequiredArtifactRef(artifact),
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
