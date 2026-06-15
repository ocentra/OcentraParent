use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryReceiptLedgerError, NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub delete_export_readiness_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDurableEnvelopeReport {
    pub receipt_ledger: NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub durable_replay_ref: SourceComponent,
    pub delete_export_readiness_ref: SourceComponent,
    pub durable_support_status_ref: SourceComponent,
    pub source_receipt_record_count: usize,
    pub durable_envelope_count: usize,
    pub durable_store_write_count: usize,
    pub durable_replay_ready_count: usize,
    pub delete_export_ready_count: usize,
    pub ordered_sequence_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
    pub durable_records_match_receipts: bool,
    pub durable_store_ready: bool,
    pub durable_replay_ready: bool,
    pub delete_export_readiness_recorded: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub durable_records: Vec<NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDurableEnvelopeError {
    ReceiptLedger(NetworkRuntimeRemoteDeliveryReceiptLedgerError),
    Eventing(EventingError),
    EmptyDurableEnvelopeStore,
    DurableEnvelopeReceiptMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryDurableEnvelopeError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
