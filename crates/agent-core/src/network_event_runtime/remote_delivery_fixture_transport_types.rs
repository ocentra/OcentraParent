use ocentra_eventing::error::EventingError;

use super::remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError;

pub type NetworkRuntimeRemoteDeliveryFixtureTransportState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryFixtureTransportState;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryFixtureTransportRecord;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryFixtureTransportReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryFixtureTransportError {
    OutboxHandoff(NetworkRuntimeRemoteDeliveryOutboxHandoffError),
    Eventing(EventingError),
    EmptyOutbox,
    UnsupportedClaim,
    FixtureRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryFixtureTransportError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
