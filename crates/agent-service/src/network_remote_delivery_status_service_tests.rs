use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue,
    NetworkRemoteDeliveryStatus, NetworkRemoteDeliveryStatusState, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_remote_delivery_status_payload::network_remote_delivery_status_payload,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn network_remote_delivery_status_payload_serializes_row10f_bridge() {
    let payload = network_remote_delivery_status_payload()
        .await
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let status: NetworkRemoteDeliveryStatus =
        status_value(&payload, constants::field::NETWORK_REMOTE_DELIVERY_STATUS);

    assert_remote_delivery_status(&status);
}

#[tokio::test]
async fn websocket_network_remote_delivery_status_command_reports_payload() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkRemoteDeliveryStatus = status_value(
        &event.payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkRemoteDeliveryStatusReported
    );
    assert_remote_delivery_status(&status);
}

fn assert_remote_delivery_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_STATUS_BRIDGE_REF
    );
    assert_eq!(
        status.broker_status,
        NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented
    );
    assert_eq!(
        status.family_hub_status,
        NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented
    );
    assert_eq!(status.broker_missing_artifact_count, 0);
    assert_eq!(status.family_hub_missing_artifact_count, 0);
    assert!(status.local_idempotency_queue_proved);
    assert!(status.queued_duplicate_rejected);
    assert!(status.completed_duplicate_rejected);
    assert_eq!(
        status.event_chain_journal_ref,
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF
    );
    assert_eq!(
        status.receipt_ledger_ref,
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
    );
    assert_eq!(
        status.durable_envelope_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
    );
    assert_eq!(
        status.durable_store_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF
    );
    assert!(status.durable_envelope_ready);
    assert_eq!(status.durable_envelope_missing_artifact_count, 0);
    assert!(!status.broker_delivery_implemented);
    assert!(!status.family_hub_delivery_implemented);
    assert!(!status.remote_delivery_ack_implemented);
    assert!(!status.provider_delivery_implemented);
    assert!(!status.child_device_delivery_implemented);
    assert!(!status.cross_process_replay_implemented);
    assert!(!status.remote_delete_export_propagation_implemented);
    assert!(!status.product_ready_remote_delivery);
    assert!(!status.policy_authority);
    assert!(!status.side_effect_authority);
    assert_eq!(status.enforcement_command_event_count, 0);
    assert_eq!(status.adapter_action_executed_count, 0);
    assert_eq!(status.raw_pcap_available_count, 0);
    assert_eq!(status.exact_url_available_count, 0);
    assert_eq!(status.page_content_available_count, 0);
    assert_eq!(status.video_content_available_count, 0);
    assert_eq!(status.private_message_content_available_count, 0);
    assert_eq!(status.search_query_available_count, 0);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_REMOTE_DELIVERY_STATUS_REPORTED.to_string(),
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
        command: AgentCommandName::AgentNetworkRemoteDeliveryStatusGet,
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
