use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxNftablesLabStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "executed-and-rolled-back")]
    ExecutedAndRolledBack,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxNftablesLabCommandStatusKind {
    #[default]
    #[serde(rename = "create-table")]
    CreateTable,
    #[serde(rename = "create-chain")]
    CreateChain,
    #[serde(rename = "add-rule")]
    AddRule,
    #[serde(rename = "verify-rule-present")]
    VerifyRulePresent,
    #[serde(rename = "delete-table")]
    DeleteTable,
    #[serde(rename = "verify-table-removed")]
    VerifyTableRemoved,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLinuxNftablesLabCommandStatusRow {
    pub kind: NetworkLinuxNftablesLabCommandStatusKind,
    pub command_ref: String,
    pub exit_status: i32,
    pub output_sha256: String,
    pub table_present_after_command: bool,
    pub chain_present_after_command: bool,
    pub rule_present_after_command: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLinuxNftablesLabStatus {
    pub status_ref: String,
    pub lab_ref: String,
    pub linux_adapter_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub distro_ref: String,
    pub kernel_ref: String,
    pub table_name: String,
    pub chain_name: String,
    pub target_remote_address: String,
    pub state: NetworkLinuxNftablesLabStatusState,
    pub wsl_host_observed: bool,
    pub root_permission_observed: bool,
    pub nft_tool_observed: bool,
    pub command_count: u64,
    pub required_command_count: u64,
    pub table_create_observed: bool,
    pub chain_create_observed: bool,
    pub rule_add_observed: bool,
    pub verify_present_observed: bool,
    pub rollback_observed: bool,
    pub verify_removed_observed: bool,
    pub lab_packet_filter_rule_executed: bool,
    pub rollback_verified: bool,
    pub production_enforcement_claimed: bool,
    pub persistent_rule_claimed: bool,
    pub generic_linux_support_claimed: bool,
    pub service_manager_install_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub policy_engine_execution_claimed: bool,
    pub enforcement_command_published: bool,
    pub command_evidence: Vec<NetworkLinuxNftablesLabCommandStatusRow>,
}
