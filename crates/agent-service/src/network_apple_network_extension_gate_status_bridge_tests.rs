use ocentra_parent_agent_protocol::{
    constants,
    network_apple_network_extension_gate_status::{
        NetworkAppleNetworkExtensionGateCapabilityStatusState,
        NetworkAppleNetworkExtensionGateStatus, NetworkAppleNetworkExtensionGateStatusState,
        NetworkAppleNetworkExtensionPlatformStatus,
    },
    policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_apple_network_extension_gate_status_bridge::network_apple_network_extension_gate_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_apple_network_extension_gate_status_payload_reports_entitlement_ready_without_execution_claims(
) {
    let payload = network_apple_network_extension_gate_status_payload()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let status: NetworkAppleNetworkExtensionGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
    );

    assert_apple_network_extension_status(&status);
}

#[tokio::test]
async fn websocket_network_apple_network_extension_gate_status_command_reports_payload() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkAppleNetworkExtensionGateStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported
    );
    assert_apple_network_extension_status(&status);
}

fn assert_apple_network_extension_status(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
    );
    assert_eq!(
        status.apple_network_extension_gate_ref,
        constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF
    );
    assert_eq!(
        status.platform,
        NetworkAppleNetworkExtensionPlatformStatus::Ios
    );
    assert_eq!(
        status.capability_state,
        NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady
    );
    assert_eq!(
        status.gate_state,
        NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.apple_entitlement_proof_ready);
    assert!(!status.supervision_required);
    assert!(!status.supervision_authority_proved);
    assert_artifact_refs(status);
    assert_non_claims(status);
}

fn assert_artifact_refs(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert_eq!(
        status.developer_team_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF)
    );
    assert_eq!(
        status.entitlement_approval_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF)
    );
    assert_eq!(
        status.network_extension_declaration_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF)
    );
    assert_eq!(
        status.rollback_plan_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF)
    );
}

fn assert_non_claims(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.simulator_only_product_support_claimed);
    assert!(!status.live_network_extension_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.app_level_control_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED
            .to_string(),
        sent_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet,
        payload: Default::default(),
    }
}

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::LogFields,
    field: &str,
) -> TStatus {
    match payload.get(field) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
