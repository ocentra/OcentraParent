use std::path::PathBuf;

use ocentra_parent_agent_core::{
    authenticated_delivery_execution::{
        authenticated_managed_process_target, AuthenticatedDeliveryExecutionReceipt,
        AuthenticatedDeliveryExecutionState, AuthenticatedDeliveryExecutionStore,
    },
    authenticated_delivery_grant::{
        execution_validation::{
            redacted_delivery_nonce_digest, validate_authenticated_delivery_grant,
        },
        AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
        AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
    },
};
use ocentra_schema::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrant,
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};

pub struct AuthenticatedDeliveryExecutionRequest {
    pub store_path: PathBuf,
    pub activity_store_path: PathBuf,
    pub issuer_key_id: String,
    pub nonce: String,
    pub correlation_id: String,
    pub completed_at: String,
    pub delivered_payload: Vec<u8>,
    pub managed_process_binding: AuthenticatedManagedProcessTargetBinding,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticatedDeliveryExecutionApiError {
    GrantRejected,
    StoreRejected,
    ExecutionRejected,
    ReplayRejected,
    TargetBindingRejected,
}
pub fn execute_authenticated_owned_process_delivery(
    request: AuthenticatedDeliveryExecutionRequest,
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionApiError> {
    validate_authenticated_delivery_grant(grant, expected, trusted_issuer)
        .map_err(|_error| AuthenticatedDeliveryExecutionApiError::GrantRejected)?;
    if request.issuer_key_id != grant.issuer_key_id || request.nonce != grant.nonce {
        return Err(AuthenticatedDeliveryExecutionApiError::GrantRejected);
    }
    let target = authenticated_managed_process_target(
        grant,
        &request.managed_process_binding,
        trusted_issuer,
        &request.activity_store_path,
    )
    .map_err(|_error| AuthenticatedDeliveryExecutionApiError::TargetBindingRejected)?;
    let mut store = AuthenticatedDeliveryExecutionStore::open(request.store_path)
        .map_err(|_error| AuthenticatedDeliveryExecutionApiError::StoreRejected)?;
    let receipt = AuthenticatedDeliveryExecutionReceipt {
        correlation_id: request.correlation_id,
        nonce_digest: redacted_delivery_nonce_digest(&grant.nonce),
        state: AuthenticatedDeliveryExecutionState::Pending,
        adapter_result: None,
        rollback_required: false,
    };
    if store
        .persist_intent(&grant.issuer_key_id, &grant.nonce, &receipt)
        .map_err(|_error| AuthenticatedDeliveryExecutionApiError::StoreRejected)?
    {
        let mut grant_consumer =
            AuthenticatedDeliveryGrantConsumer::open(&request.store_path, trusted_issuer.clone())
                .map_err(|_error| AuthenticatedDeliveryExecutionApiError::GrantRejected)?;
        match grant_consumer
            .consume(
                grant,
                expected,
                &request.delivered_payload,
                receipt.correlation_id.clone(),
            )
            .map_err(|_error| AuthenticatedDeliveryExecutionApiError::GrantRejected)?
        {
            AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_) => {}
            AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_) => {
                return Err(AuthenticatedDeliveryExecutionApiError::ReplayRejected)
            }
        }
        store
            .execute_authenticated_owned_process(
                &grant.issuer_key_id,
                &grant.nonce,
                &target,
                &receipt.correlation_id,
                &request.completed_at,
            )
            .map_err(|_error| AuthenticatedDeliveryExecutionApiError::ExecutionRejected)
    } else {
        store
            .recover_pending(&grant.issuer_key_id, &grant.nonce)
            .map_err(|_error| AuthenticatedDeliveryExecutionApiError::ReplayRejected)
    }
}
