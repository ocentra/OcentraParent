use ocentra_eventing::{
    error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType, ids::SourceComponent,
};

use super::{
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState {
    DeterministicEnvelopeAckRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub external_cross_process_transport_ref: SourceComponent,
    pub external_cross_process_transport_envelope_ref: SourceComponent,
    pub external_cross_process_transport_ack_ref: SourceComponent,
    pub transport_state: NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport {
    pub cross_process_replay: NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    pub external_cross_process_transport_ref: SourceComponent,
    pub external_cross_process_transport_envelope_ref: SourceComponent,
    pub external_cross_process_transport_ack_ref: SourceComponent,
    pub source_replay_record_count: usize,
    pub external_cross_process_transport_record_count: usize,
    pub external_cross_process_transport_envelope_count: usize,
    pub external_cross_process_transport_ack_count: usize,
    pub external_cross_process_transport_records_match_replay_records: bool,
    pub external_cross_process_transport_ack_records_match_envelopes: bool,
    pub external_cross_process_transport_implemented: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError {
    CrossProcessReplay(NetworkRuntimeRemoteDeliveryCrossProcessReplayError),
    Eventing(EventingError),
    EmptyCrossProcessReplay,
    UnsupportedClaim,
    TransportRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
