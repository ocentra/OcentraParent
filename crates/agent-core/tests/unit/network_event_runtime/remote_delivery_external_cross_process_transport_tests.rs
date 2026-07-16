use std::fmt::{Debug, Display};

use ocentra_parent_agent_core::network_event_runtime::remote_delivery_cross_process_replay::prove_network_runtime_remote_delivery_cross_process_replay;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_external_cross_process_transport::{
    prove_network_runtime_remote_delivery_external_cross_process_transport,
    prove_network_runtime_remote_delivery_external_cross_process_transport_from_replay,
};
use super::remote_delivery_external_cross_process_transport_types::{
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState,
};
use crate::test_text::TestText;

type TestResult = Result<(), TestText>;
type TestResultValue<T> = Result<T, TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> TestResultValue<T> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn records_external_cross_process_transport_from_replay_records() -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_external_cross_process_transport().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EXTERNAL_CROSS_PROCESS_TRANSPORT,
    )?;

    assert_eq!(
        report.external_cross_process_transport_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF
    );
    assert_eq!(
        report
            .external_cross_process_transport_envelope_ref
            .as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF
    );
    assert_eq!(
        report.external_cross_process_transport_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF
    );
    assert_eq!(
        report.external_cross_process_transport_record_count,
        report.source_replay_record_count
    );
    assert_eq!(
        report.external_cross_process_transport_envelope_count,
        report.external_cross_process_transport_record_count
    );
    assert_eq!(
        report.external_cross_process_transport_ack_count,
        report.external_cross_process_transport_record_count
    );
    assert!(report.external_cross_process_transport_records_match_replay_records);
    assert!(report.external_cross_process_transport_ack_records_match_envelopes);
    assert!(report.external_cross_process_transport_implemented);
    assert_external_transport_records(&report);
    assert_no_product_delivery_or_enforcement_claims(&report);

    Ok(())
}

#[tokio::test]
async fn rejects_replay_source_that_claims_remote_delivery_ack() -> TestResult {
    let mut replay = ok(
        prove_network_runtime_remote_delivery_cross_process_replay().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY,
    )?;
    replay.remote_delivery_ack_implemented = true;

    let proof_result =
        prove_network_runtime_remote_delivery_external_cross_process_transport_from_replay(replay);

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::UnsupportedClaim)
    ));

    Ok(())
}

#[tokio::test]
async fn rejects_replay_records_that_do_not_match_source_count() -> TestResult {
    let mut replay = ok(
        prove_network_runtime_remote_delivery_cross_process_replay().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY,
    )?;
    replay.cross_process_replay_record_count += 1;

    let proof_result =
        prove_network_runtime_remote_delivery_external_cross_process_transport_from_replay(replay);

    assert!(matches!(
        proof_result,
        Err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::TransportRecordMismatch
        )
    ));

    Ok(())
}

fn assert_external_transport_records(
    report: &NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
) {
    for (record, replay_record) in report
        .records
        .iter()
        .zip(report.cross_process_replay.records.iter())
    {
        assert_eq!(
            record.transport_state,
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded
        );
        assert_eq!(record.sequence, replay_record.sequence);
        assert_eq!(record.event_id, replay_record.event_id);
        assert_eq!(record.event_type, replay_record.event_type);
        assert_eq!(record.correlation_id, replay_record.correlation_id);
        assert_eq!(record.source_replay_state, replay_record.replay_state);
        assert_eq!(
            record.cross_process_replay_ref,
            replay_record.cross_process_replay_ref
        );
        assert_eq!(
            record.external_cross_process_transport_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF
        );
        assert_eq!(
            record.external_cross_process_transport_envelope_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF
        );
        assert_eq!(
            record.external_cross_process_transport_ack_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF
        );
    }
}

fn assert_no_product_delivery_or_enforcement_claims(
    report: &NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
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
