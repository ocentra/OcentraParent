use ocentra_eventing::error::EventingError;

use super::remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError;

pub type NetworkRuntimeRemoteDeliveryOutboxState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryOutboxState;
pub type NetworkRuntimeRemoteDeliveryOutboxCandidate =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryOutboxCandidate;
pub type NetworkRuntimeRemoteDeliveryOutboxHandoffReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryOutboxHandoffReport;

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
