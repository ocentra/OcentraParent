use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

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

    Ok(NetworkWindowsFirewallAdapterProof {
        firewall_adapter_plan_ref: normalized.firewall_adapter_plan_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: input.policy_mapping.evidence_grade,
        requested_action: input.requested_action,
        target_kind: input.target_kind,
        target_ref: normalized.target_ref,
        firewall_rule_ref: normalized.firewall_rule_ref,
        capability_state: input.capability_state,
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

fn normalize_windows_firewall_input(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<NormalizedWindowsFirewallInput, NetworkWindowsFirewallAdapterProofError> {
    Ok(NormalizedWindowsFirewallInput {
        firewall_adapter_plan_ref: normalize_ref(&input.firewall_adapter_plan_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyFirewallAdapterPlanRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        target_ref: normalize_ref(&input.target_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyTargetRef)?,
        firewall_rule_ref: normalize_ref(&input.firewall_rule_ref)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyFirewallRuleRef)?,
    })
}

fn normalize_artifact_refs(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<NetworkWindowsFirewallArtifactRefs, NetworkWindowsFirewallAdapterProofError> {
    Ok(NetworkWindowsFirewallArtifactRefs {
        adapter_authorization_ref: normalized_artifact_ref(
            input.adapter_authorization_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
        )?,
        adapter_capability_proof_ref: normalized_artifact_ref(
            input.adapter_capability_proof_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
        )?,
        apply_artifact_ref: normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
        )?,
        rollback_artifact_ref: normalized_artifact_ref(
            input.rollback_artifact_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkWindowsFirewallRequiredArtifact::AuditEvent,
        )?,
    })
}

fn reject_unsupported_claims(
    input: &NetworkWindowsFirewallAdapterProofInput,
) -> Result<(), NetworkWindowsFirewallAdapterProofError> {
    if input.exact_url_claimed {
        return Err(NetworkWindowsFirewallAdapterProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkWindowsFirewallAdapterProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkWindowsFirewallAdapterProofError::PageContentClaimRejected);
    }
    if input.host_firewall_mutation_claimed {
        return Err(NetworkWindowsFirewallAdapterProofError::HostFirewallMutationClaimRejected);
    }
    if input.netsh_command_invoked {
        return Err(NetworkWindowsFirewallAdapterProofError::NetshCommandInvocationRejected);
    }
    if input.powershell_command_invoked {
        return Err(NetworkWindowsFirewallAdapterProofError::PowershellCommandInvocationRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkWindowsFirewallAdapterProofInput,
    has_required_artifacts: bool,
) -> Vec<NetworkWindowsFirewallBoundaryReason> {
    let mut reasons = Vec::new();
    if input.dry_run {
        reasons.push(NetworkWindowsFirewallBoundaryReason::DryRunRequested);
    }
    match input.capability_state {
        NetworkWindowsFirewallCapabilityState::ManualRequired => {
            reasons.push(NetworkWindowsFirewallBoundaryReason::CapabilityManualRequired);
        }
        NetworkWindowsFirewallCapabilityState::Unavailable => {
            reasons.push(NetworkWindowsFirewallBoundaryReason::CapabilityUnavailable);
        }
        NetworkWindowsFirewallCapabilityState::Supported => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkWindowsFirewallBoundaryReason::EvidenceGradeBelowApplyThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkWindowsFirewallBoundaryReason::PolicyNotFirewallApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkWindowsFirewallBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn proof_state(
    dry_run: bool,
    capability_state: NetworkWindowsFirewallCapabilityState,
    boundary_reasons: &[NetworkWindowsFirewallBoundaryReason],
) -> NetworkWindowsFirewallProofState {
    if dry_run {
        return NetworkWindowsFirewallProofState::DryRun;
    }
    if capability_state == NetworkWindowsFirewallCapabilityState::Unavailable {
        return NetworkWindowsFirewallProofState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkWindowsFirewallProofState::ApplyReady
    } else {
        NetworkWindowsFirewallProofState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkWindowsFirewallArtifactRefs,
) -> Vec<NetworkWindowsFirewallRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.adapter_authorization_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_capability_proof_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
    );
    push_missing(
        &mut missing,
        artifacts.apply_artifact_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.result_artifact_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_artifact_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkWindowsFirewallRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkWindowsFirewallRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkWindowsFirewallRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_refs(
    refs: &[String],
) -> Result<Vec<String>, NetworkWindowsFirewallAdapterProofError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkWindowsFirewallAdapterProofError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkWindowsFirewallAdapterProofError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkWindowsFirewallAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkWindowsFirewallRequiredArtifact,
) -> Result<Option<String>, NetworkWindowsFirewallAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsFirewallAdapterProofError::EmptyRequiredArtifactRef(artifact)),
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
