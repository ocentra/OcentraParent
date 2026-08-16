#[path = "browser_policy_runtime_flow_impl/get.rs"]
mod get;
#[path = "browser_policy_runtime_flow_impl/patch.rs"]
mod patch;
#[path = "browser_policy_runtime_flow_impl/persist.rs"]
mod persist;
#[path = "browser_policy_runtime_flow_impl/preview.rs"]
mod preview;
#[path = "browser_policy_runtime_flow_impl/replace.rs"]
mod replace;
#[path = "browser_policy_runtime_flow_impl/rollback.rs"]
mod rollback;

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateRequest;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse;

use crate::browser_policy_runtime::BrowserPolicyRuntime;
use crate::browser_policy_runtime_support::{
    BrowserPolicyPolicyId, BrowserPolicyRequestId, BrowserPolicyRevisionId,
};
use crate::browser_policy_store::BrowserPolicyStoreError;

pub(crate) async fn handle_browser_policy_update_request(
    runtime: &BrowserPolicyRuntime,
    request: BrowserPolicyUpdateRequest,
) -> BrowserPolicyUpdateResponse {
    match request {
        BrowserPolicyUpdateRequest::Get(request) => {
            get::handle_get(
                runtime,
                BrowserPolicyRequestId(request.request_id),
                BrowserPolicyPolicyId(request.policy_id),
            )
            .await
        }
        BrowserPolicyUpdateRequest::Preview(request) => {
            preview::handle_preview(BrowserPolicyRequestId(request.request_id), request.policy)
                .await
        }
        BrowserPolicyUpdateRequest::Patch(request) => {
            patch::handle_patch(
                runtime,
                BrowserPolicyRequestId(request.request_id),
                BrowserPolicyPolicyId(request.policy_id),
                BrowserPolicyRevisionId(request.base_revision_id),
                request.patches,
            )
            .await
        }
        BrowserPolicyUpdateRequest::Replace(request) => {
            replace::handle_replace(
                runtime,
                BrowserPolicyRequestId(request.request_id),
                request.base_revision_id.map(BrowserPolicyRevisionId),
                request.policy,
            )
            .await
        }
        BrowserPolicyUpdateRequest::Rollback(request) => {
            rollback::handle_rollback(
                runtime,
                BrowserPolicyRequestId(request.request_id),
                BrowserPolicyPolicyId(request.policy_id),
                BrowserPolicyRevisionId(request.target_revision_id),
            )
            .await
        }
    }
}

async fn read_state(
    runtime: &BrowserPolicyRuntime,
) -> Result<crate::browser_policy_store::BrowserPolicyStoredState, BrowserPolicyStoreError> {
    match &runtime.persistence {
        crate::browser_policy_runtime::BrowserPolicyPersistence::LocalJson(path) => {
            crate::browser_policy_store::read_browser_policy_state(path).await
        }
    }
}

async fn write_state(
    runtime: &BrowserPolicyRuntime,
    state: &crate::browser_policy_store::BrowserPolicyStoredState,
) -> Result<(), BrowserPolicyStoreError> {
    match &runtime.persistence {
        crate::browser_policy_runtime::BrowserPolicyPersistence::LocalJson(path) => {
            crate::browser_policy_store::write_browser_policy_state(path, state).await
        }
    }
}
