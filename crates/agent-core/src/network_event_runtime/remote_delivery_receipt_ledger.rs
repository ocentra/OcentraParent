use super::remote_delivery_receipt_ledger_types::{
    NetworkRuntimeRemoteDeliveryReceiptLedgerError, NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
};

pub async fn prove_network_runtime_remote_delivery_receipt_ledger() -> Result<
    NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
> {
    Err(NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable)
}
