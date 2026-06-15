use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxAdapterKind {
    Nftables,
    Ebpf,
    Tun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxAdapterCapabilityState {
    DistroReady,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxAdapterGateState {
    ResearchOnly,
    ManualRequired,
    Unavailable,
    DistroProofReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxAdapterRequiredArtifact {
    DistroKernelProof,
    PermissionProof,
    AdapterApiCapabilityProof,
    AdapterPlanProof,
    ServiceManagerScopeProof,
    RollbackPlan,
    LabResultArtifact,
    AuditEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLinuxAdapterGateBoundaryReason {
    ResearchOnlyRequested,
    CapabilityManualRequired,
    CapabilityUnavailable,
    EvidenceGradeBelowProofThreshold,
    PolicyNotLinuxAdapterApproved,
    MissingRequiredArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLinuxAdapterGateInput {
    pub linux_adapter_gate_ref: String,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub adapter_kind: NetworkLinuxAdapterKind,
    pub distro_ref: String,
    pub kernel_ref: String,
    pub capability_state: NetworkLinuxAdapterCapabilityState,
    pub distro_kernel_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub adapter_api_capability_proof_ref: Option<String>,
    pub adapter_plan_proof_ref: Option<String>,
    pub service_manager_scope_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub lab_result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub research_only: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub generic_linux_support_claimed: bool,
    pub live_adapter_install_claimed: bool,
    pub packet_filtering_claimed: bool,
    pub kernel_hook_loaded_claimed: bool,
    pub tun_interface_mutation_claimed: bool,
    pub service_manager_install_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLinuxAdapterGateProof {
    pub linux_adapter_gate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub adapter_kind: NetworkLinuxAdapterKind,
    pub distro_ref: String,
    pub kernel_ref: String,
    pub capability_state: NetworkLinuxAdapterCapabilityState,
    pub gate_state: NetworkLinuxAdapterGateState,
    pub boundary_reasons: Vec<NetworkLinuxAdapterGateBoundaryReason>,
    pub missing_required_artifacts: Vec<NetworkLinuxAdapterRequiredArtifact>,
    pub distro_kernel_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub adapter_api_capability_proof_ref: Option<String>,
    pub adapter_plan_proof_ref: Option<String>,
    pub service_manager_scope_proof_ref: Option<String>,
    pub rollback_plan_ref: Option<String>,
    pub lab_result_artifact_ref: Option<String>,
    pub audit_event_ref: Option<String>,
    pub distro_proof_ready: bool,
    pub adapter_apply_authorized: bool,
    pub enforcement_command_published: bool,
    pub generic_linux_support_claimed: bool,
    pub live_adapter_install_claimed: bool,
    pub packet_filtering_claimed: bool,
    pub kernel_hook_loaded_claimed: bool,
    pub tun_interface_mutation_claimed: bool,
    pub service_manager_install_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLinuxAdapterGateError {
    EmptyLinuxAdapterGateRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    EmptyDistroRef,
    EmptyKernelRef,
    EmptyRequiredArtifactRef(NetworkLinuxAdapterRequiredArtifact),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    GenericLinuxSupportClaimRejected,
    LiveAdapterInstallClaimRejected,
    PacketFilteringClaimRejected,
    KernelHookLoadedClaimRejected,
    TunInterfaceMutationClaimRejected,
    ServiceManagerInstallClaimRejected,
    PolicyMappingAuthorityRejected,
}

struct NormalizedLinuxAdapterGateInput {
    linux_adapter_gate_ref: String,
    policy_decision_ref: String,
    parent_rule_ref: String,
    evidence_refs: Vec<String>,
    local_ai_result_ref: Option<String>,
    distro_ref: String,
    kernel_ref: String,
}

struct NetworkLinuxAdapterArtifactRefs {
    distro_kernel_proof_ref: Option<String>,
    permission_proof_ref: Option<String>,
    adapter_api_capability_proof_ref: Option<String>,
    adapter_plan_proof_ref: Option<String>,
    service_manager_scope_proof_ref: Option<String>,
    rollback_plan_ref: Option<String>,
    lab_result_artifact_ref: Option<String>,
    audit_event_ref: Option<String>,
}

pub fn plan_network_linux_adapter_gate(
    input: NetworkLinuxAdapterGateInput,
) -> Result<NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateError> {
    reject_unsupported_claims(&input)?;
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkLinuxAdapterGateError::PolicyMappingAuthorityRejected);
    }

    let normalized = normalize_linux_adapter_gate_input(&input)?;
    let artifacts = normalize_artifact_refs(&input)?;
    let missing_required_artifacts = missing_required_artifacts(&artifacts);
    let boundary_reasons = boundary_reasons(&input, missing_required_artifacts.is_empty());
    let gate_state = gate_state(
        input.research_only,
        input.capability_state,
        &boundary_reasons,
    );
    let distro_proof_ready = gate_state == NetworkLinuxAdapterGateState::DistroProofReady;
    let policy_mapping = input.policy_mapping;
    let adapter_kind = input.adapter_kind;
    let capability_state = input.capability_state;

    Ok(NetworkLinuxAdapterGateProof {
        linux_adapter_gate_ref: normalized.linux_adapter_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_mapping.evidence_grade,
        adapter_kind,
        distro_ref: normalized.distro_ref,
        kernel_ref: normalized.kernel_ref,
        capability_state,
        gate_state,
        boundary_reasons,
        missing_required_artifacts,
        distro_kernel_proof_ref: artifacts.distro_kernel_proof_ref,
        permission_proof_ref: artifacts.permission_proof_ref,
        adapter_api_capability_proof_ref: artifacts.adapter_api_capability_proof_ref,
        adapter_plan_proof_ref: artifacts.adapter_plan_proof_ref,
        service_manager_scope_proof_ref: artifacts.service_manager_scope_proof_ref,
        rollback_plan_ref: artifacts.rollback_plan_ref,
        lab_result_artifact_ref: artifacts.lab_result_artifact_ref,
        audit_event_ref: artifacts.audit_event_ref,
        distro_proof_ready,
        adapter_apply_authorized: false,
        enforcement_command_published: false,
        generic_linux_support_claimed: false,
        live_adapter_install_claimed: false,
        packet_filtering_claimed: false,
        kernel_hook_loaded_claimed: false,
        tun_interface_mutation_claimed: false,
        service_manager_install_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

fn normalize_linux_adapter_gate_input(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<NormalizedLinuxAdapterGateInput, NetworkLinuxAdapterGateError> {
    Ok(NormalizedLinuxAdapterGateInput {
        linux_adapter_gate_ref: normalize_ref(&input.linux_adapter_gate_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyLinuxAdapterGateRef)?,
        policy_decision_ref: normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyParentRuleRef)?,
        evidence_refs: normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        distro_ref: normalize_ref(&input.distro_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyDistroRef)?,
        kernel_ref: normalize_ref(&input.kernel_ref)
            .ok_or(NetworkLinuxAdapterGateError::EmptyKernelRef)?,
    })
}

fn normalize_artifact_refs(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<NetworkLinuxAdapterArtifactRefs, NetworkLinuxAdapterGateError> {
    Ok(NetworkLinuxAdapterArtifactRefs {
        distro_kernel_proof_ref: normalized_artifact_ref(
            input.distro_kernel_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::DistroKernelProof,
        )?,
        permission_proof_ref: normalized_artifact_ref(
            input.permission_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::PermissionProof,
        )?,
        adapter_api_capability_proof_ref: normalized_artifact_ref(
            input.adapter_api_capability_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof,
        )?,
        adapter_plan_proof_ref: normalized_artifact_ref(
            input.adapter_plan_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof,
        )?,
        service_manager_scope_proof_ref: normalized_artifact_ref(
            input.service_manager_scope_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::RollbackPlan,
        )?,
        lab_result_artifact_ref: normalized_artifact_ref(
            input.lab_result_artifact_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::LabResultArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AuditEvent,
        )?,
    })
}

fn reject_unsupported_claims(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<(), NetworkLinuxAdapterGateError> {
    if input.exact_url_claimed {
        return Err(NetworkLinuxAdapterGateError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLinuxAdapterGateError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLinuxAdapterGateError::PageContentClaimRejected);
    }
    if input.generic_linux_support_claimed {
        return Err(NetworkLinuxAdapterGateError::GenericLinuxSupportClaimRejected);
    }
    if input.live_adapter_install_claimed {
        return Err(NetworkLinuxAdapterGateError::LiveAdapterInstallClaimRejected);
    }
    if input.packet_filtering_claimed {
        return Err(NetworkLinuxAdapterGateError::PacketFilteringClaimRejected);
    }
    if input.kernel_hook_loaded_claimed {
        return Err(NetworkLinuxAdapterGateError::KernelHookLoadedClaimRejected);
    }
    if input.tun_interface_mutation_claimed {
        return Err(NetworkLinuxAdapterGateError::TunInterfaceMutationClaimRejected);
    }
    if input.service_manager_install_claimed {
        return Err(NetworkLinuxAdapterGateError::ServiceManagerInstallClaimRejected);
    }
    Ok(())
}

fn boundary_reasons(
    input: &NetworkLinuxAdapterGateInput,
    has_required_artifacts: bool,
) -> Vec<NetworkLinuxAdapterGateBoundaryReason> {
    let mut reasons = Vec::new();
    if input.research_only {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::ResearchOnlyRequested);
    }
    match input.capability_state {
        NetworkLinuxAdapterCapabilityState::ManualRequired => {
            reasons.push(NetworkLinuxAdapterGateBoundaryReason::CapabilityManualRequired);
        }
        NetworkLinuxAdapterCapabilityState::Unavailable => {
            reasons.push(NetworkLinuxAdapterGateBoundaryReason::CapabilityUnavailable);
        }
        NetworkLinuxAdapterCapabilityState::DistroReady => {}
    }
    if input.policy_mapping.evidence_grade != NetworkEvidenceGrade::A {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::EvidenceGradeBelowProofThreshold);
    }
    if input.policy_mapping.mode != NetworkEvidencePolicyMode::DryRun
        || input.policy_mapping.mapped_action != NetworkEvidencePolicyAction::Block
    {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::PolicyNotLinuxAdapterApproved);
    }
    if !has_required_artifacts {
        reasons.push(NetworkLinuxAdapterGateBoundaryReason::MissingRequiredArtifact);
    }
    reasons
}

fn gate_state(
    research_only: bool,
    capability_state: NetworkLinuxAdapterCapabilityState,
    boundary_reasons: &[NetworkLinuxAdapterGateBoundaryReason],
) -> NetworkLinuxAdapterGateState {
    if research_only {
        return NetworkLinuxAdapterGateState::ResearchOnly;
    }
    if capability_state == NetworkLinuxAdapterCapabilityState::Unavailable {
        return NetworkLinuxAdapterGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkLinuxAdapterGateState::DistroProofReady
    } else {
        NetworkLinuxAdapterGateState::ManualRequired
    }
}

fn missing_required_artifacts(
    artifacts: &NetworkLinuxAdapterArtifactRefs,
) -> Vec<NetworkLinuxAdapterRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.distro_kernel_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::DistroKernelProof,
    );
    push_missing(
        &mut missing,
        artifacts.permission_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::PermissionProof,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_api_capability_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_plan_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof,
    );
    push_missing(
        &mut missing,
        artifacts.service_manager_scope_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.lab_result_artifact_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::LabResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkLinuxAdapterRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkLinuxAdapterRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkLinuxAdapterGateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkLinuxAdapterGateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkLinuxAdapterGateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkLinuxAdapterGateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkLinuxAdapterGateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkLinuxAdapterRequiredArtifact,
) -> Result<Option<String>, NetworkLinuxAdapterGateError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkLinuxAdapterGateError::EmptyRequiredArtifactRef(artifact),
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
