use ocentra_eventing::error::EventingError;

use super::remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayError;

pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError {
    CrossProcessReplay(NetworkRuntimeRemoteDeliveryCrossProcessReplayError),
    Eventing(EventingError),
    EmptyCrossProcessReplay,
    UnsupportedClaim,
    TransportRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
