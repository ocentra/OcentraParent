use ocentra_eventing::{
    EventDeliveryRequiredArtifact, EventDeliveryRouteKind, EventingError, SourceComponent,
};

use super::{
    NetworkRuntimeRemoteDeliveryOutboxHandoffError, NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDispatchReadinessState {
    ManualRequiredTransportNotImplemented,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDispatchGate {
    pub gate_ref: SourceComponent,
    pub route_kind: EventDeliveryRouteKind,
    pub required_artifacts: Vec<EventDeliveryRequiredArtifact>,
    pub required_artifact_count: usize,
    pub missing_artifact_count: usize,
    pub fixture_requirements_satisfied: bool,
    pub transport_implemented: bool,
    pub dispatch_ready: bool,
    pub manual_required: bool,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDispatchReadinessReport {
    pub outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    pub dispatch_readiness_ref: SourceComponent,
    pub transport_requirements_ref: SourceComponent,
    pub broker_gate: NetworkRuntimeRemoteDeliveryDispatchGate,
    pub family_hub_gate: NetworkRuntimeRemoteDeliveryDispatchGate,
    pub state: NetworkRuntimeRemoteDeliveryDispatchReadinessState,
    pub source_outbox_candidate_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub manual_required_candidate_count: usize,
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDispatchReadinessError {
    OutboxHandoff(NetworkRuntimeRemoteDeliveryOutboxHandoffError),
    Eventing(EventingError),
    EmptyOutbox,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryDispatchReadinessError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
