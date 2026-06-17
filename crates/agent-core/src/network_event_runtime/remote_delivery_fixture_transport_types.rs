use ocentra_eventing::{
    error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType, ids::SourceComponent,
};

use super::{
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryFixtureTransportState {
    FixtureAckRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryFixtureTransportRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_outbox_state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub fixture_state: NetworkRuntimeRemoteDeliveryFixtureTransportState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub fixture_transport_ref: SourceComponent,
    pub fixture_dispatch_attempt_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryFixtureTransportReport {
    pub outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    pub fixture_transport_ref: SourceComponent,
    pub fixture_dispatch_attempt_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
    pub source_outbox_candidate_count: usize,
    pub fixture_dispatch_attempt_count: usize,
    pub fixture_remote_ack_count: usize,
    pub fixture_records_match_outbox_candidates: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryFixtureTransportRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryFixtureTransportError {
    OutboxHandoff(NetworkRuntimeRemoteDeliveryOutboxHandoffError),
    Eventing(EventingError),
    EmptyOutbox,
    UnsupportedClaim,
    FixtureRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryFixtureTransportError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
