use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "research-only")]
    ResearchOnly,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "physical-device-proof-ready")]
    PhysicalDeviceProofReady,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateCapabilityStatusState {
    #[serde(rename = "physical-device-ready")]
    PhysicalDeviceReady,
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateRequiredArtifact {
    #[serde(rename = "vpn-service-declaration")]
    VpnServiceDeclaration,
    #[serde(rename = "user-consent-proof")]
    UserConsentProof,
    #[serde(rename = "physical-device-proof")]
    PhysicalDeviceProof,
    #[serde(rename = "package-identity-proof")]
    PackageIdentityProof,
    #[serde(rename = "virtual-interface-proof")]
    VirtualInterfaceProof,
    #[serde(rename = "traffic-observation-proof")]
    TrafficObservationProof,
    #[serde(rename = "rollback-plan")]
    RollbackPlan,
    #[serde(rename = "audit-event")]
    AuditEvent,
    #[serde(rename = "device-owner-proof")]
    DeviceOwnerProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidVpnServiceGateBoundaryReason {
    #[serde(rename = "research-only-requested")]
    ResearchOnlyRequested,
    #[serde(rename = "capability-manual-required")]
    CapabilityManualRequired,
    #[serde(rename = "capability-unavailable")]
    CapabilityUnavailable,
    #[serde(rename = "evidence-grade-below-proof-threshold")]
    EvidenceGradeBelowProofThreshold,
    #[serde(rename = "policy-not-vpn-service-approved")]
    PolicyNotVpnServiceApproved,
    #[serde(rename = "missing-required-artifact")]
    MissingRequiredArtifact,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAndroidVpnServiceGateStatus {
    pub status_ref: String,
    pub android_vpn_service_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub package_ref: String,
    pub vpn_service_ref: String,
    pub capability_state: NetworkAndroidVpnServiceGateCapabilityStatusState,
    pub gate_state: NetworkAndroidVpnServiceGateStatusState,
    pub boundary_reasons: Vec<NetworkAndroidVpnServiceGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkAndroidVpnServiceGateRequiredArtifact>,
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

impl Default for NetworkAndroidVpnServiceGateStatus {
    fn default() -> Self {
        Self {
            status_ref: String::new(),
            android_vpn_service_gate_ref: String::new(),
            policy_decision_ref: String::new(),
            parent_rule_ref: String::new(),
            evidence_refs: Vec::new(),
            local_ai_result_ref: None,
            package_ref: String::new(),
            vpn_service_ref: String::new(),
            capability_state: NetworkAndroidVpnServiceGateCapabilityStatusState::ManualRequired,
            gate_state: NetworkAndroidVpnServiceGateStatusState::ManualRequired,
            boundary_reasons: Vec::new(),
            missing_required_artifacts: Vec::new(),
            vpn_service_declaration_ref: None,
            user_consent_proof_ref: None,
            physical_device_proof_ref: None,
            package_identity_proof_ref: None,
            virtual_interface_proof_ref: None,
            traffic_observation_proof_ref: None,
            rollback_plan_ref: None,
            audit_event_ref: None,
            device_owner_required: false,
            device_owner_proof_ref: None,
            physical_device_proof_ready: false,
            device_owner_authority_proved: false,
            adapter_apply_authorized: false,
            enforcement_command_published: false,
            emulator_only_product_support_claimed: false,
            live_vpn_tunnel_claimed: false,
            packet_block_claimed: false,
            app_package_correlation_claimed: false,
            exact_url_available: false,
            decrypted_payload_available: false,
            page_content_available: false,
        }
    }
}
