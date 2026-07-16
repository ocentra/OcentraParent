use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    apple_network_extension_gate::{
        NetworkAppleNetworkExtensionCapabilityState, NetworkAppleNetworkExtensionGateInput,
        NetworkAppleNetworkExtensionPlatform,
    },
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn gate_input() -> NetworkAppleNetworkExtensionGateInput {
    NetworkAppleNetworkExtensionGateInput {
        apple_network_extension_gate_ref:
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF.to_string(),
        policy_mapping: policy_mapping(),
        platform: NetworkAppleNetworkExtensionPlatform::Ios,
        bundle_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_BUNDLE_REF.to_string(),
        network_extension_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_REF
            .to_string(),
        capability_state: NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady,
        developer_team_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF
                .to_string(),
        ),
        entitlement_approval_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF
                .to_string(),
        ),
        provisioning_profile_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_PROVISIONING_PROFILE_PROOF_REF
                .to_string(),
        ),
        signing_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_SIGNING_PROOF_REF.to_string(),
        ),
        device_or_testflight_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVICE_OR_TESTFLIGHT_PROOF_REF
                .to_string(),
        ),
        network_extension_declaration_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF.to_string(),
        ),
        extension_configuration_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_CONFIGURATION_PROOF_REF
                .to_string(),
        ),
        rollback_plan_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_AUDIT_EVENT_REF.to_string(),
        ),
        supervision_required: false,
        supervision_or_mdm_proof_ref: None,
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        simulator_only_product_support_claimed: false,
        live_network_extension_claimed: false,
        packet_block_claimed: false,
        app_level_control_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref:
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_POLICY_DECISION_REF.to_string(),
        parent_rule_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_PARENT_RULE_REF
            .to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_EVIDENCE_REF.to_string(),
        ],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_LOCAL_AI_RESULT_REF.to_string(),
        ),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    })
    .expect_value(constants::event_id::NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED)
}
