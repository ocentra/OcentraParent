use ocentra_eventing::ids::SourceComponent;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_cross_process_custody_readiness_types::{
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_provider_child_readiness::prove_network_runtime_remote_delivery_provider_child_readiness;
use super::remote_delivery_provider_child_readiness_types::{
    NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
};

pub async fn prove_network_runtime_remote_delivery_cross_process_custody_readiness() -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
> {
    let provider_child_readiness = prove_network_runtime_remote_delivery_provider_child_readiness()
        .await
        .map_err(
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::ProviderChildReadiness,
        )?;
    prove_network_runtime_remote_delivery_cross_process_custody_readiness_from_provider_child_readiness(
        provider_child_readiness,
    )
}

pub fn prove_network_runtime_remote_delivery_cross_process_custody_readiness_from_provider_child_readiness(
    provider_child_readiness: NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
> {
    if provider_child_readiness.records.is_empty() {
        return Err(
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::EmptyProviderChildReadiness,
        );
    }
    if has_unsupported_claims(&provider_child_readiness) {
        return Err(
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::UnsupportedClaim,
        );
    }

    let refs = cross_process_custody_readiness_refs()?;
    let records = cross_process_custody_readiness_records(&provider_child_readiness.records, &refs);

    if !custody_records_match_provider_child_readiness(&records, &provider_child_readiness.records)
    {
        return Err(
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::CustodyRecordMismatch,
        );
    }

    Ok(cross_process_custody_readiness_report(
        provider_child_readiness,
        refs,
        records,
    ))
}

struct CrossProcessCustodyReadinessRefs {
    cross_process_custody_status_ref: SourceComponent,
    cross_process_replay_readiness_ref: SourceComponent,
    remote_retention_readiness_ref: SourceComponent,
    remote_delete_custody_readiness_ref: SourceComponent,
    remote_export_custody_readiness_ref: SourceComponent,
}

fn cross_process_custody_readiness_refs() -> Result<
    CrossProcessCustodyReadinessRefs,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
> {
    Ok(CrossProcessCustodyReadinessRefs {
        cross_process_custody_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF,
        )?,
        cross_process_replay_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF,
        )?,
        remote_retention_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF,
        )?,
        remote_delete_custody_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF,
        )?,
        remote_export_custody_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF,
        )?,
    })
}

fn cross_process_custody_readiness_records(
    provider_child_records: &[NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord],
    refs: &CrossProcessCustodyReadinessRefs,
) -> Vec<NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord> {
    provider_child_records
        .iter()
        .map(|record| cross_process_custody_readiness_record(record, refs))
        .collect()
}

fn cross_process_custody_readiness_report(
    provider_child_readiness: NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    refs: CrossProcessCustodyReadinessRefs,
    records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord>,
) -> NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport {
    let cross_process_replay_implemented = provider_child_readiness
        .fixture_transport
        .outbox_handoff
        .durable_envelope
        .receipt_ledger
        .remote_delivery_status
        .cross_process_replay_implemented;
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport {
        source_provider_child_readiness_record_count: provider_child_readiness.records.len(),
        cross_process_replay_readiness_record_count: records.len(),
        remote_retention_readiness_record_count: records.len(),
        remote_delete_custody_readiness_record_count: records.len(),
        remote_export_custody_readiness_record_count: records.len(),
        cross_process_replay_artifact_count: 0,
        remote_retention_artifact_count: 0,
        remote_delete_custody_artifact_count: 0,
        remote_export_custody_artifact_count: 0,
        custody_records_match_provider_child_readiness: true,
        broker_delivery_implemented: provider_child_readiness.broker_delivery_implemented,
        family_hub_delivery_implemented: provider_child_readiness.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: provider_child_readiness.remote_delivery_ack_implemented,
        provider_delivery_implemented: provider_child_readiness.provider_delivery_implemented,
        child_device_delivery_implemented: provider_child_readiness
            .child_device_delivery_implemented,
        cross_process_replay_implemented,
        remote_delete_export_propagation_implemented: provider_child_readiness
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: provider_child_readiness.product_ready_remote_delivery,
        policy_authority: provider_child_readiness.policy_authority,
        side_effect_authority: provider_child_readiness.side_effect_authority,
        enforcement_command_event_count: provider_child_readiness.enforcement_command_event_count,
        adapter_action_executed_count: provider_child_readiness.adapter_action_executed_count,
        raw_pcap_available_count: provider_child_readiness.raw_pcap_available_count,
        exact_url_available_count: provider_child_readiness.exact_url_available_count,
        decrypted_payload_available_count: provider_child_readiness
            .decrypted_payload_available_count,
        page_content_available_count: provider_child_readiness.page_content_available_count,
        video_content_available_count: provider_child_readiness.video_content_available_count,
        private_message_content_available_count: provider_child_readiness
            .private_message_content_available_count,
        search_query_available_count: provider_child_readiness.search_query_available_count,
        cross_process_custody_status_ref: refs.cross_process_custody_status_ref,
        cross_process_replay_readiness_ref: refs.cross_process_replay_readiness_ref,
        remote_retention_readiness_ref: refs.remote_retention_readiness_ref,
        remote_delete_custody_readiness_ref: refs.remote_delete_custody_readiness_ref,
        remote_export_custody_readiness_ref: refs.remote_export_custody_readiness_ref,
        custody_state:
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable,
        records,
        provider_child_readiness,
    }
}

fn cross_process_custody_readiness_record(
    record: &NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord,
    refs: &CrossProcessCustodyReadinessRefs,
) -> NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord {
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord {
        sequence: record.sequence,
        event_id: record.event_id.clone(),
        event_type: record.event_type.clone(),
        correlation_id: record.correlation_id.clone(),
        source_provider_state: record.provider_state,
        source_child_device_state: record.child_device_state,
        fixture_ack_ref: record.fixture_ack_ref.clone(),
        provider_readiness_ref: record.provider_readiness_ref.clone(),
        child_device_readiness_ref: record.child_device_readiness_ref.clone(),
        cross_process_custody_status_ref: refs.cross_process_custody_status_ref.clone(),
        cross_process_replay_readiness_ref: refs.cross_process_replay_readiness_ref.clone(),
        remote_retention_readiness_ref: refs.remote_retention_readiness_ref.clone(),
        remote_delete_custody_readiness_ref: refs.remote_delete_custody_readiness_ref.clone(),
        remote_export_custody_readiness_ref: refs.remote_export_custody_readiness_ref.clone(),
        custody_state:
            NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable,
    }
}

fn custody_records_match_provider_child_readiness(
    records: &[NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord],
    provider_child_records: &[NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord],
) -> bool {
    records.len() == provider_child_records.len()
        && records.iter().zip(provider_child_records.iter()).all(
            |(record, provider_child_record)| {
                record.sequence == provider_child_record.sequence
                    && record.event_id == provider_child_record.event_id
                    && record.event_type == provider_child_record.event_type
                    && record.correlation_id == provider_child_record.correlation_id
                    && record.fixture_ack_ref == provider_child_record.fixture_ack_ref
                    && record.provider_readiness_ref == provider_child_record.provider_readiness_ref
                    && record.child_device_readiness_ref
                        == provider_child_record.child_device_readiness_ref
                    && record.source_provider_state == provider_child_record.provider_state
                    && record.source_child_device_state == provider_child_record.child_device_state
            },
        )
}

fn has_unsupported_claims(
    report: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
) -> bool {
    report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report
            .fixture_transport
            .outbox_handoff
            .durable_envelope
            .receipt_ledger
            .remote_delivery_status
            .cross_process_replay_implemented
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
