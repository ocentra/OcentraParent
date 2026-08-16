use ocentra_parent_agent_core::authenticated_delivery_execution::{
    AuthenticatedAdapterExecutionTrace, AuthenticatedDeliveryExecutionReceipt,
    AuthenticatedDeliveryExecutionStore,
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
        .map_err(|_error| AuthenticatedDeliveryExecutionReadError::StoreRejected)?;
    store
        .read_receipt(&request.issuer_key_id, &request.nonce)
        .map_err(|_error| AuthenticatedDeliveryExecutionReadError::ReceiptRejected)
}

pub struct AuthenticatedDeliveryExecutionTraceReadRequest {
    pub store_path: PathBuf,
    pub issuer_key_id: String,
    pub nonce: String,
}

pub fn authenticated_delivery_execution_trace_read_model(
    request: AuthenticatedDeliveryExecutionTraceReadRequest,
) -> Result<Option<AuthenticatedAdapterExecutionTrace>, AuthenticatedDeliveryExecutionReadError> {
    let store = AuthenticatedDeliveryExecutionStore::open(request.store_path)
        .map_err(|_error| AuthenticatedDeliveryExecutionReadError::StoreRejected)?;
    store
        .read_trace(&request.issuer_key_id, &request.nonce)
        .map_err(|_error| AuthenticatedDeliveryExecutionReadError::ReceiptRejected)
}
