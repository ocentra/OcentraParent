use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
    windows_wfp_gate::{
        plan_network_windows_wfp_gate, NetworkWindowsWfpGateBoundaryReason,
        NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateInput,
        NetworkWindowsWfpGateProof, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_windows_wfp_gate_status::{
    NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatus,
    NetworkWindowsWfpGateStatusState,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_windows_wfp_gate_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_windows_wfp_gate_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_WINDOWS_WFP_GATE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkWindowsWfpGateStatusReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(()) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    constants::network_flow::ERROR_NETWORK_WINDOWS_WFP_GATE_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_windows_wfp_gate_status_payload() -> Result<LogFields, ()> {
    let proof = plan_network_windows_wfp_gate(gate_input()).map_err(|_error| ())?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn status_from_proof(proof: &NetworkWindowsWfpGateProof) -> NetworkWindowsWfpGateStatus {
    NetworkWindowsWfpGateStatus {
        status_ref: constants::network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF.to_string(),
        wfp_gate_ref: proof.wfp_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        local_ai_result_ref: proof.local_ai_result_ref.clone(),
        target_ref: proof.target_ref.clone(),
        wfp_provider_ref: proof.wfp_provider_ref.clone(),
        wfp_layer_ref: proof.wfp_layer_ref.clone(),
        capability_state: protocol_capability_state(proof.capability_state),
        gate_state: protocol_gate_state(proof.gate_state),
        boundary_reasons: proof.boundary_reasons.iter().map(boundary_reason).collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(required_artifact)
            .collect(),
        administrator_permission_proof_ref: proof.administrator_permission_proof_ref.clone(),
        driver_signing_proof_ref: proof.driver_signing_proof_ref.clone(),
        driver_package_proof_ref: proof.driver_package_proof_ref.clone(),
        provider_registration_plan_ref: proof.provider_registration_plan_ref.clone(),
        layer_capability_matrix_ref: proof.layer_capability_matrix_ref.clone(),
        rollback_plan_ref: proof.rollback_plan_ref.clone(),
        lab_result_artifact_ref: proof.lab_result_artifact_ref.clone(),
        audit_event_ref: proof.audit_event_ref.clone(),
        wfp_lab_proof_ready: proof.wfp_lab_proof_ready,
        adapter_apply_authorized: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
        live_driver_install_claimed: proof.live_driver_install_claimed,
        callout_registration_claimed: proof.callout_registration_claimed,
        packet_block_claimed: proof.packet_block_claimed,
        kernel_payload_inspection_claimed: proof.kernel_payload_inspection_claimed,
        command_invocation_claimed: false,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
    }
}

fn gate_input() -> NetworkWindowsWfpGateInput {
    NetworkWindowsWfpGateInput {
        wfp_gate_ref: constants::network_flow::TEST_WINDOWS_WFP_GATE_REF.to_string(),
        policy_mapping: policy_mapping(),
        target_ref: constants::network_flow::TEST_WINDOWS_WFP_TARGET_REF.to_string(),
        wfp_provider_ref: constants::network_flow::TEST_WINDOWS_WFP_PROVIDER_REF.to_string(),
        wfp_layer_ref: constants::network_flow::TEST_WINDOWS_WFP_LAYER_REF.to_string(),
        capability_state: NetworkWindowsWfpGateCapabilityState::LabReady,
        administrator_permission_proof_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_ADMIN_PERMISSION_PROOF_REF.to_string(),
        ),
        driver_signing_proof_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_DRIVER_SIGNING_PROOF_REF.to_string(),
        ),
        driver_package_proof_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_DRIVER_PACKAGE_PROOF_REF.to_string(),
        ),
        provider_registration_plan_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_PROVIDER_REGISTRATION_PLAN_REF.to_string(),
        ),
        layer_capability_matrix_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_LAYER_CAPABILITY_MATRIX_REF.to_string(),
        ),
        rollback_plan_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_ROLLBACK_PLAN_REF.to_string(),
        ),
        lab_result_artifact_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_LAB_RESULT_ARTIFACT_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_AUDIT_EVENT_REF.to_string(),
        ),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        live_driver_install_claimed: false,
        callout_registration_claimed: false,
        packet_block_claimed: false,
        kernel_payload_inspection_claimed: false,
        command_invocation_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: constants::network_flow::TEST_WINDOWS_WFP_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_WINDOWS_WFP_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![constants::network_flow::TEST_WINDOWS_WFP_EVIDENCE_REF.to_string()],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_LOCAL_AI_RESULT_REF.to_string(),
        ),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    })
    .unwrap_or_else(|_| panic!("{}", constants::error::AGENT_EVENT_SERIALIZES))
}

fn protocol_capability_state(
    state: NetworkWindowsWfpGateCapabilityState,
) -> NetworkWindowsWfpGateCapabilityStatusState {
    match state {
        NetworkWindowsWfpGateCapabilityState::LabReady => {
            NetworkWindowsWfpGateCapabilityStatusState::LabReady
        }
        NetworkWindowsWfpGateCapabilityState::ManualRequired => {
            NetworkWindowsWfpGateCapabilityStatusState::ManualRequired
        }
        NetworkWindowsWfpGateCapabilityState::Unavailable => {
            NetworkWindowsWfpGateCapabilityStatusState::Unavailable
        }
    }
}

fn protocol_gate_state(state: NetworkWindowsWfpGateState) -> NetworkWindowsWfpGateStatusState {
    match state {
        NetworkWindowsWfpGateState::ResearchOnly => NetworkWindowsWfpGateStatusState::ResearchOnly,
        NetworkWindowsWfpGateState::ManualRequired => {
            NetworkWindowsWfpGateStatusState::ManualRequired
        }
        NetworkWindowsWfpGateState::Unavailable => NetworkWindowsWfpGateStatusState::Unavailable,
        NetworkWindowsWfpGateState::LabProofReady => {
            NetworkWindowsWfpGateStatusState::LabProofReady
        }
    }
}

fn boundary_reason(reason: &NetworkWindowsWfpGateBoundaryReason) -> String {
    match reason {
        NetworkWindowsWfpGateBoundaryReason::ResearchOnlyRequested => {
            constants::network_flow::WFP_BOUNDARY_RESEARCH_ONLY_REQUESTED
        }
        NetworkWindowsWfpGateBoundaryReason::CapabilityManualRequired => {
            constants::network_flow::WFP_BOUNDARY_CAPABILITY_MANUAL_REQUIRED
        }
        NetworkWindowsWfpGateBoundaryReason::CapabilityUnavailable => {
            constants::network_flow::WFP_BOUNDARY_CAPABILITY_UNAVAILABLE
        }
        NetworkWindowsWfpGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            constants::network_flow::WFP_BOUNDARY_EVIDENCE_GRADE_BELOW_PROOF_THRESHOLD
        }
        NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved => {
            constants::network_flow::WFP_BOUNDARY_POLICY_NOT_WFP_APPROVED
        }
        NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact => {
            constants::network_flow::WFP_BOUNDARY_MISSING_REQUIRED_ARTIFACT
        }
    }
    .to_string()
}

fn required_artifact(artifact: &NetworkWindowsWfpRequiredArtifact) -> String {
    match artifact {
        NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof => {
            constants::network_flow::WFP_ARTIFACT_ADMINISTRATOR_PERMISSION_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::DriverSigningProof => {
            constants::network_flow::WFP_ARTIFACT_DRIVER_SIGNING_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::DriverPackageProof => {
            constants::network_flow::WFP_ARTIFACT_DRIVER_PACKAGE_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan => {
            constants::network_flow::WFP_ARTIFACT_PROVIDER_REGISTRATION_PLAN
        }
        NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix => {
            constants::network_flow::WFP_ARTIFACT_LAYER_CAPABILITY_MATRIX
        }
        NetworkWindowsWfpRequiredArtifact::RollbackPlan => {
            constants::network_flow::WFP_ARTIFACT_ROLLBACK_PLAN
        }
        NetworkWindowsWfpRequiredArtifact::LabResultArtifact => {
            constants::network_flow::WFP_ARTIFACT_LAB_RESULT_ARTIFACT
        }
        NetworkWindowsWfpRequiredArtifact::AuditEvent => {
            constants::network_flow::WFP_ARTIFACT_AUDIT_EVENT
        }
    }
    .to_string()
}
