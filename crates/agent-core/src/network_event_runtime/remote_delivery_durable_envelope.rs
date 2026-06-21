use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_durable_envelope_types::{
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_receipt_ledger::prove_network_runtime_remote_delivery_receipt_ledger;
use super::remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptRecord;

pub async fn prove_network_runtime_remote_delivery_durable_envelope() -> Result<
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
> {
    let receipt_ledger = prove_network_runtime_remote_delivery_receipt_ledger()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger)?;
    if receipt_ledger.enforcement_command_event_count
        + receipt_ledger.adapter_action_executed_count
        + receipt_ledger.raw_pcap_available_count
        + receipt_ledger.exact_url_available_count
        + receipt_ledger.decrypted_payload_available_count
        + receipt_ledger.page_content_available_count
        + receipt_ledger.video_content_available_count
        + receipt_ledger.private_message_content_available_count
        + receipt_ledger.search_query_available_count
        > 0
    {
        return Err(NetworkRuntimeRemoteDeliveryDurableEnvelopeError::UnsupportedClaim);
    }
    let durable_records = durable_records_from_receipts(&receipt_ledger.receipts)?;
    assert_durable_records_match_receipts(&receipt_ledger.receipts, &durable_records)?;
    Ok(NetworkRuntimeRemoteDeliveryDurableEnvelopeReport {
        durable_envelope_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF,
        )?,
        durable_store_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF,
        )?,
        durable_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF,
        )?,
        delete_export_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF,
        )?,
        durable_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF,
        )?,
        source_receipt_record_count: receipt_ledger.receipt_record_count,
        durable_envelope_count: durable_records.len(),
        durable_store_write_count: durable_records.len(),
        durable_replay_ready_count: durable_records.len(),
        delete_export_ready_count: durable_records.len(),
        ordered_sequence_count: ordered_sequence_count(&durable_records),
        unique_event_id_count: unique_event_id_count(&durable_records),
        unique_correlation_id_count: unique_correlation_id_count(&durable_records),
        durable_records_match_receipts: true,
        durable_store_ready: true,
        durable_replay_ready: true,
        delete_export_readiness_recorded: true,
        broker_delivery_implemented: receipt_ledger.broker_delivery_implemented,
        family_hub_delivery_implemented: receipt_ledger.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: receipt_ledger.remote_delivery_ack_implemented,
        provider_delivery_implemented: false,
        child_device_delivery_implemented: false,
        remote_delete_export_propagation_implemented: false,
        product_ready_remote_delivery: false,
        policy_authority: receipt_ledger.policy_authority,
        side_effect_authority: receipt_ledger.side_effect_authority,
        enforcement_command_event_count: receipt_ledger.enforcement_command_event_count,
        adapter_action_executed_count: receipt_ledger.adapter_action_executed_count,
        raw_pcap_available_count: receipt_ledger.raw_pcap_available_count,
        exact_url_available_count: receipt_ledger.exact_url_available_count,
        decrypted_payload_available_count: receipt_ledger.decrypted_payload_available_count,
        page_content_available_count: receipt_ledger.page_content_available_count,
        video_content_available_count: receipt_ledger.video_content_available_count,
        private_message_content_available_count: receipt_ledger
            .private_message_content_available_count,
        search_query_available_count: receipt_ledger.search_query_available_count,
        receipt_ledger,
        durable_records,
    })
}

fn durable_records_from_receipts(
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
) -> Result<
    Vec<NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord>,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
> {
    receipts
        .iter()
        .map(durable_record_from_receipt)
        .collect::<Result<Vec<NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord>, _>>()
}

fn durable_record_from_receipt(
    receipt: &NetworkRuntimeRemoteDeliveryReceiptRecord,
) -> Result<
    NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
> {
    Ok(NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord {
        sequence: receipt.sequence,
        event_id: receipt.event_id.clone(),
        event_type: receipt.event_type.clone(),
        correlation_id: receipt.correlation_id.clone(),
        durable_envelope_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF,
        )?,
        durable_store_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF,
        )?,
        receipt_ledger_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF,
        )?,
        local_receipt_ack_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF,
        )?,
        delete_export_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF,
        )?,
    })
}

fn assert_durable_records_match_receipts(
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
) -> Result<(), NetworkRuntimeRemoteDeliveryDurableEnvelopeError> {
    if receipts.is_empty() || durable_records.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryDurableEnvelopeError::EmptyDurableEnvelopeStore);
    }
    if receipts.len() != durable_records.len() {
        return Err(
            NetworkRuntimeRemoteDeliveryDurableEnvelopeError::DurableEnvelopeReceiptMismatch,
        );
    }
    for (receipt, durable_record) in receipts.iter().zip(durable_records.iter()) {
        if receipt.sequence != durable_record.sequence
            || receipt.event_id != durable_record.event_id
            || receipt.event_type != durable_record.event_type
            || receipt.correlation_id != durable_record.correlation_id
            || durable_record.durable_envelope_ref.as_str()
                != constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
            || durable_record.durable_store_ref.as_str()
                != constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF
            || durable_record.receipt_ledger_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
            || durable_record.local_receipt_ack_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
            || durable_record.delete_export_readiness_ref.as_str()
                != constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF
        {
            return Err(
                NetworkRuntimeRemoteDeliveryDurableEnvelopeError::DurableEnvelopeReceiptMismatch,
            );
        }
    }
    Ok(())
}

fn ordered_sequence_count(records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord]) -> usize {
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

fn unique_event_id_count(records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord]) -> usize {
    records
        .iter()
        .map(|record| record.event_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

fn unique_correlation_id_count(
    records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
) -> usize {
    records
        .iter()
        .map(|record| record.correlation_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}
