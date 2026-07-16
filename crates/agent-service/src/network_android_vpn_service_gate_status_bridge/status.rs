use ocentra_network_evidence::android_vpn_service_gate::NetworkAndroidVpnServiceGateProof;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::NetworkAndroidVpnServiceGateStatus;

use super::boundary_reason::boundary_reason;
use super::capability_state::{protocol_capability_state, protocol_gate_state};
use super::required_artifact::required_artifact;

pub(super) fn status_from_proof(
    proof: &NetworkAndroidVpnServiceGateProof,
) -> NetworkAndroidVpnServiceGateStatus {
    NetworkAndroidVpnServiceGateStatus {
        status_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF.to_string(),
        android_vpn_service_gate_ref: proof.android_vpn_service_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        local_ai_result_ref: proof.local_ai_result_ref.clone(),
        package_ref: proof.package_ref.clone(),
        vpn_service_ref: proof.vpn_service_ref.clone(),
        capability_state: protocol_capability_state(proof.capability_state),
        gate_state: protocol_gate_state(proof.gate_state),
        boundary_reasons: proof.boundary_reasons.iter().map(boundary_reason).collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(required_artifact)
            .collect(),
        vpn_service_declaration_ref: proof.vpn_service_declaration_ref.clone(),
        user_consent_proof_ref: proof.user_consent_proof_ref.clone(),
        physical_device_proof_ref: proof.physical_device_proof_ref.clone(),
        package_identity_proof_ref: proof.package_identity_proof_ref.clone(),
        virtual_interface_proof_ref: proof.virtual_interface_proof_ref.clone(),
        traffic_observation_proof_ref: proof.traffic_observation_proof_ref.clone(),
        rollback_plan_ref: proof.rollback_plan_ref.clone(),
        audit_event_ref: proof.audit_event_ref.clone(),
        device_owner_required: proof.device_owner_required,
        device_owner_proof_ref: proof.device_owner_proof_ref.clone(),
        physical_device_proof_ready: proof.physical_device_proof_ready,
        device_owner_authority_proved: proof.device_owner_authority_proved,
        adapter_apply_authorized: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
        emulator_only_product_support_claimed: proof.emulator_only_product_support_claimed,
        live_vpn_tunnel_claimed: proof.live_vpn_tunnel_claimed,
        packet_block_claimed: proof.packet_block_claimed,
        app_package_correlation_claimed: proof.app_package_correlation_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
    }
}
