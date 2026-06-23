use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::{
    NetworkAndroidVpnServiceGateCapabilityStatusState, NetworkAndroidVpnServiceGateStatus,
    NetworkAndroidVpnServiceGateStatusState,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_android_vpn_service_gate_status_bridge::network_android_vpn_service_gate_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_android_vpn_service_gate_status_payload_reports_physical_device_ready_without_execution_claims(
) {
    let payload = network_android_vpn_service_gate_status_payload().unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    let status: NetworkAndroidVpnServiceGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
    );

    assert_android_vpn_service_status(&status);
}

#[tokio::test]
async fn websocket_network_android_vpn_service_gate_status_command_reports_payload() {
    let body = serde_json::to_string(&command_envelope()).unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkAndroidVpnServiceGateStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported
    );
    assert_android_vpn_service_status(&status);
}

fn assert_android_vpn_service_status(status: &NetworkAndroidVpnServiceGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF
    );
    assert_eq!(
        status.android_vpn_service_gate_ref,
        constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF
    );
    assert_eq!(
        status.capability_state,
        NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady
    );
    assert_eq!(
        status.gate_state,
        NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.physical_device_proof_ready);
    assert!(!status.device_owner_required);
    assert!(!status.device_owner_authority_proved);
    assert_artifact_refs(status);
    assert_non_claims(status);
}

fn assert_artifact_refs(status: &NetworkAndroidVpnServiceGateStatus) {
    assert_eq!(
        status.vpn_service_declaration_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_SERVICE_DECLARATION_REF)
    );
    assert_eq!(
        status.user_consent_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF)
    );
    assert_eq!(
        status.physical_device_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF)
    );
    assert_eq!(
        status.rollback_plan_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_ROLLBACK_PLAN_REF)
    );
}

fn assert_non_claims(status: &NetworkAndroidVpnServiceGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.emulator_only_product_support_claimed);
    assert!(!status.live_vpn_tunnel_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.app_package_correlation_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS_REPORTED
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
        command: AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet,
        payload: Default::default(),
    }
}

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &str,
) -> TStatus {
    match payload.get(field) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_else(|error| {
            panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
        }),
        other => panic!(
            "{}: missing or non-string payload field {field}: {other:?}",
            constants::error::AGENT_EVENT_SERIALIZES
        ),
    }
}
