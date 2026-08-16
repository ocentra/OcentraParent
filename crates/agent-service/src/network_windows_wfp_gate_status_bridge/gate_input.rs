use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
    windows_wfp_gate::{NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateInput},
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn gate_input() -> NetworkWindowsWfpGateInput {
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
    .expect_value(constants::event_id::NETWORK_WINDOWS_WFP_GATE_STATUS_REPORTED)
}
