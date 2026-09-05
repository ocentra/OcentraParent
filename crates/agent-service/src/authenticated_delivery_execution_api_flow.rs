use super::{
    AuthenticatedDeliveryExecutionApiError, AuthenticatedDeliveryExecutionContext,
    AuthenticatedDeliveryExecutionRequest,
};
use ocentra_parent_agent_core::authenticated_delivery_execution::{
    authenticated_managed_process_target, AuthenticatedDeliveryExecutionReceipt,
    AuthenticatedDeliveryExecutionState, AuthenticatedDeliveryExecutionStore,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::execution_validation::{
    redacted_delivery_nonce_digest, validate_authenticated_delivery_grant,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
    AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
};
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

pub(super) fn execute(
    context: &AuthenticatedDeliveryExecutionContext,
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
        &context.activity_store_path,
    )
    .map_err(|_error| AuthenticatedDeliveryExecutionApiError::TargetBindingRejected)?;
    let mut store = AuthenticatedDeliveryExecutionStore::open(&context.store_path)
        .map_err(|_error| AuthenticatedDeliveryExecutionApiError::StoreRejected)?;
    let receipt = AuthenticatedDeliveryExecutionReceipt {
        correlation_id: request.correlation_id.clone(),
        nonce_digest: redacted_delivery_nonce_digest(&grant.nonce),
        state: AuthenticatedDeliveryExecutionState::Pending,
        adapter_result: None,
        rollback_required: false,
    };
    persist_intent_and_execute(PendingExecution {
        context,
        store: &mut store,
        request,
        grant,
        expected,
        trusted_issuer,
        target,
        receipt,
    })
}

struct PendingExecution<'a> {
    context: &'a AuthenticatedDeliveryExecutionContext,
    store: &'a mut AuthenticatedDeliveryExecutionStore,
    request: AuthenticatedDeliveryExecutionRequest,
    grant: &'a AuthenticatedDeliveryGrant,
    expected: &'a AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &'a AuthenticatedDeliveryGrantTrustedIssuer,
    target:
        ocentra_parent_agent_core::enforcement_adapter::AuthenticatedOwnedProcessTerminationTarget,
    receipt: AuthenticatedDeliveryExecutionReceipt,
}

fn persist_intent_and_execute(
    execution: PendingExecution<'_>,
) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionApiError> {
    let persisted = execution
        .store
        .persist_intent(
            &execution.grant.issuer_key_id,
            &execution.grant.nonce,
            &execution.receipt,
        )
        .map_err(|_error| AuthenticatedDeliveryExecutionApiError::StoreRejected)?;
    if !persisted {
        return Err(AuthenticatedDeliveryExecutionApiError::ReplayRejected);
    }
    consume_grant_and_execute(execution)
}

fn consume_grant_and_execute(
    execution: PendingExecution<'_>,
) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionApiError> {
    let PendingExecution {
        context,
        store,
        request,
        grant,
        expected,
        trusted_issuer,
        target,
        receipt,
    } = execution;
    let mut grant_consumer =
        AuthenticatedDeliveryGrantConsumer::open(&context.store_path, trusted_issuer.clone())
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
}
