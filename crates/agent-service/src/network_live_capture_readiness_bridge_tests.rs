use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue,
    NetworkLiveCaptureProofStatusState, NetworkLiveCaptureStatus,
    NetworkRawCaptureStorageStatusState, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_live_capture_readiness_bridge::network_live_capture_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_live_capture_status_payload_reports_row13_and_row03a_readiness_without_live_claims() {
    let payload =
        network_live_capture_status_payload().expect(constants::error::AGENT_EVENT_SERIALIZES);
    let status: NetworkLiveCaptureStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
    );

    assert_live_capture_status(&status);
}

#[tokio::test]
async fn websocket_network_live_capture_status_command_reports_payload() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkLiveCaptureStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkLiveCaptureStatusReported
    );
    assert_live_capture_status(&status);
}

fn assert_live_capture_status(status: &NetworkLiveCaptureStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_STATUS_REF
    );
    assert_eq!(
        status.row13_status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF
    );
    assert_eq!(
        status.raw_storage_status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
    );
    assert_eq!(status.platform_row_count, 4);
    assert_eq!(status.proof_ready_count, 1);
    assert_eq!(status.manual_required_count, 1);
    assert_eq!(status.unavailable_count, 1);
    assert_eq!(status.degraded_count, 1);
    assert_eq!(status.storage_custody_ready_count, 1);
    assert_eq!(status.storage_manual_required_count, 1);
    assert_eq!(status.storage_unavailable_count, 1);
    assert_eq!(status.storage_degraded_count, 1);
    assert_eq!(status.capture_ready_count, 1);
    assert_eq!(status.raw_artifact_storage_authorized_count, 1);
    assert!(status.missing_artifact_count > 0);
    assert!(status.storage_missing_artifact_count > 0);
    assert_live_capture_rows(status);
    assert_live_capture_non_claims(status);
}

fn assert_live_capture_rows(status: &NetworkLiveCaptureStatus) {
    let proof_ready = &status.rows[0];
    assert_eq!(
        proof_ready.capture_proof_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF
    );
    assert_eq!(
        proof_ready.proof_state,
        NetworkLiveCaptureProofStatusState::ProofReady
    );
    assert_eq!(
        proof_ready.storage_state,
        NetworkRawCaptureStorageStatusState::CustodyReady
    );
    assert_eq!(proof_ready.missing_artifact_count, 0);
    assert_eq!(proof_ready.storage_missing_artifact_count, 0);
    assert!(proof_ready.capture_ready);
    assert!(proof_ready.raw_artifact_storage_authorized);

    assert_eq!(
        status.rows[1].proof_state,
        NetworkLiveCaptureProofStatusState::ManualRequired
    );
    assert_eq!(
        status.rows[2].proof_state,
        NetworkLiveCaptureProofStatusState::Unavailable
    );
    assert_eq!(
        status.rows[3].proof_state,
        NetworkLiveCaptureProofStatusState::Degraded
    );
}

fn assert_live_capture_non_claims(status: &NetworkLiveCaptureStatus) {
    assert_eq!(status.driver_invoked_count, 0);
    assert_eq!(status.live_capture_executed_count, 0);
    assert_eq!(status.remote_upload_enabled_count, 0);
    assert_eq!(status.raw_pcap_without_custody_available_count, 0);
    assert_eq!(status.exact_url_available_count, 0);
    assert_eq!(status.decrypted_payload_available_count, 0);
    assert_eq!(status.page_content_available_count, 0);
    assert_eq!(status.private_message_available_count, 0);
    assert_eq!(status.search_query_available_count, 0);
    assert_eq!(status.policy_authority_count, 0);
    assert_eq!(status.adapter_authority_count, 0);
    assert_eq!(status.enforcement_command_event_count, 0);
    assert_eq!(status.netstat_metadata_substitution_count, 0);
    assert_eq!(status.host_filtering_claim_count, 0);

    for row in &status.rows {
        assert!(!row.driver_invoked);
        assert!(!row.live_capture_executed);
        assert!(!row.remote_upload_enabled);
        assert!(!row.raw_pcap_without_custody_available);
        assert!(!row.exact_url_available);
        assert!(!row.decrypted_payload_available);
        assert!(!row.page_content_available);
        assert!(!row.private_message_available);
        assert!(!row.search_query_available);
        assert!(!row.policy_authority);
        assert!(!row.adapter_authority);
        assert_eq!(row.enforcement_commands_published, 0);
        assert!(!row.netstat_metadata_substituted_for_live_capture);
        assert!(!row.host_filtering_claimed);
    }
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_LIVE_CAPTURE_STATUS_REPORTED.to_string(),
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
        command: AgentCommandName::AgentNetworkLiveCaptureStatusGet,
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
