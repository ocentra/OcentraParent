use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
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
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkActionResultError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_action_result_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts = missing_required_artifacts(&artifacts);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let result_state = result_state(&input, &boundary_reasons);
    let adapter_result_accepted = matches!(
        result_state,
        NetworkActionResultState::Blocked | NetworkActionResultState::Terminated
    );

    Ok(NetworkActionResultProof {
        action_result_ref: normalized.action_result_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: input.policy_mapping.evidence_grade,
        requested_action: input.requested_action,
        target_kind: input.target_kind,
        target_ref: normalized.target_ref,
        capability_state: input.capability_state,
        adapter_proof_state: input.adapter_proof_state,
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

fn normalize_action_result_input(
    input: &NetworkActionResultInput,
) -> Result<NormalizedActionResultInput, NetworkActionResultError> {
    Ok(NormalizedActionResultInput {
        action_result_ref: normalize_ref(&input.action_result_ref)
            .ok_or(NetworkActionResultError::EmptyActionResultRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkActionResultError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkActionResultError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_optional_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
            NetworkActionResultError::EmptyLocalAiResultRef,
        )?,
        target_ref: normalize_ref(&input.target_ref)
            .ok_or(NetworkActionResultError::EmptyTargetRef)?,
    })
}

fn normalize_artifact_refs(
    input: &NetworkActionResultInput,
) -> Result<NetworkActionResultArtifactRefs, NetworkActionResultError> {
    Ok(NetworkActionResultArtifactRefs {
        adapter_proof_ref: normalized_artifact_ref(
            input.adapter_proof_ref.as_deref(),
            NetworkActionResultRequiredArtifact::AdapterProof,
        )?,
        apply_artifact_ref: normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkActionResultRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkActionResultRequiredArtifact::ResultArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkActionResultRequiredArtifact::AuditEvent,
        )?,
    })
}

fn reject_unsupported_claims(
    input: &NetworkActionResultInput,
) -> Result<(), NetworkActionResultError> {
    if input.exact_url_claimed {
        return Err(NetworkActionResultError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkActionResultError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkActionResultError::PageContentClaimRejected);
    }
    if input.host_mutation_claimed {
        return Err(NetworkActionResultError::HostMutationClaimRejected);
    }
    if input.enforcement_command_published {
        return Err(NetworkActionResultError::EnforcementCommandPublishedRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkActionResultInput,
    has_required_artifacts: bool,
) -> Vec<NetworkActionResultBoundaryReason> {
    let mut reasons = Vec::new();
    push_state_reasons(input, &mut reasons);
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkActionResultBoundaryReason::EvidenceGradeBelowApplyThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkActionResultBoundaryReason::PolicyNotAdapterApproved);
    }
    if input.requested_action == NetworkActionResultRequestedAction::TerminateProcess
        && !matches!(
            input.target_kind,
            NetworkActionResultTargetKind::Process | NetworkActionResultTargetKind::App
        )
    {
        reasons.push(NetworkActionResultBoundaryReason::TerminateTargetNotProcessOrApp);
    }
    if !has_required_artifacts {
        reasons.push(NetworkActionResultBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn push_state_reasons(
    input: &NetworkActionResultInput,
    reasons: &mut Vec<NetworkActionResultBoundaryReason>,
) {
    if input.dry_run {
        reasons.push(NetworkActionResultBoundaryReason::DryRunRequested);
    }
    match input.capability_state {
        NetworkActionResultCapabilityState::Supported => {}
        NetworkActionResultCapabilityState::ManualRequired => {
            reasons.push(NetworkActionResultBoundaryReason::CapabilityManualRequired);
        }
        NetworkActionResultCapabilityState::Unavailable => {
            reasons.push(NetworkActionResultBoundaryReason::CapabilityUnavailable);
        }
    }
    match input.adapter_proof_state {
        NetworkActionResultAdapterProofState::ApplyReady => {}
        NetworkActionResultAdapterProofState::DryRun => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofDryRun);
        }
        NetworkActionResultAdapterProofState::ManualRequired => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofManualRequired);
        }
        NetworkActionResultAdapterProofState::Unavailable => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofUnavailable);
        }
    }
}

fn result_state(
    input: &NetworkActionResultInput,
    reasons: &[NetworkActionResultBoundaryReason],
) -> NetworkActionResultState {
    if input.dry_run || input.adapter_proof_state == NetworkActionResultAdapterProofState::DryRun {
        return NetworkActionResultState::DryRun;
    }
    if input.capability_state == NetworkActionResultCapabilityState::Unavailable
        || input.adapter_proof_state == NetworkActionResultAdapterProofState::Unavailable
    {
        return NetworkActionResultState::Unavailable;
    }
    if reasons.is_empty() {
        return match input.requested_action {
            NetworkActionResultRequestedAction::Block => NetworkActionResultState::Blocked,
            NetworkActionResultRequestedAction::TerminateProcess => {
                NetworkActionResultState::Terminated
            }
        };
    }
    NetworkActionResultState::ManualRequired
}

fn missing_required_artifacts(
    artifacts: &NetworkActionResultArtifactRefs,
) -> Vec<NetworkActionResultRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.adapter_proof_ref.as_ref(),
        NetworkActionResultRequiredArtifact::AdapterProof,
    );
    push_missing(
        &mut missing,
        artifacts.apply_artifact_ref.as_ref(),
        NetworkActionResultRequiredArtifact::ApplyArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.result_artifact_ref.as_ref(),
        NetworkActionResultRequiredArtifact::ResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkActionResultRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkActionResultRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkActionResultRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkActionResultError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkActionResultError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkActionResultError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_optional_ref(
    value: Option<&str>,
    empty_error: NetworkActionResultError,
) -> Result<Option<String>, NetworkActionResultError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(empty_error),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkActionResultRequiredArtifact,
) -> Result<Option<String>, NetworkActionResultError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkActionResultError::EmptyRequiredArtifactRef(artifact)),
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
