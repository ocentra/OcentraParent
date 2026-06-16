use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpGateCapabilityState {
    LabReady,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpGateState {
    ResearchOnly,
    ManualRequired,
    Unavailable,
    LabProofReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpRequiredArtifact {
    AdministratorPermissionProof,
    DriverSigningProof,
    DriverPackageProof,
    ProviderRegistrationPlan,
    LayerCapabilityMatrix,
    RollbackPlan,
    LabResultArtifact,
    AuditEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkWindowsWfpGateBoundaryReason {
    ResearchOnlyRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowProofThreshold,
    PolicyNotWfpApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsWfpGateInput {
    pub wfp_gate_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub target_ref: String,
    pub wfp_provider_ref: String,
    pub wfp_layer_ref: String,
    pub capability_state: NetworkWindowsWfpGateCapabilityState,
    pub administrator_permission_proof_ref: Option<String>,
    pub driver_signing_proof_ref: Option<String>,
    pub driver_package_proof_ref: Option<String>,
    pub provider_registration_plan_ref: Option<String>,
    pub layer_capability_matrix_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub lab_result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub research_only: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub live_driver_install_claimed: bool,
    pub callout_registration_claimed: bool,
    pub packet_block_claimed: bool,
    pub kernel_payload_inspection_claimed: bool,
    pub command_invocation_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkWindowsWfpGateProof {
    pub wfp_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub target_ref: String,
    pub wfp_provider_ref: String,
    pub wfp_layer_ref: String,
    pub capability_state: NetworkWindowsWfpGateCapabilityState,
    pub gate_state: NetworkWindowsWfpGateState,
    pub boundary_reasons: Vec<NetworkWindowsWfpGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkWindowsWfpRequiredArtifact>,
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
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkWindowsWfpGateError {
    EmptyWfpGateRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyTargetRef,
    EmptyWfpProviderRef,
    EmptyWfpLayerRef,
    EmptyRequiredArtifactRef(NetworkWindowsWfpRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    LiveDriverInstallClaimRejected,
    CalloutRegistrationClaimRejected,
    PacketBlockClaimRejected,
    KernelPayloadInspectionClaimRejected,
    CommandInvocationRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedWindowsWfpGateInput {
    wfp_gate_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    target_ref: String,
    wfp_provider_ref: String,
    wfp_layer_ref: String,
}

struct NetworkWindowsWfpArtifactRefs {
    administrator_permission_proof_ref: Option<String>,
    driver_signing_proof_ref: Option<String>,
    driver_package_proof_ref: Option<String>,
    provider_registration_plan_ref: Option<String>,
    layer_capability_matrix_ref: Option<String>,
    rollback_plan_ref: Option<String>,
    lab_result_artifact_ref: Option<String>,
    audit_event_ref: Option<String>,
}

pub fn plan_network_windows_wfp_gate(
    input: NetworkWindowsWfpGateInput,
) -> Result<NetworkWindowsWfpGateProof, NetworkWindowsWfpGateError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkWindowsWfpGateError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_windows_wfp_gate_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts = missing_required_artifacts(&artifacts);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let gate_state = gate_state(
        input.research_only,
        input.capability_state,
        &boundary_reasons,
    );
    let wfp_lab_proof_ready = gate_state == NetworkWindowsWfpGateState::LabProofReady;
    let policy_evidence_grade = input.policy_mapping.evidence_grade;
    let capability_state = input.capability_state;
    drop(input);

    Ok(NetworkWindowsWfpGateProof {
        wfp_gate_ref: normalized.wfp_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_evidence_grade,
        target_ref: normalized.target_ref,
        wfp_provider_ref: normalized.wfp_provider_ref,
        wfp_layer_ref: normalized.wfp_layer_ref,
        capability_state,
        gate_state,
        boundary_reasons,
        missing_required_artifacts,
        administrator_permission_proof_ref: artifacts.administrator_permission_proof_ref,
        driver_signing_proof_ref: artifacts.driver_signing_proof_ref,
        driver_package_proof_ref: artifacts.driver_package_proof_ref,
        provider_registration_plan_ref: artifacts.provider_registration_plan_ref,
        layer_capability_matrix_ref: artifacts.layer_capability_matrix_ref,
        rollback_plan_ref: artifacts.rollback_plan_ref,
        lab_result_artifact_ref: artifacts.lab_result_artifact_ref,
        audit_event_ref: artifacts.audit_event_ref,
        wfp_lab_proof_ready,
        adapter_apply_authorized: false,
        enforcement_command_published: false,
        live_driver_install_claimed: false,
        callout_registration_claimed: false,
        packet_block_claimed: false,
        kernel_payload_inspection_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

fn normalize_windows_wfp_gate_input(
    input: &NetworkWindowsWfpGateInput,
) -> Result<NormalizedWindowsWfpGateInput, NetworkWindowsWfpGateError> {
    Ok(NormalizedWindowsWfpGateInput {
        wfp_gate_ref: normalize_ref(&input.wfp_gate_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        target_ref: normalize_ref(&input.target_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyTargetRef)?,
        wfp_provider_ref: normalize_ref(&input.wfp_provider_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpProviderRef)?,
        wfp_layer_ref: normalize_ref(&input.wfp_layer_ref)
            .ok_or(NetworkWindowsWfpGateError::EmptyWfpLayerRef)?,
    })
}

fn normalize_artifact_refs(
    input: &NetworkWindowsWfpGateInput,
) -> Result<NetworkWindowsWfpArtifactRefs, NetworkWindowsWfpGateError> {
    Ok(NetworkWindowsWfpArtifactRefs {
        administrator_permission_proof_ref: normalized_artifact_ref(
            input.administrator_permission_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof,
        )?,
        driver_signing_proof_ref: normalized_artifact_ref(
            input.driver_signing_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::DriverSigningProof,
        )?,
        driver_package_proof_ref: normalized_artifact_ref(
            input.driver_package_proof_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::DriverPackageProof,
        )?,
        provider_registration_plan_ref: normalized_artifact_ref(
            input.provider_registration_plan_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan,
        )?,
        layer_capability_matrix_ref: normalized_artifact_ref(
            input.layer_capability_matrix_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::RollbackPlan,
        )?,
        lab_result_artifact_ref: normalized_artifact_ref(
            input.lab_result_artifact_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::LabResultArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkWindowsWfpRequiredArtifact::AuditEvent,
        )?,
    })
}

fn reject_unsupported_claims(
    input: &NetworkWindowsWfpGateInput,
) -> Result<(), NetworkWindowsWfpGateError> {
    if input.exact_url_claimed {
        return Err(NetworkWindowsWfpGateError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkWindowsWfpGateError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkWindowsWfpGateError::PageContentClaimRejected);
    }
    if input.live_driver_install_claimed {
        return Err(NetworkWindowsWfpGateError::LiveDriverInstallClaimRejected);
    }
    if input.callout_registration_claimed {
        return Err(NetworkWindowsWfpGateError::CalloutRegistrationClaimRejected);
    }
    if input.packet_block_claimed {
        return Err(NetworkWindowsWfpGateError::PacketBlockClaimRejected);
    }
    if input.kernel_payload_inspection_claimed {
        return Err(NetworkWindowsWfpGateError::KernelPayloadInspectionClaimRejected);
    }
    if input.command_invocation_claimed {
        return Err(NetworkWindowsWfpGateError::CommandInvocationRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkWindowsWfpGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkWindowsWfpGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkWindowsWfpGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkWindowsWfpGateCapabilityState::ManualRequired => {
            reasons.push(NetworkWindowsWfpGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkWindowsWfpGateCapabilityState::Unavailable => {
            reasons.push(NetworkWindowsWfpGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkWindowsWfpGateCapabilityState::LabReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkWindowsWfpGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn gate_state(
    research_only: bool,
    capability_state: NetworkWindowsWfpGateCapabilityState,
    boundary_reasons: &[NetworkWindowsWfpGateBoundaryReason],
) -> NetworkWindowsWfpGateState {
    if research_only {
        return NetworkWindowsWfpGateState::ResearchOnly;
    }
    if capability_state == NetworkWindowsWfpGateCapabilityState::Unavailable {
        return NetworkWindowsWfpGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkWindowsWfpGateState::LabProofReady
    } else {
        NetworkWindowsWfpGateState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkWindowsWfpArtifactRefs,
) -> Vec<NetworkWindowsWfpRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.administrator_permission_proof_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof,
    );
    push_missing(
        &mut missing,
        artifacts.driver_signing_proof_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::DriverSigningProof,
    );
    push_missing(
        &mut missing,
        artifacts.driver_package_proof_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::DriverPackageProof,
    );
    push_missing(
        &mut missing,
        artifacts.provider_registration_plan_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan,
    );
    push_missing(
        &mut missing,
        artifacts.layer_capability_matrix_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.lab_result_artifact_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::LabResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkWindowsWfpRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkWindowsWfpRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkWindowsWfpRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkWindowsWfpGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkWindowsWfpGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkWindowsWfpGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkWindowsWfpGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkWindowsWfpGateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkWindowsWfpRequiredArtifact,
) -> Result<Option<String>, NetworkWindowsWfpGateError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkWindowsWfpGateError::EmptyRequiredArtifactRef(artifact),
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
