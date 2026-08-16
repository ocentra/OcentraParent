use ocentra_network_evidence::apple_network_extension_gate::NetworkAppleNetworkExtensionGateProof;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::NetworkAppleNetworkExtensionGateStatus;

use super::boundary_reason::boundary_reason;
use super::capability_state::{protocol_capability_state, protocol_gate_state, protocol_platform};
use super::required_artifact::required_artifact;

pub(super) fn status_from_proof(
    proof: &NetworkAppleNetworkExtensionGateProof,
) -> NetworkAppleNetworkExtensionGateStatus {
    NetworkAppleNetworkExtensionGateStatus {
        status_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
            .to_string(),
        apple_network_extension_gate_ref: proof.apple_network_extension_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        local_ai_result_ref: proof.local_ai_result_ref.clone(),
        platform: protocol_platform(proof.platform),
        bundle_ref: proof.bundle_ref.clone(),
        network_extension_ref: proof.network_extension_ref.clone(),
        capability_state: protocol_capability_state(proof.capability_state),
        gate_state: protocol_gate_state(proof.gate_state),
        boundary_reasons: proof.boundary_reasons.iter().map(boundary_reason).collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(required_artifact)
            .collect(),
        developer_team_proof_ref: proof.developer_team_proof_ref.clone(),
        entitlement_approval_proof_ref: proof.entitlement_approval_proof_ref.clone(),
        provisioning_profile_proof_ref: proof.provisioning_profile_proof_ref.clone(),
        signing_proof_ref: proof.signing_proof_ref.clone(),
        device_or_testflight_proof_ref: proof.device_or_testflight_proof_ref.clone(),
        network_extension_declaration_ref: proof.network_extension_declaration_ref.clone(),
        extension_configuration_proof_ref: proof.extension_configuration_proof_ref.clone(),
        rollback_plan_ref: proof.rollback_plan_ref.clone(),
        audit_event_ref: proof.audit_event_ref.clone(),
        supervision_required: proof.supervision_required,
        supervision_or_mdm_proof_ref: proof.supervision_or_mdm_proof_ref.clone(),
        apple_entitlement_proof_ready: proof.apple_entitlement_proof_ready,
        supervision_authority_proved: proof.supervision_authority_proved,
        adapter_apply_authorized: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
        simulator_only_product_support_claimed: proof.simulator_only_product_support_claimed,
        live_network_extension_claimed: proof.live_network_extension_claimed,
        packet_block_claimed: proof.packet_block_claimed,
        app_level_control_claimed: proof.app_level_control_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
    }
}
