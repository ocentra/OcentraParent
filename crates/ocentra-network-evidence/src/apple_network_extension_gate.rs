use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

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

fn normalize_apple_network_extension_gate_input(
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

fn normalize_artifact_refs(
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

fn reject_unsupported_claims(
    input: &NetworkAppleNetworkExtensionGateInput,
) -> Result<(), NetworkAppleNetworkExtensionGateError> {
    if input.exact_url_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::PageContentClaimRejected);
    }
    if input.simulator_only_product_support_claimed {
        return Err(
            NetworkAppleNetworkExtensionGateError::SimulatorOnlyProductSupportClaimRejected,
        );
    }
    if input.live_network_extension_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::LiveNetworkExtensionClaimRejected);
    }
    if input.packet_block_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::PacketBlockClaimRejected);
    }
    if input.app_level_control_claimed {
        return Err(NetworkAppleNetworkExtensionGateError::AppLevelControlClaimRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkAppleNetworkExtensionGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkAppleNetworkExtensionGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkAppleNetworkExtensionCapabilityState::ManualRequired => {
            reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkAppleNetworkExtensionCapabilityState::Unavailable => {
            reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons
            .push(NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(
            NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved,
        );
    }
    if !has_required_artifacts {
        reasons.push(NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn gate_state(
    research_only: bool,
    capability_state: NetworkAppleNetworkExtensionCapabilityState,
    boundary_reasons: &[NetworkAppleNetworkExtensionGateBoundaryReason],
) -> NetworkAppleNetworkExtensionGateState {
    if research_only {
        return NetworkAppleNetworkExtensionGateState::ResearchOnly;
    }
    if capability_state == NetworkAppleNetworkExtensionCapabilityState::Unavailable {
        return NetworkAppleNetworkExtensionGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady
    } else {
        NetworkAppleNetworkExtensionGateState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkAppleNetworkExtensionArtifactRefs,
    supervision_required: bool,
) -> Vec<NetworkAppleNetworkExtensionRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.developer_team_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof,
    );
    push_missing(
        &mut missing,
        artifacts.entitlement_approval_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof,
    );
    push_missing(
        &mut missing,
        artifacts.provisioning_profile_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof,
    );
    push_missing(
        &mut missing,
        artifacts.signing_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::SigningProof,
    );
    push_missing(
        &mut missing,
        artifacts.device_or_testflight_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof,
    );
    push_missing(
        &mut missing,
        artifacts.network_extension_declaration_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration,
    );
    push_missing(
        &mut missing,
        artifacts.extension_configuration_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent,
    );
    if supervision_required {
        push_missing(
            &mut missing,
            artifacts.supervision_or_mdm_proof_ref.as_ref(),
            NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof,
        );
    }
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkAppleNetworkExtensionRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
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
