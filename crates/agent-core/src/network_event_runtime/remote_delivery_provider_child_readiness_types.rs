use ocentra_eventing::error::EventingError;

use super::remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportError;

pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryProviderChildReadinessState;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryProviderChildReadinessReport;

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
