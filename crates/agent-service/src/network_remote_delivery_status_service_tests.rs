use ocentra_parent_agent_core::prove_network_runtime_remote_delivery_transport_dispatch_state;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue,
    NetworkRemoteDeliveryProviderChildReadinessState, NetworkRemoteDeliveryStatus,
    NetworkRemoteDeliveryStatusState, NetworkRemoteDeliveryTransportDispatchState,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_remote_delivery_status_payload::{
        blocked_dispatch_records_match_outbox_candidates, network_remote_delivery_status_payload,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn network_remote_delivery_status_payload_serializes_row10n_status_with_row10k_dispatch_state(
) {
    let payload = network_remote_delivery_status_payload()
        .await
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let status: NetworkRemoteDeliveryStatus =
        status_value(&payload, constants::field::NETWORK_REMOTE_DELIVERY_STATUS);

    assert_remote_delivery_status(&status);
}

#[tokio::test]
async fn network_remote_delivery_status_payload_reuses_stable_row10n_status_snapshot() {
    let first_payload = network_remote_delivery_status_payload()
        .await
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let second_payload = network_remote_delivery_status_payload()
        .await
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let first_status: NetworkRemoteDeliveryStatus = status_value(
        &first_payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
    );
    let second_status: NetworkRemoteDeliveryStatus = status_value(
        &second_payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
    );

    assert_eq!(first_status, second_status);
    assert_remote_delivery_status(&first_status);
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

#[tokio::test]
async fn network_remote_delivery_status_rejects_blocked_dispatch_identity_mismatches() {
    let mut report = prove_network_runtime_remote_delivery_transport_dispatch_state()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE);
    let outbox_report = report
        .no_enforcement_invariant
        .dispatch_readiness
        .outbox_handoff
        .clone();

    assert!(blocked_dispatch_records_match_outbox_candidates(
        &report,
        &outbox_report
    ));

    report.blocked_dispatch_records[0].sequence += 1;

    assert!(!blocked_dispatch_records_match_outbox_candidates(
        &report,
        &outbox_report
    ));
}

fn assert_remote_delivery_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_STATUS_BRIDGE_REF
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
    assert_remote_delivery_transport_dispatch_status(status);
    assert_remote_delivery_fixture_transport_status(status);
    assert_remote_delivery_delete_export_status(status);
    assert_remote_delivery_provider_child_readiness_status(status);
    assert_remote_delivery_outbox_status(status);
    assert_remote_delivery_non_claims(status);
}

fn assert_remote_delivery_transport_dispatch_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.transport_dispatch_state_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF
    );
    assert_eq!(
        status.blocked_dispatch_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF
    );
    assert_eq!(
        status.future_transport_seam_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF
    );
    assert_eq!(
        status.transport_dispatch_state,
        NetworkRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
    );
    assert_eq!(
        status.source_outbox_candidate_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.blocked_dispatch_record_count,
        status.outbox_candidate_count
    );
    assert!(status.blocked_dispatch_records_match_outbox_candidates);
    assert_eq!(status.dispatch_ready_candidate_count, 0);
}

fn assert_remote_delivery_fixture_transport_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.fixture_transport_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF
    );
    assert_eq!(
        status.fixture_dispatch_attempt_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF
    );
    assert_eq!(
        status.fixture_ack_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF
    );
    assert_eq!(
        status.fixture_source_outbox_candidate_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.fixture_dispatch_attempt_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.fixture_remote_ack_count,
        status.outbox_candidate_count
    );
    assert!(status.fixture_records_match_outbox_candidates);
}

fn assert_remote_delivery_delete_export_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.delete_export_propagation_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF
    );
    assert_eq!(
        status.remote_delete_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF
    );
    assert_eq!(
        status.remote_export_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF
    );
    assert_eq!(
        status.delete_export_readiness_record_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.remote_delete_ready_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.remote_export_ready_count,
        status.outbox_candidate_count
    );
    assert!(status.delete_export_records_match_fixture_acks);
}

fn assert_remote_delivery_provider_child_readiness_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.provider_route_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF
    );
    assert_eq!(
        status.child_device_route_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF
    );
    assert_eq!(
        status.provider_delivery_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF
    );
    assert_eq!(
        status.child_device_delivery_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF
    );
    assert_eq!(
        status.provider_delivery_readiness_state,
        NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        status.child_device_delivery_readiness_state,
        NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        status.provider_delivery_readiness_record_count,
        status.fixture_remote_ack_count
    );
    assert_eq!(
        status.child_device_delivery_readiness_record_count,
        status.fixture_remote_ack_count
    );
    assert_eq!(status.provider_delivery_artifact_count, 0);
    assert_eq!(status.child_device_delivery_artifact_count, 0);
    assert!(status.provider_delivery_records_match_fixture_acks);
    assert!(status.child_device_delivery_records_match_fixture_acks);
}

fn assert_remote_delivery_outbox_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.outbox_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF
    );
    assert_eq!(
        status.outbox_handoff_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF
    );
    assert_eq!(
        status.outbox_replay_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF
    );
    assert_eq!(
        status.outbox_support_status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF
    );
    assert_eq!(
        status.outbox_candidate_count,
        status.prepared_not_dispatched_count
    );
    assert!(status.outbox_candidate_count > 0);
    assert_eq!(status.dispatch_attempt_count, 0);
    assert_eq!(status.remote_ack_count, 0);
    assert!(status.duplicate_durable_envelope_rejected);
    assert!(status.outbox_candidates_match_durable_envelopes);
    assert!(status.outbox_candidates_match_receipts);
    assert_eq!(status.sequence_gap_count, 0);
    assert_eq!(status.event_id_mismatch_count, 0);
    assert_eq!(status.event_type_mismatch_count, 0);
    assert_eq!(status.correlation_mismatch_count, 0);
}

fn assert_remote_delivery_non_claims(status: &NetworkRemoteDeliveryStatus) {
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
    assert_eq!(status.decrypted_payload_available_count, 0);
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
