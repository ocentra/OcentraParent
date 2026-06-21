use ocentra_eventing::error::EventingError;

use super::remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessError;

pub type NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementStage =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryNoEnforcementStage;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError {
    DispatchReadiness(NetworkRuntimeRemoteDeliveryDispatchReadinessError),
    Eventing(EventingError),
    MissingAvailableMetadata,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
