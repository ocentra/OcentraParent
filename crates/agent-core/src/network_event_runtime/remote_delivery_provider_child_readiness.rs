use ocentra_eventing::ids::SourceComponent;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport;
use super::remote_delivery_fixture_transport_types::{
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
};
use super::remote_delivery_provider_child_readiness_types::{
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
};

pub async fn prove_network_runtime_remote_delivery_provider_child_readiness() -> Result<
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
> {
    let fixture_transport = prove_network_runtime_remote_delivery_fixture_transport()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryProviderChildReadinessError::FixtureTransport)?;
    prove_network_runtime_remote_delivery_provider_child_readiness_from_fixture_transport(
        fixture_transport,
    )
}

pub fn prove_network_runtime_remote_delivery_provider_child_readiness_from_fixture_transport(
    fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
> {
    if fixture_transport.records.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryProviderChildReadinessError::EmptyFixtureTransport);
    }
    if has_unsupported_claims(&fixture_transport) {
        return Err(NetworkRuntimeRemoteDeliveryProviderChildReadinessError::UnsupportedClaim);
    }

    let refs = provider_child_readiness_refs()?;
    let records = provider_child_readiness_records(&fixture_transport.records, &refs);

    if !readiness_records_match_fixture_acks(&records, &fixture_transport.records) {
        return Err(
            NetworkRuntimeRemoteDeliveryProviderChildReadinessError::ReadinessRecordMismatch,
        );
    }

    Ok(provider_child_readiness_report(
        fixture_transport,
        refs,
        records,
    ))
}

struct ProviderChildReadinessRefs {
    provider_route_ref: SourceComponent,
    child_device_route_ref: SourceComponent,
    provider_readiness_ref: SourceComponent,
    child_device_readiness_ref: SourceComponent,
}

fn provider_child_readiness_refs(
) -> Result<ProviderChildReadinessRefs, NetworkRuntimeRemoteDeliveryProviderChildReadinessError> {
    Ok(ProviderChildReadinessRefs {
        provider_route_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF,
        )?,
        child_device_route_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF,
        )?,
        provider_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF,
        )?,
        child_device_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF,
        )?,
    })
}

fn provider_child_readiness_records(
    fixture_records: &[NetworkRuntimeRemoteDeliveryFixtureTransportRecord],
    refs: &ProviderChildReadinessRefs,
) -> Vec<NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord> {
    fixture_records
        .iter()
        .map(|record| provider_child_readiness_record(record, refs))
        .collect()
}

fn provider_child_readiness_report(
    fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    refs: ProviderChildReadinessRefs,
    records: Vec<NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord>,
) -> NetworkRuntimeRemoteDeliveryProviderChildReadinessReport {
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport {
        source_fixture_ack_count: fixture_transport.fixture_remote_ack_count,
        provider_delivery_readiness_record_count: records.len(),
        child_device_delivery_readiness_record_count: records.len(),
        provider_delivery_artifact_count: 0,
        child_device_delivery_artifact_count: 0,
        provider_delivery_records_match_fixture_acks: true,
        child_device_delivery_records_match_fixture_acks: true,
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
        provider_route_ref: refs.provider_route_ref,
        child_device_route_ref: refs.child_device_route_ref,
        provider_readiness_ref: refs.provider_readiness_ref,
        child_device_readiness_ref: refs.child_device_readiness_ref,
        provider_state:
            NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
        child_device_state:
            NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
        records,
        fixture_transport,
    }
}

fn provider_child_readiness_record(
    record: &NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    refs: &ProviderChildReadinessRefs,
) -> NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord {
    NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord {
        sequence: record.sequence,
        event_id: record.event_id.clone(),
        event_type: record.event_type.clone(),
        correlation_id: record.correlation_id.clone(),
        fixture_ack_ref: record.fixture_ack_ref.clone(),
        provider_route_ref: refs.provider_route_ref.clone(),
        child_device_route_ref: refs.child_device_route_ref.clone(),
        provider_readiness_ref: refs.provider_readiness_ref.clone(),
        child_device_readiness_ref: refs.child_device_readiness_ref.clone(),
        provider_state:
            NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
        child_device_state:
            NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
    }
}

fn readiness_records_match_fixture_acks(
    records: &[NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord],
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
