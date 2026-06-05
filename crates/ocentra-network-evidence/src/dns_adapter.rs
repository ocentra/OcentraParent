use serde::{Deserialize, Serialize};

use crate::{
    normalize_domain_with_public_suffix, DomainNormalizationError, NetworkEvidenceGrade,
    NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping, NetworkEvidencePolicyMode,
    PublicSuffixModel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDnsAdapterAction {
    Block,
    Redirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDnsAdapterCapabilityState {
    Supported,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDnsAdapterProofState {
    DryRun,
    ManualRequired,
    Unavailable,
    ApplyReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDnsAdapterRequiredArtifact {
    AdapterAuthorization,
    CapabilityProof,
    ApplyArtifact,
    ResultArtifact,
    RollbackArtifact,
    AuditEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDnsAdapterBoundaryReason {
    DryRunRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowApplyThreshold,
    PolicyNotAdapterApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkDnsAdapterProofInput {
    pub dns_adapter_plan_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub requested_action: NetworkDnsAdapterAction,
    pub target_domain: String,
    pub redirect_target_domain: Option<String>,
    pub capability_state: NetworkDnsAdapterCapabilityState,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkDnsAdapterProof {
    pub dns_adapter_plan_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub requested_action: NetworkDnsAdapterAction,
    pub target_domain: String,
    pub redirect_target_domain: Option<String>,
    pub capability_state: NetworkDnsAdapterCapabilityState,
    pub proof_state: NetworkDnsAdapterProofState,
    pub boundary_reasons: Vec<NetworkDnsAdapterBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkDnsAdapterRequiredArtifact>,
    pub adapter_authorization_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub rollback_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub host_dns_mutation_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkDnsAdapterProofError {
    EmptyDnsAdapterPlanRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyRequiredArtifactRef(NetworkDnsAdapterRequiredArtifact),
    InvalidTargetDomain(DomainNormalizationError),
    InvalidRedirectTargetDomain(DomainNormalizationError),
    MissingRedirectTargetDomain,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedDnsAdapterInput {
    dns_adapter_plan_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    target_domain: String,
    redirect_target_domain: Option<String>,
}

struct NetworkDnsAdapterArtifactRefs {
    adapter_authorization_ref: Option<String>,
    adapter_capability_proof_ref: Option<String>,
    apply_artifact_ref: Option<String>,
    result_artifact_ref: Option<String>,
    rollback_artifact_ref: Option<String>,
    audit_event_ref: Option<String>,
}

pub fn plan_network_dns_adapter_proof(
    input: NetworkDnsAdapterProofInput,
) -> Result<NetworkDnsAdapterProof, NetworkDnsAdapterProofError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkDnsAdapterProofError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_dns_adapter_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts = missing_required_artifacts(&artifacts);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let proof_state = proof_state(input.dry_run, input.capability_state, &boundary_reasons);
    let adapter_apply_authorized = proof_state == NetworkDnsAdapterProofState::ApplyReady;

    Ok(NetworkDnsAdapterProof {
        dns_adapter_plan_ref: normalized.dns_adapter_plan_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: input.policy_mapping.evidence_grade,
        requested_action: input.requested_action,
        target_domain: normalized.target_domain,
        redirect_target_domain: normalized.redirect_target_domain,
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
        host_dns_mutation_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

fn normalize_dns_adapter_input(
    input: &NetworkDnsAdapterProofInput,
) -> Result<NormalizedDnsAdapterInput, NetworkDnsAdapterProofError> {
    Ok(NormalizedDnsAdapterInput {
        dns_adapter_plan_ref: normalize_ref(&input.dns_adapter_plan_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyDnsAdapterPlanRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        target_domain: normalized_target_domain(&input.target_domain)?,
        redirect_target_domain: normalized_redirect_target(input.requested_action, input)?,
    })
}

fn normalize_artifact_refs(
    input: &NetworkDnsAdapterProofInput,
) -> Result<NetworkDnsAdapterArtifactRefs, NetworkDnsAdapterProofError> {
    Ok(NetworkDnsAdapterArtifactRefs {
        adapter_authorization_ref: normalized_artifact_ref(
            input.adapter_authorization_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
        )?,
        adapter_capability_proof_ref: normalized_artifact_ref(
            input.adapter_capability_proof_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::CapabilityProof,
        )?,
        apply_artifact_ref: normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::ResultArtifact,
        )?,
        rollback_artifact_ref: normalized_artifact_ref(
            input.rollback_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::AuditEvent,
        )?,
    })
}

fn reject_unsupported_claims(
    input: &NetworkDnsAdapterProofInput,
) -> Result<(), NetworkDnsAdapterProofError> {
    if input.exact_url_claimed {
        return Err(NetworkDnsAdapterProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkDnsAdapterProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkDnsAdapterProofError::PageContentClaimRejected);
    }
    Ok(())
}

fn normalized_target_domain(input: &str) -> Result<String, NetworkDnsAdapterProofError> {
    normalize_domain_with_public_suffix(input, &PublicSuffixModel::ocentra_fixture())
        .map(|evidence| evidence.normalized_domain)
        .map_err(NetworkDnsAdapterProofError::InvalidTargetDomain)
}

fn normalized_redirect_target(
    action: NetworkDnsAdapterAction,
    input: &NetworkDnsAdapterProofInput,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match (action, input.redirect_target_domain.as_deref()) {
        (NetworkDnsAdapterAction::Redirect, Some(target)) => {
            normalize_domain_with_public_suffix(target, &PublicSuffixModel::ocentra_fixture())
                .map(|evidence| Some(evidence.normalized_domain))
                .map_err(NetworkDnsAdapterProofError::InvalidRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Redirect, None) => {
            Err(NetworkDnsAdapterProofError::MissingRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Block, Some(target)) => {
            normalize_domain_with_public_suffix(target, &PublicSuffixModel::ocentra_fixture())
                .map(|evidence| Some(evidence.normalized_domain))
                .map_err(NetworkDnsAdapterProofError::InvalidRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Block, None) => Ok(None),
    }
}

fn boundary_reasons(
    input: &NetworkDnsAdapterProofInput,
    has_required_artifacts: bool,
) -> Vec<NetworkDnsAdapterBoundaryReason> {
    let mut reasons = Vec::new();
    if input.dry_run {
        reasons.push(NetworkDnsAdapterBoundaryReason::DryRunRequested);
    }
    match input.capability_state {
        NetworkDnsAdapterCapabilityState::ManualRequired => {
            reasons.push(NetworkDnsAdapterBoundaryReason::CapabilityManualRequired);
        }
        NetworkDnsAdapterCapabilityState::Unavailable => {
            reasons.push(NetworkDnsAdapterBoundaryReason::CapabilityUnavailable);
        }
        NetworkDnsAdapterCapabilityState::Supported => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkDnsAdapterBoundaryReason::EvidenceGradeBelowApplyThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkDnsAdapterBoundaryReason::PolicyNotAdapterApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkDnsAdapterBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn proof_state(
    dry_run: bool,
    capability_state: NetworkDnsAdapterCapabilityState,
    boundary_reasons: &[NetworkDnsAdapterBoundaryReason],
) -> NetworkDnsAdapterProofState {
    if dry_run {
        return NetworkDnsAdapterProofState::DryRun;
    }
    if capability_state == NetworkDnsAdapterCapabilityState::Unavailable {
        return NetworkDnsAdapterProofState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkDnsAdapterProofState::ApplyReady
    } else {
        NetworkDnsAdapterProofState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkDnsAdapterArtifactRefs,
) -> Vec<NetworkDnsAdapterRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.adapter_authorization_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_capability_proof_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::CapabilityProof,
    );
    push_missing(
        &mut missing,
        artifacts.apply_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.result_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::ResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkDnsAdapterRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkDnsAdapterRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkDnsAdapterProofError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkDnsAdapterProofError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkDnsAdapterProofError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkDnsAdapterProofError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkDnsAdapterRequiredArtifact,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkDnsAdapterProofError::EmptyRequiredArtifactRef(artifact),
        ),
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
