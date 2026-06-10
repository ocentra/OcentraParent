use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallLabStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "executed-and-rolled-back")]
    ExecutedAndRolledBack,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallLabCommandStatusKind {
    #[default]
    #[serde(rename = "apply-rule")]
    ApplyRule,
    #[serde(rename = "verify-rule-present")]
    VerifyRulePresent,
    #[serde(rename = "rollback-rule")]
    RollbackRule,
    #[serde(rename = "verify-rule-removed")]
    VerifyRuleRemoved,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkWindowsFirewallLabCommandStatusRow {
    pub kind: NetworkWindowsFirewallLabCommandStatusKind,
    pub command_ref: String,
    pub exit_status: i32,
    pub output_sha256: String,
    pub rule_present_after_command: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkWindowsFirewallLabStatus {
    pub status_ref: String,
    pub lab_ref: String,
    pub firewall_adapter_plan_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub windows_os_scope_ref: String,
    pub target_ref: String,
    pub firewall_rule_ref: String,
    pub rule_name: String,
    pub target_remote_address: String,
    pub state: NetworkWindowsFirewallLabStatusState,
    pub windows_host_observed: bool,
    pub administrator_permission_observed: bool,
    pub command_count: u64,
    pub required_command_count: u64,
    pub apply_command_observed: bool,
    pub verify_present_observed: bool,
    pub rollback_command_observed: bool,
    pub verify_removed_observed: bool,
    pub lab_firewall_mutation_executed: bool,
    pub rollback_verified: bool,
    pub adapter_apply_authorized: bool,
    pub production_enforcement_claimed: bool,
    pub persistent_rule_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub host_firewall_mutation_claimed: bool,
    pub netsh_command_invoked: bool,
    pub powershell_command_invoked: bool,
    pub policy_engine_execution_claimed: bool,
    pub enforcement_command_published: bool,
    pub command_evidence: Vec<NetworkWindowsFirewallLabCommandStatusRow>,
}
