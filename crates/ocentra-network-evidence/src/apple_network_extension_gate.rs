use serde::{Deserialize, Serialize};

use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyMapping};

mod artifacts;
mod boundary;
mod normalize;
mod reject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionPlatform {
    MacOs,
    Ios,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionCapabilityState {
    AppleDeviceReady,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateState {
    ResearchOnly,
    ManualRequired,
    Unavailable,
    AppleEntitlementProofReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionRequiredArtifact {
    DeveloperTeamProof,
    EntitlementApprovalProof,
    ProvisioningProfileProof,
    SigningProof,
    DeviceOrTestFlightProof,
    NetworkExtensionDeclaration,
    ExtensionConfigurationProof,
    RollbackPlan,
    AuditEvent,
    SupervisionOrMdmProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateBoundaryReason {
    ResearchOnlyRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowProofThreshold,
    PolicyNotNetworkExtensionApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppleNetworkExtensionGateInput {
    pub apple_network_extension_gate_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub platform: NetworkAppleNetworkExtensionPlatform,
    pub bundle_ref: String,
    pub network_extension_ref: String,
    pub capability_state: NetworkAppleNetworkExtensionCapabilityState,
    pub developer_team_proof_ref: Option<String>,
    pub entitlement_approval_proof_ref: Option<String>,
    pub provisioning_profile_proof_ref: Option<String>,
    pub signing_proof_ref: Option<String>,
    pub device_or_testflight_proof_ref: Option<String>,
    pub network_extension_declaration_ref: Option<String>,
    pub extension_configuration_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub supervision_required: bool,
    pub supervision_or_mdm_proof_ref: Option<String>,
    pub research_only: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub simulator_only_product_support_claimed: bool,
    pub live_network_extension_claimed: bool,
    pub packet_block_claimed: bool,
    pub app_level_control_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppleNetworkExtensionGateProof {
    pub apple_network_extension_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub platform: NetworkAppleNetworkExtensionPlatform,
    pub bundle_ref: String,
    pub network_extension_ref: String,
    pub capability_state: NetworkAppleNetworkExtensionCapabilityState,
    pub gate_state: NetworkAppleNetworkExtensionGateState,
    pub boundary_reasons: Vec<NetworkAppleNetworkExtensionGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkAppleNetworkExtensionRequiredArtifact>,
    pub developer_team_proof_ref: Option<String>,
    pub entitlement_approval_proof_ref: Option<String>,
    pub provisioning_profile_proof_ref: Option<String>,
    pub signing_proof_ref: Option<String>,
    pub device_or_testflight_proof_ref: Option<String>,
    pub network_extension_declaration_ref: Option<String>,
    pub extension_configuration_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub supervision_required: bool,
    pub supervision_or_mdm_proof_ref: Option<String>,
    pub apple_entitlement_proof_ready: bool,
    pub supervision_authority_proved: bool,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub simulator_only_product_support_claimed: bool,
    pub live_network_extension_claimed: bool,
    pub packet_block_claimed: bool,
    pub app_level_control_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAppleNetworkExtensionGateError {
    EmptyAppleNetworkExtensionGateRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyBundleRef,
    EmptyNetworkExtensionRef,
    EmptyRequiredArtifactRef(NetworkAppleNetworkExtensionRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    SimulatorOnlyProductSupportClaimRejected,
    LiveNetworkExtensionClaimRejected,
    PacketBlockClaimRejected,
    AppLevelControlClaimRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedAppleNetworkExtensionGateInput {
    apple_network_extension_gate_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    bundle_ref: String,
    network_extension_ref: String,
}

struct NetworkAppleNetworkExtensionArtifactRefs {
    developer_team_proof_ref: Option<String>,
    entitlement_approval_proof_ref: Option<String>,
    provisioning_profile_proof_ref: Option<String>,
    signing_proof_ref: Option<String>,
    device_or_testflight_proof_ref: Option<String>,
    network_extension_declaration_ref: Option<String>,
    extension_configuration_proof_ref: Option<String>,
    rollback_plan_ref: Option<String>,
    audit_event_ref: Option<String>,
    supervision_or_mdm_proof_ref: Option<String>,
}

use self::{
    artifacts::missing_required_artifacts,
    boundary::{boundary_reasons, gate_state},
    normalize::{normalize_apple_network_extension_gate_input, normalize_artifact_refs},
    reject::reject_unsupported_claims,
};

pub fn plan_network_apple_network_extension_gate(
    input: NetworkAppleNetworkExtensionGateInput,
) -> Result<NetworkAppleNetworkExtensionGateProof, NetworkAppleNetworkExtensionGateError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkAppleNetworkExtensionGateError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_apple_network_extension_gate_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts =
        missing_required_artifacts(&artifacts, input.supervision_required);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let gate_state = gate_state(
        input.research_only,
        input.capability_state,
        &boundary_reasons,
    );
    let apple_entitlement_proof_ready =
        gate_state == NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady;
    let supervision_authority_proved =
        input.supervision_required && artifacts.supervision_or_mdm_proof_ref.is_some();
    let policy_evidence_grade = input.policy_mapping.evidence_grade;
    let platform = input.platform;
    let capability_state = input.capability_state;
    let supervision_required = input.supervision_required;
    drop(input);

    Ok(NetworkAppleNetworkExtensionGateProof {
        apple_network_extension_gate_ref: normalized.apple_network_extension_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_evidence_grade,
        platform,
        bundle_ref: normalized.bundle_ref,
        network_extension_ref: normalized.network_extension_ref,
        capability_state,
        gate_state,
        boundary_reasons,
        missing_required_artifacts,
        developer_team_proof_ref: artifacts.developer_team_proof_ref,
        entitlement_approval_proof_ref: artifacts.entitlement_approval_proof_ref,
        provisioning_profile_proof_ref: artifacts.provisioning_profile_proof_ref,
        signing_proof_ref: artifacts.signing_proof_ref,
        device_or_testflight_proof_ref: artifacts.device_or_testflight_proof_ref,
        network_extension_declaration_ref: artifacts.network_extension_declaration_ref,
        extension_configuration_proof_ref: artifacts.extension_configuration_proof_ref,
        rollback_plan_ref: artifacts.rollback_plan_ref,
        audit_event_ref: artifacts.audit_event_ref,
        supervision_required,
        supervision_or_mdm_proof_ref: artifacts.supervision_or_mdm_proof_ref,
        apple_entitlement_proof_ready,
        supervision_authority_proved,
        adapter_apply_authorized: false,
        enforcement_command_published: false,
        simulator_only_product_support_claimed: false,
        live_network_extension_claimed: false,
        packet_block_claimed: false,
        app_level_control_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}
