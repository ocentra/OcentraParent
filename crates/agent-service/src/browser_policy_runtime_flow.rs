#[path = "browser_policy_runtime_flow_impl.rs"]
mod browser_policy_runtime_flow_impl;

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateRequest;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse;

use crate::browser_policy_runtime::BrowserPolicyRuntime;

pub(crate) async fn handle_browser_policy_update_request(
    runtime: &BrowserPolicyRuntime,
    request: BrowserPolicyUpdateRequest,
) -> BrowserPolicyUpdateResponse {
    browser_policy_runtime_flow_impl::handle_browser_policy_update_request(runtime, request).await
}
