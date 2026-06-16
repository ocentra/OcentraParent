use ocentra_eventing::{
    error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType, ids::SourceComponent,
};

use super::{
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDeleteExportPropagationState {
    ReadinessRecordedNotPropagated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_fixture_state: NetworkRuntimeRemoteDeliveryFixtureTransportState,
    pub propagation_state: NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
    pub delete_export_propagation_ref: SourceComponent,
    pub remote_delete_readiness_ref: SourceComponent,
    pub remote_export_readiness_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport {
    pub fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    pub delete_export_propagation_ref: SourceComponent,
    pub remote_delete_readiness_ref: SourceComponent,
    pub remote_export_readiness_ref: SourceComponent,
    pub source_fixture_record_count: usize,
    pub propagation_readiness_record_count: usize,
    pub remote_delete_ready_count: usize,
    pub remote_export_ready_count: usize,
    pub propagation_records_match_fixture_records: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDeleteExportPropagationError {
    FixtureTransport(NetworkRuntimeRemoteDeliveryFixtureTransportError),
    Eventing(EventingError),
    EmptyFixtureTransport,
    UnsupportedClaim,
    PropagationRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryDeleteExportPropagationError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
