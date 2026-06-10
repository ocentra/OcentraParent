use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpGateStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "research-only")]
    ResearchOnly,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "lab-proof-ready")]
    LabProofReady,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpGateCapabilityStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "lab-ready")]
    LabReady,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkWindowsWfpGateStatus {
    pub status_ref: String,
    pub wfp_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub target_ref: String,
    pub wfp_provider_ref: String,
    pub wfp_layer_ref: String,
    pub capability_state: NetworkWindowsWfpGateCapabilityStatusState,
    pub gate_state: NetworkWindowsWfpGateStatusState,
    pub boundary_reasons: Vec<String>,
    pub missing_required_artifacts: Vec<String>,
    pub administrator_permission_proof_ref: Option<String>,
    pub driver_signing_proof_ref: Option<String>,
    pub driver_package_proof_ref: Option<String>,
    pub provider_registration_plan_ref: Option<String>,
    pub layer_capability_matrix_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub lab_result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub wfp_lab_proof_ready: bool,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub live_driver_install_claimed: bool,
    pub callout_registration_claimed: bool,
    pub packet_block_claimed: bool,
    pub kernel_payload_inspection_claimed: bool,
    pub command_invocation_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

impl Default for NetworkWindowsWfpGateStatus {
    fn default() -> Self {
        Self {
            status_ref: String::new(),
            wfp_gate_ref: String::new(),
            policy_decision_ref: String::new(),
            parent_rule_ref: String::new(),
            evidence_refs: Vec::new(),
            local_ai_result_ref: None,
            target_ref: String::new(),
            wfp_provider_ref: String::new(),
            wfp_layer_ref: String::new(),
            capability_state: NetworkWindowsWfpGateCapabilityStatusState::ManualRequired,
            gate_state: NetworkWindowsWfpGateStatusState::ManualRequired,
            boundary_reasons: Vec::new(),
            missing_required_artifacts: Vec::new(),
            administrator_permission_proof_ref: None,
            driver_signing_proof_ref: None,
            driver_package_proof_ref: None,
            provider_registration_plan_ref: None,
            layer_capability_matrix_ref: None,
            rollback_plan_ref: None,
            lab_result_artifact_ref: None,
            audit_event_ref: None,
            wfp_lab_proof_ready: false,
            adapter_apply_authorized: false,
            enforcement_command_published: false,
            live_driver_install_claimed: false,
            callout_registration_claimed: false,
            packet_block_claimed: false,
            kernel_payload_inspection_claimed: false,
            command_invocation_claimed: false,
            exact_url_available: false,
            decrypted_payload_available: false,
            page_content_available: false,
        }
    }
}
