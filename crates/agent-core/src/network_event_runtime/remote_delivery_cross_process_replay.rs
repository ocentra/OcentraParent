use ocentra_eventing::ids::SourceComponent;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_cross_process_custody_readiness::prove_network_runtime_remote_delivery_cross_process_custody_readiness;
use super::remote_delivery_cross_process_custody_readiness_types::{
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
};
use super::remote_delivery_cross_process_replay_types::{
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
};
use super::remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord;
use super::remote_delivery_event_chain_store::source_component;

pub async fn prove_network_runtime_remote_delivery_cross_process_replay() -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
> {
    let cross_process_custody_readiness =
        prove_network_runtime_remote_delivery_cross_process_custody_readiness()
            .await
            .map_err(
                NetworkRuntimeRemoteDeliveryCrossProcessReplayError::CrossProcessCustodyReadiness,
            )?;
    prove_network_runtime_remote_delivery_cross_process_replay_from_custody_readiness(
        cross_process_custody_readiness,
    )
}

pub fn prove_network_runtime_remote_delivery_cross_process_replay_from_custody_readiness(
    cross_process_custody_readiness: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
> {
    if cross_process_custody_readiness.records.is_empty() {
        return Err(
            NetworkRuntimeRemoteDeliveryCrossProcessReplayError::EmptyCrossProcessCustodyReadiness,
        );
    }
    if has_unsupported_claims(&cross_process_custody_readiness) {
        return Err(NetworkRuntimeRemoteDeliveryCrossProcessReplayError::UnsupportedClaim);
    }

    let refs = cross_process_replay_refs()?;
    let durable_records = &cross_process_custody_readiness
        .provider_child_readiness
        .fixture_transport
        .outbox_handoff
        .durable_envelope
        .durable_records;
    let records = cross_process_replay_records(
        durable_records,
        &cross_process_custody_readiness.records,
        &refs,
    )?;

    Ok(cross_process_replay_report(
        cross_process_custody_readiness,
        refs,
        records,
    ))
}

struct CrossProcessReplayRefs {
    cross_process_replay_ref: SourceComponent,
    cross_process_replay_store_ref: SourceComponent,
    cross_process_replay_cursor_ref: SourceComponent,
}

fn cross_process_replay_refs(
) -> Result<CrossProcessReplayRefs, NetworkRuntimeRemoteDeliveryCrossProcessReplayError> {
    Ok(CrossProcessReplayRefs {
        cross_process_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF,
        )?,
        cross_process_replay_store_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF,
        )?,
        cross_process_replay_cursor_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF,
        )?,
    })
}

fn cross_process_replay_records(
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
    custody_records: &[NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord],
    refs: &CrossProcessReplayRefs,
) -> Result<
    Vec<NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord>,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
> {
    if durable_records.is_empty() || durable_records.len() != custody_records.len() {
        return Err(NetworkRuntimeRemoteDeliveryCrossProcessReplayError::ReplayRecordMismatch);
    }
    durable_records
        .iter()
        .zip(custody_records.iter())
        .map(|(durable_record, custody_record)| {
            cross_process_replay_record(durable_record, custody_record, refs)
        })
        .collect()
}

fn cross_process_replay_record(
    durable_record: &NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord,
    custody_record: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord,
    refs: &CrossProcessReplayRefs,
) -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
> {
    if durable_record.sequence != custody_record.sequence
        || durable_record.event_id != custody_record.event_id
        || durable_record.event_type != custody_record.event_type
        || durable_record.correlation_id != custody_record.correlation_id
    {
        return Err(NetworkRuntimeRemoteDeliveryCrossProcessReplayError::ReplayRecordMismatch);
    }
    Ok(NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord {
        sequence: durable_record.sequence,
        event_id: durable_record.event_id.clone(),
        event_type: durable_record.event_type.clone(),
        correlation_id: durable_record.correlation_id.clone(),
        durable_envelope_ref: durable_record.durable_envelope_ref.clone(),
        durable_store_ref: durable_record.durable_store_ref.clone(),
        receipt_ledger_ref: durable_record.receipt_ledger_ref.clone(),
        local_receipt_ack_ref: durable_record.local_receipt_ack_ref.clone(),
        cross_process_custody_status_ref: custody_record.cross_process_custody_status_ref.clone(),
        cross_process_replay_readiness_ref: custody_record
            .cross_process_replay_readiness_ref
            .clone(),
        cross_process_replay_ref: refs.cross_process_replay_ref.clone(),
        cross_process_replay_store_ref: refs.cross_process_replay_store_ref.clone(),
        cross_process_replay_cursor_ref: refs.cross_process_replay_cursor_ref.clone(),
        replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState::DurableReplayRecorded,
    })
}

fn cross_process_replay_report(
    cross_process_custody_readiness: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    refs: CrossProcessReplayRefs,
    records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord>,
) -> NetworkRuntimeRemoteDeliveryCrossProcessReplayReport {
    let cursor_next_sequence = records
        .last()
        .map(|record| record.sequence.saturating_add(1))
        .unwrap_or(1);
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport {
        source_durable_envelope_count: cross_process_custody_readiness
            .provider_child_readiness
            .fixture_transport
            .outbox_handoff
            .durable_envelope
            .durable_envelope_count,
        source_custody_readiness_record_count: cross_process_custody_readiness.records.len(),
        cross_process_replay_record_count: records.len(),
        cross_process_replay_store_write_count: records.len(),
        cross_process_replay_cursor_next_sequence: cursor_next_sequence,
        cross_process_replay_records_match_durable_envelopes: true,
        cross_process_replay_records_match_custody_readiness: true,
        cross_process_replay_implemented: true,
        broker_delivery_implemented: cross_process_custody_readiness.broker_delivery_implemented,
        family_hub_delivery_implemented: cross_process_custody_readiness
            .family_hub_delivery_implemented,
        remote_delivery_ack_implemented: cross_process_custody_readiness
            .remote_delivery_ack_implemented,
        provider_delivery_implemented: cross_process_custody_readiness
            .provider_delivery_implemented,
        child_device_delivery_implemented: cross_process_custody_readiness
            .child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: cross_process_custody_readiness
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: cross_process_custody_readiness
            .product_ready_remote_delivery,
        policy_authority: cross_process_custody_readiness.policy_authority,
        side_effect_authority: cross_process_custody_readiness.side_effect_authority,
        enforcement_command_event_count: cross_process_custody_readiness
            .enforcement_command_event_count,
        adapter_action_executed_count: cross_process_custody_readiness
            .adapter_action_executed_count,
        raw_pcap_available_count: cross_process_custody_readiness.raw_pcap_available_count,
        exact_url_available_count: cross_process_custody_readiness.exact_url_available_count,
        decrypted_payload_available_count: cross_process_custody_readiness
            .decrypted_payload_available_count,
        page_content_available_count: cross_process_custody_readiness.page_content_available_count,
        video_content_available_count: cross_process_custody_readiness
            .video_content_available_count,
        private_message_content_available_count: cross_process_custody_readiness
            .private_message_content_available_count,
        search_query_available_count: cross_process_custody_readiness.search_query_available_count,
        cross_process_replay_ref: refs.cross_process_replay_ref,
        cross_process_replay_store_ref: refs.cross_process_replay_store_ref,
        cross_process_replay_cursor_ref: refs.cross_process_replay_cursor_ref,
        replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState::DurableReplayRecorded,
        records,
        cross_process_custody_readiness,
    }
}

fn has_unsupported_claims(
    report: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) -> bool {
    report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report.cross_process_replay_implemented
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
