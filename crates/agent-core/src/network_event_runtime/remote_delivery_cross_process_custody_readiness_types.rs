use ocentra_eventing::error::EventingError;

use super::remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessError;

pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError {
    ProviderChildReadiness(NetworkRuntimeRemoteDeliveryProviderChildReadinessError),
    Eventing(EventingError),
    EmptyProviderChildReadiness,
    UnsupportedClaim,
    CustodyRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
