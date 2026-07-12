use ocentra_network_evidence::windows_wfp_gate::NetworkWindowsWfpGateProof;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_windows_wfp_gate_status::NetworkWindowsWfpGateStatus;

use super::boundary_reason::boundary_reason;
use super::capability_state::{protocol_capability_state, protocol_gate_state};
use super::required_artifact::required_artifact;

pub(super) fn status_from_proof(proof: &NetworkWindowsWfpGateProof) -> NetworkWindowsWfpGateStatus {
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
        boundary_reasons: proof
            .boundary_reasons
            .iter()
            .map(|reason| boundary_reason(reason).to_string())
            .collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(|artifact| required_artifact(artifact).to_string())
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
