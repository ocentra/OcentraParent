use std::path::PathBuf;

use ocentra_parent_agent_core::{
    authenticated_delivery_execution::AuthenticatedDeliveryExecutionReceipt,
    authenticated_delivery_grant::{
        AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
    },
};
use ocentra_schema::{
    authenticated_delivery_grant::AuthenticatedDeliveryGrant,
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};

#[path = "authenticated_delivery_execution_api_flow.rs"]
mod authenticated_delivery_execution_api_flow;

pub struct AuthenticatedDeliveryExecutionRequest {
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

struct AuthenticatedDeliveryExecutionContext {
    store_path: PathBuf,
    activity_store_path: PathBuf,
}

pub struct AuthenticatedDeliveryExecutionExecutor {
    context: AuthenticatedDeliveryExecutionContext,
}

impl AuthenticatedDeliveryExecutionExecutor {
    pub fn execute_authenticated_owned_process_delivery(
        &self,
        request: AuthenticatedDeliveryExecutionRequest,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    ) -> Result<AuthenticatedDeliveryExecutionReceipt, AuthenticatedDeliveryExecutionApiError> {
        authenticated_delivery_execution_api_flow::execute(
            &self.context,
            request,
            grant,
            expected,
            trusted_issuer,
        )
    }
}
