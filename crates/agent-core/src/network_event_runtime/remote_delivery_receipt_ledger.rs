use std::collections::BTreeSet;

use ocentra_eventing::replay::ReplayRecord;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_event_chain_store::{
    exported_event_type_count, publish_network_runtime_remote_event_chain_store, source_component,
    unsupported_claim_counts,
};
use super::remote_delivery_receipt_ledger_types::{
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    NetworkRuntimeRemoteDeliveryReceiptLedgerReport, NetworkRuntimeRemoteDeliveryReceiptRecord,
};
use super::remote_delivery_status::prove_network_runtime_remote_delivery_status;

pub async fn prove_network_runtime_remote_delivery_receipt_ledger() -> Result<
    NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
> {
    let remote_delivery_status = prove_network_runtime_remote_delivery_status()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::RemoteDeliveryStatus)?;
    let store = publish_network_runtime_remote_event_chain_store()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::EventChainJournal)?;
    let unsupported = unsupported_claim_counts(&store.payloads, &store.projection.records);
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::UnsupportedClaim);
    }
    let receipts = receipt_records_from_projection(&store.projection.records)?;
    assert_receipt_ledger_matches_projection(&store.projection.records, &receipts)?;
    Ok(NetworkRuntimeRemoteDeliveryReceiptLedgerReport {
        event_chain_journal_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF,
        )?,
        event_chain_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF,
        )?,
        receipt_ledger_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF,
        )?,
        local_receipt_ack_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF,
        )?,
        receipt_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_REPLAY_REF,
        )?,
        receipt_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_SUPPORT_STATUS_REF,
        )?,
        source_projection_replay_record_count: store.projection.records.len(),
        receipt_record_count: receipts.len(),
        local_receipt_ack_count: receipts.len(),
        ordered_sequence_count: ordered_sequence_count(&receipts),
        unique_event_id_count: unique_event_id_count(&receipts),
        unique_correlation_id_count: unique_correlation_id_count(&receipts),
        exported_event_type_count: exported_event_type_count(&store.projection.records),
        replay_cursor_next_sequence: store.projection.cursor.next_sequence,
        projection_replay_mode: store.projection.mode,
        receipt_ledger_ready: true,
        receipt_replay_ready: true,
        receipt_records_match_projection: true,
        receipt_sequence_gap_count: 0,
        receipt_event_id_mismatch_count: 0,
        receipt_event_type_mismatch_count: 0,
        receipt_correlation_mismatch_count: 0,
        broker_delivery_implemented: remote_delivery_status.external_transport_delivery_implemented,
        family_hub_delivery_implemented: remote_delivery_status.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: false,
        policy_authority: remote_delivery_status.policy_authority,
        side_effect_authority: remote_delivery_status.side_effect_authority,
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
        remote_delivery_status,
        receipts,
    })
}

fn receipt_records_from_projection(
    records: &[ReplayRecord],
) -> Result<
    Vec<NetworkRuntimeRemoteDeliveryReceiptRecord>,
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
> {
    records
        .iter()
        .map(receipt_record_from_projection)
        .collect::<Result<Vec<NetworkRuntimeRemoteDeliveryReceiptRecord>, _>>()
}

fn receipt_record_from_projection(
    record: &ReplayRecord,
) -> Result<NetworkRuntimeRemoteDeliveryReceiptRecord, NetworkRuntimeRemoteDeliveryReceiptLedgerError>
{
    Ok(NetworkRuntimeRemoteDeliveryReceiptRecord {
        sequence: record.sequence,
        event_id: record.envelope.event_id.clone(),
        event_type: record.envelope.contract.event_type.clone(),
        correlation_id: record.envelope.correlation_id.clone(),
        event_chain_journal_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF,
        )?,
        local_receipt_ack_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF,
        )?,
    })
}

fn assert_receipt_ledger_matches_projection(
    records: &[ReplayRecord],
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
) -> Result<(), NetworkRuntimeRemoteDeliveryReceiptLedgerError> {
    if records.is_empty() || receipts.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::EmptyReceiptLedger);
    }
    if records.len() != receipts.len() {
        return Err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::ReceiptProjectionMismatch);
    }
    for (index, (record, receipt)) in records.iter().zip(receipts.iter()).enumerate() {
        let expected_sequence = match u64::try_from(index) {
            Ok(value) => value.saturating_add(1),
            Err(_) => {
                return Err(
                    NetworkRuntimeRemoteDeliveryReceiptLedgerError::ReceiptProjectionMismatch,
                );
            }
        };
        if record.sequence != expected_sequence
            || receipt.sequence != record.sequence
            || receipt.event_id != record.envelope.event_id
            || receipt.event_type != record.envelope.contract.event_type
            || receipt.correlation_id != record.envelope.correlation_id
            || receipt.event_chain_journal_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF
            || receipt.local_receipt_ack_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
        {
            return Err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::ReceiptProjectionMismatch);
        }
    }
    Ok(())
}

fn ordered_sequence_count(records: &[NetworkRuntimeRemoteDeliveryReceiptRecord]) -> usize {
    records
        .iter()
        .enumerate()
        .filter(|(index, record)| {
            u64::try_from(*index)
                .map(|value| value.saturating_add(1) == record.sequence)
                .unwrap_or(false)
        })
        .count()
}

fn unique_event_id_count(records: &[NetworkRuntimeRemoteDeliveryReceiptRecord]) -> usize {
    records
        .iter()
        .map(|record| record.event_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

fn unique_correlation_id_count(records: &[NetworkRuntimeRemoteDeliveryReceiptRecord]) -> usize {
    records
        .iter()
        .map(|record| record.correlation_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}
