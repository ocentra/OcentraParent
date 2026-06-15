use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryProviderChildReadinessState {
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub fixture_ack_ref: SourceComponent,
    pub provider_route_ref: SourceComponent,
    pub child_device_route_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryProviderChildReadinessReport {
    pub fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    pub provider_route_ref: SourceComponent,
    pub child_device_route_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub source_fixture_ack_count: usize,
    pub provider_delivery_readiness_record_count: usize,
    pub child_device_delivery_readiness_record_count: usize,
    pub provider_delivery_artifact_count: usize,
    pub child_device_delivery_artifact_count: usize,
    pub provider_delivery_records_match_fixture_acks: bool,
    pub child_device_delivery_records_match_fixture_acks: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryProviderChildReadinessError {
    FixtureTransport(NetworkRuntimeRemoteDeliveryFixtureTransportError),
    Eventing(EventingError),
    EmptyFixtureTransport,
    UnsupportedClaim,
    ReadinessRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryProviderChildReadinessError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
