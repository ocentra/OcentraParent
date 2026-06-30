use std::path::Path;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::BrowserPolicyValue;

pub fn default_browser_policy_for_test(policy_id: String) -> BrowserPolicyValue {
    crate::browser_policy_runtime_support::default_policy(policy_id)
}

pub async fn handle_local_command_text_with_browser_policy_store_for_test(
    body: &str,
    store_path: &Path,
) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text_with_browser_policy_store(
        body, store_path,
    )
    .await
}
