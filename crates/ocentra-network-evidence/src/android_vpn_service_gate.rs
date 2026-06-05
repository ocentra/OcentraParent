use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceCapabilityState {
    PhysicalDeviceReady,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateState {
    ResearchOnly,
    ManualRequired,
    Unavailable,
    PhysicalDeviceProofReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceRequiredArtifact {
    VpnServiceDeclaration,
    UserConsentProof,
    PhysicalDeviceProof,
    PackageIdentityProof,
    VirtualInterfaceProof,
    TrafficObservationProof,
    RollbackPlan,
    AuditEvent,
    DeviceOwnerProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateBoundaryReason {
    ResearchOnlyRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowProofThreshold,
    PolicyNotVpnServiceApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidVpnServiceGateInput {
    pub android_vpn_service_gate_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub package_ref: String,
    pub vpn_service_ref: String,
    pub capability_state: NetworkAndroidVpnServiceCapabilityState,
    pub vpn_service_declaration_ref: Option<String>,
    pub user_consent_proof_ref: Option<String>,
    pub physical_device_proof_ref: Option<String>,
    pub package_identity_proof_ref: Option<String>,
    pub virtual_interface_proof_ref: Option<String>,
    pub traffic_observation_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub device_owner_required: bool,
    pub device_owner_proof_ref: Option<String>,
    pub research_only: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub emulator_only_product_support_claimed: bool,
    pub live_vpn_tunnel_claimed: bool,
    pub packet_block_claimed: bool,
    pub app_package_correlation_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidVpnServiceGateProof {
    pub android_vpn_service_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub package_ref: String,
    pub vpn_service_ref: String,
    pub capability_state: NetworkAndroidVpnServiceCapabilityState,
    pub gate_state: NetworkAndroidVpnServiceGateState,
    pub boundary_reasons: Vec<NetworkAndroidVpnServiceGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkAndroidVpnServiceRequiredArtifact>,
    pub vpn_service_declaration_ref: Option<String>,
    pub user_consent_proof_ref: Option<String>,
    pub physical_device_proof_ref: Option<String>,
    pub package_identity_proof_ref: Option<String>,
    pub virtual_interface_proof_ref: Option<String>,
    pub traffic_observation_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub device_owner_required: bool,
    pub device_owner_proof_ref: Option<String>,
    pub physical_device_proof_ready: bool,
    pub device_owner_authority_proved: bool,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub emulator_only_product_support_claimed: bool,
    pub live_vpn_tunnel_claimed: bool,
    pub packet_block_claimed: bool,
    pub app_package_correlation_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAndroidVpnServiceGateError {
    EmptyAndroidVpnServiceGateRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyPackageRef,
    EmptyVpnServiceRef,
    EmptyRequiredArtifactRef(NetworkAndroidVpnServiceRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    EmulatorOnlyProductSupportClaimRejected,
    LiveVpnTunnelClaimRejected,
    PacketBlockClaimRejected,
    AppPackageCorrelationClaimRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedAndroidVpnServiceGateInput {
    android_vpn_service_gate_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    package_ref: String,
    vpn_service_ref: String,
}

struct NetworkAndroidVpnServiceArtifactRefs {
    vpn_service_declaration_ref: Option<String>,
    user_consent_proof_ref: Option<String>,
    physical_device_proof_ref: Option<String>,
    package_identity_proof_ref: Option<String>,
    virtual_interface_proof_ref: Option<String>,
    traffic_observation_proof_ref: Option<String>,
    rollback_plan_ref: Option<String>,
    audit_event_ref: Option<String>,
    device_owner_proof_ref: Option<String>,
}

pub fn plan_network_android_vpn_service_gate(
    input: NetworkAndroidVpnServiceGateInput,
) -> Result<NetworkAndroidVpnServiceGateProof, NetworkAndroidVpnServiceGateError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkAndroidVpnServiceGateError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_android_vpn_service_gate_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts =
        missing_required_artifacts(&artifacts, input.device_owner_required);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let gate_state = gate_state(
        input.research_only,
        input.capability_state,
        &boundary_reasons,
    );
    let physical_device_proof_ready =
        gate_state == NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady;
    let device_owner_authority_proved =
        input.device_owner_required && artifacts.device_owner_proof_ref.is_some();

    Ok(NetworkAndroidVpnServiceGateProof {
        android_vpn_service_gate_ref: normalized.android_vpn_service_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: input.policy_mapping.evidence_grade,
        package_ref: normalized.package_ref,
        vpn_service_ref: normalized.vpn_service_ref,
        capability_state: input.capability_state,
        gate_state,
        boundary_reasons,
        missing_required_artifacts,
        vpn_service_declaration_ref: artifacts.vpn_service_declaration_ref,
        user_consent_proof_ref: artifacts.user_consent_proof_ref,
        physical_device_proof_ref: artifacts.physical_device_proof_ref,
        package_identity_proof_ref: artifacts.package_identity_proof_ref,
        virtual_interface_proof_ref: artifacts.virtual_interface_proof_ref,
        traffic_observation_proof_ref: artifacts.traffic_observation_proof_ref,
        rollback_plan_ref: artifacts.rollback_plan_ref,
        audit_event_ref: artifacts.audit_event_ref,
        device_owner_required: input.device_owner_required,
        device_owner_proof_ref: artifacts.device_owner_proof_ref,
        physical_device_proof_ready,
        device_owner_authority_proved,
        adapter_apply_authorized: false,
        enforcement_command_published: false,
        emulator_only_product_support_claimed: false,
        live_vpn_tunnel_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

fn normalize_android_vpn_service_gate_input(
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

fn normalize_artifact_refs(
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

fn reject_unsupported_claims(
    input: &NetworkAndroidVpnServiceGateInput,
) -> Result<(), NetworkAndroidVpnServiceGateError> {
    if input.exact_url_claimed {
        return Err(NetworkAndroidVpnServiceGateError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAndroidVpnServiceGateError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAndroidVpnServiceGateError::PageContentClaimRejected);
    }
    if input.emulator_only_product_support_claimed {
        return Err(NetworkAndroidVpnServiceGateError::EmulatorOnlyProductSupportClaimRejected);
    }
    if input.live_vpn_tunnel_claimed {
        return Err(NetworkAndroidVpnServiceGateError::LiveVpnTunnelClaimRejected);
    }
    if input.packet_block_claimed {
        return Err(NetworkAndroidVpnServiceGateError::PacketBlockClaimRejected);
    }
    if input.app_package_correlation_claimed {
        return Err(NetworkAndroidVpnServiceGateError::AppPackageCorrelationClaimRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkAndroidVpnServiceGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkAndroidVpnServiceGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkAndroidVpnServiceCapabilityState::ManualRequired => {
            reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkAndroidVpnServiceCapabilityState::Unavailable => {
            reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn gate_state(
    research_only: bool,
    capability_state: NetworkAndroidVpnServiceCapabilityState,
    boundary_reasons: &[NetworkAndroidVpnServiceGateBoundaryReason],
) -> NetworkAndroidVpnServiceGateState {
    if research_only {
        return NetworkAndroidVpnServiceGateState::ResearchOnly;
    }
    if capability_state == NetworkAndroidVpnServiceCapabilityState::Unavailable {
        return NetworkAndroidVpnServiceGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady
    } else {
        NetworkAndroidVpnServiceGateState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkAndroidVpnServiceArtifactRefs,
    device_owner_required: bool,
) -> Vec<NetworkAndroidVpnServiceRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.vpn_service_declaration_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration,
    );
    push_missing(
        &mut missing,
        artifacts.user_consent_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof,
    );
    push_missing(
        &mut missing,
        artifacts.physical_device_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof,
    );
    push_missing(
        &mut missing,
        artifacts.package_identity_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof,
    );
    push_missing(
        &mut missing,
        artifacts.virtual_interface_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof,
    );
    push_missing(
        &mut missing,
        artifacts.traffic_observation_proof_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkAndroidVpnServiceRequiredArtifact::AuditEvent,
    );
    if device_owner_required {
        push_missing(
            &mut missing,
            artifacts.device_owner_proof_ref.as_ref(),
            NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof,
        );
    }
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkAndroidVpnServiceRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
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
