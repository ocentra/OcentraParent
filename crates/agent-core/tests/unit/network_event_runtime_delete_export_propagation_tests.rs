use std::fmt::{Debug, Display};

use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_delete_export_propagation::{
        prove_network_runtime_remote_delivery_delete_export_propagation,
        prove_network_runtime_remote_delivery_delete_export_propagation_from_fixture_transport,
    },
    remote_delivery_delete_export_propagation_types::{
        NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
        NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
        NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
    },
    remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport,
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn network_runtime_remote_delivery_delete_export_propagation_records_readiness_refs(
) -> TestResult {
    let report: NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport = ok(
        prove_network_runtime_remote_delivery_delete_export_propagation().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELETE_EXPORT_PROPAGATION,
    )?;

    assert_eq!(
        report.delete_export_propagation_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF
    );
    assert_eq!(
        report.remote_delete_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF
    );
    assert_eq!(
        report.remote_export_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF
    );
    assert_eq!(
        report.source_fixture_record_count,
        report.fixture_transport.records.len()
    );
    assert_eq!(
        report.propagation_readiness_record_count,
        report.source_fixture_record_count
    );
    assert_eq!(
        report.remote_delete_ready_count,
        report.source_fixture_record_count
    );
    assert_eq!(
        report.remote_export_ready_count,
        report.source_fixture_record_count
    );
    assert!(report.propagation_records_match_fixture_records);
    assert_records_preserve_delete_export_refs(&report);

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_delete_export_propagation_stays_proof_only() -> TestResult
{
    let report = ok(
        prove_network_runtime_remote_delivery_delete_export_propagation().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELETE_EXPORT_PROPAGATION,
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
async fn network_runtime_remote_delivery_delete_export_propagation_rejects_product_claims(
) -> TestResult {
    let mut fixture_transport = ok(
        prove_network_runtime_remote_delivery_fixture_transport().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT,
    )?;
    fixture_transport.remote_delete_export_propagation_implemented = true;

    let proof_result =
        prove_network_runtime_remote_delivery_delete_export_propagation_from_fixture_transport(
            fixture_transport,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::UnsupportedClaim)
    ));

    Ok(())
}

fn assert_records_preserve_delete_export_refs(
    report: &NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
) {
    for record in &report.records {
        assert_eq!(
            record.propagation_state,
            NetworkRuntimeRemoteDeliveryDeleteExportPropagationState::ReadinessRecordedNotPropagated
        );
        assert_eq!(
            record.delete_export_propagation_ref,
            report.delete_export_propagation_ref
        );
        assert_eq!(
            record.remote_delete_readiness_ref,
            report.remote_delete_readiness_ref
        );
        assert_eq!(
            record.remote_export_readiness_ref,
            report.remote_export_readiness_ref
        );
    }
}
