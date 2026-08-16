use ocentra_eventing::error::EventingError;

use super::remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError;

pub type NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord;
pub type NetworkRuntimeRemoteDeliveryDurableEnvelopeReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryDurableEnvelopeReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDurableEnvelopeError {
    ReceiptLedger(NetworkRuntimeRemoteDeliveryReceiptLedgerError),
    Eventing(EventingError),
    EmptyDurableEnvelopeStore,
    DurableEnvelopeReceiptMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryDurableEnvelopeError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
