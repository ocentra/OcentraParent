use ocentra_eventing::{envelope::StoredEventEnvelope, replay::ReplayReadReport};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkInterventionState;

use super::remote_delivery_event_chain_journal_types::{
    NetworkRuntimeRemoteEventChainJournalError, NetworkRuntimeRemoteEventChainJournalReport,
    UnsupportedClaimCounts,
};
use super::remote_delivery_event_chain_store::{
    exported_event_type_count, publish_network_runtime_remote_event_chain_store, source_component,
    unsupported_claim_counts,
};
use super::remote_delivery_status::{
    prove_network_runtime_remote_delivery_status, NetworkRuntimeRemoteDeliveryStatusReport,
};
use super::NetworkRuntimeEventPayload;

pub async fn prove_network_runtime_remote_event_chain_journal(
) -> Result<NetworkRuntimeRemoteEventChainJournalReport, NetworkRuntimeRemoteEventChainJournalError>
{
    let remote_delivery_status = prove_network_runtime_remote_delivery_status()
        .await
        .map_err(NetworkRuntimeRemoteEventChainJournalError::RemoteDeliveryStatus)?;
    let store = publish_network_runtime_remote_event_chain_store().await?;
    let unsupported = unsupported_claim_counts(&store.payloads, &store.projection.records);
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::UnsupportedClaim);
    }
    build_report(
        remote_delivery_status,
        &store.stored_events,
        &store.projection,
        &store.payloads,
        &unsupported,
    )
}

fn build_report(
    remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    stored_events: &[StoredEventEnvelope],
    projection: &ReplayReadReport,
    payloads: &[NetworkRuntimeEventPayload],
    unsupported: &UnsupportedClaimCounts,
) -> Result<NetworkRuntimeRemoteEventChainJournalReport, NetworkRuntimeRemoteEventChainJournalError>
{
    if stored_events.is_empty() || projection.records.is_empty() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::EmptyJournal);
    }
    if stored_events.len() != projection.records.len() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch);
    }
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::UnsupportedClaim);
    }
    Ok(NetworkRuntimeRemoteEventChainJournalReport {
        event_chain_journal_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF,
        )?,
        event_chain_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_REPLAY_REF,
        )?,
        event_chain_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF,
        )?,
        event_chain_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF,
        )?,
        stored_event_count: stored_events.len(),
        journal_entry_count: projection.records.len(),
        projection_replay_record_count: projection.records.len(),
        replay_cursor_next_sequence: projection.cursor.next_sequence,
        exported_event_type_count: exported_event_type_count(&projection.records),
        exportable_remote_envelope_count: projection.records.len(),
        unavailable_event_count: payloads
            .iter()
            .filter(|payload| payload.intervention_state == NetworkInterventionState::Unavailable)
            .count(),
        enforcement_command_event_count: unsupported.enforcement_command_event_count,
        adapter_action_executed_count: unsupported.adapter_action_executed_count,
        raw_pcap_available_count: unsupported.raw_pcap_available_count,
        exact_url_available_count: unsupported.exact_url_available_count,
        decrypted_payload_available_count: unsupported.decrypted_payload_available_count,
        page_content_available_count: unsupported.page_content_available_count,
        video_content_available_count: unsupported.video_content_available_count,
        private_message_content_available_count: unsupported
            .private_message_content_available_count,
        search_query_available_count: unsupported.search_query_available_count,
        projection_replay_mode: projection.mode,
        broker_delivery_implemented: remote_delivery_status.external_transport_delivery_implemented,
        family_hub_delivery_implemented: remote_delivery_status.family_hub_delivery_implemented,
        policy_authority: remote_delivery_status.policy_authority,
        side_effect_authority: remote_delivery_status.side_effect_authority,
        remote_delivery_status,
    })
}
