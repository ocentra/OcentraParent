use serde::{Deserialize, Serialize};

use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyMapping};

mod artifacts;
mod boundary;
mod normalize;
mod reject;

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

use self::{
    artifacts::missing_required_artifacts,
    boundary::{boundary_reasons, gate_state},
    normalize::{normalize_android_vpn_service_gate_input, normalize_artifact_refs},
    reject::reject_unsupported_claims,
};

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
    let policy_evidence_grade = input.policy_mapping.evidence_grade;
    let capability_state = input.capability_state;
    let device_owner_required = input.device_owner_required;
    drop(input);

    Ok(NetworkAndroidVpnServiceGateProof {
        android_vpn_service_gate_ref: normalized.android_vpn_service_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_evidence_grade,
        package_ref: normalized.package_ref,
        vpn_service_ref: normalized.vpn_service_ref,
        capability_state,
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
        device_owner_required,
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
