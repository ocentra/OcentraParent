use ocentra_eventing::{
    CorrelationId, EventId, EventType, EventingError, ReplayMode, SourceComponent,
};

use super::{
    NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
    NetworkRuntimeRemoteEventChainJournalError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryReceiptRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub event_chain_journal_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryReceiptLedgerReport {
    pub remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    pub event_chain_journal_ref: SourceComponent,
    pub event_chain_export_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub receipt_replay_ref: SourceComponent,
    pub receipt_support_status_ref: SourceComponent,
    pub source_projection_replay_record_count: usize,
    pub receipt_record_count: usize,
    pub local_receipt_ack_count: usize,
    pub ordered_sequence_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
    pub exported_event_type_count: usize,
    pub replay_cursor_next_sequence: u64,
    pub projection_replay_mode: ReplayMode,
    pub receipt_ledger_ready: bool,
    pub receipt_replay_ready: bool,
    pub receipt_records_match_projection: bool,
    pub receipt_sequence_gap_count: usize,
    pub receipt_event_id_mismatch_count: usize,
    pub receipt_event_type_mismatch_count: usize,
    pub receipt_correlation_mismatch_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub receipts: Vec<NetworkRuntimeRemoteDeliveryReceiptRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryReceiptLedgerError {
    RemoteDeliveryStatus(NetworkRuntimeRemoteDeliveryStatusError),
    EventChainJournal(NetworkRuntimeRemoteEventChainJournalError),
    Eventing(EventingError),
    EmptyReceiptLedger,
    ReceiptProjectionMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryReceiptLedgerError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
