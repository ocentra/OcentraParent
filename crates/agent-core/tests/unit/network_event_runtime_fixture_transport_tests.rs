use std::fmt::{Debug, Display};

use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_fixture_transport::{
        prove_network_runtime_remote_delivery_fixture_transport,
        prove_network_runtime_remote_delivery_fixture_transport_from_outbox,
    },
    remote_delivery_fixture_transport_types::{
        NetworkRuntimeRemoteDeliveryFixtureTransportError,
        NetworkRuntimeRemoteDeliveryFixtureTransportReport,
        NetworkRuntimeRemoteDeliveryFixtureTransportState,
    },
    remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff,
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn network_runtime_remote_delivery_fixture_transport_records_attempts_and_acks() -> TestResult
{
    let report: NetworkRuntimeRemoteDeliveryFixtureTransportReport = ok(
        prove_network_runtime_remote_delivery_fixture_transport().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT,
    )?;

    assert_eq!(
        report.fixture_transport_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF
    );
    assert_eq!(
        report.fixture_dispatch_attempt_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF
    );
    assert_eq!(
        report.fixture_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF
    );
    assert_eq!(
        report.source_outbox_candidate_count,
        report.outbox_handoff.outbox_candidate_count
    );
    assert_eq!(
        report.fixture_dispatch_attempt_count,
        report.source_outbox_candidate_count
    );
    assert_eq!(
        report.fixture_remote_ack_count,
        report.source_outbox_candidate_count
    );
    assert!(report.fixture_records_match_outbox_candidates);
    assert_records_preserve_fixture_refs(&report);

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_fixture_transport_keeps_product_delivery_false(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_fixture_transport().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT,
    )?;

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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_fixture_transport_rejects_product_claims() -> TestResult {
    let mut outbox_handoff = ok(
        prove_network_runtime_remote_delivery_outbox_handoff().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF,
    )?;
    outbox_handoff.product_ready_remote_delivery = true;

    let proof_result =
        prove_network_runtime_remote_delivery_fixture_transport_from_outbox(outbox_handoff);

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryFixtureTransportError::UnsupportedClaim)
    ));

    Ok(())
}

fn assert_records_preserve_fixture_refs(
    report: &NetworkRuntimeRemoteDeliveryFixtureTransportReport,
) {
    for record in &report.records {
        assert_eq!(
            record.fixture_state,
            NetworkRuntimeRemoteDeliveryFixtureTransportState::FixtureAckRecorded
        );
        assert_eq!(record.fixture_transport_ref, report.fixture_transport_ref);
        assert_eq!(
            record.fixture_dispatch_attempt_ref,
            report.fixture_dispatch_attempt_ref
        );
        assert_eq!(record.fixture_ack_ref, report.fixture_ack_ref);
    }
}
