use std::path::PathBuf;

use ocentra_parent_agent_core::{
    authenticated_delivery_execution::{
        AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionState,
        AuthenticatedDeliveryExecutionStore,
    },
    authenticated_delivery_grant::{
        redacted_delivery_nonce_digest, validate_authenticated_delivery_grant,
        AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
    },
    enforcement_adapter::OwnedProcessTerminationTarget,
};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

pub struct AuthenticatedDeliveryExecutionRequest {
    pub store_path: PathBuf,
    pub issuer_key_id: String,
    pub nonce: String,
    pub correlation_id: String,
    pub completed_at: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticatedDeliveryExecutionApiError {
    GrantRejected,
    StoreRejected,
    ExecutionRejected,
    ReplayRejected,
}
pub fn execute_authenticated_owned_process_delivery(
    request: AuthenticatedDeliveryExecutionRequest,
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    target: OwnedProcessTerminationTarget,
) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionApiError> {
    validate_authenticated_delivery_grant(grant, expected, trusted_issuer)
        .map_err(|_| AuthenticatedDeliveryExecutionApiError::GrantRejected)?;
    if request.issuer_key_id != grant.issuer_key_id || request.nonce != grant.nonce {
        return Err(AuthenticatedDeliveryExecutionApiError::GrantRejected);
    }
    let mut store = AuthenticatedDeliveryExecutionStore::open(request.store_path)
        .map_err(|_| AuthenticatedDeliveryExecutionApiError::StoreRejected)?;
    let receipt = AuthenticatedDeliveryExecutionReceipt {
        correlation_id: request.correlation_id,
        nonce_digest: redacted_delivery_nonce_digest(&grant.nonce),
        state: AuthenticatedDeliveryExecutionState::Pending,
        adapter_result: None,
        rollback_required: false,
    };
    if store
        .persist_intent(&grant.issuer_key_id, &grant.nonce, &receipt)
        .map_err(|_| AuthenticatedDeliveryExecutionApiError::StoreRejected)?
    {
        store
            .execute_owned_process(
                &grant.issuer_key_id,
                &grant.nonce,
                target,
                &request.completed_at,
            )
            .map_err(|_| AuthenticatedDeliveryExecutionApiError::ExecutionRejected)
    } else {
        store
            .recover_pending(&grant.issuer_key_id, &grant.nonce)
            .map_err(|_| AuthenticatedDeliveryExecutionApiError::ReplayRejected)
    }
}
