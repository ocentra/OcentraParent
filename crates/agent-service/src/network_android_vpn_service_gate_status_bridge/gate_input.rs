use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    android_vpn_service_gate::{
        NetworkAndroidVpnServiceCapabilityState, NetworkAndroidVpnServiceGateInput,
    },
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn gate_input() -> NetworkAndroidVpnServiceGateInput {
    NetworkAndroidVpnServiceGateInput {
        android_vpn_service_gate_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF
            .to_string(),
        policy_mapping: policy_mapping(),
        package_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_PACKAGE_REF.to_string(),
        vpn_service_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_REF.to_string(),
        capability_state: NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady,
        vpn_service_declaration_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_DECLARATION_REF.to_string(),
        ),
        user_consent_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF.to_string(),
        ),
        physical_device_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF.to_string(),
        ),
        package_identity_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_PACKAGE_IDENTITY_PROOF_REF.to_string(),
        ),
        virtual_interface_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_VIRTUAL_INTERFACE_PROOF_REF.to_string(),
        ),
        traffic_observation_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_TRAFFIC_OBSERVATION_PROOF_REF.to_string(),
        ),
        rollback_plan_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_ROLLBACK_PLAN_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_AUDIT_EVENT_REF.to_string(),
        ),
        device_owner_required: false,
        device_owner_proof_ref: None,
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        emulator_only_product_support_claimed: false,
        live_vpn_tunnel_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_PARENT_RULE_REF
            .to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_EVIDENCE_REF.to_string(),
        ],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_LOCAL_AI_RESULT_REF.to_string(),
        ),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    })
    .expect_value(constants::event_id::NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS_REPORTED)
}
