use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    remote_delivery_transport_dispatch_state::prove_network_runtime_remote_delivery_transport_dispatch_state,
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkRemoteDeliveryCrossProcessCustodyReadinessState,
    NetworkRemoteDeliveryExternalCrossProcessTransportState,
    NetworkRemoteDeliveryProviderChildReadinessState, NetworkRemoteDeliveryStatus,
    NetworkRemoteDeliveryStatusState, NetworkRemoteDeliveryTransportDispatchState,
};
use ocentra_parent_agent_service::test_support::{
    blocked_dispatch_records_match_outbox_candidates_for_test,
    network_remote_delivery_status_payload_for_test,
};
use serde::de::DeserializeOwned;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[tokio::test]
async fn network_remote_delivery_status_payload_serializes_row10t_external_transport_status(
) -> TestResult {
    let payload = network_remote_delivery_status_payload_for_test()
        .await
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })?;
    let status: NetworkRemoteDeliveryStatus =
        status_value(&payload, constants::field::NETWORK_REMOTE_DELIVERY_STATUS)?;

    assert_remote_delivery_status(&status);

    Ok(())
}

#[tokio::test]
async fn network_remote_delivery_status_payload_reuses_stable_row10t_status_snapshot() -> TestResult
{
    let first_payload = network_remote_delivery_status_payload_for_test()
        .await
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })?;
    let second_payload = network_remote_delivery_status_payload_for_test()
        .await
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })?;
    let first_status: NetworkRemoteDeliveryStatus = status_value(
        &first_payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
    )?;
    let second_status: NetworkRemoteDeliveryStatus = status_value(
        &second_payload,
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
    )?;

    assert_eq!(first_status, second_status);
    assert_remote_delivery_status(&first_status);

    Ok(())
}

#[tokio::test]
async fn network_remote_delivery_status_rejects_blocked_dispatch_identity_and_order_mismatches(
) -> TestResult {
    let report = prove_network_runtime_remote_delivery_transport_dispatch_state()
        .await
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE
            ))
        })?;
    let outbox_report = &report
        .no_enforcement_invariant
        .dispatch_readiness
        .outbox_handoff;

    assert!(blocked_dispatch_records_match_outbox_candidates_for_test(
        &report,
        outbox_report
    ));

    let mut sequence_mismatch = report.clone();
    sequence_mismatch.blocked_dispatch_records[0].sequence += 1;
    assert_blocked_dispatch_mismatch(&sequence_mismatch, outbox_report);

    let mut outbox_ref_mismatch = report.clone();
    outbox_ref_mismatch.blocked_dispatch_records[0].outbox_ref =
        report.future_transport_seam_ref.clone();
    assert_blocked_dispatch_mismatch(&outbox_ref_mismatch, outbox_report);

    let mut reordered_records = report.clone();
    reordered_records.blocked_dispatch_records.swap(0, 1);
    assert_blocked_dispatch_mismatch(&reordered_records, outbox_report);

    Ok(())
}

fn assert_remote_delivery_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF
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
    assert_remote_delivery_cross_process_custody_readiness_status(status);
    assert_remote_delivery_cross_process_replay_status(status);
    assert_remote_delivery_external_cross_process_transport_status(status);
    assert_remote_delivery_outbox_status(status);
    assert_remote_delivery_non_claims(status);
}

fn assert_blocked_dispatch_mismatch(
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    outbox_report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) {
    assert!(!blocked_dispatch_records_match_outbox_candidates_for_test(
        report,
        outbox_report
    ));
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

fn assert_remote_delivery_cross_process_custody_readiness_status(
    status: &NetworkRemoteDeliveryStatus,
) {
    assert_eq!(
        status.cross_process_custody_status_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF
    );
    assert_eq!(
        status.cross_process_replay_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF
    );
    assert_eq!(
        status.remote_retention_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF
    );
    assert_eq!(
        status.remote_delete_custody_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF
    );
    assert_eq!(
        status.remote_export_custody_readiness_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF
    );
    assert_eq!(
        status.cross_process_custody_readiness_state,
        NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        status.cross_process_replay_readiness_record_count,
        status.provider_delivery_readiness_record_count
    );
    assert_eq!(
        status.remote_retention_readiness_record_count,
        status.provider_delivery_readiness_record_count
    );
    assert_eq!(
        status.remote_delete_custody_readiness_record_count,
        status.provider_delivery_readiness_record_count
    );
    assert_eq!(
        status.remote_export_custody_readiness_record_count,
        status.provider_delivery_readiness_record_count
    );
    assert!(status.cross_process_custody_records_match_provider_child_readiness);
    assert_eq!(status.cross_process_replay_artifact_count, 0);
    assert_eq!(status.remote_retention_artifact_count, 0);
    assert_eq!(status.remote_delete_custody_artifact_count, 0);
    assert_eq!(status.remote_export_custody_artifact_count, 0);
}

fn assert_remote_delivery_cross_process_replay_status(status: &NetworkRemoteDeliveryStatus) {
    assert_eq!(
        status.cross_process_replay_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF
    );
    assert_eq!(
        status.cross_process_replay_store_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF
    );
    assert_eq!(
        status.cross_process_replay_cursor_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF
    );
    assert_eq!(
        status.cross_process_replay_record_count,
        status.outbox_candidate_count
    );
    assert_eq!(
        status.cross_process_replay_store_write_count,
        status.cross_process_replay_record_count
    );
    assert_eq!(
        status.cross_process_replay_cursor_next_sequence,
        status.cross_process_replay_record_count + 1
    );
    assert!(status.cross_process_replay_records_match_durable_envelopes);
    assert!(status.cross_process_replay_records_match_custody_readiness);
}

fn assert_remote_delivery_external_cross_process_transport_status(
    status: &NetworkRemoteDeliveryStatus,
) {
    assert_eq!(
        status.external_cross_process_transport_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF
    );
    assert_eq!(
        status.external_cross_process_transport_envelope_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF
    );
    assert_eq!(
        status.external_cross_process_transport_ack_ref,
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF
    );
    assert_eq!(
        status.external_cross_process_transport_state,
        NetworkRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded
    );
    assert_eq!(
        status.external_cross_process_transport_record_count,
        status.cross_process_replay_record_count
    );
    assert_eq!(
        status.external_cross_process_transport_envelope_count,
        status.external_cross_process_transport_record_count
    );
    assert_eq!(
        status.external_cross_process_transport_ack_count,
        status.external_cross_process_transport_record_count
    );
    assert!(status.external_cross_process_transport_records_match_replay_records);
    assert!(status.external_cross_process_transport_ack_records_match_envelopes);
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
    assert!(status.cross_process_replay_implemented);
    assert!(status.external_cross_process_transport_implemented);
    assert!(!status.remote_delete_export_propagation_implemented);
    assert!(!status.product_ready_remote_delivery);
    assert!(!status.policy_authority);
    assert!(!status.side_effect_authority);
    assert!(!status.host_filtering_claimed);
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

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &TestStr,
) -> TestResult<TStatus> {
    match payload.get(field) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).map_err(|error| {
            IoError::other(format!(
                "{} ({field}): {error:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
            .into()
        }),
        _ => Err(IoError::other(format!(
            "{} ({field})",
            constants::error::AGENT_EVENT_SERIALIZES
        ))
        .into()),
    }
}
use std::primitive::str as TestStr;
