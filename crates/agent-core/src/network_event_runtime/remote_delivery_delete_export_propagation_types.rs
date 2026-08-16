use ocentra_eventing::error::EventingError;

use super::remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportError;

pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDeleteExportPropagationState;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport;

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
