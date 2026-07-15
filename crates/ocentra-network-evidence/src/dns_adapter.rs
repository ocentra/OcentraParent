use serde::{Deserialize, Serialize};

use crate::{DomainNormalizationError, NetworkEvidenceGrade, NetworkEvidencePolicyMapping};

mod artifacts;
mod normalization;
mod policy;
mod redirects;
mod refs;
mod validation;

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
    validation::reject_unsupported_claims(&input)?;
    validation::reject_policy_mapping_authority(&input)?;

    let normalized = normalization::normalize_dns_adapter_input(&input)?;
    let artifacts = normalization::normalize_artifact_refs(&input)?;
    let missing_required_artifacts = artifacts::missing_required_artifacts(&artifacts);
    let boundary_reasons = policy::boundary_reasons(&input, missing_required_artifacts.is_empty());
    let proof_state = policy::proof_state(input.dry_run, input.capability_state, &boundary_reasons);
    let adapter_apply_authorized = proof_state == NetworkDnsAdapterProofState::ApplyReady;
    let NetworkDnsAdapterProofInput {
        dns_adapter_plan_ref: _dns_adapter_plan_ref,
        policy_mapping,
        requested_action,
        target_domain: _,
        redirect_target_domain: _,
        capability_state,
        adapter_authorization_ref: _,
        adapter_capability_proof_ref: _,
        apply_artifact_ref: _,
        result_artifact_ref: _,
        rollback_artifact_ref: _,
        audit_event_ref: _,
        dry_run: _,
        exact_url_claimed: _,
        decrypted_payload_claimed: _,
        page_content_claimed: _,
    } = input;

    Ok(NetworkDnsAdapterProof {
        dns_adapter_plan_ref: normalized.dns_adapter_plan_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_mapping.evidence_grade,
        requested_action,
        target_domain: normalized.target_domain,
        redirect_target_domain: normalized.redirect_target_domain,
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
        host_dns_mutation_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}
