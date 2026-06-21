use ocentra_eventing::error::EventingError;

use super::remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError;

pub type NetworkRuntimeRemoteDeliveryTransportDispatchState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryTransportDispatchState;
pub type NetworkRuntimeRemoteDeliveryBlockedDispatchRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryBlockedDispatchRecord;
pub type NetworkRuntimeRemoteDeliveryTransportDispatchStateReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryTransportDispatchStateReport;

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
