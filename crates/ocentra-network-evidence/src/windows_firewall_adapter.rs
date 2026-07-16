mod boundary;
mod refs;

use serde::{Deserialize, Serialize};

use self::{
    boundary::{
        boundary_reasons, missing_required_artifacts, proof_state, reject_unsupported_claims,
    },
    refs::{normalize_artifact_refs, normalize_windows_firewall_input},
};

use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyMapping};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallAdapterAction {
    BlockOutbound,
    BlockInbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallTargetKind {
    App,
    RemoteAddress,
    RemotePort,
    LocalPort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallCapabilityState {
    Supported,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallProofState {
    DryRun,
    ManualRequired,
    Unavailable,
    ApplyReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallRequiredArtifact {
    AdapterAuthorization,
    CapabilityProof,
    ApplyArtifact,
    ResultArtifact,
    RollbackArtifact,
    AuditEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsFirewallBoundaryReason {
    DryRunRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowApplyThreshold,
    PolicyNotFirewallApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallAdapterProofInput {
    pub firewall_adapter_plan_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub requested_action: NetworkWindowsFirewallAdapterAction,
    pub windows_os_scope_ref: String,
    pub target_kind: NetworkWindowsFirewallTargetKind,
    pub target_ref: String,
    pub firewall_rule_ref: String,
    pub capability_state: NetworkWindowsFirewallCapabilityState,
    pub adapter_authorization_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub rollback_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub dry_run: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub host_firewall_mutation_claimed: bool,
    pub netsh_command_invoked: bool,
    pub powershell_command_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsFirewallAdapterProof {
    pub firewall_adapter_plan_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub requested_action: NetworkWindowsFirewallAdapterAction,
    pub windows_os_scope_ref: String,
    pub target_kind: NetworkWindowsFirewallTargetKind,
    pub target_ref: String,
    pub firewall_rule_ref: String,
    pub capability_state: NetworkWindowsFirewallCapabilityState,
    pub proof_state: NetworkWindowsFirewallProofState,
    pub boundary_reasons: Vec<NetworkWindowsFirewallBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkWindowsFirewallRequiredArtifact>,
    pub adapter_authorization_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub rollback_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub host_firewall_mutation_claimed: bool,
    pub netsh_command_invoked: bool,
    pub powershell_command_invoked: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkWindowsFirewallAdapterProofError {
    EmptyFirewallAdapterPlanRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyWindowsOsScopeRef,
    EmptyTargetRef,
    EmptyFirewallRuleRef,
    EmptyRequiredArtifactRef(NetworkWindowsFirewallRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    HostFirewallMutationClaimRejected,
    NetshCommandInvocationRejected,
    PowershellCommandInvocationRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedWindowsFirewallInput {
    firewall_adapter_plan_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    windows_os_scope_ref: String,
    target_ref: String,
    firewall_rule_ref: String,
}

struct NetworkWindowsFirewallArtifactRefs {
    adapter_authorization_ref: Option<String>,
    adapter_capability_proof_ref: Option<String>,
    apply_artifact_ref: Option<String>,
    result_artifact_ref: Option<String>,
    rollback_artifact_ref: Option<String>,
    audit_event_ref: Option<String>,
}

pub fn plan_network_windows_firewall_adapter_proof(
    input: NetworkWindowsFirewallAdapterProofInput,
) -> Result<NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallAdapterProofError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkWindowsFirewallAdapterProofError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_windows_firewall_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts = missing_required_artifacts(&artifacts);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let proof_state = proof_state(input.dry_run, input.capability_state, &boundary_reasons);
    let adapter_apply_authorized = proof_state == NetworkWindowsFirewallProofState::ApplyReady;
    let policy_evidence_grade = input.policy_mapping.evidence_grade;
    let requested_action = input.requested_action;
    let target_kind = input.target_kind;
    let capability_state = input.capability_state;
    drop(input);

    Ok(NetworkWindowsFirewallAdapterProof {
        firewall_adapter_plan_ref: normalized.firewall_adapter_plan_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_evidence_grade,
        requested_action,
        windows_os_scope_ref: normalized.windows_os_scope_ref,
        target_kind,
        target_ref: normalized.target_ref,
        firewall_rule_ref: normalized.firewall_rule_ref,
        capability_state,
        proof_state,
        boundary_reasons,
        missing_required_artifacts,
        adapter_authorization_ref: artifacts.adapter_authorization_ref,
        adapter_capability_proof_ref: artifacts.adapter_capability_proof_ref,
        apply_artifact_ref: artifacts.apply_artifact_ref,
        result_artifact_ref: artifacts.result_artifact_ref,
        rollback_artifact_ref: artifacts.rollback_artifact_ref,
        audit_event_ref: artifacts.audit_event_ref,
        adapter_apply_authorized,
        enforcement_command_published: false,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}
