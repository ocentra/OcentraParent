use serde::{Deserialize, Serialize};

use crate::windows_firewall_adapter::NetworkWindowsFirewallAdapterProof;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallLabExecutionState {
    ExecutedAndRolledBack,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallLabCommandKind {
    ApplyRule,
    VerifyRulePresent,
    RollbackRule,
    VerifyRuleRemoved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallLabCommandEvidence {
    pub kind: NetworkWindowsFirewallLabCommandKind,
    pub command_ref: String,
    pub exit_status: i32,
    pub output_sha256: String,
    pub rule_present_after_command: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallLabUnsupportedClaims {
    pub production_enforcement_claimed: bool,
    pub persistent_rule_claimed: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub policy_engine_execution_claimed: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallLabExecutionInput {
    pub lab_ref: String,
    pub adapter_proof: NetworkWindowsFirewallAdapterProof,
    pub rule_name: String,
    pub target_remote_address: String,
    pub windows_host_observed: bool,
    pub administrator_permission_observed: bool,
    pub command_evidence: Vec<NetworkWindowsFirewallLabCommandEvidence>,
    pub unsupported_claims: NetworkWindowsFirewallLabUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallLabExecutionProof {
    pub lab_ref: String,
    pub adapter_plan_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub rule_name: String,
    pub target_remote_address: String,
    pub state: NetworkWindowsFirewallLabExecutionState,
    pub command_evidence: Vec<NetworkWindowsFirewallLabCommandEvidence>,
    pub command_count: usize,
    pub apply_command_observed: bool,
    pub verify_present_observed: bool,
    pub rollback_command_observed: bool,
    pub verify_removed_observed: bool,
    pub lab_firewall_mutation_executed: bool,
    pub rollback_verified: bool,
    pub production_enforcement_claimed: bool,
    pub persistent_rule_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub policy_engine_execution_claimed: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkWindowsFirewallLabExecutionError {
    EmptyLabRef,
    AdapterProofNotApplyReady,
    UnsupportedTargetKind,
    EmptyRuleName,
    UnsafeRuleName,
    EmptyTargetRemoteAddress,
    UnsafeTargetRemoteAddress,
    EmptyCommandRef(NetworkWindowsFirewallLabCommandKind),
    EmptyCommandOutputHash(NetworkWindowsFirewallLabCommandKind),
    DuplicateCommandEvidence(NetworkWindowsFirewallLabCommandKind),
    CommandEvidenceFailure(NetworkWindowsFirewallLabCommandKind),
    MissingCommandEvidence(NetworkWindowsFirewallLabCommandKind),
    ApplyRuleNotObserved,
    RollbackRuleStillPresent,
    ProductionEnforcementClaimRejected,
    PersistentRuleClaimRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PolicyEngineExecutionClaimRejected,
    EnforcementCommandPublishedRejected,
}
