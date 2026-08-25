use ocentra_eventing::error::EventingError;

use super::remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalError;
use super::remote_delivery_status::NetworkRuntimeRemoteDeliveryStatusError;

pub type NetworkRuntimeRemoteDeliveryReceiptRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryReceiptRecord;
pub type NetworkRuntimeRemoteDeliveryReceiptLedgerReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryReceiptLedgerReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryReceiptLedgerError {
    RemoteDeliveryStatus(NetworkRuntimeRemoteDeliveryStatusError),
    EventChainJournal(NetworkRuntimeRemoteEventChainJournalError),
    Eventing(EventingError),
    EmptyReceiptLedger,
    ReceiptProjectionMismatch,
    RuntimeOwnerUnavailable,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryReceiptLedgerError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
