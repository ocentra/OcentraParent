use crate::{
    constants,
    network_windows_wfp_gate_status::{
        NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatus,
        NetworkWindowsWfpGateStatusState,
    },
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn windows_wfp_gate_status_serializes_to_camel_case_contract_shape() {
    let status = NetworkWindowsWfpGateStatus {
        status_ref: constants::network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF.to_string(),
        wfp_gate_ref: constants::network_flow::TEST_WINDOWS_WFP_GATE_REF.to_string(),
        policy_decision_ref: constants::network_flow::TEST_WINDOWS_WFP_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_WINDOWS_WFP_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![constants::network_flow::TEST_WINDOWS_WFP_EVIDENCE_REF.to_string()],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_WINDOWS_WFP_LOCAL_AI_RESULT_REF.to_string(),
        ),
        target_ref: constants::network_flow::TEST_WINDOWS_WFP_TARGET_REF.to_string(),
        wfp_provider_ref: constants::network_flow::TEST_WINDOWS_WFP_PROVIDER_REF.to_string(),
        wfp_layer_ref: constants::network_flow::TEST_WINDOWS_WFP_LAYER_REF.to_string(),
        capability_state: NetworkWindowsWfpGateCapabilityStatusState::LabReady,
        gate_state: NetworkWindowsWfpGateStatusState::LabProofReady,
        wfp_lab_proof_ready: true,
        ..NetworkWindowsWfpGateStatus::default()
    };

    let serialized = serde_json::to_value(status).expect_value("status serializes: {error}");

    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF
    );
    assert_eq!(serialized["capabilityState"], "lab-ready");
    assert_eq!(serialized["gateState"], "lab-proof-ready");
    assert_eq!(serialized["wfpLabProofReady"], true);
    assert_eq!(serialized["adapterApplyAuthorized"], false);
    assert_eq!(serialized["enforcementCommandPublished"], false);
}
