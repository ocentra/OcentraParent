use ocentra_eventing::error::EventingError;

use super::remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError;

pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessReplayState;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryCrossProcessReplayReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessReplayError {
    CrossProcessCustodyReadiness(NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError),
    Eventing(EventingError),
    EmptyCrossProcessCustodyReadiness,
    UnsupportedClaim,
    ReplayRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryCrossProcessReplayError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
