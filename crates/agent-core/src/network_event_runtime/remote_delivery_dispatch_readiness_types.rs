use ocentra_eventing::error::EventingError;

use super::remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError;

pub type NetworkRuntimeRemoteDeliveryDispatchReadinessState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDispatchReadinessState;
pub type NetworkRuntimeRemoteDeliveryDispatchGate =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDispatchGate;
pub type NetworkRuntimeRemoteDeliveryDispatchReadinessReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDispatchReadinessReport;

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
