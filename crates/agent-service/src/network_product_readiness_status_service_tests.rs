use ocentra_network_evidence::{
    NetworkLiveCaptureCustodyStatus, NetworkLiveCaptureCustodyStatusState,
    NetworkProductReadinessStatus, NetworkProductReadinessStatusState,
};
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    NetworkRemoteDeliveryStatus, NetworkRemoteDeliveryStatusState, AGENT_PROTOCOL_SCHEMA_VERSION,
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
    let remote_delivery_status: NetworkRemoteDeliveryStatus =
        status_value(&payload, constants::field::NETWORK_REMOTE_DELIVERY_STATUS);

    assert_live_capture_status(&live_capture_status);
    assert_product_readiness_status(&product_status);
    assert_remote_delivery_status(&remote_delivery_status);
}

fn assert_live_capture_status(status: &NetworkLiveCaptureCustodyStatus) {
    assert_eq!(
        status.state,
        NetworkLiveCaptureCustodyStatusState::CustodyReady
    );
    assert!(!status.driver_invoked);
    assert!(!status.live_capture_executed);
    assert!(!status.raw_artifact_created);
    assert!(!status.remote_upload_enabled);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.policy_authority);
    assert!(!status.adapter_authority);
    assert_eq!(status.enforcement_commands_published, 0);
}

fn assert_product_readiness_status(status: &NetworkProductReadinessStatus) {
    assert_eq!(
        status.readiness_state,
        NetworkProductReadinessStatusState::ManualRequired
    );
    assert!(status.portal_read_model_ready);
    assert!(status.retention_export_refs_visible);
    assert_eq!(
        status.risk_evaluation_ref,
        constants::network_flow::TEST_RISK_EVALUATION_REF
    );
    assert_eq!(status.risk_total_points, 42);
    assert_eq!(
        status.risk_cited_evidence_refs,
        vec![constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_owned()]
    );
    assert!(status.risk_budget_advisory_only);
    assert_eq!(
        status.performance_benchmark_run_ref,
        constants::network_flow::TEST_PERFORMANCE_BENCHMARK_REF
    );
    assert_eq!(status.performance_packet_count, 2_000);
    assert_eq!(status.performance_event_throughput_per_second, 3_200);
    assert!(!status.performance_realtime_response_claimed);
    assert!(!status.performance_adapter_action_executed);
    assert!(!status.performance_host_filtering_executed);
    assert_platform_claim_status(status);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.ui_policy_authority);
    assert!(!status.portal_adapter_dispatch_claimed);
    assert!(!status.live_adapter_execution_claimed);
    assert_eq!(status.enforcement_commands_published, 0);
}

fn assert_platform_claim_status(status: &NetworkProductReadinessStatus) {
    assert_eq!(status.platform_manual_required_claims, 1);
    assert_eq!(status.platform_entries.len(), 2);
    assert_eq!(
        status.platform_entries[0].target,
        ocentra_network_evidence::NetworkPlatformClaimTarget::WindowsFirewall
    );
    assert!(status.platform_entries[0].adapter_authorized_by_proof);
    assert!(!status.platform_entries[0].enforcement_command_published);
    assert_eq!(
        status.platform_entries[1].target,
        ocentra_network_evidence::NetworkPlatformClaimTarget::WindowsWfp
    );
    assert!(!status.platform_entries[1].adapter_authorized_by_proof);
    assert!(!status.platform_entries[1]
        .missing_required_artifacts
        .is_empty());
}

fn assert_remote_delivery_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_STATUS_REF
    );
    assert_eq!(
        status.broker_status,
        NetworkRemoteDeliveryStatusState::RequirementsSatisfiedButNotImplemented
    );
    assert_eq!(
        status.family_hub_status,
        NetworkRemoteDeliveryStatusState::RequirementsSatisfiedButNotImplemented
    );
    assert_eq!(
        status.custody_proof_ref,
        constants::network_flow::TEST_BROKER_CUSTODY_PROOF_REF
    );
    assert_eq!(
        status.publisher_auth_ref,
        constants::network_flow::TEST_BROKER_PUBLISHER_AUTH_REF
    );
    assert_eq!(
        status.relay_identity_ref,
        constants::network_flow::TEST_FAMILY_HUB_IDENTITY_REF
    );
    assert_eq!(status.broker_missing_artifact_count, 0);
    assert_eq!(status.family_hub_missing_artifact_count, 0);
    assert_eq!(status.accepted_event_type_count, 3);
    assert!(status.local_idempotency_queue_proved);
    assert_eq!(status.dropped_event_dead_letter_count, 1);
    assert!(status.queued_duplicate_rejected);
    assert!(status.completed_duplicate_rejected);
    assert!(!status.external_transport_delivery_implemented);
    assert!(!status.family_hub_delivery_implemented);
    assert!(!status.cross_process_replay_implemented);
    assert!(!status.remote_retention_delete_export_propagation_implemented);
    assert!(!status.policy_authority);
    assert!(!status.side_effect_authority);
    assert_eq!(status.enforcement_command_event_count, 0);
    assert_eq!(status.adapter_action_executed_count, 0);
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
    let remote_delivery_status: NetworkRemoteDeliveryStatus = status_value(
        &event.payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
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
    assert_eq!(
        remote_delivery_status.family_hub_status,
        NetworkRemoteDeliveryStatusState::RequirementsSatisfiedButNotImplemented
    );
    assert!(!remote_delivery_status.family_hub_delivery_implemented);
    assert_eq!(remote_delivery_status.adapter_action_executed_count, 0);
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
