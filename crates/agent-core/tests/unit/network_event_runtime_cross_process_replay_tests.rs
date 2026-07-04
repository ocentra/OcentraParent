use std::fmt::{Debug, Display};

use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_cross_process_custody_readiness::prove_network_runtime_remote_delivery_cross_process_custody_readiness,
    remote_delivery_cross_process_replay::{
        prove_network_runtime_remote_delivery_cross_process_replay,
        prove_network_runtime_remote_delivery_cross_process_replay_from_custody_readiness,
    },
    remote_delivery_cross_process_replay_types::{
        NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
        NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
        NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    },
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn records_cross_process_replay_from_durable_envelopes_and_custody_refs() -> TestResult {
    let report: NetworkRuntimeRemoteDeliveryCrossProcessReplayReport = ok(
        prove_network_runtime_remote_delivery_cross_process_replay().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY,
    )?;

    assert_eq!(
        report.cross_process_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF
    );
    assert_eq!(
        report.cross_process_replay_store_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF
    );
    assert_eq!(
        report.cross_process_replay_cursor_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF
    );
    assert_eq!(
        report.replay_state,
        NetworkRuntimeRemoteDeliveryCrossProcessReplayState::DurableReplayRecorded
    );
    assert_eq!(
        report.cross_process_replay_record_count,
        report.source_durable_envelope_count
    );
    assert_eq!(
        report.cross_process_replay_record_count,
        report.source_custody_readiness_record_count
    );
    assert_eq!(
        report.cross_process_replay_store_write_count,
        report.cross_process_replay_record_count
    );
    assert_eq!(
        report.cross_process_replay_cursor_next_sequence,
        report.cross_process_replay_record_count as u64 + 1
    );
    assert!(report.cross_process_replay_records_match_durable_envelopes);
    assert!(report.cross_process_replay_records_match_custody_readiness);
    assert!(report.cross_process_replay_implemented);
    assert_cross_process_replay_records(&report);
    assert_cross_process_replay_no_delivery_or_enforcement_claims(&report);

    Ok(())
}

#[tokio::test]
async fn rejects_source_readiness_that_already_claims_cross_process_replay() -> TestResult {
    let mut readiness = ok(
        prove_network_runtime_remote_delivery_cross_process_custody_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_CUSTODY_READINESS,
    )?;
    readiness.cross_process_replay_implemented = true;

    let proof_result =
        prove_network_runtime_remote_delivery_cross_process_replay_from_custody_readiness(
            readiness,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryCrossProcessReplayError::UnsupportedClaim)
    ));

    Ok(())
}

#[tokio::test]
async fn rejects_replay_inputs_that_do_not_match_durable_envelopes() -> TestResult {
    let mut readiness = ok(
        prove_network_runtime_remote_delivery_cross_process_custody_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_CUSTODY_READINESS,
    )?;
    readiness
        .provider_child_readiness
        .fixture_transport
        .outbox_handoff
        .durable_envelope
        .durable_records[0]
        .sequence += 1;

    let proof_result =
        prove_network_runtime_remote_delivery_cross_process_replay_from_custody_readiness(
            readiness,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryCrossProcessReplayError::ReplayRecordMismatch)
    ));

    Ok(())
}

fn assert_cross_process_replay_records(
    report: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
) {
    for (record, durable_record) in report.records.iter().zip(
        report
            .cross_process_custody_readiness
            .provider_child_readiness
            .fixture_transport
            .outbox_handoff
            .durable_envelope
            .durable_records
            .iter(),
    ) {
        assert_eq!(
            record.replay_state,
            NetworkRuntimeRemoteDeliveryCrossProcessReplayState::DurableReplayRecorded
        );
        assert_eq!(record.sequence, durable_record.sequence);
        assert_eq!(record.event_id, durable_record.event_id);
        assert_eq!(record.event_type, durable_record.event_type);
        assert_eq!(record.correlation_id, durable_record.correlation_id);
        assert_eq!(
            record.durable_envelope_ref,
            durable_record.durable_envelope_ref
        );
        assert_eq!(record.durable_store_ref, durable_record.durable_store_ref);
        assert_eq!(record.receipt_ledger_ref, durable_record.receipt_ledger_ref);
        assert_eq!(
            record.local_receipt_ack_ref,
            durable_record.local_receipt_ack_ref
        );
        assert_eq!(
            record.cross_process_replay_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF
        );
        assert_eq!(
            record.cross_process_replay_store_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF
        );
        assert_eq!(
            record.cross_process_replay_cursor_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF
        );
    }
}

fn assert_cross_process_replay_no_delivery_or_enforcement_claims(
    report: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
) {
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.remote_delivery_ack_implemented);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.remote_delete_export_propagation_implemented);
    assert!(!report.product_ready_remote_delivery);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.raw_pcap_available_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
    assert_eq!(report.video_content_available_count, 0);
    assert_eq!(report.private_message_content_available_count, 0);
    assert_eq!(report.search_query_available_count, 0);
}
