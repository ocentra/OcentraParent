use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryOutboxState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryTransportDispatchState {
    ManualRequiredBlocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryBlockedDispatchRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_outbox_state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub blocked_state: NetworkRuntimeRemoteDeliveryTransportDispatchState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub dispatch_state_ref: SourceComponent,
    pub blocked_dispatch_ref: SourceComponent,
    pub future_transport_seam_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryTransportDispatchStateReport {
    pub no_enforcement_invariant: NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    pub dispatch_state_ref: SourceComponent,
    pub blocked_dispatch_ref: SourceComponent,
    pub future_transport_seam_ref: SourceComponent,
    pub state: NetworkRuntimeRemoteDeliveryTransportDispatchState,
    pub source_outbox_candidate_count: usize,
    pub blocked_dispatch_record_count: usize,
    pub dispatch_ready_candidate_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
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
    pub blocked_dispatch_records: Vec<NetworkRuntimeRemoteDeliveryBlockedDispatchRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryTransportDispatchStateError {
    NoEnforcementInvariant(NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError),
    Eventing(EventingError),
    EmptyOutbox,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryTransportDispatchStateError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
