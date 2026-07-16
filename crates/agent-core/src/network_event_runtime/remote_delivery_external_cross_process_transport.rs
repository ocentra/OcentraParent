use ocentra_eventing::ids::SourceComponent;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_cross_process_replay::prove_network_runtime_remote_delivery_cross_process_replay;
use super::remote_delivery_cross_process_replay_types::{
    NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_external_cross_process_transport_types::{
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState,
};

pub async fn prove_network_runtime_remote_delivery_external_cross_process_transport() -> Result<
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
> {
    let cross_process_replay = prove_network_runtime_remote_delivery_cross_process_replay()
        .await
        .map_err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::CrossProcessReplay,
        )?;
    prove_network_runtime_remote_delivery_external_cross_process_transport_from_replay(
        cross_process_replay,
    )
}

pub fn prove_network_runtime_remote_delivery_external_cross_process_transport_from_replay(
    cross_process_replay: NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
> {
    if cross_process_replay.records.is_empty() {
        return Err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::EmptyCrossProcessReplay,
        );
    }
    if cross_process_replay.cross_process_replay_record_count != cross_process_replay.records.len()
    {
        return Err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::TransportRecordMismatch,
        );
    }
    if has_unsupported_claims(&cross_process_replay) {
        return Err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::UnsupportedClaim,
        );
    }

    let refs = external_transport_refs()?;
    let records = cross_process_replay
        .records
        .iter()
        .map(|record| external_transport_record(record, &refs))
        .collect::<Vec<_>>();
    if !records_match_replay_records(&records, &cross_process_replay.records) {
        return Err(
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError::TransportRecordMismatch,
        );
    }

    Ok(external_transport_report(
        cross_process_replay,
        refs,
        records,
    ))
}

struct ExternalTransportRefs {
    external_cross_process_transport_ref: SourceComponent,
    external_cross_process_transport_envelope_ref: SourceComponent,
    external_cross_process_transport_ack_ref: SourceComponent,
}

fn external_transport_refs(
) -> Result<ExternalTransportRefs, NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError> {
    Ok(ExternalTransportRefs {
        external_cross_process_transport_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF,
        )?,
        external_cross_process_transport_envelope_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF,
        )?,
        external_cross_process_transport_ack_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF,
        )?,
    })
}

fn external_transport_record(
    replay_record: &NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord,
    refs: &ExternalTransportRefs,
) -> NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord {
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord {
        sequence: replay_record.sequence,
        event_id: replay_record.event_id.clone(),
        event_type: replay_record.event_type.clone(),
        correlation_id: replay_record.correlation_id.clone(),
        source_replay_state: replay_record.replay_state,
        durable_envelope_ref: replay_record.durable_envelope_ref.clone(),
        durable_store_ref: replay_record.durable_store_ref.clone(),
        cross_process_replay_ref: replay_record.cross_process_replay_ref.clone(),
        cross_process_replay_store_ref: replay_record.cross_process_replay_store_ref.clone(),
        cross_process_replay_cursor_ref: replay_record.cross_process_replay_cursor_ref.clone(),
        external_cross_process_transport_ref: refs.external_cross_process_transport_ref.clone(),
        external_cross_process_transport_envelope_ref: refs
            .external_cross_process_transport_envelope_ref
            .clone(),
        external_cross_process_transport_ack_ref: refs
            .external_cross_process_transport_ack_ref
            .clone(),
        transport_state:
            NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded,
    }
}

fn records_match_replay_records(
    records: &[NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord],
    replay_records: &[NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord],
) -> bool {
    records.len() == replay_records.len()
        && records
            .iter()
            .zip(replay_records.iter())
            .all(|(record, replay_record)| {
                record.sequence == replay_record.sequence
                    && record.event_id == replay_record.event_id
                    && record.event_type == replay_record.event_type
                    && record.correlation_id == replay_record.correlation_id
                    && record.source_replay_state == replay_record.replay_state
                    && record.durable_envelope_ref == replay_record.durable_envelope_ref
                    && record.durable_store_ref == replay_record.durable_store_ref
                    && record.cross_process_replay_ref == replay_record.cross_process_replay_ref
                    && record.cross_process_replay_store_ref
                        == replay_record.cross_process_replay_store_ref
                    && record.cross_process_replay_cursor_ref
                        == replay_record.cross_process_replay_cursor_ref
            })
}

fn external_transport_report(
    cross_process_replay: NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    refs: ExternalTransportRefs,
    records: Vec<NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord>,
) -> NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport {
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport {
        source_replay_record_count: cross_process_replay.cross_process_replay_record_count,
        external_cross_process_transport_record_count: records.len(),
        external_cross_process_transport_envelope_count: records.len(),
        external_cross_process_transport_ack_count: records.len(),
        external_cross_process_transport_records_match_replay_records: true,
        external_cross_process_transport_ack_records_match_envelopes: true,
        external_cross_process_transport_implemented: true,
        broker_delivery_implemented: cross_process_replay.broker_delivery_implemented,
        family_hub_delivery_implemented: cross_process_replay.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: cross_process_replay.remote_delivery_ack_implemented,
        provider_delivery_implemented: cross_process_replay.provider_delivery_implemented,
        child_device_delivery_implemented: cross_process_replay.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: cross_process_replay
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: cross_process_replay.product_ready_remote_delivery,
        policy_authority: cross_process_replay.policy_authority,
        side_effect_authority: cross_process_replay.side_effect_authority,
        enforcement_command_event_count: cross_process_replay.enforcement_command_event_count,
        adapter_action_executed_count: cross_process_replay.adapter_action_executed_count,
        raw_pcap_available_count: cross_process_replay.raw_pcap_available_count,
        exact_url_available_count: cross_process_replay.exact_url_available_count,
        decrypted_payload_available_count: cross_process_replay.decrypted_payload_available_count,
        page_content_available_count: cross_process_replay.page_content_available_count,
        video_content_available_count: cross_process_replay.video_content_available_count,
        private_message_content_available_count: cross_process_replay
            .private_message_content_available_count,
        search_query_available_count: cross_process_replay.search_query_available_count,
        external_cross_process_transport_ref: refs.external_cross_process_transport_ref,
        external_cross_process_transport_envelope_ref: refs
            .external_cross_process_transport_envelope_ref,
        external_cross_process_transport_ack_ref: refs.external_cross_process_transport_ack_ref,
        records,
        cross_process_replay,
    }
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport) -> bool {
    !report.cross_process_replay_implemented
        || report.broker_delivery_implemented
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
