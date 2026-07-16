use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionPlatformStatus {
    #[serde(rename = "mac-os")]
    MacOs,
    #[serde(rename = "ios")]
    Ios,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateCapabilityStatusState {
    #[serde(rename = "apple-device-ready")]
    AppleDeviceReady,
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateStatusState {
    #[serde(rename = "research-only")]
    ResearchOnly,
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "apple-entitlement-proof-ready")]
    AppleEntitlementProofReady,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateRequiredArtifact {
    #[serde(rename = "developer-team-proof")]
    DeveloperTeamProof,
    #[serde(rename = "entitlement-approval-proof")]
    EntitlementApprovalProof,
    #[serde(rename = "provisioning-profile-proof")]
    ProvisioningProfileProof,
    #[serde(rename = "signing-proof")]
    SigningProof,
    #[serde(rename = "device-or-testflight-proof")]
    DeviceOrTestflightProof,
    #[serde(rename = "network-extension-declaration")]
    NetworkExtensionDeclaration,
    #[serde(rename = "extension-configuration-proof")]
    ExtensionConfigurationProof,
    #[serde(rename = "rollback-plan")]
    RollbackPlan,
    #[serde(rename = "audit-event")]
    AuditEvent,
    #[serde(rename = "supervision-or-mdm-proof")]
    SupervisionOrMdmProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppleNetworkExtensionGateBoundaryReason {
    #[serde(rename = "research-only-requested")]
    ResearchOnlyRequested,
    #[serde(rename = "capability-manual-required")]
    CapabilityManualRequired,
    #[serde(rename = "capability-unavailable")]
    CapabilityUnavailable,
    #[serde(rename = "evidence-grade-below-proof-threshold")]
    EvidenceGradeBelowProofThreshold,
    #[serde(rename = "policy-not-network-extension-approved")]
    PolicyNotNetworkExtensionApproved,
    #[serde(rename = "missing-required-artifact")]
    MissingRequiredArtifact,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAppleNetworkExtensionGateStatus {
    pub status_ref: String,
    pub apple_network_extension_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub platform: NetworkAppleNetworkExtensionPlatformStatus,
    pub bundle_ref: String,
    pub network_extension_ref: String,
    pub capability_state: NetworkAppleNetworkExtensionGateCapabilityStatusState,
    pub gate_state: NetworkAppleNetworkExtensionGateStatusState,
    pub boundary_reasons: Vec<NetworkAppleNetworkExtensionGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkAppleNetworkExtensionGateRequiredArtifact>,
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

impl Default for NetworkAppleNetworkExtensionGateStatus {
    fn default() -> Self {
        Self {
            status_ref: String::new(),
            apple_network_extension_gate_ref: String::new(),
            policy_decision_ref: String::new(),
            parent_rule_ref: String::new(),
            evidence_refs: Vec::new(),
            local_ai_result_ref: None,
            platform: NetworkAppleNetworkExtensionPlatformStatus::Ios,
            bundle_ref: String::new(),
            network_extension_ref: String::new(),
            capability_state: NetworkAppleNetworkExtensionGateCapabilityStatusState::ManualRequired,
            gate_state: NetworkAppleNetworkExtensionGateStatusState::ManualRequired,
            boundary_reasons: Vec::new(),
            missing_required_artifacts: Vec::new(),
            developer_team_proof_ref: None,
            entitlement_approval_proof_ref: None,
            provisioning_profile_proof_ref: None,
            signing_proof_ref: None,
            device_or_testflight_proof_ref: None,
            network_extension_declaration_ref: None,
            extension_configuration_proof_ref: None,
            rollback_plan_ref: None,
            audit_event_ref: None,
            supervision_required: false,
            supervision_or_mdm_proof_ref: None,
            apple_entitlement_proof_ready: false,
            supervision_authority_proved: false,
            adapter_apply_authorized: false,
            enforcement_command_published: false,
            simulator_only_product_support_claimed: false,
            live_network_extension_claimed: false,
            packet_block_claimed: false,
            app_level_control_claimed: false,
            exact_url_available: false,
            decrypted_payload_available: false,
            page_content_available: false,
        }
    }
}
