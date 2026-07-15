use std::path::Path as TestPath;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::BrowserPolicyValue;

use crate::browser_policy_runtime_support::BrowserPolicyPolicyId;
use crate::test_text::TestText;

pub fn default_browser_policy_id_for_test() -> BrowserPolicyPolicyId {
    BrowserPolicyPolicyId(
        ocentra_parent_agent_protocol::constants::browser_policy::POLICY_ID.to_string(),
    )
}

pub fn default_browser_policy_for_test(policy_id: BrowserPolicyPolicyId) -> BrowserPolicyValue {
    crate::browser_policy_runtime_support::default_policy(policy_id)
}

pub async fn handle_local_command_text_with_browser_policy_store_for_test(
    body: TestText,
    store_path: &TestPath,
) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text_with_browser_policy_store(
        crate::agent_service_lib::websocket::WebsocketCommandText(body.0),
        crate::agent_service_lib::websocket::WebsocketBrowserPolicyStorePath(
            store_path.to_path_buf(),
        ),
    )
    .await
}

#[cfg(test)]
mod clippy_linkage {
    use super::*;

    #[test]
    fn browser_policy_test_support_helper_is_linked() {
        let _ = handle_local_command_text_with_browser_policy_store_for_test;
    }
}
