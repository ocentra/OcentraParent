mod boundaries;
mod normalize;

use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;
use crate::policy::{
    NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping, NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultRequestedAction {
    Block,
    TerminateProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultTargetKind {
    Domain,
    IpEndpoint,
    Process,
    App,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultCapabilityState {
    Supported,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultAdapterProofState {
    ApplyReady,
    DryRun,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultState {
    Blocked,
    Terminated,
    DryRun,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultRequiredArtifact {
    AdapterProof,
    ApplyArtifact,
    ResultArtifact,
    AuditEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkActionResultBoundaryReason {
    DryRunRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    AdapterProofDryRun,
    AdapterProofManualRequired,
    AdapterProofUnavailable,
    EvidenceGradeBelowApplyThreshold,
    PolicyNotAdapterApproved,
    TerminateTargetNotProcessOrApp,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkActionResultInput {
    pub action_result_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub requested_action: NetworkActionResultRequestedAction,
    pub target_kind: NetworkActionResultTargetKind,
    pub target_ref: String,
    pub capability_state: NetworkActionResultCapabilityState,
    pub adapter_proof_state: NetworkActionResultAdapterProofState,
    pub adapter_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub dry_run: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub host_mutation_claimed: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkActionResultProof {
    pub action_result_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub requested_action: NetworkActionResultRequestedAction,
    pub target_kind: NetworkActionResultTargetKind,
    pub target_ref: String,
    pub capability_state: NetworkActionResultCapabilityState,
    pub adapter_proof_state: NetworkActionResultAdapterProofState,
    pub result_state: NetworkActionResultState,
    pub boundary_reasons: Vec<NetworkActionResultBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkActionResultRequiredArtifact>,
    pub adapter_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub adapter_result_accepted: bool,
    pub enforcement_command_published: bool,
    pub host_mutation_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkActionResultError {
    EmptyActionResultRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyTargetRef,
    EmptyRequiredArtifactRef(NetworkActionResultRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    HostMutationClaimRejected,
    EnforcementCommandPublishedRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedActionResultInput {
    action_result_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    target_ref: String,
}

struct NetworkActionResultArtifactRefs {
    adapter_proof_ref: Option<String>,
    apply_artifact_ref: Option<String>,
    result_artifact_ref: Option<String>,
    audit_event_ref: Option<String>,
}

pub fn plan_network_action_result_state(
    input: NetworkActionResultInput,
) -> Result<NetworkActionResultProof, NetworkActionResultError> {
    normalize::reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkActionResultError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize::normalize_action_result_input(&input)?;
    let artifacts = normalize::normalize_artifact_refs(&input)?;
    let missing_required_artifacts = boundaries::missing_required_artifacts(&artifacts);
    let boundary_reasons =
        boundaries::boundary_reasons(&input, missing_required_artifacts.is_empty());
    let result_state = boundaries::result_state(&input, &boundary_reasons);
    let adapter_result_accepted = matches!(
        result_state,
        NetworkActionResultState::Blocked | NetworkActionResultState::Terminated
    );
    let NetworkActionResultInput {
        action_result_ref: _action_result_ref,
        policy_mapping,
        requested_action,
        target_kind,
        target_ref: _,
        capability_state,
        adapter_proof_state,
        adapter_proof_ref: _,
        apply_artifact_ref: _,
        result_artifact_ref: _,
        audit_event_ref: _,
        dry_run: _,
        exact_url_claimed: _,
        decrypted_payload_claimed: _,
        page_content_claimed: _,
        host_mutation_claimed: _,
        enforcement_command_published: _,
    } = input;

    Ok(NetworkActionResultProof {
        action_result_ref: normalized.action_result_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_mapping.evidence_grade,
        requested_action,
        target_kind,
        target_ref: normalized.target_ref,
        capability_state,
        adapter_proof_state,
        result_state,
        boundary_reasons,
        missing_required_artifacts,
        adapter_proof_ref: artifacts.adapter_proof_ref,
        apply_artifact_ref: artifacts.apply_artifact_ref,
        result_artifact_ref: artifacts.result_artifact_ref,
        audit_event_ref: artifacts.audit_event_ref,
        adapter_result_accepted,
        enforcement_command_published: false,
        host_mutation_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}
