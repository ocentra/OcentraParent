use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryOutboxState {
    PreparedNotDispatched,
    DispatchBlockedManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryOutboxCandidate {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryOutboxHandoffReport {
    pub durable_envelope: NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub outbox_replay_ref: SourceComponent,
    pub outbox_support_status_ref: SourceComponent,
    pub source_durable_envelope_count: usize,
    pub source_receipt_record_count: usize,
    pub outbox_candidate_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub duplicate_durable_envelope_rejected: bool,
    pub outbox_candidates_match_durable_envelopes: bool,
    pub outbox_candidates_match_receipts: bool,
    pub sequence_gap_count: usize,
    pub event_id_mismatch_count: usize,
    pub event_type_mismatch_count: usize,
    pub correlation_mismatch_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
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
    pub candidates: Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryOutboxHandoffError {
    DurableEnvelope(NetworkRuntimeRemoteDeliveryDurableEnvelopeError),
    Eventing(EventingError),
    EmptyOutbox,
    DuplicateOutboxCandidate,
    OutboxDurableEnvelopeMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryOutboxHandoffError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
