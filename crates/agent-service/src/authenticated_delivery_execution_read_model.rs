use ocentra_parent_agent_core::authenticated_delivery_execution::{
    AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionStore,
};
use std::path::PathBuf;
pub struct AuthenticatedDeliveryExecutionReadRequest {
    pub store_path: PathBuf,
    pub issuer_key_id: String,
    pub nonce: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticatedDeliveryExecutionReadError {
    StoreRejected,
    ReceiptRejected,
}
pub fn authenticated_delivery_execution_read_model(
    request: AuthenticatedDeliveryExecutionReadRequest,
) -> Result<Option<AuthenticatedDeliveryExecutionReceipt>, AuthenticatedDeliveryExecutionReadError>
{
    let store = AuthenticatedDeliveryExecutionStore::open(request.store_path)
        .map_err(|_| AuthenticatedDeliveryExecutionReadError::StoreRejected)?;
    store
        .read_receipt(&request.issuer_key_id, &request.nonce)
        .map_err(|_| AuthenticatedDeliveryExecutionReadError::ReceiptRejected)
}
