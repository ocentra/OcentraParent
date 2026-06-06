use ocentra_eventing::{
    CorrelationId, EventId, EventType, EventingError, IdempotencyKey, ReplayMode, SourceComponent,
    TargetHandler,
};

use super::{
    NetworkRuntimeRemoteDeliveryReceiptLedgerError, NetworkRuntimeRemoteDeliveryStatusError,
    NetworkRuntimeRemoteDeliveryStatusReport, NetworkRuntimeRemoteEventChainJournalError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryOutboxState {
    PreparedNotDispatched,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryOutboxCandidate {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub idempotency_key: IdempotencyKey,
    pub target_handler: Option<TargetHandler>,
    pub state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub event_chain_export_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryOutboxHandoffReport {
    pub remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    pub event_chain_export_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub outbox_replay_ref: SourceComponent,
    pub outbox_support_status_ref: SourceComponent,
    pub source_projection_replay_record_count: usize,
    pub receipt_record_count: usize,
    pub outbox_candidate_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub outbox_candidates_match_projection: bool,
    pub outbox_candidates_match_receipts: bool,
    pub sequence_gap_count: usize,
    pub event_id_mismatch_count: usize,
    pub event_type_mismatch_count: usize,
    pub correlation_mismatch_count: usize,
    pub unique_event_id_count: usize,
    pub unique_idempotency_key_count: usize,
    pub target_handler_count: usize,
    pub broker_requirement_ref_count: usize,
    pub receipt_ref_count: usize,
    pub projection_replay_mode: ReplayMode,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
    pub remote_retention_delete_export_propagation_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub candidates: Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryOutboxHandoffError {
    RemoteDeliveryStatus(NetworkRuntimeRemoteDeliveryStatusError),
    EventChainJournal(NetworkRuntimeRemoteEventChainJournalError),
    ReceiptLedger(NetworkRuntimeRemoteDeliveryReceiptLedgerError),
    Eventing(EventingError),
    EmptyOutbox,
    OutboxProjectionMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryOutboxHandoffError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
