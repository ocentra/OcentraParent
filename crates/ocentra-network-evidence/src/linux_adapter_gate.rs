use serde::{Deserialize, Serialize};

use crate::{NetworkEvidenceGrade, NetworkEvidencePolicyMapping};

mod artifacts;
mod boundary;
mod normalization;
mod state;
mod validation;

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

pub fn plan_network_linux_adapter_gate(
    input: NetworkLinuxAdapterGateInput,
) -> Result<NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateError> {
    validation::reject_unsupported_claims(&input)?;
    validation::reject_policy_mapping_authority(&input)?;

    let normalized = normalization::normalize_linux_adapter_gate_input(&input)?;
    let artifacts = artifacts::normalize_artifact_refs(&input)?;
    let missing_required_artifacts = artifacts::missing_required_artifacts(&artifacts);
    let boundary_reasons =
        boundary::boundary_reasons(&input, missing_required_artifacts.is_empty());
    let gate_state = state::gate_state(
        input.research_only,
        input.capability_state,
        &boundary_reasons,
    );
    let distro_proof_ready = gate_state == NetworkLinuxAdapterGateState::DistroProofReady;
    let policy_evidence_grade = input.policy_mapping.evidence_grade;
    let adapter_kind = input.adapter_kind;
    let capability_state = input.capability_state;
    drop(input);

    Ok(NetworkLinuxAdapterGateProof {
        linux_adapter_gate_ref: normalized.linux_adapter_gate_ref,
        policy_decision_ref: normalized.policy_decision_ref,
        parent_rule_ref: normalized.parent_rule_ref,
        evidence_refs: normalized.evidence_refs,
        local_ai_result_ref: normalized.local_ai_result_ref,
        evidence_grade: policy_evidence_grade,
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
