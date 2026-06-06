use ocentra_network_evidence::{
    NetworkLiveCaptureCustodyStatus, NetworkLiveCaptureCustodyStatusState,
    NetworkProductReadinessStatus, NetworkProductReadinessStatusState,
};
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_product_readiness_status_payload::network_product_readiness_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_product_readiness_status_payload_serializes_materializer_outputs() {
    let payload = network_product_readiness_status_payload();
    let live_capture_status: NetworkLiveCaptureCustodyStatus = status_value(
        &payload,
        constants::field::NETWORK_LIVE_CAPTURE_CUSTODY_STATUS,
    );
    let product_status: NetworkProductReadinessStatus =
        status_value(&payload, constants::field::NETWORK_PRODUCT_READINESS_STATUS);

    assert_eq!(
        live_capture_status.state,
        NetworkLiveCaptureCustodyStatusState::CustodyReady
    );
    assert!(!live_capture_status.driver_invoked);
    assert!(!live_capture_status.live_capture_executed);
    assert!(!live_capture_status.raw_artifact_created);
    assert!(!live_capture_status.remote_upload_enabled);
    assert!(!live_capture_status.exact_url_available);
    assert!(!live_capture_status.decrypted_payload_available);
    assert!(!live_capture_status.policy_authority);
    assert!(!live_capture_status.adapter_authority);
    assert_eq!(live_capture_status.enforcement_commands_published, 0);
    assert_eq!(
        product_status.readiness_state,
        NetworkProductReadinessStatusState::ManualRequired
    );
    assert!(product_status.portal_read_model_ready);
    assert!(product_status.retention_export_refs_visible);
    assert_eq!(product_status.platform_manual_required_claims, 1);
    assert_eq!(product_status.platform_entries.len(), 2);
    assert_eq!(
        product_status.platform_entries[0].target,
        ocentra_network_evidence::NetworkPlatformClaimTarget::WindowsFirewall
    );
    assert!(product_status.platform_entries[0].adapter_authorized_by_proof);
    assert!(!product_status.platform_entries[0].enforcement_command_published);
    assert_eq!(
        product_status.platform_entries[1].target,
        ocentra_network_evidence::NetworkPlatformClaimTarget::WindowsWfp
    );
    assert!(!product_status.platform_entries[1].adapter_authorized_by_proof);
    assert!(!product_status.platform_entries[1]
        .missing_required_artifacts
        .is_empty());
    assert!(!product_status.exact_url_available);
    assert!(!product_status.decrypted_payload_available);
    assert!(!product_status.ui_policy_authority);
    assert!(!product_status.portal_adapter_dispatch_claimed);
    assert!(!product_status.live_adapter_execution_claimed);
    assert_eq!(product_status.enforcement_commands_published, 0);
}

#[tokio::test]
async fn websocket_network_product_readiness_status_command_reports_payload() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let product_status: NetworkProductReadinessStatus = status_value(
        &event.payload,
        constants::field::NETWORK_PRODUCT_READINESS_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkProductReadinessStatusReported
    );
    assert_eq!(
        product_status.readiness_state,
        NetworkProductReadinessStatusState::ManualRequired
    );
    assert_eq!(product_status.platform_entries.len(), 2);
    assert!(!product_status.policy_authority);
    assert!(!product_status.adapter_authority);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_PRODUCT_READINESS_STATUS_REPORTED.to_owned(),
        sent_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_owned(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_owned(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_owned(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_owned(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkProductReadinessStatusGet,
        payload: LogFields::new(),
    }
}

fn status_value<TStatus: DeserializeOwned>(payload: &LogFields, field: &str) -> TStatus {
    match payload.get(field) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
