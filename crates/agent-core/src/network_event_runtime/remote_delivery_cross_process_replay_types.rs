use ocentra_eventing::{CorrelationId, EventId, EventType, EventingError, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessReplayState {
    DurableReplayRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessReplayReport {
    pub cross_process_custody_readiness:
        NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    pub source_durable_envelope_count: usize,
    pub source_custody_readiness_record_count: usize,
    pub cross_process_replay_record_count: usize,
    pub cross_process_replay_store_write_count: usize,
    pub cross_process_replay_cursor_next_sequence: u64,
    pub cross_process_replay_records_match_durable_envelopes: bool,
    pub cross_process_replay_records_match_custody_readiness: bool,
    pub cross_process_replay_implemented: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessReplayError {
    CrossProcessCustodyReadiness(NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError),
    Eventing(EventingError),
    EmptyCrossProcessCustodyReadiness,
    UnsupportedClaim,
    ReplayRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryCrossProcessReplayError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
