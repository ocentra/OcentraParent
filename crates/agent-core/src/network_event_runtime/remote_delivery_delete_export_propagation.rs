use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_delete_export_propagation_types::{
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport;
use super::remote_delivery_fixture_transport_types::{
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
};

pub async fn prove_network_runtime_remote_delivery_delete_export_propagation() -> Result<
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
> {
    let fixture_transport = prove_network_runtime_remote_delivery_fixture_transport()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::FixtureTransport)?;
    prove_network_runtime_remote_delivery_delete_export_propagation_from_fixture_transport(
        fixture_transport,
    )
}

pub fn prove_network_runtime_remote_delivery_delete_export_propagation_from_fixture_transport(
    fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
> {
    build_delete_export_propagation_report(fixture_transport)
}

fn build_delete_export_propagation_report(
    fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
> {
    if fixture_transport.records.is_empty() {
        return Err(
            NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::EmptyFixtureTransport,
        );
    }
    if has_unsupported_claims(&fixture_transport) {
        return Err(NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::UnsupportedClaim);
    }

    let delete_export_propagation_ref = source_component(
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF,
    )?;
    let remote_delete_readiness_ref =
        source_component(constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF)?;
    let remote_export_readiness_ref =
        source_component(constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF)?;
    let records = fixture_transport
        .records
        .iter()
        .map(|record| {
            propagation_record(
                record,
                &delete_export_propagation_ref,
                &remote_delete_readiness_ref,
                &remote_export_readiness_ref,
            )
        })
        .collect::<Vec<_>>();

    if !propagation_records_match_fixture_records(&records, &fixture_transport.records) {
        return Err(
            NetworkRuntimeRemoteDeliveryDeleteExportPropagationError::PropagationRecordMismatch,
        );
    }

    Ok(NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport {
        source_fixture_record_count: fixture_transport.records.len(),
        propagation_readiness_record_count: records.len(),
        remote_delete_ready_count: records.len(),
        remote_export_ready_count: records.len(),
        propagation_records_match_fixture_records: true,
        broker_delivery_implemented: fixture_transport.broker_delivery_implemented,
        family_hub_delivery_implemented: fixture_transport.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: fixture_transport.remote_delivery_ack_implemented,
        provider_delivery_implemented: fixture_transport.provider_delivery_implemented,
        child_device_delivery_implemented: fixture_transport.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: fixture_transport
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: fixture_transport.product_ready_remote_delivery,
        policy_authority: fixture_transport.policy_authority,
        side_effect_authority: fixture_transport.side_effect_authority,
        enforcement_command_event_count: fixture_transport.enforcement_command_event_count,
        adapter_action_executed_count: fixture_transport.adapter_action_executed_count,
        raw_pcap_available_count: fixture_transport.raw_pcap_available_count,
        exact_url_available_count: fixture_transport.exact_url_available_count,
        decrypted_payload_available_count: fixture_transport.decrypted_payload_available_count,
        page_content_available_count: fixture_transport.page_content_available_count,
        video_content_available_count: fixture_transport.video_content_available_count,
        private_message_content_available_count: fixture_transport
            .private_message_content_available_count,
        search_query_available_count: fixture_transport.search_query_available_count,
        delete_export_propagation_ref,
        remote_delete_readiness_ref,
        remote_export_readiness_ref,
        records,
        fixture_transport,
    })
}

fn propagation_record(
    fixture_record: &NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    delete_export_propagation_ref: &ocentra_eventing::ids::SourceComponent,
    remote_delete_readiness_ref: &ocentra_eventing::ids::SourceComponent,
    remote_export_readiness_ref: &ocentra_eventing::ids::SourceComponent,
) -> NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord {
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord {
        sequence: fixture_record.sequence,
        event_id: fixture_record.event_id.clone(),
        event_type: fixture_record.event_type.clone(),
        correlation_id: fixture_record.correlation_id.clone(),
        source_fixture_state: fixture_record.fixture_state,
        propagation_state:
            NetworkRuntimeRemoteDeliveryDeleteExportPropagationState::ReadinessRecordedNotPropagated,
        outbox_ref: fixture_record.outbox_ref.clone(),
        handoff_ref: fixture_record.handoff_ref.clone(),
        fixture_ack_ref: fixture_record.fixture_ack_ref.clone(),
        delete_export_propagation_ref: delete_export_propagation_ref.clone(),
        remote_delete_readiness_ref: remote_delete_readiness_ref.clone(),
        remote_export_readiness_ref: remote_export_readiness_ref.clone(),
    }
}

fn propagation_records_match_fixture_records(
    records: &[NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord],
    fixture_records: &[NetworkRuntimeRemoteDeliveryFixtureTransportRecord],
) -> bool {
    records.len() == fixture_records.len()
        && records
            .iter()
            .zip(fixture_records.iter())
            .all(|(record, fixture_record)| {
                record.sequence == fixture_record.sequence
                    && record.event_id == fixture_record.event_id
                    && record.event_type == fixture_record.event_type
                    && record.correlation_id == fixture_record.correlation_id
                    && record.source_fixture_state == fixture_record.fixture_state
                    && record.outbox_ref == fixture_record.outbox_ref
                    && record.handoff_ref == fixture_record.handoff_ref
                    && record.fixture_ack_ref == fixture_record.fixture_ack_ref
            })
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryFixtureTransportReport) -> bool {
    report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report.remote_delete_export_propagation_implemented
        || report.product_ready_remote_delivery
        || report.policy_authority
        || report.side_effect_authority
        || report.enforcement_command_event_count > 0
        || report.adapter_action_executed_count > 0
        || report.raw_pcap_available_count > 0
        || report.exact_url_available_count > 0
        || report.decrypted_payload_available_count > 0
        || report.page_content_available_count > 0
        || report.video_content_available_count > 0
        || report.private_message_content_available_count > 0
        || report.search_query_available_count > 0
}
